use crate::ast::{Arg2, Instruction, R8Regs, I8, R8};

trait Compile {
    fn compile(&self) -> Vec<u8>;
}

impl Compile for Arg2<R8, R8> {
    fn compile(&self) -> Vec<u8> {
        let arg = (self.arg2.compile()[0] << 4) | self.arg1.compile()[0];
        vec![arg]
    }
}

impl Compile for Arg2<R8, I8> {
    fn compile(&self) -> Vec<u8> {
        let arg = self.arg1.compile()[0];
        vec![arg, self.arg2.compile()[0]]
    }
}

impl Compile for R8 {
    fn compile(&self) -> Vec<u8> {
        let val: u8 = match self.register {
            R8Regs::R0 => 0b0000,
            R8Regs::R1 => 0b0001,
            R8Regs::R2 => 0b0010,
            R8Regs::R3 => 0b0011,
            R8Regs::R4 => 0b0100,
            R8Regs::R5 => 0b0101,
            R8Regs::R6 => 0b0110,
            R8Regs::R7 => 0b0111,
        };
        vec![val]
    }
}

impl Compile for I8 {
    fn compile(&self) -> Vec<u8> {
        vec![self.value]
    }
}

enum ALUType {
    Add,
    Subtract,
    And,
    Or,
    Xor,
    Not,
    Equality,
    IncrDecr,
}

/// If `opcode[2:4] != 0b111`:
/// `opcode[5]` - Source
///  * `0b0` - Register
///  * `0b1` - Immediate
///
/// Otherwise:
/// `opcode[5]` - Function
///  * `0b0` - Increment
///  * `0b1` - Decrement
enum ALUSrc {
    Register,
    Immidiate,
    Incerement,
    Decrement,
}

fn compile_alu_equality(tt: ALUType, source: ALUSrc, is_16b: bool, noop: bool) -> u8 {
    let op = match tt {
        ALUType::Add => 0b00_000_0_0_0,
        ALUType::Subtract => 0b00_001_0_0_0,
        ALUType::And => 0b00_010_0_0_0,
        ALUType::Or => 0b00_011_0_0_0,
        ALUType::Xor => 0b00_100_0_0_0,
        ALUType::Not => 0b00_101_0_0_0,
        ALUType::Equality => 0b00_110_0_0_0,
        ALUType::IncrDecr => 0b00_111_0_0_0,
    };

    let mut op = match source {
        ALUSrc::Immidiate | ALUSrc::Decrement => op | 0b100,
        _ => op,
    };

    if is_16b {
        op |= 0b10;
    }

    if noop {
        op |= 0b1;
    }

    op
}

pub fn compile_instructions(instrs: Vec<Instruction>) -> Vec<u8> {
    let instrs: Vec<u8> = instrs
        .iter()
        .flat_map(|instr| match instr {
            Instruction::Add(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_alu_equality(ALUType::Add, ALUSrc::Register, false, false);
                args.insert(0, op);
                args
            }
            Instruction::AddI(instr) => {
                let mut args = instr.inner().compile();
                let op = compile_alu_equality(ALUType::Add, ALUSrc::Immidiate, false, false);
                args.insert(0, op);
                args
            }
        })
        .collect();

    instrs
}
