#![no_std]

//! Smol ISA
//!
//! Handle low-level encoding/decoding of smol.

/// An error used when an encoded part is considered invalid.
#[derive(Copy, Clone, Debug)]
pub enum ParsingError<T> {
    InvalidEncoding(T),
}

/// A 8-bit register for smol.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Register8 {
    /// R0 register (8-bit).
    R0,
    /// R1 register (8-bit).
    R1,
    /// R2 register (8-bit).
    R2,
    /// R3 register (8-bit).
    R3,
    /// R4 register (8-bit).
    R4,
    /// R5 register (8-bit).
    R5,
    /// R6 register (8-bit).
    R6,
    /// R7 register (8-bit).
    R7,
}

impl From<Register8> for u8 {
    fn from(value: Register8) -> Self {
        match value {
            Register8::R0 => 0,
            Register8::R1 => 1,
            Register8::R2 => 2,
            Register8::R3 => 3,
            Register8::R4 => 4,
            Register8::R5 => 5,
            Register8::R6 => 6,
            Register8::R7 => 7,
        }
    }
}

impl TryFrom<u8> for Register8 {
    type Error = ParsingError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Register8::R0),
            1 => Ok(Register8::R1),
            2 => Ok(Register8::R2),
            3 => Ok(Register8::R3),
            4 => Ok(Register8::R4),
            5 => Ok(Register8::R5),
            6 => Ok(Register8::R6),
            7 => Ok(Register8::R7),
            // 8 is reserved (because aki forgot it)
            value => Err(ParsingError::InvalidEncoding(value)),
        }
    }
}

/// A Register for smol.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Register16 {
    /// L0 register (16-bit).
    L0,
    /// L1 register (16-bit).
    L1,
    /// IC (Instruction Counter) register (16-bit).
    IC,
    /// FG (Flags) register (16-bit).
    FG,
    /// CR (Call) register (16-bit).
    CR,
    /// SP (Stack Pointer) register (16-bit).
    SP,
    /// ZR (Zero) register (16-bit).
    ZR,
}

impl From<Register16> for u8 {
    fn from(value: Register16) -> Self {
        match value {
            Register16::L0 => 9,
            Register16::L1 => 10,
            Register16::IC => 11,
            Register16::FG => 12,
            Register16::CR => 13,
            Register16::SP => 14,
            Register16::ZR => 15,
        }
    }
}

impl TryFrom<u8> for Register16 {
    type Error = ParsingError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            9 => Ok(Register16::L0),
            10 => Ok(Register16::L1),
            11 => Ok(Register16::IC),
            12 => Ok(Register16::FG),
            13 => Ok(Register16::CR),
            14 => Ok(Register16::SP),
            15 => Ok(Register16::ZR),
            value => Err(ParsingError::InvalidEncoding(value)),
        }
    }
}

// TODO
pub enum Instruction {
    /// ALU / Equality.
    Arithmetic,
    /// Load / Store.
    LoadStore,
    /// Stack / Interrupt.
    StackInterrupt,
    /// Branch.
    Branch,
}

// TODO: Maybe merge that with Instruction actually?
/// The familly of an [Instruction].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum InstructionFamily {
    /// ALU / Equality.
    Arithmetic,
    /// Load / Store.
    LoadStore,
    /// Stack / Interrupt.
    StackInterrupt,
    /// Branch.
    Branch,
}

impl From<InstructionFamily> for u8 {
    fn from(value: InstructionFamily) -> Self {
        match value {
            InstructionFamily::Arithmetic => 0,
            InstructionFamily::LoadStore => 1,
            InstructionFamily::StackInterrupt => 2,
            InstructionFamily::Branch => 3,
        }
    }
}

impl TryFrom<u8> for InstructionFamily {
    type Error = ParsingError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(InstructionFamily::Arithmetic),
            1 => Ok(InstructionFamily::LoadStore),
            2 => Ok(InstructionFamily::StackInterrupt),
            3 => Ok(InstructionFamily::Branch),
            value => Err(ParsingError::InvalidEncoding(value)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticFamily {
    /// Addition.
    Add,
    /// Substraction.
    Sub,
    /// Binary AND.
    And,
    /// Binary OR.
    Or,
    /// Binary XOR.
    Xor,
    /// Binary NOT.
    Not,
    /// Equal.
    Eq,
    /// Increment / Decrement.
    Increment,
}

impl From<ArithmeticFamily> for u8 {
    fn from(value: ArithmeticFamily) -> Self {
        match value {
            ArithmeticFamily::Add => 0,
            ArithmeticFamily::Sub => 1,
            ArithmeticFamily::And => 2,
            ArithmeticFamily::Or => 3,
            ArithmeticFamily::Xor => 4,
            ArithmeticFamily::Not => 5,
            ArithmeticFamily::Eq => 6,
            ArithmeticFamily::Increment => 7,
        }
    }
}

impl TryFrom<u8> for ArithmeticFamily {
    type Error = ParsingError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ArithmeticFamily::Add),
            1 => Ok(ArithmeticFamily::Sub),
            2 => Ok(ArithmeticFamily::Add),
            3 => Ok(ArithmeticFamily::Or),
            4 => Ok(ArithmeticFamily::Xor),
            5 => Ok(ArithmeticFamily::Not),
            6 => Ok(ArithmeticFamily::Eq),
            7 => Ok(ArithmeticFamily::Increment),
            value => Err(ParsingError::InvalidEncoding(value)),
        }
    }
}

/// Arithmetic operation source type
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticOperationSourceType {
    /// Register.
    Register,
    /// Immediate.
    Immediate,
}

impl From<ArithmeticOperationSourceType> for u8 {
    fn from(value: ArithmeticOperationSourceType) -> Self {
        match value {
            ArithmeticOperationSourceType::Register => 0,
            ArithmeticOperationSourceType::Immediate => 1,
        }
    }
}

impl TryFrom<u8> for ArithmeticOperationSourceType {
    type Error = ParsingError<u8>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ArithmeticOperationSourceType::Register),
            1 => Ok(ArithmeticOperationSourceType::Immediate),
            value => Err(ParsingError::InvalidEncoding(value)),
        }
    }
}

pub type ArgumentRegisterEncoding8 = (Register8, Register8);
pub type ArgumentImmediateEncoding8 = (Register8, u8);
pub type ArgumentRegisterEncoding16 = (Register16, Register16);
pub type ArgumentImmediateEncoding16 = (Register16, u16);

pub enum InstructionTest {
    Add(ArgumentRegisterEncoding8),
    AddI(ArgumentImmediateEncoding8),
    Sub(ArgumentRegisterEncoding8),
    SubI(ArgumentImmediateEncoding8),
    Inc(Register8),
    Dec(Register8),
    And(ArgumentRegisterEncoding8),
    AndI(ArgumentImmediateEncoding8),
    Or(ArgumentRegisterEncoding8),
    OrI(ArgumentImmediateEncoding8),
    Xor(ArgumentRegisterEncoding8),
    XorI(ArgumentImmediateEncoding8),
    Not(Register8),
    EqR(ArgumentRegisterEncoding8),
    EqI(ArgumentImmediateEncoding8),

    AddL(ArgumentRegisterEncoding16),
    AddIL(ArgumentImmediateEncoding16),
    SubL(ArgumentRegisterEncoding16),
    SubIL(ArgumentImmediateEncoding16),
    IncL(Register16),
    DecL(Register16),
    AndL(ArgumentRegisterEncoding16),
    AndIL(ArgumentImmediateEncoding16),
    OrL(ArgumentRegisterEncoding16),
    OrIL(ArgumentImmediateEncoding16),
    XorL(ArgumentRegisterEncoding16),
    XorIL(ArgumentImmediateEncoding16),
    NotL(Register16),
    EqRL(ArgumentRegisterEncoding16),
    EqIL(ArgumentImmediateEncoding16),

    Nop,
}
