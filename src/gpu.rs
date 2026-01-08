use crate::memory::Memory;

pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White = 0xFFFFFF,
    LightGray = 0xAAAAAA,
    DarkGray = 0x555555,
    Black = 0x000000,
}

impl Color {
    fn from_id(id: u8) -> Self {
        match id {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => Color::White,
        }
    }

    pub fn to_u32(self) -> u32 {
        self as u32
    }
}

pub struct GPU {
    pub framebuffer: [u32; SCREEN_WIDTH * SCREEN_HEIGHT],
    pub cycles: u32,
}

impl GPU {
    pub fn new() -> Self {
        Self {
            framebuffer: [Color::White.to_u32(); SCREEN_WIDTH * SCREEN_HEIGHT],
            cycles: 0,
        }
    }

    pub fn step(&mut self, cycles: u8, memory: &Memory) {
        self.cycles += cycles as u32;
        
        // Simple rendering: just update the framebuffer based on VRAM
        // In a full implementation, this would handle different GPU modes and timing
        if self.cycles >= 70224 { // Full frame
            self.cycles = 0;
            self.render_screen(memory);
        }
    }

    fn render_screen(&mut self, memory: &Memory) {
        // Clear screen
        for pixel in self.framebuffer.iter_mut() {
            *pixel = Color::White.to_u32();
        }

        // Read LCD control register
        let lcdc = memory.read(0xFF40);
        let bg_enabled = (lcdc & 0x01) != 0;
        
        if !bg_enabled {
            return;
        }

        // Get scroll positions
        let scy = memory.read(0xFF42);
        let scx = memory.read(0xFF43);

        // Determine tile map and tile data addresses
        let bg_map = if (lcdc & 0x08) != 0 { 0x9C00 } else { 0x9800 };
        let tile_data = if (lcdc & 0x10) != 0 { 0x8000 } else { 0x8800 };
        let use_signed = (lcdc & 0x10) == 0;

        // Render background
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let map_y = ((y as u8).wrapping_add(scy)) as usize;
                let map_x = ((x as u8).wrapping_add(scx)) as usize;
                
                let tile_row = map_y / 8;
                let tile_col = map_x / 8;
                let tile_y = map_y % 8;
                let tile_x = map_x % 8;

                // Get tile number from background map
                let tile_addr = bg_map + (tile_row as u16 % 32) * 32 + (tile_col as u16 % 32);
                let tile_num = memory.read(tile_addr);

                // Calculate tile data address
                let tile_data_addr = if use_signed {
                    let signed_tile = tile_num as i8;
                    ((tile_data as i32) + (signed_tile as i32) * 16) as u16
                } else {
                    tile_data + (tile_num as u16) * 16
                };

                // Each tile is 16 bytes, 2 bytes per row
                let byte1 = memory.read(tile_data_addr + (tile_y as u16 * 2));
                let byte2 = memory.read(tile_data_addr + (tile_y as u16 * 2) + 1);

                // Get color for this pixel (bits are in reverse order)
                let bit_pos = 7 - tile_x;
                let color_low = (byte1 >> bit_pos) & 1;
                let color_high = (byte2 >> bit_pos) & 1;
                let color_id = (color_high << 1) | color_low;

                let color = Color::from_id(color_id);
                self.framebuffer[y * SCREEN_WIDTH + x] = color.to_u32();
            }
        }
    }
}
