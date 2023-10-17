use valida_assembler::assemble;
use valida_machine::{ExtensionField, InstructionWord, PrimeField64, ProgramROM};

pub fn generate_fibonacci_program<Val: PrimeField64, Challenge: ExtensionField<Val>>(
) -> Vec<InstructionWord<i32>> {
    let assembly_code = include_str!("fibonacci.val");
    let machine_code = assemble(assembly_code).unwrap();
    let program = ProgramROM::from_machine_code(&machine_code);
    program.0
}
