trait Arg {
    fn args(self) -> Vec<RegType>;
    fn try_parse(input: &str) -> Result<Self, String>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Arg0 {}

/* impl Arg for Arg0 {
    fn args(self) -> Vec<RegType> {
        Vec::new()
    }
} */

#[derive(Debug)]
#[allow(dead_code)]
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
    pub arg1: A1,
    pub arg2: A2,
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
    pub register: R8Regs,
}

impl TryFrom<&str> for R8 {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let val = value.trim();

        fn fail(value: &str) -> Result<R8, String> {
            Err(format!("Expected r0-7, received {value}"))
        }

        if val.len() != 2 || val.as_bytes()[0] != b'r' {
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
    pub value: u8,
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
#[allow(dead_code)]
pub struct InstrLine<T> {
    instr: T,
    line: usize,
}

impl<T> InstrLine<T> {
    fn new(instr: T, line: usize) -> Self {
        Self { instr, line }
    }

    pub fn inner(&self) -> &T {
        &self.instr
    }
}

#[derive(Debug)]
pub enum Instruction {
    Add(InstrLine<Arg2<R8, R8>>),
    AddI(InstrLine<Arg2<R8, I8>>),
    Syscall(InstrLine<Arg0>),
    Sv(InstrLine<String>),
    Uv(InstrLine<Arg0>),
}

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub size: u16,
    pub bytes: Option<Vec<u8>>,
}

#[derive(Debug)]
pub struct ASTTree {
    pub variables: Vec<Variable>,
    pub instructions: Vec<Instruction>,
}

fn parse_variable_value(value: &str) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    if value.starts_with('"') {
        let mut closed = false;
        let chars = value.as_bytes();
        let mut idx = 1; // skip first "
        while idx < chars.len() {
            let byte = chars[idx];
            // TODO: properly handle escaping
            if byte == b'\\' && chars[idx + 1] == b'n' {
                bytes.push(b'\n');
                idx += 2;
                continue;
            }

            if byte == b'"' {
                closed = true;
                break;
            }

            bytes.push(byte);
            idx += 1;
        }

        if !closed {
            panic!("Unclosed string");
        }
    } else {
        panic!("Currently only string literal variables are suppored");
    }

    bytes
}

fn parse_variable_line(line: &str) -> Variable {
    let mut items = line.split_ascii_whitespace();
    let name = items.next().unwrap();
    let size_str = items.next().expect("Variable needs a size");
    let rest: String = items.collect::<Vec<&str>>().join(" ");
    let bytes = if !rest.is_empty() {
        Some(parse_variable_value(&rest))
    } else {
        None
    };

    let size = size_str.parse::<u16>().unwrap();

    Variable {
        name: name.into(),
        size,
        bytes,
    }
}

fn parse_variables(src: &str) -> Vec<Variable> {
    let mut variables: Vec<Variable> = Vec::new();

    let mut var_start = false;
    for line in src.lines() {
        if line.starts_with("---") {
            if !var_start {
                var_start = true;
            } else if var_start {
                break;
            }
            continue;
        }

        if !var_start {
            continue;
        }

        if line.starts_with('#') {
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        let var = parse_variable_line(line);
        variables.push(var);
    }

    variables
}

fn parse_instruction_line(idx: usize, line: &str) -> Result<Instruction, String> {
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
    } else if instr == "syscall" {
        Ok(Instruction::Syscall(InstrLine::new(Arg0 {}, idx)))
    } else if instr == "uv" {
        Ok(Instruction::Uv(InstrLine::new(Arg0 {}, idx)))
    } else if instr == "sv" {
        let args: Vec<&str> = line.split_ascii_whitespace().collect();
        if args.len() < 2 {
            panic!("SV requires an argument");
        }
        Ok(Instruction::Sv(InstrLine::new(args[1].into(), idx)))
    } else {
        Err(format!(
            "On line: {idx}: Instruction '{instr}' has not been implemented"
        ))
    }
}

pub fn parse_source(source: &str) -> Result<ASTTree, String> {
    let variables = parse_variables(source);
    let var_end = if let Some(idx) = source.rfind('-') {
        idx + 1
    } else {
        0
    };

    let instructions: Vec<Instruction> = source[var_end..]
        .lines()
        .enumerate()
        .map(|(idx, line)| (idx, line.trim()))
        .filter(|(_, line)| !line.is_empty())
        .filter(|(_, line)| !line.starts_with('#'))
        .map(|(idx, line)| parse_instruction_line(idx, line).unwrap())
        .collect();

    Ok(ASTTree {
        variables,
        instructions,
    })
}
