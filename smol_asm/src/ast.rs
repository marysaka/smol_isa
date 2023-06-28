trait Arg {
    fn args(self) -> Vec<RegType>;
    fn try_parse(input: &str) -> Result<Self, String>
    where
        Self: Sized;
}

#[derive(Debug)]
struct Arg0 {}

/* impl Arg for Arg0 {
    fn args(self) -> Vec<RegType> {
        Vec::new()
    }
} */

#[derive(Debug)]
struct Arg1<A1: Register> {
    arg1: A1,
}

/* impl<A1: Register> Arg for Arg1<A1> {
    fn args(self) -> Vec<RegType> {
        vec![self.arg1.parse()]
    }
} */

#[derive(Debug)]
pub struct Arg2<A1: Register, A2: Register> {
    arg1: A1,
    arg2: A2,
}

impl<A1: Register, A2: Register> Arg for Arg2<A1, A2> {
    fn args(self) -> Vec<RegType> {
        vec![self.arg1.parse(), self.arg2.parse()]
    }
    fn try_parse(input: &str) -> Result<Self, String> {
        input.try_into()
    }
}

impl<A1: Register, A2: Register> TryFrom<&str> for Arg2<A1, A2> {
    type Error = String;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let items: Vec<&str> = line.split_ascii_whitespace().collect();
        // This includes the operator
        if items.len() < 3 {
            return Err(format!("Exected 2 arguments got {}", items.len() - 1));
        }

        let arg1 = A1::try_parse(items[1])?;
        let arg2 = A2::try_parse(items[2])?;
        Ok(Self { arg1, arg2 })
    }
}

#[derive(Debug)]
pub enum R8Regs {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

#[derive(Debug)]
pub enum RegType {
    /// 8-bit register
    R8(R8Regs),
    /// 8-bit immediate
    I8(u8),
}

pub trait Register {
    fn parse(self) -> RegType;
    fn try_parse(input: &str) -> Result<Self, String>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct R8 {
    register: R8Regs,
}

impl TryFrom<&str> for R8 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = value.trim();

        fn fail(value: &str) -> Result<R8, String> {
            Err(format!("Expected r0-7, received {value}"))
        }

        if val.len() != 2 {
            return fail(value);
        } else if val.as_bytes()[0] != b'r' {
            return fail(value);
        }

        // TODO: handle the ascii values properly
        let reg = val.as_bytes()[1] - 48;
        if reg > 7 {
            return fail(value);
        }

        let register = match reg {
            0 => R8Regs::R0,
            1 => R8Regs::R1,
            2 => R8Regs::R2,
            3 => R8Regs::R3,
            4 => R8Regs::R4,
            5 => R8Regs::R5,
            6 => R8Regs::R6,
            7 => R8Regs::R7,
            _ => unreachable!(),
        };

        Ok(Self { register })
    }
}

impl Register for R8 {
    fn parse(self) -> RegType {
        RegType::R8(self.register)
    }

    fn try_parse(input: &str) -> Result<Self, String> {
        input.try_into()
    }
}

#[derive(Debug)]
pub struct I8 {
    value: u8,
}

impl TryFrom<&str> for I8 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = value.trim();
        let value = val
            .parse::<u8>()
            .map_err(|_| format!("Expected 0-255 value, got {val}"))?;

        Ok(Self { value })
    }
}

impl Register for I8 {
    fn parse(self) -> RegType {
        RegType::I8(self.value)
    }

    fn try_parse(input: &str) -> Result<Self, String> {
        input.try_into()
    }
}

#[derive(Debug)]
pub struct InstrLine<T> {
    instr: T,
    line: usize,
}

impl<T> InstrLine<T> {
    fn new(instr: T, line: usize) -> Self {
        Self { instr, line }
    }
}

#[derive(Debug)]
pub enum Instruction {
    Add(InstrLine<Arg2<R8, R8>>),
    AddI(InstrLine<Arg2<R8, I8>>),
}

fn parse_line(idx: usize, line: &str) -> Result<Instruction, String> {
    let instr = line.split_ascii_whitespace().next().unwrap().to_lowercase();

    if instr == "add" {
        Ok(Instruction::Add(InstrLine::new(
            Arg2::<R8, R8>::try_parse(line)?,
            idx,
        )))
    } else if instr == "addi" {
        Ok(Instruction::AddI(InstrLine::new(
            Arg2::<R8, I8>::try_parse(line)?,
            idx,
        )))
    } else {
        Err(format!(
            "On line: {}\nInstruction '{instr}' has not been implemented",
            line
        ))
    }
}

pub fn parse_source(source: &str) -> Result<Vec<Instruction>, String> {
    let lines: Vec<Instruction> = source
        .lines()
        .enumerate()
        .map(|(idx, line)| (idx, line.trim()))
        .filter(|(_, line)| !line.is_empty())
        .filter(|(_, line)| !line.starts_with('#'))
        .map(|(idx, line)| parse_line(idx, line).unwrap())
        .collect();

    Ok(lines)
}
