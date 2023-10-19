use clap::{Args, Parser, Subcommand, ValueEnum};
use std::time::Instant;
use valida_basic::BasicMachine;
use valida_benchmark_programs::{
    fibonacci::generate_fibonacci_program, sha256::generate_sha256_program,
    sudoku::generate_sudoku_program,
};
use valida_cpu::MachineWithCpuChip;
use valida_machine::__internal::p3_commit::ExtensionMmcs;
use valida_machine::config::{StarkConfig, StarkConfigImpl};
use valida_machine::{Machine, PrimeField64, ProgramROM};
use valida_program::MachineWithProgramChip;

use p3_baby_bear::BabyBear;
use p3_challenger::DuplexChallenger;
use p3_dft::Radix2Bowers;
use p3_field::extension::BinomialExtensionField;
use p3_field::Field;
use p3_fri::{FriBasedPcs, FriConfigImpl, FriLdt};
use p3_keccak::Keccak256Hash;
use p3_ldt::QuotientMmcs;
use p3_mds::coset_mds::CosetMds;
use p3_merkle_tree::FieldMerkleTreeMmcs;
use p3_poseidon::Poseidon;
use p3_symmetric::CompressionFunctionFromHasher;
use p3_symmetric::SerializingHasher32;
use rand::thread_rng;
use tracing_forest::ForestLayer;
use tracing_subscriber::filter::LevelFilter;
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
    #[arg(value_enum, default_value = "baby-bear-quintic")]
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
    BabyBearQuartic,
    Mersenne31,
}

// TODO: Add options to enum variants
#[derive(ValueEnum, Clone, Copy, Debug)]
enum HashFunction {
    Poseidon,
    Keccak,
}

fn new_baby_bear_keccak_config(options: &UniOptions) -> impl StarkConfig<Val = BabyBear> {
    type Val = BabyBear;
    type Domain = Val;
    type Challenge = BinomialExtensionField<Val, 4>;
    type PackedChallenge = BinomialExtensionField<<Domain as Field>::Packing, 4>;

    type Mds16 = CosetMds<Val, 16>;
    let mds16 = Mds16::default();

    type Perm16 = Poseidon<Val, Mds16, 16, 5>;
    let perm16 = Perm16::new_from_rng(4, 22, mds16, &mut thread_rng()); // TODO: Use deterministic RNG

    type MyHash = SerializingHasher32<Keccak256Hash>;
    let hash = MyHash::new(Keccak256Hash {});

    type MyCompress = CompressionFunctionFromHasher<Val, MyHash, 2, 8>;
    let compress = MyCompress::new(hash);

    type ValMmcs = FieldMerkleTreeMmcs<Val, MyHash, MyCompress, 8>;
    let val_mmcs = ValMmcs::new(hash, compress);

    type ChallengeMmcs = ExtensionMmcs<Val, Challenge, ValMmcs>;
    let challenge_mmcs = ChallengeMmcs::new(val_mmcs.clone());

    type Dft = Radix2Bowers;
    let dft = Dft::default();

    type Challenger = DuplexChallenger<Val, Perm16, 16>;

    type Quotient = QuotientMmcs<Domain, Challenge, ValMmcs>;
    type MyFriConfig = FriConfigImpl<Val, Domain, Challenge, Quotient, ChallengeMmcs, Challenger>;
    let fri_config = MyFriConfig::new(options.num_fri_queries, challenge_mmcs);
    let ldt = FriLdt { config: fri_config };

    type Pcs = FriBasedPcs<MyFriConfig, ValMmcs, Dft, Challenger>;
    type MyConfig = StarkConfigImpl<Val, Domain, Challenge, PackedChallenge, Pcs, Challenger>;

    let pcs = Pcs::new(dft, 1, val_mmcs, ldt);
    let challenger = DuplexChallenger::new(perm16);
    MyConfig::new(pcs, challenger)
}

fn bench_uni(options: &UniOptions, example_program: ExampleProgram, stack_height: usize) {
    match (options.base, options.extension, options.hash) {
        (BaseField::BabyBear, ExtField::BabyBearQuartic, HashFunction::Keccak) => {
            let config = new_baby_bear_keccak_config(options);
            prove_program(example_program, stack_height, config);
        }
        _ => panic!("That set of options is not currently supported"),
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
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    Registry::default()
        .with(env_filter)
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
