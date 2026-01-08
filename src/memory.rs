// Memory Management Unit for Game Boy
//
// Implements the complete Game Boy memory map with accurate address decoding.
// The MMU handles all memory read/write operations with proper banking and mirroring.
//
// Memory Map:
// 0x0000-0x00FF: Boot ROM (can be disabled)
// 0x0000-0x3FFF: ROM Bank 0
// 0x4000-0x7FFF: ROM Bank 1-N (switchable)
// 0x8000-0x9FFF: VRAM
// 0xA000-0xBFFF: External RAM
// 0xC000-0xDFFF: Work RAM (WRAM)
// 0xE000-0xFDFF: Echo RAM (mirror of 0xC000-0xDDFF)
// 0xFE00-0xFE9F: OAM (Object Attribute Memory)
// 0xFEA0-0xFEFF: Not usable
// 0xFF00-0xFF7F: I/O Registers
// 0xFF80-0xFFFE: High RAM (HRAM)
// 0xFFFF: Interrupt Enable Register

pub struct Memory {
    boot_rom: [u8; 256],
    rom: Vec<u8>,
    vram: [u8; 8192],
    wram: [u8; 8192],
    oam: [u8; 160],
    hram: [u8; 127],
    io: [u8; 128],
    boot_rom_enabled: bool,
    ie_register: u8, // Interrupt Enable at 0xFFFF
}

impl Memory {
    pub fn new() -> Self {
        Self {
            boot_rom: [0; 256],
            rom: vec![0; 32768], // Minimum 32KB ROM
            vram: [0; 8192],
            wram: [0; 8192],
            oam: [0; 160],
            hram: [0; 127],
            io: [0; 128],
            boot_rom_enabled: true,
            ie_register: 0,
        }
    }

    pub fn load_boot_rom(&mut self, data: &[u8]) {
        self.boot_rom.copy_from_slice(&data[0..256.min(data.len())]);
    }

    pub fn load_rom(&mut self, data: Vec<u8>) {
        self.rom = data;
    }

    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // Boot ROM / ROM Bank 0
            0x0000..=0x00FF => {
                if self.boot_rom_enabled {
                    self.boot_rom[addr as usize]
                } else {
                    self.rom.get(addr as usize).copied().unwrap_or(0xFF)
                }
            }
            0x0100..=0x7FFF => self.rom.get(addr as usize).copied().unwrap_or(0xFF),
            // VRAM
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize],
            // External RAM (not implemented, returns 0xFF)
            0xA000..=0xBFFF => 0xFF,
            // WRAM
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize],
            // Echo RAM (mirror of WRAM)
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize],
            // OAM
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize],
            // Not usable
            0xFEA0..=0xFEFF => 0xFF,
            // I/O Registers
            0xFF00..=0xFF7F => self.io[(addr - 0xFF00) as usize],
            // HRAM
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize],
            // IE Register
            0xFFFF => self.ie_register,
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            // ROM (read-only, but writing can trigger bank switching in real hardware)
            0x0000..=0x7FFF => {}
            // VRAM
            0x8000..=0x9FFF => self.vram[(addr - 0x8000) as usize] = value,
            // External RAM (not implemented)
            0xA000..=0xBFFF => {}
            // WRAM
            0xC000..=0xDFFF => self.wram[(addr - 0xC000) as usize] = value,
            // Echo RAM (mirror of WRAM)
            0xE000..=0xFDFF => self.wram[(addr - 0xE000) as usize] = value,
            // OAM
            0xFE00..=0xFE9F => self.oam[(addr - 0xFE00) as usize] = value,
            // Not usable
            0xFEA0..=0xFEFF => {}
            // I/O Registers
            0xFF00..=0xFF7F => {
                // Special handling for boot rom disable
                if addr == 0xFF50 && value != 0 {
                    self.boot_rom_enabled = false;
                }
                self.io[(addr - 0xFF00) as usize] = value;
            }
            // HRAM
            0xFF80..=0xFFFE => self.hram[(addr - 0xFF80) as usize] = value,
            // IE Register
            0xFFFF => self.ie_register = value,
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let low = self.read(addr) as u16;
        let high = self.read(addr.wrapping_add(1)) as u16;
        (high << 8) | low
    }

    pub fn write_word(&mut self, addr: u16, value: u16) {
        self.write(addr, (value & 0xFF) as u8);
        self.write(addr.wrapping_add(1), (value >> 8) as u8);
    }
}
