use clap::{Args, Parser, Subcommand, ValueEnum};
use std::time::Instant;
use valida_basic::BasicMachine;
use valida_benchmark_programs::{
    fibonacci::generate_fibonacci_program, sha256::generate_sha256_program,
    sudoku::generate_sudoku_program,
};
use valida_cpu::MachineWithCpuChip;
use valida_machine::config::{StarkConfig, StarkConfigImpl};
use valida_machine::{Machine, PrimeField64, ProgramROM};
use valida_program::MachineWithProgramChip;

use p3_baby_bear::BabyBear;
use p3_challenger::DuplexChallenger;
use p3_dft::Radix2Bowers;
use p3_field::{AbstractExtensionField, ExtensionField, PackedField, PrimeField32, TwoAdicField};
use p3_fri::{FriBasedPcs, FriConfigImpl, FriLdt};
use p3_keccak::Keccak256Hash;
use p3_mds::coset_mds::CosetMds;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_poseidon::Poseidon;
use p3_symmetric::compression::CompressionFunctionFromHasher;
use p3_symmetric::hasher::SerializingHasher32;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::thread_rng;
use tracing_forest::ForestLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Registry};

#[derive(Parser)]
struct Cli {
    #[arg(value_enum, default_value = "fibonacci")]
    program: ExampleProgram,

    #[command(subcommand)]
    prover_type: ProverType,

    /// Stack height (which is also the initial frame pointer value)
    #[arg(long, default_value = "16777216")]
    stack_height: usize,
}

// TODO: Handle non-unit variants with Clap macro
#[derive(ValueEnum, Clone, Debug)]
enum ExampleProgram {
    Fibonacci, // (u32),
    Sha256,    // (Vec<u8>),
    Sudoku,    // ([u8; 81]),
}

#[derive(Subcommand, Debug)]
enum ProverType {
    Univariate(UniOptions),
    Multivariate(MultiOptions),
}

#[derive(Args, Clone, Copy, Debug)]
struct UniOptions {
    #[arg(value_enum, default_value = "baby-bear")]
    base: BaseField,
    #[arg(value_enum, default_value = "baby-bear")]
    extension: ExtField,
    #[arg(value_enum, default_value = "keccak")]
    hash: HashFunction,
    #[arg(default_value = "40")]
    num_fri_queries: usize,
}

#[derive(Args, Debug)]
struct MultiOptions {
    base: BaseField,
    extension: ExtField,
    hash: HashFunction,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum BaseField {
    BabyBear,
    Mersenne31,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
enum ExtField {
    BabyBear,
    Mersenne31,
}

// TODO: Add options to enum variants
#[derive(ValueEnum, Clone, Copy, Debug)]
enum HashFunction {
    Poseidon,
    Keccak,
}

// Univariate types
type MyDft = Radix2Bowers;
type FriPCS<MyFriConfig, MyMmcs, Chal> = FriBasedPcs<MyFriConfig, MyMmcs, MyDft, Chal>;
type MyMmcs<Val, MyHash, MyCompress> = MerkleTreeMmcs<Val, [Val; 8], MyHash, MyCompress>;
type Mds16<Val> = CosetMds<Val, 16>;
type Perm16<Val> = Poseidon<Val, Mds16<Val>, 16, 5>;
type MyHash<Val> = SerializingHasher32<Val, Keccak256Hash>;
type MyCompress<Val, MyHash> = CompressionFunctionFromHasher<Val, MyHash, 2, 8>;

fn new_uni_stark_config<
    Val: PrimeField32 + TwoAdicField,
    Dom: ExtensionField<Val> + TwoAdicField,
    PackedDom: PackedField<Scalar = Dom>,
    Challenge: ExtensionField<Val> + ExtensionField<Dom> + TwoAdicField,
    PackedChallenge: PackedField<Scalar = Challenge> + AbstractExtensionField<PackedDom>,
>(
    options: &UniOptions,
) -> impl StarkConfig<Val = Val>
where
    Standard: Distribution<Val>,
{
    // TODO: Pass these in as arguments
    let hash = MyHash::new(Keccak256Hash {});
    let compress = MyCompress::new(hash);

    let mmcs = MyMmcs::new(hash, compress);
    let fri_config = FriConfigImpl::new(options.num_fri_queries, mmcs.clone());
    let ldt = FriLdt { config: fri_config };
    let dft = MyDft::default();
    let pcs = FriPCS::new(dft.clone(), 1, mmcs, ldt);
    let mds16 = Mds16::default();
    let perm16 = Perm16::new_from_rng(4, 22, mds16, &mut thread_rng()); // TODO: Use deterministic RNG
    let challenger = DuplexChallenger::new(perm16);
    StarkConfigImpl::new(pcs, dft, challenger)
}

fn bench_uni(options: &UniOptions, example_program: ExampleProgram, stack_height: usize) {
    match (options.base, options.extension, options.hash) {
        (BaseField::BabyBear, ExtField::BabyBear, HashFunction::Keccak) => {
            type Val = BabyBear;
            type Dom = BabyBear;
            type Challenge = BabyBear;
            type PackedChallenge = BabyBear;

            let config = new_uni_stark_config::<Val, Dom, Dom, Challenge, PackedChallenge>(options);
            prove_program(example_program, stack_height, config);
        }
        _ => unimplemented!(),
    };
}

fn bench_multi(_options: &MultiOptions, _example_program: ExampleProgram, _stack_height: usize) {
    unimplemented!("Multivariate proving is not currently supported");
}

fn prove_program<C: StarkConfig>(example_program: ExampleProgram, stack_height: usize, config: C)
where
    <C as StarkConfig>::Val: PrimeField64,
{
    // Generate the program ROM for the desired example program, with fixed program input
    let rom = match example_program {
        ExampleProgram::Fibonacci => generate_fibonacci_program::<C::Val, C::Challenge>(),
        ExampleProgram::Sha256 => generate_sha256_program::<C::Val, C::Challenge>(),
        ExampleProgram::Sudoku => generate_sudoku_program::<C::Val, C::Challenge>(),
    };

    // Set up the basic machine state
    let mut machine = BasicMachine::<C::Val, C::Challenge>::default();
    let rom = ProgramROM::new(rom);
    machine.program_mut().set_program_rom(&rom);
    machine.cpu_mut().fp = stack_height as u32;
    machine.cpu_mut().save_register_state();

    // Time the execution and proof generation
    let start = Instant::now();
    machine.run(&rom);
    machine.prove(&config);
    let duration = start.elapsed();
    println!("Time elapsed in milliseconds: {:?}", duration.as_millis());
}

fn main() {
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(ForestLayer::default())
        .init();

    let args = Cli::parse();

    println!("Benchmarking with the following parameters:");
    println!("Program: {:?}", args.program);
    println!("Prover type: {:?}", args.prover_type);
    println!("Stack height: {:?}", args.stack_height);

    match args.prover_type {
        ProverType::Univariate(options) => {
            bench_uni(&options, args.program, args.stack_height);
        }
        ProverType::Multivariate(options) => {
            bench_multi(&options, args.program, args.stack_height);
        }
    }
}
