use smol_file::{SmolFile, Storage, StorageItem};

use crate::ast::{ASTTree, Arg2, Instruction, R8Regs, Variable, I8, R8};

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

#[allow(dead_code)]
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
#[allow(dead_code)]
enum ALUSrc {
    Register,
    Immidiate,
    Incerement,
    Decrement,
}

fn compile_alu_equality(tt: ALUType, source: ALUSrc, is_16b: bool, noop: bool) -> u8 {
    #[allow(clippy::unusual_byte_groupings)]
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

fn compile_variables(vars: &Vec<Variable>) -> Storage {
    // total size in bytes!
    let mut total_size = 0;
    let mut offset = 0;
    let mut items: Vec<StorageItem> = Vec::new();
    for var in vars {
        let mut size = var.size;
        let init_data = if let Some(data) = &var.bytes {
            if data.len() != var.size as usize {
                panic!(
                    "Variable '{}' initial value's length expected to be {}, was {}",
                    var.name,
                    var.size,
                    data.len()
                );
            }

            // Set the higest bit to signal initialised data
            size |= 0x8000;
            // Save space for the variable size
            total_size += var.size;
            Some(data)
        } else {
            None
        };

        items.push(StorageItem {
            size,
            offset,
            init_data: init_data.cloned(), // TODO: get rid of clone
        });

        // 4 bytes for the two u16
        total_size += 4;
        offset += var.size;
    }

    Storage { total_size, items }
}

fn variable_offset(name: &str, ast: &ASTTree, storage: &Storage) -> u16 {
    // Variable and storage items are handeled in order so they have the same indexes
    let idx = ast
        .variables
        .iter()
        .position(|v| v.name == name)
        .unwrap_or_else(|| panic!("Variable '{name}' is not defined"));

    storage.items[idx].offset
}

pub fn compile_ast(ast: ASTTree) -> SmolFile {
    let storage = compile_variables(&ast.variables);

    let instructions: Vec<u8> = ast
        .instructions
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
            Instruction::Sv(name) => {
                let name = name.inner();
                let offset = variable_offset(name, &ast, &storage);
                let [li, mi] = offset.to_le_bytes();
                // Stack load variable immediate 16 bit
                [0b10101100, li, mi].into()
            }
            Instruction::Uv(_) => {
                // hardcoded UV
                [0b10110000].to_vec()
            }
            Instruction::Syscall(_) => {
                // hardcoded syscall binary
                [0b11101111].to_vec()
            }
        })
        .collect();

    SmolFile {
        storage,
        instructions,
    }
}
