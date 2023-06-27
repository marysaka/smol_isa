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
    value: u16, // TODO: Either u8 or u16
    register: Register,
}

impl RegisterValue {
    fn new(value: u16, register: Register) -> Self {
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
            0b0000 => RegisterValue::new(self.registers.r0 as u16, Register::R0),
            0b0001 => RegisterValue::new(self.registers.r1 as u16, Register::R1),
            0b0010 => RegisterValue::new(self.registers.r2 as u16, Register::R2),
            0b0011 => RegisterValue::new(self.registers.r3 as u16, Register::R3),
            0b0100 => RegisterValue::new(self.registers.r4 as u16, Register::R4),
            0b0101 => RegisterValue::new(self.registers.r5 as u16, Register::R5),
            0b0110 => RegisterValue::new(self.registers.r6 as u16, Register::R6),
            0b0111 => RegisterValue::new(self.registers.r7 as u16, Register::R7),
            0b1000 => todo!("What register is 0b1000?"),
            0b1001 => RegisterValue::new(self.registers.l0, Register::L0),
            0b1010 => RegisterValue::new(self.registers.l1, Register::L1),
            0b1011 => RegisterValue::new(self.registers.ic, Register::Ic),
            0b1100 => RegisterValue::new(self.registers.fg, Register::Fg),
            0b1101 => RegisterValue::new(self.registers.cr, Register::Cr),
            0b1110 => RegisterValue::new(self.registers.sp, Register::Sp),
            0b1111 => RegisterValue::new(self.registers.zr, Register::Zr),
            _ => unreachable!("Tried to access nonexsisting register {reg}"),
        }
    }

    fn register_save(&mut self, reg: RegisterValue) {
        match reg.register {
            Register::R0 => self.registers.r0 = reg.value as u8,
            Register::R1 => self.registers.r1 = reg.value as u8,
            Register::R2 => self.registers.r2 = reg.value as u8,
            Register::R3 => self.registers.r3 = reg.value as u8,
            Register::R4 => self.registers.r4 = reg.value as u8,
            Register::R5 => self.registers.r5 = reg.value as u8,
            Register::R6 => self.registers.r6 = reg.value as u8,
            Register::R7 => self.registers.r7 = reg.value as u8,
            Register::L0 => self.registers.l0 = reg.value,
            Register::L1 => self.registers.l1 = reg.value,
            Register::Ic => self.registers.ic = reg.value,
            Register::Fg => self.registers.fg = reg.value,
            Register::Cr => self.registers.cr = reg.value,
            Register::Sp => self.registers.sp = reg.value,
            Register::Zr => self.registers.zr = reg.value,
        }
    }

    fn decode_registers(&self, regs: u8) -> (RegisterValue, RegisterValue) {
        let r0 = regs & 0b1111;
        let r1 = (regs >> 4) & 0b1111;
        (self.register_val(r0), self.register_val(r1))
    }

    fn decode_alu_instr(&mut self, instr: u8) {
        // TODO: function if Increment/Decrement
        let mut source_vals = match instr & 0b100000 {
            0b000000 => {
                let regs = self.instructions.get(self.registers.ic + 1);
                self.decode_registers(regs)
            }
            0b100000 => todo!("Impl Immediate ALUSource"),
            _ => unreachable!(),
        };

        match (instr >> 2) & 0b111 {
            // Add
            0b000 => {
                source_vals.0.value += source_vals.1.value;
                self.register_save(source_vals.0);
            }
            // Since we use and (&) we limit ourself to values 0-3
            _ => unimplemented!("Only Add AluFamily is implemnted"),
        }
    }

    fn decode_next_instr(&mut self) {
        let instr = self.instructions.get(self.registers.ic);

        match (instr >> 6) & 0b11 {
            0b00 => {
                self.decode_alu_instr(instr);
                self.registers.ic += 2;
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
