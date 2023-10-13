use valida_alu_u32::add::Add32Instruction;
use valida_basic::BasicMachine;
use valida_cpu::{
    BeqInstruction, BneInstruction, Imm32Instruction, JalInstruction, JalvInstruction,
    StopInstruction, Store32Instruction,
};
use valida_machine::{ExtensionField, Instruction, InstructionWord, Operands, PrimeField64};

pub fn generate_fibonacci_program<Val: PrimeField64, Challenge: ExtensionField<Val>>(
) -> Vec<InstructionWord<i32>> {
    let mut program = vec![];

    // Label locations
    let fib_bb0 = 8;
    let fib_bb0_1 = 13;
    let fib_bb0_2 = 15;
    let fib_bb0_3 = 19;
    let fib_bb0_4 = 21;

    //main:                                   ; @main
    //; %bb.0:
    //	imm32	-4(fp), 0, 0, 0, 0
    //	imm32	-8(fp), 0, 0, 0, 10
    //	sw	-16(fp), -8(fp)
    //	imm32	-20(fp), 0, 0, 0, 28
    //	jal	-28(fp), fib, -28
    //	sw	-12(fp), -24(fp)
    //	sw	4(fp), -12(fp)
    //	exit
    //...
    program.extend([
        InstructionWord {
            opcode: <Imm32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([-4, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: <Imm32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([-8, 0, 0, 0x27, 0x10]),
            //operands: Operands([-8, 0, 1, 86, 160]),
            //operands: Operands([-8, 0, 0x03, 0xD0, 0x90]),
        },
        InstructionWord {
            opcode: <Store32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([0, -16, -8, 0, 0]),
        },
        InstructionWord {
            opcode: <Imm32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([-20, 0, 0, 0, 28]),
        },
        InstructionWord {
            opcode: <JalInstruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([-28, fib_bb0, -28, 0, 0]),
        },
        InstructionWord {
            opcode: <Store32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([0, -12, -24, 0, 0]),
        },
        InstructionWord {
            opcode: <Store32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([0, 4, -12, 0, 0]),
        },
        InstructionWord {
            opcode: <StopInstruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands::default(),
        },
    ]);

    //fib:                                    ; @fib
    //; %bb.0:
    //	sw	-4(fp), 12(fp)
    //	imm32	-8(fp), 0, 0, 0, 0
    //	imm32	-12(fp), 0, 0, 0, 1
    //	imm32	-16(fp), 0, 0, 0, 0
    //	beq	.LBB0_1, 0(fp), 0(fp)
    program.extend([
        InstructionWord {
            opcode: <Store32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([0, -4, 12, 0, 0]),
        },
        InstructionWord {
            opcode: <Imm32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([-8, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: <Imm32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([-12, 0, 0, 0, 1]),
        },
        InstructionWord {
            opcode: <Imm32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([-16, 0, 0, 0, 0]),
        },
        InstructionWord {
            opcode: <BeqInstruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([fib_bb0_1, 0, 0, 0, 0]),
        },
    ]);

    //.LBB0_1:
    //	bne	.LBB0_2, -16(fp), -4(fp)
    //	beq	.LBB0_4, 0(fp), 0(fp)
    program.extend([
        InstructionWord {
            opcode: <BneInstruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([fib_bb0_2, -16, -4, 0, 0]),
        },
        InstructionWord {
            opcode: <BeqInstruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([fib_bb0_4, 0, 0, 0, 0]),
        },
    ]);

    //; %bb.2:
    //	add	-20(fp), -8(fp), -12(fp)
    //	sw	-8(fp), -12(fp)
    //	sw	-12(fp), -20(fp)
    //	beq	.LBB0_3, 0(fp), 0(fp)
    program.extend([
        InstructionWord {
            opcode: <Add32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([-20, -8, -12, 0, 0]),
        },
        InstructionWord {
            opcode: <Store32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([0, -8, -12, 0, 0]),
        },
        InstructionWord {
            opcode: <Store32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([0, -12, -20, 0, 0]),
        },
        InstructionWord {
            opcode: <BeqInstruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([fib_bb0_3, 0, 0, 0, 0]),
        },
    ]);

    //; %bb.3:
    //	addi	-16(fp), -16(fp), 1
    //	beq	.LBB0_1, 0(fp), 0(fp)
    program.extend([
        InstructionWord {
            opcode: <Add32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([-16, -16, 1, 0, 1]),
        },
        InstructionWord {
            opcode: <BeqInstruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([fib_bb0_1, 0, 0, 0, 0]),
        },
    ]);

    //.LBB0_4:
    //	sw	4(fp), -8(fp)
    //	jalv	-4(fp), 0(fp), 8(fp)
    program.extend([
        InstructionWord {
            opcode: <Store32Instruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([0, 4, -8, 0, 0]),
        },
        InstructionWord {
            opcode: <JalvInstruction as Instruction<BasicMachine<Val, Challenge>>>::OPCODE,
            operands: Operands([-4, 0, 8, 0, 0]),
        },
    ]);

    program
}
