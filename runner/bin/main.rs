use clap::{Args, Parser, Subcommand, ValueEnum};
use std::time::Instant;
use valida_basic::BasicMachine;
use valida_benchmark_programs::{
    fibonacci::generate_fibonacci_program, sha256::generate_sha256_program,
    sudoku::generate_sudoku_program,
};
use valida_cpu::MachineWithCpuChip;
use valida_machine::config::{StarkConfig, StarkConfigImpl};
use valida_machine::{Machine, ProgramROM};
use valida_program::MachineWithProgramChip;

use p3_baby_bear::BabyBear;
use p3_challenger::DuplexChallenger;
use p3_dft::Radix2Bowers;
use p3_field::{
    AbstractExtensionField, ExtensionField, Field, PackedField, PrimeField32, TwoAdicField,
};
use p3_fri::{FriBasedPcs, FriConfigImpl, FriLdt};
use p3_keccak::Keccak256Hash;
use p3_ldt::QuotientMmcs;
use p3_mds::coset_mds::CosetMds;
use p3_merkle_tree::MerkleTreeMmcs;
use p3_poseidon::Poseidon;
use p3_symmetric::compression::CompressionFunctionFromHasher;
use p3_symmetric::hasher::SerializingHasher32;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::thread_rng;

#[derive(Parser)]
struct Cli {
    #[arg(value_enum)]
    program: ExampleProgram,

    #[command(subcommand)]
    prover_type: ProverType,

    /// Stack height (which is also the initial frame pointer value)
    #[arg(long, default_value = "16777216")]
    stack_height: u32,
}

// TODO: Handle non-unit variants with Clap macro
#[derive(ValueEnum, Clone)]
enum ExampleProgram {
    Fibonacci(u32),
    Sha256(Vec<u8>),
    Sudoku([u8; 81]),
}

#[derive(Subcommand)]
enum ProverType {
    Univariate(UniOptions),
    Multivariate(MultiOptions),
}

#[derive(Args)]
struct UniOptions {
    #[arg(value_enum)]
    base: BaseField,
    #[arg(value_enum)]
    extension: ExtField,
    #[arg(value_enum)]
    hash: HashFunction,
    num_fri_queries: usize,
}

#[derive(Args)]
struct MultiOptions {
    base: BaseField,
    extension: ExtField,
    hash: HashFunction,
}

#[derive(ValueEnum, Clone, Debug)]
enum BaseField {
    BabyBear,
    Mersenne31,
}

#[derive(ValueEnum, Clone, Debug)]
enum ExtField {
    BabyBear,
    Mersenne31,
}

// TODO: Add options to enum variants
#[derive(ValueEnum, Clone, Debug)]
enum HashFunction {
    Poseidon,
    Keccak,
}

// Univariate types
type MyDft = Radix2Bowers;
type FriPCS<MyFriConfig, MyMmcs, Chal> = FriBasedPcs<MyFriConfig, MyMmcs, MyDft, Chal>;
type MyFriConfig<Val, Dom, Challenge, Quotient, MyMmcs, Chal> =
    FriConfigImpl<Val, Dom, Challenge, Quotient, MyMmcs, Chal>;
type Chal<Val> = DuplexChallenger<Val, Perm16<Val>, 16>;
type MyMmcs<Val, MyHash, MyCompress> = MerkleTreeMmcs<Val, [Val; 8], MyHash, MyCompress>;
type Quotient<Dom, Challenge, MyMmcs> = QuotientMmcs<Dom, Challenge, MyMmcs>;
type Mds16<Val> = CosetMds<Val, 16>;
type Perm16<Val> = Poseidon<Val, Mds16<Val>, 16, 5>;
type MyHash<Val> = SerializingHasher32<Val, Keccak256Hash>;
type MyCompress<Val, MyHash> = CompressionFunctionFromHasher<Val, MyHash, 2, 8>;

fn new_uni_stark_config<
    Val: Field + PrimeField32 + TwoAdicField,
    Dom: ExtensionField<Val> + TwoAdicField,
    PackedDom: PackedField,
    Challenge: PackedField + ExtensionField<Dom> + ExtensionField<Val> + TwoAdicField,
    PackedChallenge: PackedField + AbstractExtensionField<PackedDom>,
>(
    options: UniOptions,
) -> impl StarkConfig
    where Standard: Distribution<Val>
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
    let config = StarkConfigImpl::new(pcs, dft, challenger);

    config
}

fn bench_uni(options: UniOptions, example_program: ExampleProgram, stack_height: usize) {
    match (options.base, options.extension, options.hash) {
        (BaseField::BabyBear, ExtField::BabyBear, HashFunction::Keccak) => {
            type Val = BabyBear;
            type Dom = BabyBear;
            type Challenge = BabyBear;
            type PackedChallenge = BabyBear;
            type MyHash = SerializingHasher32<Val, Keccak256Hash>;
            type MyCompress = CompressionFunctionFromHasher<Val, MyHash, 2, 8>;

            let config = new_uni_stark_config::<Val, Dom, Challenge, PackedChallenge>(options);
            prove_program(example_program, stack_height, config);
        }
        _ => unimplemented!(),
    };
}

fn bench_multi(options: MultiOptions, example_program: ExampleProgram, stack_height: usize) {
    unimplemented!()
}

fn prove_program<C: StarkConfig>(example_program: ExampleProgram, stack_height: u32, config: C) {
    // Generate the program ROM for the desired example program, with fixed program input
    let rom = match example_program {
        ExampleProgram::Fibonacci(number) => generate_fibonacci_program(),
        ExampleProgram::Sha256(input) => generate_sha256_program(),
        ExampleProgram::Sudoku(puzzle) => generate_sudoku_program(),
        _ => unimplemented!(),
    };

    let mut machine = BasicMachine::default();
    let rom = ProgramROM::new(rom);
    machine.program_mut().set_program_rom(&rom);
    machine.cpu_mut().fp = stack_height;
    machine.cpu_mut().save_register_state();

    let start = Instant::now();
    machine.run(&rom);
    machine.prove(&config);
    let duration = start.elapsed();
    println!("Time elapsed in milliseconds: {:?}", duration.as_millis());
}

fn main() {
    let args = Cli::parse();

    match args.prover_type {
        ProverType::Univariate(options) => {
            bench_uni(options, args.program, 10);
        }
        ProverType::Multivariate(options) => {
            let example_program = ExampleProgram::Fibonacci(10);
            bench_multi(options, args.program, 10);
        }
    }
}
