use valida_machine::{ExtensionField, InstructionWord, PrimeField64};

pub fn generate_sha256_program<Val: PrimeField64, Challenge: ExtensionField<Val>>(
) -> Vec<InstructionWord<i32>> {
    vec![]
}
