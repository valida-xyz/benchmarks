pub mod fibonacci;
pub mod sha256;
pub mod sudoku;

use seq_macro::seq;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use valida_machine::{ExtensionField, InstructionWord, Operands, PrimeField64};
use valida_opcodes::*;

pub fn generate_sha256_program<Val: PrimeField64, Challenge: ExtensionField<Val>>(
) -> Vec<InstructionWord<i32>> {
    let instructions = parse_ll("programs/src/sha256/sha256.ll");
    instructions
}

#[derive(Clone, Debug)]
struct Instruction {
    opcode: u32,
    operands: Vec<Operand>,
}

struct Stack {
    pub inner: HashMap<String, i32>,
    pub fp: i32,
}

impl Stack {
    fn new() -> Self {
        Stack {
            inner: HashMap::new(),
            fp: 0,
        }
    }

    fn insert(&mut self, name: String, offset: Option<i32>) {
        // If name already exists, do nothing
        if self.inner.contains_key(&name) {
            return;
        }
        if let Some(offset) = offset {
            self.inner.insert(name, offset);
        } else {
            self.inner.insert(name, self.fp);
            self.fp += 4;
        }
    }

    fn get(&self, name: &str) -> i32 {
        *self.inner.get(name).unwrap()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Operand {
    Register(String),
    Immediate(u32),
}

impl Operand {
    fn reg(&self) -> String {
        match self {
            Operand::Register(s) => s.clone(),
            _ => panic!("not a register"),
        }
    }

    fn imm(&self) -> u32 {
        match self {
            Operand::Immediate(i) => *i,
            _ => panic!("not immediate"),
        }
    }
}

// Custom instructions
const GETELEMENTPTR: u32 = 400;
const ZEXT: u32 = 401;
const FSHL: u32 = 402;
const BSWAP: u32 = 403;

pub fn parse_ll(path: &str) -> Vec<InstructionWord<i32>> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Parse LLVM IR into a vector of instructions with virtual register operands
    let mut ir_instructions = Vec::new();
    let mut arguments = Vec::new();
    let mut lines = contents.lines();
    while let Some(line) = lines.next() {
        if line.contains("define") {
            let mut tokens = line.split_whitespace();
            while let Some(token) = tokens.next() {
                if token.starts_with("%") {
                    arguments.push(token);
                }
            }
        } else if line.contains("%") {
            // Split on whitespace and on ( and )
            let mut tokens = line.split(|c| c == ' ' || c == '(' || c == ')');
            let mut opcode = None;
            let mut operands = Vec::new();
            while let Some(token) = tokens.next() {
                if token.starts_with("%") {
                    operands.push(Operand::Register(token.to_string()));
                } else if token.parse::<u32>().is_ok() {
                    if let Some(imm) = Some(token.parse::<u32>().unwrap()) {
                        operands.push(Operand::Immediate(imm));
                    }
                } else {
                    // Pattern match token
                    let matched_opcode = match token {
                        "load" => Some(LOAD32),
                        "add" => Some(ADD32),
                        "xor" => Some(XOR32),
                        "or" => Some(OR32),
                        "lshr" => Some(SHR32),          // Logical shift right
                        "shl" => Some(SHL32),           // Shift left
                        "@llvm.fshl.i32" => Some(FSHL), // Concat shift left
                        "getelementptr" => Some(GETELEMENTPTR),
                        "zext" => Some(ZEXT),
                        "fshl" => Some(FSHL),
                        "bswap" => Some(BSWAP),
                        _ => None,
                    };
                    if matched_opcode.is_some() {
                        opcode = matched_opcode;
                    }
                }
            }
            if let Some(opcode) = opcode {
                let instruction = Instruction { opcode, operands };
                ir_instructions.push(instruction);
            }
        }
    }

    let mut expanded_ir_instructions = HashMap::new();

    // Replace custom instructions with sequences of Valida instructions
    for (n, ir_inst) in ir_instructions.iter().enumerate() {
        let opcode = ir_inst.opcode;
        match opcode {
            FSHL => {
                // Shift the value to the left (value << n);
                // Shift the value to the right by (32-n) bits
                // OR the two values together

                let dst = ir_inst.operands[0].clone();
                let src1 = ir_inst.operands[1].clone();
                let src2 = ir_inst.operands[2].clone();
                let src3 = ir_inst.operands[3].clone();

                if src1 != src2 {
                    panic!("FSHL operands must be the same");
                }

                let temp1 = Operand::Register(src1.reg() + "_shl");
                let inst1 = Instruction {
                    opcode: SHL32,
                    operands: vec![temp1.clone(), src2.clone(), src3.clone()],
                };

                let temp2 = Operand::Register(src1.reg() + "_shr");
                let inst2 = Instruction {
                    opcode: SHR32,
                    operands: vec![
                        temp2.clone(),
                        src2.clone(),
                        Operand::Immediate(32 - src3.imm()),
                    ],
                };

                let inst3 = Instruction {
                    opcode: OR32,
                    operands: vec![dst, temp1, temp2],
                };

                expanded_ir_instructions.insert(n, vec![inst1, inst2, inst3]);
            }
            BSWAP => {
                // (value & 0xFF000000) >> 24;  // Extract byte A and shift it to the least significant byte
                // (value & 0x00FF0000) >> 8;   // Extract byte B and shift it right by one byte.
                // (value & 0x0000FF00) << 8;   // Extract byte C and shift it left by one byte.
                // (value & 0x000000FF) << 24;  // Extract byte D and shift it to the most significant byte

                let dst = ir_inst.operands[0].clone();
                let src1 = ir_inst.operands[1].clone();

                let temp1 = Operand::Register(src1.reg() + "_or_1");
                let temp2 = Operand::Register(src1.reg() + "_or_2");
                let temp3 = Operand::Register(src1.reg() + "_or_3");
                let temp4 = Operand::Register(src1.reg() + "_or_4");

                let temp5 = Operand::Register(src1.reg() + "_shr_24");
                let temp6 = Operand::Register(src1.reg() + "_shr_8");
                let temp7 = Operand::Register(src1.reg() + "_shl_8");
                let temp8 = Operand::Register(src1.reg() + "_shl_24");

                let temp9 = Operand::Register(src1.reg() + "_or_5");
                let temp10 = Operand::Register(src1.reg() + "_or_6");

                // Bitwise OR instructions to extract bytes
                let inst1 = Instruction {
                    opcode: OR32,
                    operands: vec![temp1.clone(), src1.clone(), Operand::Immediate(0xFF000000)],
                };
                let inst2 = Instruction {
                    opcode: OR32,
                    operands: vec![temp2.clone(), src1.clone(), Operand::Immediate(0x00FF0000)],
                };
                let inst3 = Instruction {
                    opcode: OR32,
                    operands: vec![temp3.clone(), src1.clone(), Operand::Immediate(0x0000FF00)],
                };
                let inst4 = Instruction {
                    opcode: OR32,
                    operands: vec![temp4.clone(), src1, Operand::Immediate(0x000000FF)],
                };

                // Logical shift instructions to move bytes to the correct position
                let inst5 = Instruction {
                    opcode: SHR32,
                    operands: vec![temp5.clone(), temp1, Operand::Immediate(24)],
                };
                let inst6 = Instruction {
                    opcode: SHR32,
                    operands: vec![temp6.clone(), temp2, Operand::Immediate(8)],
                };
                let inst7 = Instruction {
                    opcode: SHL32,
                    operands: vec![temp7.clone(), temp3, Operand::Immediate(8)],
                };
                let inst8 = Instruction {
                    opcode: SHL32,
                    operands: vec![temp8.clone(), temp4, Operand::Immediate(24)],
                };

                // Bitwise OR instructions to combine the results
                let inst9 = Instruction {
                    opcode: OR32,
                    operands: vec![temp9.clone(), temp5, temp6],
                };
                let inst10 = Instruction {
                    opcode: OR32,
                    operands: vec![temp10.clone(), temp9, temp7],
                };
                let inst11 = Instruction {
                    opcode: OR32,
                    operands: vec![dst, temp10, temp8],
                };

                expanded_ir_instructions.insert(
                    n,
                    vec![
                        inst1, inst2, inst3, inst4, inst5, inst6, inst7, inst8, inst9, inst10,
                        inst11,
                    ],
                );
            }

            _ => {}
        };
    }

    // Replace the original instructions with the expanded instructions
    let mut new_instructions = Vec::new();
    for (n, ir_inst) in ir_instructions.iter().enumerate() {
        if let Some(expanded_insts) = expanded_ir_instructions.get(&n) {
            for inst in expanded_insts {
                new_instructions.push(inst.clone());
            }
        } else {
            new_instructions.push(ir_inst.clone());
        }
    }
    ir_instructions = new_instructions;

    // Replace virtual registers with stack locations
    let mut stack = Stack::new();
    for (n, ir_inst) in ir_instructions.iter().enumerate() {
        match ir_inst.opcode {
            ZEXT => {
                // If opcode is ZEXT, then both registers should get assigned to the same
                // stack slot
                stack.insert(ir_inst.operands[0].reg(), None);
                stack.insert(ir_inst.operands[1].reg(), Some(stack.fp));
            }
            GETELEMENTPTR => {
                println!("GETELEMENTPTR: {:?}", ir_inst);
                // Replace the register with the stack location
                let dst = ir_inst.operands[0].reg();
                let src1 = ir_inst.operands[1].reg();
                let src3 = ir_inst.operands[2].imm();
                let offset = match src1.as_str() {
                    "%block," => 0,
                    "%st," => -8 * 4,
                    _ => panic!("Invalid GETELEMENTPTR instruction"),
                };
                stack.insert(dst, Some(-(src3 as i32) * 4 + offset));
            }
            _ => {
                for operand in ir_inst.operands.iter().cloned() {
                    match operand {
                        Operand::Register(reg) => {
                            stack.insert(reg.clone(), None);
                            if n < 100 {
                                println!("{}: {:?} {:?}", n, reg, stack.fp);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Generate Valida program instructions
    let mut program = vec![];
    for ir_inst in ir_instructions.iter() {
        // Skip ZEXT and GETELEMENTPR instructions
        if ir_inst.opcode == ZEXT || ir_inst.opcode == GETELEMENTPTR {
            continue;
        }

        let mut is_imm_op = false;

        // Lookup stack locations for operands
        let opcode = ir_inst.opcode;
        let mut operands = [0i32; 5];
        for (i, operand) in ir_inst.operands.iter().enumerate() {
            match operand {
                Operand::Register(reg) => {
                    operands[i] = -(stack.get(reg) as i32);
                }
                Operand::Immediate(imm) => {
                    operands[i] = *imm as i32;
                    is_imm_op = true;
                }
                _ => {
                    unreachable!();
                }
            }
        }

        match ir_inst.opcode {
            ADD32 | SUB32 | MUL32 | DIV32 | AND32 | OR32 | XOR32 | SHL32 | SHR32 => {
                operands[4] = 1;
            }
            _ => {}
        }

        // Reorder operands depending on the opcode
        match opcode {
            LOAD32 => {
                operands.swap(1, 2);
            }
            STORE32 => {
                operands.swap(1, 2);
                operands.swap(0, 1);
            }
            _ => {}
        };

        let inst = InstructionWord {
            opcode,
            operands: Operands(operands),
        };
        program.push(inst);
    }

    // Generate Valida program input instructions
    // ||   Address   |  Value     ||
    // || -(64+8) * 4 |  block[64] ||
    // ||     ...     |    ...     ||
    // ||  -(8+1)*4   |  block[0]  ||
    // ||   -8*4      |  st[8]     ||
    // ||     ...     |    ...     ||
    // ||   -4        |  st[0]     ||
    let mut input = vec![];
    let block = [0u8; 64];
    let st = [0u8; 8];
    for (n, byte) in block.into_iter().enumerate() {
        input.push(InstructionWord {
            opcode: IMM32,
            operands: Operands([-(n as i32) * 4 - 36, 0, 0, 0, byte as i32]),
        });
    }
    for (n, byte) in st.into_iter().enumerate() {
        input.push(InstructionWord {
            opcode: IMM32,
            operands: Operands([-(n as i32) * 4 - 4, 0, 0, 0, byte as i32]),
        });
    }
    program.splice(0..0, input.iter().cloned());

    // Print the first 100 instructions
    for inst in program.iter().take(100) {
        println!("{:?}", inst);
    }
    println!("inst: {:?}", program[72]);
    println!("Program size: {}", program.len());

    // Write the program to a file:
    let mut file = File::create("sha256.val").unwrap();
    file.write_all(b"main:").unwrap();
    for inst in program.iter() {
        let mnemonic = inst_to_mnemonic(inst);
        file.write_all(
            format!(
                "\t{}\t{}, {}, {}, {}, {}",
                mnemonic,
                inst.operands.0[0],
                inst.operands.0[1],
                inst.operands.0[2],
                inst.operands.0[3],
                inst.operands.0[4]
            )
            .as_bytes(),
        )
        .unwrap();
        //file.write_all(b"InstructionWord {").unwrap();
        //file.write_all(format!("opcode: {:?}, ", inst.opcode).as_bytes())
        //    .unwrap();
        //file.write_all(
        //    format!(
        //        "operands: Operands([{}, {}, {}, {}, {}])",
        //        inst.operands.0[0],
        //        inst.operands.0[1],
        //        inst.operands.0[2],
        //        inst.operands.0[3],
        //        inst.operands.0[4]
        //    )
        //    .as_bytes(),
        //)
        //.unwrap();
        //file.write_all(b"},\n").unwrap();
    }

    program
}

fn inst_to_mnemonic(inst: &InstructionWord<i32>) -> String {
    let mut mnemonic = match inst.opcode {
        // Core CPU
        LOAD32 => "lw",
        STORE32 => "sw",
        JAL => "jal",
        JALV => "jalv",
        BEQ => "beq",
        BNE => "bne",
        IMM32 => "imm32",
        STOP => "stop",

        // Nondeterministic input
        READ_ADVICE => "advread",
        WRITE_ADVICE => "advwrite",

        // U32 ALU
        ADD32 => "add", // Note: This assumes that "addi" and "add" have different opcodes
        SUB32 => "sub", // Similarly for other mnemonics with multiple matches
        MUL32 => "mul",
        DIV32 => "div",
        LT32 => "lt",
        SHL32 => "shl",
        SHR32 => "shr",
        AND32 => "and",
        OR32 => "or",
        XOR32 => "xor",

        // Native field
        ADD => "feadd",
        SUB => "fesub",
        MUL => "femul",

        // Output
        WRITE => "write",

        _ => panic!("Unknown opcode"),
    }
    .to_string();

    let is_imm_op = inst.operands.is_imm();
    if is_imm_op == 1 {
        mnemonic = format!("{}i", mnemonic);
    }

    mnemonic.to_string()
}

const K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}

fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

fn rotr(x: u32, n: u32) -> u32 {
    (x >> n) | (x << (32 - n))
}

fn sigma0(x: u32) -> u32 {
    rotr(x, 2) ^ rotr(x, 13) ^ rotr(x, 22)
}

fn sigma1(x: u32) -> u32 {
    rotr(x, 6) ^ rotr(x, 11) ^ rotr(x, 25)
}

fn gamma0(x: u32) -> u32 {
    rotr(x, 7) ^ rotr(x, 18) ^ (x >> 3)
}

fn gamma1(x: u32) -> u32 {
    rotr(x, 17) ^ rotr(x, 19) ^ (x >> 10)
}

#[no_mangle]
pub fn sha256_compress(block: &[u8; 64], st: &mut [u32; 8]) {
    let mut w = [0u32; 64];

    // Initialize the message schedule
    seq!(t in 0..16 {
        w[t] = u32::from_be_bytes([
            block[t * 4],
            block[t * 4 + 1],
            block[t * 4 + 2],
            block[t * 4 + 3],
        ]);
    });

    seq!(t in 16..64 {
        w[t] = gamma1(w[t - 2]) + w[t - 7] + gamma0(w[t - 15]) + w[t - 16];
    });

    let mut a = st[0];
    let mut b = st[1];
    let mut c = st[2];
    let mut d = st[3];
    let mut e = st[4];
    let mut f = st[5];
    let mut g = st[6];
    let mut h = st[7];

    seq!(t in 0..64 {
        let t1 = h + sigma1(e) + ch(e, f, g) + K[t] + w[t];
        let t2 = sigma0(a) + maj(a, b, c);
        h = g;
        g = f;
        f = e;
        e = d + t1;
        d = c;
        c = b;
        b = a;
        a = t1 + t2;
    });

    st[0] += a;
    st[1] += b;
    st[2] += c;
    st[3] += d;
    st[4] += e;
    st[5] += f;
    st[6] += g;
    st[7] += h;
}

mod tests {
    use super::*;

    // Test sha256 compression function
    #[test]
    fn test_sha256() {
        let mut st = [
            0x6a09e667u32,
            0xbb67ae85u32,
            0x3c6ef372u32,
            0xa54ff53au32,
            0x510e527fu32,
            0x9b05688cu32,
            0x1f83d9abu32,
            0x5be0cd19u32,
        ];
        let block = [0u8; 64];
        sha256_compress(&block, &mut st);
        assert_eq!(
            st,
            [
                0xba7816bdu32,
                0x8f01cfea,
                0x414140de,
                0x5dae2223,
                0xb00361a3,
                0x96177a9c,
                0xb410ff61,
                0xf20015ad
            ]
        );
    }
}
