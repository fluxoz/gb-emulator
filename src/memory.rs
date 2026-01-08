// Memory map for Game Boy
// 0x0000-0x00FF: Boot ROM (when mapped) / ROM Bank 0
// 0x0100-0x3FFF: ROM Bank 0
// 0x4000-0x7FFF: ROM Bank 1-N (switchable)
// 0x8000-0x9FFF: Video RAM (VRAM)
// 0xA000-0xBFFF: External RAM (in cartridge)
// 0xC000-0xDFFF: Work RAM (WRAM)
// 0xE000-0xFDFF: Echo RAM (mirror of WRAM)
// 0xFE00-0xFE9F: Object Attribute Memory (OAM)
// 0xFEA0-0xFEFF: Unusable
// 0xFF00-0xFF7F: I/O Registers
// 0xFF80-0xFFFE: High RAM (HRAM)
// 0xFFFF: Interrupt Enable Register

use crate::input::Input;

pub struct Memory {
    boot_rom: [u8; 256],
    rom: Vec<u8>,
    vram: [u8; 0x2000],
    wram: [u8; 0x2000],
    oam: [u8; 0xA0],
    hram: [u8; 0x7F],
    io_registers: [u8; 0x80],
    boot_rom_enabled: bool,
    ie_register: u8,
    pub input: Input,
}

impl Memory {
    pub fn new(boot_rom: Vec<u8>, rom: Option<Vec<u8>>) -> Self {
        let mut boot_rom_array = [0u8; 256];
        let len = boot_rom.len().min(256);
        boot_rom_array[..len].copy_from_slice(&boot_rom[..len]);
        
        let rom = rom.unwrap_or_else(|| vec![0; 0x8000]);
        
        Self {
            boot_rom: boot_rom_array,
            rom,
            vram: [0; 0x2000],
            wram: [0; 0x2000],
            oam: [0; 0xA0],
            hram: [0; 0x7F],
            io_registers: [0; 0x80],
            boot_rom_enabled: true,
            ie_register: 0,
            input: Input::new(),
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x00FF if self.boot_rom_enabled => self.boot_rom[address as usize],
            0x0000..=0x7FFF => {
                if (address as usize) < self.rom.len() {
                    self.rom[address as usize]
                } else {
                    0xFF
                }
            }
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize],
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize],
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize], // Echo RAM
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize],
            0xFF00..=0xFF7F => self.read_io_register(address),
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize],
            0xFFFF => self.ie_register,
            _ => 0xFF,
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => {
                // ROM is read-only, but writing here can control MBC (Memory Bank Controller)
                // For simplicity, we ignore writes to ROM area
            }
            0x8000..=0x9FFF => self.vram[(address - 0x8000) as usize] = value,
            0xC000..=0xDFFF => self.wram[(address - 0xC000) as usize] = value,
            0xE000..=0xFDFF => self.wram[(address - 0xE000) as usize] = value, // Echo RAM
            0xFE00..=0xFE9F => self.oam[(address - 0xFE00) as usize] = value,
            0xFF00..=0xFF7F => self.write_io_register(address, value),
            0xFF80..=0xFFFE => self.hram[(address - 0xFF80) as usize] = value,
            0xFFFF => self.ie_register = value,
            _ => {}
        }
    }

    fn read_io_register(&self, address: u16) -> u8 {
        let offset = (address - 0xFF00) as usize;
        
        // Handle joypad register specially
        if address == 0xFF00 {
            let joypad_reg = self.io_registers[0];
            return self.input.get_joypad_state(joypad_reg);
        }
        
        if offset < self.io_registers.len() {
            self.io_registers[offset]
        } else {
            0xFF
        }
    }

    fn write_io_register(&mut self, address: u16, value: u8) {
        let offset = (address - 0xFF00) as usize;
        if offset < self.io_registers.len() {
            // Special handling for boot ROM disable register
            if address == 0xFF50 && value != 0 {
                self.boot_rom_enabled = false;
            }
            self.io_registers[offset] = value;
        }
    }

    pub fn read_vram(&self, address: u16) -> u8 {
        if address >= 0x8000 && address <= 0x9FFF {
            self.vram[(address - 0x8000) as usize]
        } else {
            0xFF
        }
    }
}
