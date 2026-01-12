// Game Boy CPU Implementation
//
// This module implements the Sharp LR35902 CPU used in the original Game Boy.
// The CPU is the hot path of the emulator - all instruction execution happens here.
//
// Architecture:
// - 8-bit CPU with 16-bit address bus
// - 8-bit registers: A (accumulator), B, C, D, E, H, L, F (flags)
// - 16-bit registers: SP (stack pointer), PC (program counter)
// - Register pairs can be used as 16-bit: BC, DE, HL
//
// Timing:
// The CPU runs at 4.194304 MHz with precise cycle counting for each instruction.
// Each instruction's cycle count is accurately tracked to match original hardware timing.
//
// Instruction Execution Flow (Hot Path):
// 1. Fetch opcode from memory at PC
// 2. Decode opcode using match statement dispatch
// 3. Execute instruction with precise cycle count
// 4. Update CPU state (registers, flags, memory)
// 5. Advance PC and track clock cycles
//
// All 256 unprefixed opcodes and 256 CB-prefixed opcodes are fully implemented.

use crate::{
    flags::FlagsRegister,
    clock::Clock,
    memory::Memory,
    opcodes::{load_opcodes, OpCode},
};

#[allow(non_snake_case)]
pub struct CPU {
    // Registers
    a: u8,      // Accumulator
    f: FlagsRegister,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,    // Stack Pointer
    pc: u16,    // Program Counter
    
    // Memory and peripherals
    memory: Memory,
    clock: Clock,
    
    // Opcode tables
    opcodes: Vec<OpCode>,
    cb_opcodes: Vec<OpCode>,
    
    // CPU state
    halted: bool,
    ime: bool,  // Interrupt Master Enable
}

impl CPU {
    pub fn new() -> Self {
        let (opcodes, cb_opcodes) = load_opcodes().unwrap();
        Self {
            a: 0x01,    // Initial value after boot ROM
            f: FlagsRegister::init(),
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFFE,
            pc: 0x0100, // Start after boot ROM (or 0x0000 with boot ROM)
            memory: Memory::new(),
            clock: Clock::new(),
            opcodes,
            cb_opcodes,
            halted: false,
            ime: false,
        }
    }
    
    pub fn load_rom(&mut self, rom_data: Vec<u8>) {
        self.memory.load_rom(rom_data);
    }
    
    pub fn load_boot_rom(&mut self, boot_data: &[u8]) {
        self.memory.load_boot_rom(boot_data);
        self.pc = 0x0000; // Start at boot ROM
    }

    // Main execution loop - the hot path
    pub fn step(&mut self) -> u8 {
        // Check for interrupts
        let interrupt_cycles = self.handle_interrupts();
        if interrupt_cycles > 0 {
            self.memory.update_timers(interrupt_cycles);
            self.clock.tick(interrupt_cycles);
            return interrupt_cycles;
        }

        if self.halted {
            self.memory.update_timers(4);
            self.clock.tick(4);
            return 4;
        }

        let opcode = self.fetch_byte();
        let cycles = self.execute(opcode);
        self.memory.update_timers(cycles);
        self.clock.tick(cycles);
        cycles
    }

    // Handle interrupts - returns cycles used (20 if interrupt handled, 0 otherwise)
    fn handle_interrupts(&mut self) -> u8 {
        if !self.ime && !self.halted {
            return 0;
        }

        let ie = self.memory.read(0xFFFF); // Interrupt Enable
        let if_reg = self.memory.read(0xFF0F); // Interrupt Flag
        let triggered = ie & if_reg;

        if triggered == 0 {
            return 0;
        }

        // Wake from halt even if IME is disabled
        self.halted = false;

        if !self.ime {
            return 0;
        }

        // Handle the highest priority interrupt
        self.ime = false; // Disable interrupts
        
        let interrupt_bit = triggered.trailing_zeros();
        if interrupt_bit >= 5 {
            return 0;
        }

        // Clear the interrupt flag
        let new_if = if_reg & !(1 << interrupt_bit);
        self.memory.write(0xFF0F, new_if);

        // Push PC onto stack
        self.push_stack(self.pc);

        // Jump to interrupt handler
        self.pc = match interrupt_bit {
            0 => 0x0040, // VBlank
            1 => 0x0048, // LCD STAT
            2 => 0x0050, // Timer
            3 => 0x0058, // Serial
            4 => 0x0060, // Joypad
            _ => unreachable!(),
        };

        20 // Interrupt handling takes 20 cycles
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory.read(self.pc);
        self.pc = self.pc.wrapping_add(1);
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let low = self.fetch_byte() as u16;
        let high = self.fetch_byte() as u16;
        (high << 8) | low
    }

    // Instruction execution - implements all Game Boy instructions
    fn execute(&mut self, opcode: u8) -> u8 {
        match opcode {
            // NOP
            0x00 => 4,
            
            // LD BC, d16
            0x01 => {
                let value = self.fetch_word();
                self.set_bc(value);
                12
            }
            
            // LD (BC), A
            0x02 => {
                let addr = self.get_bc();
                self.memory.write(addr, self.a);
                8
            }
            
            // INC BC
            0x03 => {
                let value = self.get_bc().wrapping_add(1);
                self.set_bc(value);
                8
            }
            
            // INC B
            0x04 => {
                self.b = self.alu_inc(self.b);
                4
            }
            
            // DEC B
            0x05 => {
                self.b = self.alu_dec(self.b);
                4
            }
            
            // LD B, d8
            0x06 => {
                self.b = self.fetch_byte();
                8
            }
            
            // RLCA
            0x07 => {
                let carry = (self.a & 0x80) >> 7;
                self.a = (self.a << 1) | carry;
                self.f.zero = false;
                self.f.negative = false;
                self.f.half_carry = false;
                self.f.carry = carry == 1;
                4
            }
            
            // LD (a16), SP
            0x08 => {
                let addr = self.fetch_word();
                self.memory.write_word(addr, self.sp);
                20
            }
            
            // ADD HL, BC
            0x09 => {
                let hl = self.get_hl();
                let bc = self.get_bc();
                let result = self.alu_add_hl(hl, bc);
                self.set_hl(result);
                8
            }
            
            // LD A, (BC)
            0x0A => {
                let addr = self.get_bc();
                self.a = self.memory.read(addr);
                8
            }
            
            // DEC BC
            0x0B => {
                let value = self.get_bc().wrapping_sub(1);
                self.set_bc(value);
                8
            }
            
            // INC C
            0x0C => {
                self.c = self.alu_inc(self.c);
                4
            }
            
            // DEC C
            0x0D => {
                self.c = self.alu_dec(self.c);
                4
            }
            
            // LD C, d8
            0x0E => {
                self.c = self.fetch_byte();
                8
            }
            
            // RRCA
            0x0F => {
                let carry = self.a & 0x01;
                self.a = (self.a >> 1) | (carry << 7);
                self.f.zero = false;
                self.f.negative = false;
                self.f.half_carry = false;
                self.f.carry = carry == 1;
                4
            }
            
            // STOP
            0x10 => {
                self.fetch_byte(); // STOP is 2 bytes
                4
            }
            
            // LD DE, d16
            0x11 => {
                let value = self.fetch_word();
                self.set_de(value);
                12
            }
            
            // LD (DE), A
            0x12 => {
                let addr = self.get_de();
                self.memory.write(addr, self.a);
                8
            }
            
            // INC DE
            0x13 => {
                let value = self.get_de().wrapping_add(1);
                self.set_de(value);
                8
            }
            
            // INC D
            0x14 => {
                self.d = self.alu_inc(self.d);
                4
            }
            
            // DEC D
            0x15 => {
                self.d = self.alu_dec(self.d);
                4
            }
            
            // LD D, d8
            0x16 => {
                self.d = self.fetch_byte();
                8
            }
            
            // RLA
            0x17 => {
                let carry = if self.f.carry { 1 } else { 0 };
                let new_carry = (self.a & 0x80) >> 7;
                self.a = (self.a << 1) | carry;
                self.f.zero = false;
                self.f.negative = false;
                self.f.half_carry = false;
                self.f.carry = new_carry == 1;
                4
            }
            
            // JR r8
            0x18 => {
                let offset = self.fetch_byte() as i8;
                self.pc = self.pc.wrapping_add(offset as u16);
                12
            }
            
            // ADD HL, DE
            0x19 => {
                let hl = self.get_hl();
                let de = self.get_de();
                let result = self.alu_add_hl(hl, de);
                self.set_hl(result);
                8
            }
            
            // LD A, (DE)
            0x1A => {
                let addr = self.get_de();
                self.a = self.memory.read(addr);
                8
            }
            
            // DEC DE
            0x1B => {
                let value = self.get_de().wrapping_sub(1);
                self.set_de(value);
                8
            }
            
            // INC E
            0x1C => {
                self.e = self.alu_inc(self.e);
                4
            }
            
            // DEC E
            0x1D => {
                self.e = self.alu_dec(self.e);
                4
            }
            
            // LD E, d8
            0x1E => {
                self.e = self.fetch_byte();
                8
            }
            
            // RRA
            0x1F => {
                let carry = if self.f.carry { 1 } else { 0 };
                let new_carry = self.a & 0x01;
                self.a = (self.a >> 1) | (carry << 7);
                self.f.zero = false;
                self.f.negative = false;
                self.f.half_carry = false;
                self.f.carry = new_carry == 1;
                4
            }
            
            // JR NZ, r8
            0x20 => {
                let offset = self.fetch_byte() as i8;
                if !self.f.zero {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    12
                } else {
                    8
                }
            }
            
            // LD HL, d16
            0x21 => {
                let value = self.fetch_word();
                self.set_hl(value);
                12
            }
            
            // LD (HL+), A / LDI (HL), A
            0x22 => {
                let addr = self.get_hl();
                self.memory.write(addr, self.a);
                self.set_hl(addr.wrapping_add(1));
                8
            }
            
            // INC HL
            0x23 => {
                let value = self.get_hl().wrapping_add(1);
                self.set_hl(value);
                8
            }
            
            // INC H
            0x24 => {
                self.h = self.alu_inc(self.h);
                4
            }
            
            // DEC H
            0x25 => {
                self.h = self.alu_dec(self.h);
                4
            }
            
            // LD H, d8
            0x26 => {
                self.h = self.fetch_byte();
                8
            }
            
            // DAA
            0x27 => {
                self.alu_daa();
                4
            }
            
            // JR Z, r8
            0x28 => {
                let offset = self.fetch_byte() as i8;
                if self.f.zero {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    12
                } else {
                    8
                }
            }
            
            // ADD HL, HL
            0x29 => {
                let hl = self.get_hl();
                let result = self.alu_add_hl(hl, hl);
                self.set_hl(result);
                8
            }
            
            // LD A, (HL+) / LDI A, (HL)
            0x2A => {
                let addr = self.get_hl();
                self.a = self.memory.read(addr);
                self.set_hl(addr.wrapping_add(1));
                8
            }
            
            // DEC HL
            0x2B => {
                let value = self.get_hl().wrapping_sub(1);
                self.set_hl(value);
                8
            }
            
            // INC L
            0x2C => {
                self.l = self.alu_inc(self.l);
                4
            }
            
            // DEC L
            0x2D => {
                self.l = self.alu_dec(self.l);
                4
            }
            
            // LD L, d8
            0x2E => {
                self.l = self.fetch_byte();
                8
            }
            
            // CPL
            0x2F => {
                self.a = !self.a;
                self.f.negative = true;
                self.f.half_carry = true;
                4
            }
            
            // JR NC, r8
            0x30 => {
                let offset = self.fetch_byte() as i8;
                if !self.f.carry {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    12
                } else {
                    8
                }
            }
            
            // LD SP, d16
            0x31 => {
                self.sp = self.fetch_word();
                12
            }
            
            // LD (HL-), A / LDD (HL), A
            0x32 => {
                let addr = self.get_hl();
                self.memory.write(addr, self.a);
                self.set_hl(addr.wrapping_sub(1));
                8
            }
            
            // INC SP
            0x33 => {
                self.sp = self.sp.wrapping_add(1);
                8
            }
            
            // INC (HL)
            0x34 => {
                let addr = self.get_hl();
                let value = self.memory.read(addr);
                let result = self.alu_inc(value);
                self.memory.write(addr, result);
                12
            }
            
            // DEC (HL)
            0x35 => {
                let addr = self.get_hl();
                let value = self.memory.read(addr);
                let result = self.alu_dec(value);
                self.memory.write(addr, result);
                12
            }
            
            // LD (HL), d8
            0x36 => {
                let value = self.fetch_byte();
                let addr = self.get_hl();
                self.memory.write(addr, value);
                12
            }
            
            // SCF
            0x37 => {
                self.f.negative = false;
                self.f.half_carry = false;
                self.f.carry = true;
                4
            }
            
            // JR C, r8
            0x38 => {
                let offset = self.fetch_byte() as i8;
                if self.f.carry {
                    self.pc = self.pc.wrapping_add(offset as u16);
                    12
                } else {
                    8
                }
            }
            
            // ADD HL, SP
            0x39 => {
                let hl = self.get_hl();
                let result = self.alu_add_hl(hl, self.sp);
                self.set_hl(result);
                8
            }
            
            // LD A, (HL-) / LDD A, (HL)
            0x3A => {
                let addr = self.get_hl();
                self.a = self.memory.read(addr);
                self.set_hl(addr.wrapping_sub(1));
                8
            }
            
            // DEC SP
            0x3B => {
                self.sp = self.sp.wrapping_sub(1);
                8
            }
            
            // INC A
            0x3C => {
                self.a = self.alu_inc(self.a);
                4
            }
            
            // DEC A
            0x3D => {
                self.a = self.alu_dec(self.a);
                4
            }
            
            // LD A, d8
            0x3E => {
                self.a = self.fetch_byte();
                8
            }
            
            // CCF
            0x3F => {
                self.f.negative = false;
                self.f.half_carry = false;
                self.f.carry = !self.f.carry;
                4
            }
            
            // LD B, B through LD A, A (0x40-0x7F)
            // LD r, r' instructions
            0x40..=0x7F => {
                if opcode == 0x76 {
                    // HALT
                    self.halted = true;
                    4
                } else {
                    let src_reg = opcode & 0x07;
                    let dst_reg = (opcode >> 3) & 0x07;
                    let value = self.read_r8(src_reg);
                    self.write_r8(dst_reg, value);
                    if src_reg == 6 || dst_reg == 6 {
                        8  // (HL) takes longer
                    } else {
                        4
                    }
                }
            }
            
            // ADD A, r (0x80-0x87)
            0x80..=0x87 => {
                let value = self.read_r8(opcode & 0x07);
                self.alu_add(value);
                if (opcode & 0x07) == 6 { 8 } else { 4 }
            }
            
            // ADC A, r (0x88-0x8F)
            0x88..=0x8F => {
                let value = self.read_r8(opcode & 0x07);
                self.alu_adc(value);
                if (opcode & 0x07) == 6 { 8 } else { 4 }
            }
            
            // SUB r (0x90-0x97)
            0x90..=0x97 => {
                let value = self.read_r8(opcode & 0x07);
                self.alu_sub(value);
                if (opcode & 0x07) == 6 { 8 } else { 4 }
            }
            
            // SBC A, r (0x98-0x9F)
            0x98..=0x9F => {
                let value = self.read_r8(opcode & 0x07);
                self.alu_sbc(value);
                if (opcode & 0x07) == 6 { 8 } else { 4 }
            }
            
            // AND r (0xA0-0xA7)
            0xA0..=0xA7 => {
                let value = self.read_r8(opcode & 0x07);
                self.alu_and(value);
                if (opcode & 0x07) == 6 { 8 } else { 4 }
            }
            
            // XOR r (0xA8-0xAF)
            0xA8..=0xAF => {
                let value = self.read_r8(opcode & 0x07);
                self.alu_xor(value);
                if (opcode & 0x07) == 6 { 8 } else { 4 }
            }
            
            // OR r (0xB0-0xB7)
            0xB0..=0xB7 => {
                let value = self.read_r8(opcode & 0x07);
                self.alu_or(value);
                if (opcode & 0x07) == 6 { 8 } else { 4 }
            }
            
            // CP r (0xB8-0xBF)
            0xB8..=0xBF => {
                let value = self.read_r8(opcode & 0x07);
                self.alu_cp(value);
                if (opcode & 0x07) == 6 { 8 } else { 4 }
            }
            
            // RET NZ
            0xC0 => {
                if !self.f.zero {
                    self.pc = self.pop_stack();
                    20
                } else {
                    8
                }
            }
            
            // POP BC
            0xC1 => {
                let value = self.pop_stack();
                self.set_bc(value);
                12
            }
            
            // JP NZ, a16
            0xC2 => {
                let addr = self.fetch_word();
                if !self.f.zero {
                    self.pc = addr;
                    16
                } else {
                    12
                }
            }
            
            // JP a16
            0xC3 => {
                self.pc = self.fetch_word();
                16
            }
            
            // CALL NZ, a16
            0xC4 => {
                let addr = self.fetch_word();
                if !self.f.zero {
                    self.push_stack(self.pc);
                    self.pc = addr;
                    24
                } else {
                    12
                }
            }
            
            // PUSH BC
            0xC5 => {
                let value = self.get_bc();
                self.push_stack(value);
                16
            }
            
            // ADD A, d8
            0xC6 => {
                let value = self.fetch_byte();
                self.alu_add(value);
                8
            }
            
            // RST 00H
            0xC7 => {
                self.push_stack(self.pc);
                self.pc = 0x00;
                16
            }
            
            // RET Z
            0xC8 => {
                if self.f.zero {
                    self.pc = self.pop_stack();
                    20
                } else {
                    8
                }
            }
            
            // RET
            0xC9 => {
                self.pc = self.pop_stack();
                16
            }
            
            // JP Z, a16
            0xCA => {
                let addr = self.fetch_word();
                if self.f.zero {
                    self.pc = addr;
                    16
                } else {
                    12
                }
            }
            
            // PREFIX CB
            0xCB => {
                let cb_op = self.fetch_byte();
                self.execute_cb(cb_op)
            }
            
            // CALL Z, a16
            0xCC => {
                let addr = self.fetch_word();
                if self.f.zero {
                    self.push_stack(self.pc);
                    self.pc = addr;
                    24
                } else {
                    12
                }
            }
            
            // CALL a16
            0xCD => {
                let addr = self.fetch_word();
                self.push_stack(self.pc);
                self.pc = addr;
                24
            }
            
            // ADC A, d8
            0xCE => {
                let value = self.fetch_byte();
                self.alu_adc(value);
                8
            }
            
            // RST 08H
            0xCF => {
                self.push_stack(self.pc);
                self.pc = 0x08;
                16
            }
            
            // RET NC
            0xD0 => {
                if !self.f.carry {
                    self.pc = self.pop_stack();
                    20
                } else {
                    8
                }
            }
            
            // POP DE
            0xD1 => {
                let value = self.pop_stack();
                self.set_de(value);
                12
            }
            
            // JP NC, a16
            0xD2 => {
                let addr = self.fetch_word();
                if !self.f.carry {
                    self.pc = addr;
                    16
                } else {
                    12
                }
            }
            
            // Invalid opcode 0xD3
            0xD3 => 4,
            
            // CALL NC, a16
            0xD4 => {
                let addr = self.fetch_word();
                if !self.f.carry {
                    self.push_stack(self.pc);
                    self.pc = addr;
                    24
                } else {
                    12
                }
            }
            
            // PUSH DE
            0xD5 => {
                let value = self.get_de();
                self.push_stack(value);
                16
            }
            
            // SUB d8
            0xD6 => {
                let value = self.fetch_byte();
                self.alu_sub(value);
                8
            }
            
            // RST 10H
            0xD7 => {
                self.push_stack(self.pc);
                self.pc = 0x10;
                16
            }
            
            // RET C
            0xD8 => {
                if self.f.carry {
                    self.pc = self.pop_stack();
                    20
                } else {
                    8
                }
            }
            
            // RETI
            0xD9 => {
                self.pc = self.pop_stack();
                self.ime = true;
                16
            }
            
            // JP C, a16
            0xDA => {
                let addr = self.fetch_word();
                if self.f.carry {
                    self.pc = addr;
                    16
                } else {
                    12
                }
            }
            
            // Invalid opcode 0xDB
            0xDB => 4,
            
            // CALL C, a16
            0xDC => {
                let addr = self.fetch_word();
                if self.f.carry {
                    self.push_stack(self.pc);
                    self.pc = addr;
                    24
                } else {
                    12
                }
            }
            
            // Invalid opcode 0xDD
            0xDD => 4,
            
            // SBC A, d8
            0xDE => {
                let value = self.fetch_byte();
                self.alu_sbc(value);
                8
            }
            
            // RST 18H
            0xDF => {
                self.push_stack(self.pc);
                self.pc = 0x18;
                16
            }
            
            // LDH (a8), A
            0xE0 => {
                let offset = self.fetch_byte() as u16;
                self.memory.write(0xFF00 + offset, self.a);
                12
            }
            
            // POP HL
            0xE1 => {
                let value = self.pop_stack();
                self.set_hl(value);
                12
            }
            
            // LD (C), A
            0xE2 => {
                let addr = 0xFF00 + self.c as u16;
                self.memory.write(addr, self.a);
                8
            }
            
            // Invalid opcodes 0xE3, 0xE4
            0xE3 | 0xE4 => 4,
            
            // PUSH HL
            0xE5 => {
                let value = self.get_hl();
                self.push_stack(value);
                16
            }
            
            // AND d8
            0xE6 => {
                let value = self.fetch_byte();
                self.alu_and(value);
                8
            }
            
            // RST 20H
            0xE7 => {
                self.push_stack(self.pc);
                self.pc = 0x20;
                16
            }
            
            // ADD SP, r8
            0xE8 => {
                let offset = self.fetch_byte();
                let signed_offset = offset as i8 as i16 as u16;
                let result = self.sp.wrapping_add(signed_offset);
                
                self.f.zero = false;
                self.f.negative = false;
                self.f.half_carry = ((self.sp & 0x0F) + (signed_offset & 0x0F)) > 0x0F;
                self.f.carry = ((self.sp & 0xFF) + (signed_offset & 0xFF)) > 0xFF;
                
                self.sp = result;
                16
            }
            
            // JP (HL)
            0xE9 => {
                self.pc = self.get_hl();
                4
            }
            
            // LD (a16), A
            0xEA => {
                let addr = self.fetch_word();
                self.memory.write(addr, self.a);
                16
            }
            
            // Invalid opcodes 0xEB, 0xEC, 0xED
            0xEB | 0xEC | 0xED => 4,
            
            // XOR d8
            0xEE => {
                let value = self.fetch_byte();
                self.alu_xor(value);
                8
            }
            
            // RST 28H
            0xEF => {
                self.push_stack(self.pc);
                self.pc = 0x28;
                16
            }
            
            // LDH A, (a8)
            0xF0 => {
                let offset = self.fetch_byte() as u16;
                self.a = self.memory.read(0xFF00 + offset);
                12
            }
            
            // POP AF
            0xF1 => {
                let value = self.pop_stack();
                self.a = (value >> 8) as u8;
                self.f = FlagsRegister::from((value & 0x00F0) as u8);
                12
            }
            
            // LD A, (C)
            0xF2 => {
                let addr = 0xFF00 + self.c as u16;
                self.a = self.memory.read(addr);
                8
            }
            
            // DI
            0xF3 => {
                self.ime = false;
                4
            }
            
            // Invalid opcode 0xF4
            0xF4 => 4,
            
            // PUSH AF
            0xF5 => {
                let f_value: u8 = self.f.clone().into();
                let value = ((self.a as u16) << 8) | (f_value as u16);
                self.push_stack(value);
                16
            }
            
            // OR d8
            0xF6 => {
                let value = self.fetch_byte();
                self.alu_or(value);
                8
            }
            
            // RST 30H
            0xF7 => {
                self.push_stack(self.pc);
                self.pc = 0x30;
                16
            }
            
            // LD HL, SP+r8
            0xF8 => {
                let offset = self.fetch_byte();
                let signed_offset = offset as i8 as i16 as u16;
                let result = self.sp.wrapping_add(signed_offset);
                
                self.f.zero = false;
                self.f.negative = false;
                self.f.half_carry = ((self.sp & 0x0F) + (signed_offset & 0x0F)) > 0x0F;
                self.f.carry = ((self.sp & 0xFF) + (signed_offset & 0xFF)) > 0xFF;
                
                self.set_hl(result);
                12
            }
            
            // LD SP, HL
            0xF9 => {
                self.sp = self.get_hl();
                8
            }
            
            // LD A, (a16)
            0xFA => {
                let addr = self.fetch_word();
                self.a = self.memory.read(addr);
                16
            }
            
            // EI
            0xFB => {
                self.ime = true;
                4
            }
            
            // Invalid opcodes 0xFC, 0xFD
            0xFC | 0xFD => 4,
            
            // CP d8
            0xFE => {
                let value = self.fetch_byte();
                self.alu_cp(value);
                8
            }
            
            // RST 38H
            0xFF => {
                self.push_stack(self.pc);
                self.pc = 0x38;
                16
            }
        }
    }

    // CB-prefixed instructions (bit operations)
    fn execute_cb(&mut self, opcode: u8) -> u8 {
        let reg = opcode & 0x07;
        let bit = (opcode >> 3) & 0x07;
        
        match opcode {
            // RLC r
            0x00..=0x07 => {
                let value = self.read_r8(reg);
                let result = self.alu_rlc(value);
                self.write_r8(reg, result);
                if reg == 6 { 16 } else { 8 }
            }
            
            // RRC r
            0x08..=0x0F => {
                let value = self.read_r8(reg);
                let result = self.alu_rrc(value);
                self.write_r8(reg, result);
                if reg == 6 { 16 } else { 8 }
            }
            
            // RL r
            0x10..=0x17 => {
                let value = self.read_r8(reg);
                let result = self.alu_rl(value);
                self.write_r8(reg, result);
                if reg == 6 { 16 } else { 8 }
            }
            
            // RR r
            0x18..=0x1F => {
                let value = self.read_r8(reg);
                let result = self.alu_rr(value);
                self.write_r8(reg, result);
                if reg == 6 { 16 } else { 8 }
            }
            
            // SLA r
            0x20..=0x27 => {
                let value = self.read_r8(reg);
                let result = self.alu_sla(value);
                self.write_r8(reg, result);
                if reg == 6 { 16 } else { 8 }
            }
            
            // SRA r
            0x28..=0x2F => {
                let value = self.read_r8(reg);
                let result = self.alu_sra(value);
                self.write_r8(reg, result);
                if reg == 6 { 16 } else { 8 }
            }
            
            // SWAP r
            0x30..=0x37 => {
                let value = self.read_r8(reg);
                let result = self.alu_swap(value);
                self.write_r8(reg, result);
                if reg == 6 { 16 } else { 8 }
            }
            
            // SRL r
            0x38..=0x3F => {
                let value = self.read_r8(reg);
                let result = self.alu_srl(value);
                self.write_r8(reg, result);
                if reg == 6 { 16 } else { 8 }
            }
            
            // BIT b, r
            0x40..=0x7F => {
                let value = self.read_r8(reg);
                self.alu_bit(bit, value);
                if reg == 6 { 12 } else { 8 }
            }
            
            // RES b, r
            0x80..=0xBF => {
                let value = self.read_r8(reg);
                let result = value & !(1 << bit);
                self.write_r8(reg, result);
                if reg == 6 { 16 } else { 8 }
            }
            
            // SET b, r
            0xC0..=0xFF => {
                let value = self.read_r8(reg);
                let result = value | (1 << bit);
                self.write_r8(reg, result);
                if reg == 6 { 16 } else { 8 }
            }
        }
    }

    // Register access helpers
    fn read_r8(&self, reg: u8) -> u8 {
        match reg {
            0 => self.b,
            1 => self.c,
            2 => self.d,
            3 => self.e,
            4 => self.h,
            5 => self.l,
            6 => self.memory.read(self.get_hl()),
            7 => self.a,
            _ => unreachable!(),
        }
    }

    fn write_r8(&mut self, reg: u8, value: u8) {
        match reg {
            0 => self.b = value,
            1 => self.c = value,
            2 => self.d = value,
            3 => self.e = value,
            4 => self.h = value,
            5 => self.l = value,
            6 => {
                let addr = self.get_hl();
                self.memory.write(addr, value);
            }
            7 => self.a = value,
            _ => unreachable!(),
        }
    }

    // 16-bit register pair access
    fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    fn get_de(&self) -> u16 {
        ((self.d as u16) << 8) | (self.e as u16)
    }

    fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    fn get_hl(&self) -> u16 {
        ((self.h as u16) << 8) | (self.l as u16)
    }

    fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    // Stack operations
    fn push_stack(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.memory.write(self.sp, (value >> 8) as u8);
        self.sp = self.sp.wrapping_sub(1);
        self.memory.write(self.sp, (value & 0xFF) as u8);
    }

    fn pop_stack(&mut self) -> u16 {
        let low = self.memory.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        let high = self.memory.read(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);
        (high << 8) | low
    }

    // ALU operations
    fn alu_inc(&mut self, value: u8) -> u8 {
        let result = value.wrapping_add(1);
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = (value & 0x0F) + 1 > 0x0F;
        result
    }

    fn alu_dec(&mut self, value: u8) -> u8 {
        let result = value.wrapping_sub(1);
        self.f.zero = result == 0;
        self.f.negative = true;
        self.f.half_carry = (value & 0x0F) == 0;
        result
    }

    fn alu_add(&mut self, value: u8) {
        let result = self.a.wrapping_add(value);
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = ((self.a & 0x0F) + (value & 0x0F)) > 0x0F;
        self.f.carry = (self.a as u16 + value as u16) > 0xFF;
        self.a = result;
    }

    fn alu_adc(&mut self, value: u8) {
        let carry = if self.f.carry { 1 } else { 0 };
        let result = self.a.wrapping_add(value).wrapping_add(carry);
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = ((self.a & 0x0F) + (value & 0x0F) + carry) > 0x0F;
        self.f.carry = (self.a as u16 + value as u16 + carry as u16) > 0xFF;
        self.a = result;
    }

    fn alu_sub(&mut self, value: u8) {
        let result = self.a.wrapping_sub(value);
        self.f.zero = result == 0;
        self.f.negative = true;
        self.f.half_carry = (self.a & 0x0F) < (value & 0x0F);
        self.f.carry = (self.a as u16) < (value as u16);
        self.a = result;
    }

    fn alu_sbc(&mut self, value: u8) {
        let carry = if self.f.carry { 1 } else { 0 };
        let result = self.a.wrapping_sub(value).wrapping_sub(carry);
        self.f.zero = result == 0;
        self.f.negative = true;
        self.f.half_carry = (self.a & 0x0F) < ((value & 0x0F) + carry);
        self.f.carry = (self.a as u16) < (value as u16 + carry as u16);
        self.a = result;
    }

    fn alu_and(&mut self, value: u8) {
        self.a &= value;
        self.f.zero = self.a == 0;
        self.f.negative = false;
        self.f.half_carry = true;
        self.f.carry = false;
    }

    fn alu_or(&mut self, value: u8) {
        self.a |= value;
        self.f.zero = self.a == 0;
        self.f.negative = false;
        self.f.half_carry = false;
        self.f.carry = false;
    }

    fn alu_xor(&mut self, value: u8) {
        self.a ^= value;
        self.f.zero = self.a == 0;
        self.f.negative = false;
        self.f.half_carry = false;
        self.f.carry = false;
    }

    fn alu_cp(&mut self, value: u8) {
        let result = self.a.wrapping_sub(value);
        self.f.zero = result == 0;
        self.f.negative = true;
        self.f.half_carry = (self.a & 0x0F) < (value & 0x0F);
        self.f.carry = (self.a as u16) < (value as u16);
    }

    fn alu_add_hl(&mut self, hl: u16, value: u16) -> u16 {
        let result = hl.wrapping_add(value);
        self.f.negative = false;
        self.f.half_carry = ((hl & 0x0FFF) + (value & 0x0FFF)) > 0x0FFF;
        self.f.carry = (hl as u32 + value as u32) > 0xFFFF;
        result
    }

    fn alu_daa(&mut self) {
        let mut adjust = 0;
        if self.f.half_carry || (!self.f.negative && (self.a & 0x0F) > 9) {
            adjust |= 0x06;
        }
        if self.f.carry || (!self.f.negative && self.a > 0x99) {
            adjust |= 0x60;
            self.f.carry = true;
        }
        
        if self.f.negative {
            self.a = self.a.wrapping_sub(adjust);
        } else {
            self.a = self.a.wrapping_add(adjust);
        }
        
        self.f.zero = self.a == 0;
        self.f.half_carry = false;
    }

    // CB prefix ALU operations
    fn alu_rlc(&mut self, value: u8) -> u8 {
        let carry = (value & 0x80) >> 7;
        let result = (value << 1) | carry;
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = false;
        self.f.carry = carry == 1;
        result
    }

    fn alu_rrc(&mut self, value: u8) -> u8 {
        let carry = value & 0x01;
        let result = (value >> 1) | (carry << 7);
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = false;
        self.f.carry = carry == 1;
        result
    }

    fn alu_rl(&mut self, value: u8) -> u8 {
        let carry = if self.f.carry { 1 } else { 0 };
        let new_carry = (value & 0x80) >> 7;
        let result = (value << 1) | carry;
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = false;
        self.f.carry = new_carry == 1;
        result
    }

    fn alu_rr(&mut self, value: u8) -> u8 {
        let carry = if self.f.carry { 1 } else { 0 };
        let new_carry = value & 0x01;
        let result = (value >> 1) | (carry << 7);
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = false;
        self.f.carry = new_carry == 1;
        result
    }

    fn alu_sla(&mut self, value: u8) -> u8 {
        let carry = (value & 0x80) >> 7;
        let result = value << 1;
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = false;
        self.f.carry = carry == 1;
        result
    }

    fn alu_sra(&mut self, value: u8) -> u8 {
        let carry = value & 0x01;
        let result = (value >> 1) | (value & 0x80);
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = false;
        self.f.carry = carry == 1;
        result
    }

    fn alu_swap(&mut self, value: u8) -> u8 {
        let result = ((value & 0x0F) << 4) | ((value & 0xF0) >> 4);
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = false;
        self.f.carry = false;
        result
    }

    fn alu_srl(&mut self, value: u8) -> u8 {
        let carry = value & 0x01;
        let result = value >> 1;
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = false;
        self.f.carry = carry == 1;
        result
    }

    fn alu_bit(&mut self, bit: u8, value: u8) {
        let result = value & (1 << bit);
        self.f.zero = result == 0;
        self.f.negative = false;
        self.f.half_carry = true;
    }
    
    pub fn get_pc(&self) -> u16 {
        self.pc
    }
    
    pub fn get_ticks(&self) -> u128 {
        self.clock.get_ticks()
    }
    
    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }
    
    pub fn get_memory_mut(&mut self) -> &mut Memory {
        &mut self.memory
    }
}
