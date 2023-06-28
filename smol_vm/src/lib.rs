use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub,
    SubAssign,
};

#[derive(Debug, Clone, Copy)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

type RegEither = Either<u8, u16>;

impl RegEither {
    fn as_u8(self) -> u8 {
        match self {
            Self::Left(v) => v,
            Self::Right(v) => v as u8,
        }
    }

    fn as_u16(self) -> u16 {
        match self {
            Self::Left(v) => v as u16,
            Self::Right(v) => v,
        }
    }
}

impl From<u8> for RegEither {
    fn from(value: u8) -> Self {
        RegEither::Left(value)
    }
}

impl From<u16> for RegEither {
    fn from(value: u16) -> Self {
        RegEither::Right(value)
    }
}

macro_rules! either_oper {
    ($lvalue:expr, $rvalue:expr, $op:tt) => {
        match $lvalue {
            Either::Left(value) => Either::Left(value $op $rvalue.as_u8()),
            Either::Right(value) => Either::Right(value $op $rvalue.as_u16()),
        }
    };
}

impl Add for RegEither {
    type Output = RegEither;

    fn add(self, rhs: Self) -> Self::Output {
        either_oper!(self, rhs, +)
    }
}

impl Sub for RegEither {
    type Output = RegEither;

    fn sub(self, rhs: Self) -> Self::Output {
        either_oper!(self, rhs, -)
    }
}

impl BitAnd for RegEither {
    type Output = RegEither;

    fn bitand(self, rhs: Self) -> Self::Output {
        either_oper!(self, rhs, &)
    }
}

impl BitOr for RegEither {
    type Output = RegEither;

    fn bitor(self, rhs: Self) -> Self::Output {
        either_oper!(self, rhs, |)
    }
}

impl BitXor for RegEither {
    type Output = RegEither;

    fn bitxor(self, rhs: Self) -> Self::Output {
        either_oper!(self, rhs, ^)
    }
}

impl Not for RegEither {
    type Output = RegEither;

    fn not(self) -> Self::Output {
        match self {
            Either::Left(value) => Either::Left(!value),
            Either::Right(value) => Either::Right(!value),
        }
    }
}

impl AddAssign for RegEither {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl SubAssign for RegEither {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl BitXorAssign for RegEither {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = *self ^ rhs
    }
}

impl BitOrAssign for RegEither {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs
    }
}

impl BitAndAssign for RegEither {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = *self & rhs
    }
}

enum Register {
    Ic,
    Fg,
    Cr,
    Sp,
    Zr,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    L0,
    L1,
}

struct RegisterValue {
    value: RegEither,
    register: Register,
}

impl RegisterValue {
    fn new(value: RegEither, register: Register) -> Self {
        Self { value, register }
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Registers {
    // Special registers
    /// (16,ro) - Instruction Counter
    pub ic: u16,
    /// (16,ro) - Core Flags
    pub fg: u16,
    /// (16,rw) - Call Register
    pub cr: u16,
    /// (16,rw) - Stack pointer
    pub sp: u16,
    /// (16,rw) - Zero register (n-bits wide)
    pub zr: u16,

    // General purpose registers
    /// 0th 8-bit general
    pub r0: u8,
    /// 1st 8-bit general
    pub r1: u8,
    /// 2nd 8-bit general
    pub r2: u8,
    /// 3th 8-bit general
    pub r3: u8,
    /// 4th 8-bit general
    pub r4: u8,
    /// 5th 8-bit general
    pub r5: u8,
    /// 6th 8-bit general
    pub r6: u8,
    /// 7th 8-bit general
    pub r7: u8,
    /// 0th 16-bit general
    pub l0: u16,
    /// 1th 16-bit general
    pub l1: u16,
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Stack {
    /// Stack size of 16kib since that's what the u16 stack pointer allows.
    pub memory: [u8; u16::MAX as usize],
}

impl Default for Stack {
    fn default() -> Self {
        Self {
            memory: [0; u16::MAX as usize],
        }
    }
}

#[derive(Debug, Default)]
pub struct Instructions {
    /// Linear set of instructions.
    /// [Registers.ic] points here.
    pub instructions: Vec<u8>,
}

impl Instructions {
    pub fn size(&self) -> usize {
        self.instructions.len()
    }

    pub fn get(&self, idx: u16) -> u8 {
        self.instructions[idx as usize]
    }
}

#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct Vm {
    pub registers: Registers,
    pub stack: Stack,
    pub instructions: Instructions,
}

impl Vm {
    fn register_val(&self, reg: u8) -> RegisterValue {
        match reg {
            0b0000 => RegisterValue::new(self.registers.r0.into(), Register::R0),
            0b0001 => RegisterValue::new(self.registers.r1.into(), Register::R1),
            0b0010 => RegisterValue::new(self.registers.r2.into(), Register::R2),
            0b0011 => RegisterValue::new(self.registers.r3.into(), Register::R3),
            0b0100 => RegisterValue::new(self.registers.r4.into(), Register::R4),
            0b0101 => RegisterValue::new(self.registers.r5.into(), Register::R5),
            0b0110 => RegisterValue::new(self.registers.r6.into(), Register::R6),
            0b0111 => RegisterValue::new(self.registers.r7.into(), Register::R7),
            0b1000 => todo!("What register is 0b1000?"),
            0b1001 => RegisterValue::new(self.registers.l0.into(), Register::L0),
            0b1010 => RegisterValue::new(self.registers.l1.into(), Register::L1),
            0b1011 => RegisterValue::new(self.registers.ic.into(), Register::Ic),
            0b1100 => RegisterValue::new(self.registers.fg.into(), Register::Fg),
            0b1101 => RegisterValue::new(self.registers.cr.into(), Register::Cr),
            0b1110 => RegisterValue::new(self.registers.sp.into(), Register::Sp),
            0b1111 => RegisterValue::new(self.registers.zr.into(), Register::Zr),
            _ => unreachable!("Tried to access nonexsisting register {reg}"),
        }
    }

    fn register_save(&mut self, reg: RegisterValue) {
        match reg.register {
            Register::R0 => self.registers.r0 = reg.value.as_u8(),
            Register::R1 => self.registers.r1 = reg.value.as_u8(),
            Register::R2 => self.registers.r2 = reg.value.as_u8(),
            Register::R3 => self.registers.r3 = reg.value.as_u8(),
            Register::R4 => self.registers.r4 = reg.value.as_u8(),
            Register::R5 => self.registers.r5 = reg.value.as_u8(),
            Register::R6 => self.registers.r6 = reg.value.as_u8(),
            Register::R7 => self.registers.r7 = reg.value.as_u8(),
            Register::L0 => self.registers.l0 = reg.value.as_u16(),
            Register::L1 => self.registers.l1 = reg.value.as_u16(),
            Register::Ic => self.registers.ic = reg.value.as_u16(),
            Register::Fg => self.registers.fg = reg.value.as_u16(),
            Register::Cr => self.registers.cr = reg.value.as_u16(),
            Register::Sp => self.registers.sp = reg.value.as_u16(),
            Register::Zr => self.registers.zr = reg.value.as_u16(),
        }
    }

    fn decode_registers(&self, regs: u8) -> (RegisterValue, RegisterValue) {
        let r0 = regs & 0b1111;
        let r1 = (regs >> 4) & 0b1111;
        (self.register_val(r0), self.register_val(r1))
    }

    fn decode_alu_instr(&mut self, instr: u8) -> u16 {
        let (used, source_vals) = match instr & 0b100 {
            0b000 => {
                let regs = self.instructions.get(self.registers.ic + 1);
                (2, self.decode_registers(regs))
            }
            // TODO: Do something less hacky
            0b100 if (instr >> 3) & 0b111 == 0b111 => {
                let regs = self.instructions.get(self.registers.ic + 1);
                (2, self.decode_registers(regs))
            }
            0b100 => {
                // TODO: Don't hackily ignore the second encoded register
                let regs = self.instructions.get(self.registers.ic + 1);
                let value = self.instructions.get(self.registers.ic + 2);
                let mut regs = self.decode_registers(regs);
                regs.1.value = value.into();
                (3, regs)
            }
            _ => unreachable!(),
        };

        let mut source_vals = source_vals;

        match (instr >> 3) & 0b111 {
            // Add
            0b000 => source_vals.0.value += source_vals.1.value,
            // Subtract
            0b001 => source_vals.0.value -= source_vals.1.value,
            // Binary and
            0b010 => source_vals.0.value &= source_vals.1.value,
            // Binary or
            0b011 => source_vals.0.value |= source_vals.1.value,
            // Binary xor
            0b100 => source_vals.0.value ^= source_vals.1.value,
            // Binary not
            0b101 => source_vals.0.value = !source_vals.0.value,
            // Equality
            0b110 => {
                // TODO: implement this with branching
                unimplemented!("ALUEquality is not implemented")
            }
            0b111 => {
                // Decode the increment/decrement function
                source_vals.0.value = match instr & 0b100 {
                    // Increment
                    0b000 => source_vals.0.value + (1_u8).into(),
                    // Decrement
                    0b100 => source_vals.0.value - (1_u8).into(),
                    _ => unreachable!("This statement should be literally impossible"),
                }
            }

            // Since we use and (&) we limit ourself to values 0-3
            _ => unimplemented!("Only Add AluFamily is implemnted"),
        }
        self.register_save(source_vals.0);

        used
    }

    fn decode_next_instr(&mut self) {
        let instr = self.instructions.get(self.registers.ic);

        match (instr >> 6) & 0b11 {
            0b00 => {
                let used = self.decode_alu_instr(instr);
                self.registers.ic += used;
            }
            0b01 => unimplemented!("LoadStore is not implemented"),
            0b10 => unimplemented!("StackIntr is not implemented"),
            0b11 => unimplemented!("Branch is not implemented"),
            // Since we use and (&) we limit ourself to values 0-3
            _ => unreachable!(),
        }
    }

    pub fn run(&mut self) {
        loop {
            let ic = self.registers.ic;

            // Break after the last instruction
            if ic as usize == self.instructions.size() {
                break;
            }

            if ic as usize > self.instructions.size() {
                panic!("Tried to access non-exsisitng instruction {}", ic);
            }

            self.decode_next_instr();
        }
    }
}
