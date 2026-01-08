use crate::{
    flags::FlagsRegister,
    memory::Memory,
};

#[allow(non_snake_case)]
pub struct CPU {
    // Registers
    pub A: u8,  // Accumulator
    pub F: FlagsRegister,
    pub B: u8,
    pub C: u8,
    pub D: u8,
    pub E: u8,
    pub H: u8,
    pub L: u8,
    pub SP: u16, // Stack Pointer
    pub PC: u16, // Program Counter
    
    pub halted: bool,
    pub cycles: u64,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            A: 0,
            F: FlagsRegister::init(),
            B: 0,
            C: 0,
            D: 0,
            E: 0,
            H: 0,
            L: 0,
            SP: 0xFFFE,
            PC: 0x0000,
            halted: false,
            cycles: 0,
        }
    }

    // Helper functions for 16-bit register pairs
    fn get_bc(&self) -> u16 {
        ((self.B as u16) << 8) | (self.C as u16)
    }

    fn set_bc(&mut self, value: u16) {
        self.B = (value >> 8) as u8;
        self.C = value as u8;
    }

    fn get_de(&self) -> u16 {
        ((self.D as u16) << 8) | (self.E as u16)
    }

    fn set_de(&mut self, value: u16) {
        self.D = (value >> 8) as u8;
        self.E = value as u8;
    }

    fn get_hl(&self) -> u16 {
        ((self.H as u16) << 8) | (self.L as u16)
    }

    fn set_hl(&mut self, value: u16) {
        self.H = (value >> 8) as u8;
        self.L = value as u8;
    }

    fn get_af(&self) -> u16 {
        ((self.A as u16) << 8) | (u8::from(self.F.clone()) as u16)
    }

    fn set_af(&mut self, value: u16) {
        self.A = (value >> 8) as u8;
        self.F = ((value & 0xFF) as u8).into();
    }

    pub fn step(&mut self, memory: &mut Memory) -> u8 {
        if self.halted {
            return 4;
        }

        let opcode = self.fetch_byte(memory);
        self.execute(opcode, memory)
    }

    fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let byte = memory.read_byte(self.PC);
        self.PC = self.PC.wrapping_add(1);
        byte
    }

    fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let low = self.fetch_byte(memory) as u16;
        let high = self.fetch_byte(memory) as u16;
        (high << 8) | low
    }

    fn execute(&mut self, opcode: u8, memory: &mut Memory) -> u8 {
        match opcode {
            0x00 => 4, // NOP
            0x01 => { // LD BC,d16
                let value = self.fetch_word(memory);
                self.set_bc(value);
                12
            }
            0x02 => { // LD (BC),A
                memory.write_byte(self.get_bc(), self.A);
                8
            }
            0x03 => { // INC BC
                let value = self.get_bc().wrapping_add(1);
                self.set_bc(value);
                8
            }
            0x04 => { // INC B
                self.B = self.inc(self.B);
                4
            }
            0x05 => { // DEC B
                self.B = self.dec(self.B);
                4
            }
            0x06 => { // LD B,d8
                self.B = self.fetch_byte(memory);
                8
            }
            0x0C => { // INC C
                self.C = self.inc(self.C);
                4
            }
            0x0D => { // DEC C
                self.C = self.dec(self.C);
                4
            }
            0x0E => { // LD C,d8
                self.C = self.fetch_byte(memory);
                8
            }
            0x11 => { // LD DE,d16
                let value = self.fetch_word(memory);
                self.set_de(value);
                12
            }
            0x13 => { // INC DE
                let value = self.get_de().wrapping_add(1);
                self.set_de(value);
                8
            }
            0x15 => { // DEC D
                self.D = self.dec(self.D);
                4
            }
            0x16 => { // LD D,d8
                self.D = self.fetch_byte(memory);
                8
            }
            0x17 => { // RLA
                self.rla();
                4
            }
            0x18 => { // JR r8
                let offset = self.fetch_byte(memory) as i8;
                self.PC = self.PC.wrapping_add(offset as u16);
                12
            }
            0x1A => { // LD A,(DE)
                self.A = memory.read_byte(self.get_de());
                8
            }
            0x1E => { // LD E,d8
                self.E = self.fetch_byte(memory);
                8
            }
            0x20 => { // JR NZ,r8
                let offset = self.fetch_byte(memory) as i8;
                if !self.F.zero {
                    self.PC = self.PC.wrapping_add(offset as u16);
                    12
                } else {
                    8
                }
            }
            0x21 => { // LD HL,d16
                let value = self.fetch_word(memory);
                self.set_hl(value);
                12
            }
            0x22 => { // LD (HL+),A
                memory.write_byte(self.get_hl(), self.A);
                self.set_hl(self.get_hl().wrapping_add(1));
                8
            }
            0x23 => { // INC HL
                let value = self.get_hl().wrapping_add(1);
                self.set_hl(value);
                8
            }
            0x24 => { // INC H
                self.H = self.inc(self.H);
                4
            }
            0x28 => { // JR Z,r8
                let offset = self.fetch_byte(memory) as i8;
                if self.F.zero {
                    self.PC = self.PC.wrapping_add(offset as u16);
                    12
                } else {
                    8
                }
            }
            0x31 => { // LD SP,d16
                self.SP = self.fetch_word(memory);
                12
            }
            0x32 => { // LD (HL-),A
                memory.write_byte(self.get_hl(), self.A);
                self.set_hl(self.get_hl().wrapping_sub(1));
                8
            }
            0x3D => { // DEC A
                self.A = self.dec(self.A);
                4
            }
            0x3E => { // LD A,d8
                self.A = self.fetch_byte(memory);
                8
            }
            0x4F => { // LD C,A
                self.C = self.A;
                4
            }
            0x57 => { // LD D,A
                self.D = self.A;
                4
            }
            0x67 => { // LD H,A
                self.H = self.A;
                4
            }
            0x77 => { // LD (HL),A
                memory.write_byte(self.get_hl(), self.A);
                8
            }
            0x78 => { // LD A,B
                self.A = self.B;
                4
            }
            0x7B => { // LD A,E
                self.A = self.E;
                4
            }
            0x7C => { // LD A,H
                self.A = self.H;
                4
            }
            0x7D => { // LD A,L
                self.A = self.L;
                4
            }
            0xAF => { // XOR A
                self.A = 0;
                self.F.zero = true;
                self.F.negative = false;
                self.F.half_carry = false;
                self.F.carry = false;
                4
            }
            0xC1 => { // POP BC
                let value = self.pop_stack(memory);
                self.set_bc(value);
                12
            }
            0xC5 => { // PUSH BC
                self.push_stack(memory, self.get_bc());
                16
            }
            0xC9 => { // RET
                self.PC = self.pop_stack(memory);
                16
            }
            0xCD => { // CALL a16
                let addr = self.fetch_word(memory);
                self.push_stack(memory, self.PC);
                self.PC = addr;
                24
            }
            0xE0 => { // LDH (a8),A
                let offset = self.fetch_byte(memory) as u16;
                memory.write_byte(0xFF00 + offset, self.A);
                12
            }
            0xE2 => { // LD (C),A
                memory.write_byte(0xFF00 + self.C as u16, self.A);
                8
            }
            0xEA => { // LD (a16),A
                let addr = self.fetch_word(memory);
                memory.write_byte(addr, self.A);
                16
            }
            0xF0 => { // LDH A,(a8)
                let offset = self.fetch_byte(memory) as u16;
                self.A = memory.read_byte(0xFF00 + offset);
                12
            }
            0xF3 => { // DI (Disable Interrupts)
                // Interrupt handling would go here
                4
            }
            0xFE => { // CP d8
                let value = self.fetch_byte(memory);
                self.cp(value);
                8
            }
            0xCB => { // CB prefix
                let cb_opcode = self.fetch_byte(memory);
                self.execute_cb(cb_opcode)
            }
            _ => {
                // Unimplemented opcode, treat as NOP
                4
            }
        }
    }

    fn execute_cb(&mut self, opcode: u8) -> u8 {
        match opcode {
            0x11 => { // RL C
                self.C = self.rl(self.C);
                8
            }
            0x7C => { // BIT 7,H
                self.bit(7, self.H);
                8
            }
            _ => 8,
        }
    }

    fn inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.F.zero = result == 0;
        self.F.negative = false;
        self.F.half_carry = (value & 0x0F) == 0x0F;
        result
    }

    fn dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.F.zero = result == 0;
        self.F.negative = true;
        self.F.half_carry = (value & 0x0F) == 0;
        result
    }

    fn cp(&mut self, value: u8) {
        let result = self.A.wrapping_sub(value);
        self.F.zero = result == 0;
        self.F.negative = true;
        self.F.half_carry = (self.A & 0x0F) < (value & 0x0F);
        self.F.carry = self.A < value;
    }

    fn rla(&mut self) {
        let carry = if self.F.carry { 1 } else { 0 };
        let new_carry = (self.A & 0x80) != 0;
        self.A = (self.A << 1) | carry;
        self.F.zero = false;
        self.F.negative = false;
        self.F.half_carry = false;
        self.F.carry = new_carry;
    }

    fn rl(&mut self, value: u8) -> u8 {
        let carry = if self.F.carry { 1 } else { 0 };
        let new_carry = (value & 0x80) != 0;
        let result = (value << 1) | carry;
        self.F.zero = result == 0;
        self.F.negative = false;
        self.F.half_carry = false;
        self.F.carry = new_carry;
        result
    }

    fn bit(&mut self, bit: u8, value: u8) {
        let result = value & (1 << bit);
        self.F.zero = result == 0;
        self.F.negative = false;
        self.F.half_carry = true;
    }

    fn push_stack(&mut self, memory: &mut Memory, value: u16) {
        self.SP = self.SP.wrapping_sub(1);
        memory.write_byte(self.SP, (value >> 8) as u8);
        self.SP = self.SP.wrapping_sub(1);
        memory.write_byte(self.SP, value as u8);
    }

    fn pop_stack(&mut self, memory: &Memory) -> u16 {
        let low = memory.read_byte(self.SP) as u16;
        self.SP = self.SP.wrapping_add(1);
        let high = memory.read_byte(self.SP) as u16;
        self.SP = self.SP.wrapping_add(1);
        (high << 8) | low
    }
}
