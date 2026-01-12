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

    pub fn step(&mut self, cycles: u8, memory: &mut Memory) {
        self.cycles += cycles as u32;
        
        // Simple rendering: just update the framebuffer based on VRAM
        // In a full implementation, this would handle different GPU modes and timing
        if self.cycles >= 70224 { // Full frame
            self.cycles = 0;
            self.render_screen(memory);
            
            // Request VBlank interrupt (bit 0 of IF register at 0xFF0F)
            let if_reg = memory.read(0xFF0F);
            memory.write(0xFF0F, if_reg | 0x01);
        }
    }

    fn render_screen(&mut self, memory: &Memory) {
        // Read LCD control register
        let lcdc = memory.read(0xFF40);
        
        // Check if LCD is enabled (bit 7)
        let lcd_enabled = (lcdc & 0x80) != 0;
        
        if !lcd_enabled {
            // LCD is off - keep current framebuffer (don't clear)
            // This preserves the last frame when LCD is temporarily disabled
            return;
        }
        
        // Clear screen to white before rendering
        for pixel in self.framebuffer.iter_mut() {
            *pixel = Color::White.to_u32();
        }
        
        // Check if background is enabled (bit 0)
        let bg_enabled = (lcdc & 0x01) != 0;
        
        if !bg_enabled {
            // Background disabled - screen stays white
            // (sprites could still be visible but not implemented yet)
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
        
        // Render sprites (OAM) if enabled
        let sprites_enabled = (lcdc & 0x02) != 0;
        if sprites_enabled {
            self.render_sprites(memory, lcdc);
        }
    }
    
    fn render_sprites(&mut self, memory: &Memory, lcdc: u8) {
        // Sprite size: 8x8 or 8x16
        let sprite_height = if (lcdc & 0x04) != 0 { 16 } else { 8 };
        
        // OAM is at 0xFE00-0xFE9F (160 bytes = 40 sprites x 4 bytes each)
        // Each sprite: Y pos, X pos, Tile number, Attributes
        for sprite_index in 0..40 {
            let oam_addr = 0xFE00 + (sprite_index * 4);
            
            let y_pos = memory.read(oam_addr).wrapping_sub(16); // Y position minus 16
            let x_pos = memory.read(oam_addr + 1).wrapping_sub(8); // X position minus 8
            let tile_num = memory.read(oam_addr + 2);
            let attributes = memory.read(oam_addr + 3);
            
            // Skip if sprite is off-screen
            if y_pos >= 144 && y_pos < 240 {
                continue;
            }
            
            // Attributes: bit 7 = priority, bit 6 = Y flip, bit 5 = X flip, bit 4 = palette
            let _priority = (attributes & 0x80) != 0; // 0 = above bg, 1 = behind bg colors 1-3
            let y_flip = (attributes & 0x40) != 0;
            let x_flip = (attributes & 0x20) != 0;
            let _palette = (attributes & 0x10) != 0; // OBP0 or OBP1
            
            // Render sprite tile
            for tile_y in 0..sprite_height {
                let y = y_pos.wrapping_add(tile_y);
                if y >= 144 {
                    continue;
                }
                
                // Calculate which tile line to read (handle Y flip)
                let line = if y_flip {
                    sprite_height - 1 - tile_y
                } else {
                    tile_y
                };
                
                // Tile data is always at 0x8000 for sprites
                let tile_addr = 0x8000u16 + (tile_num as u16) * 16 + (line as u16 * 2);
                let byte1 = memory.read(tile_addr);
                let byte2 = memory.read(tile_addr + 1);
                
                // Render the 8 pixels of this sprite line
                for tile_x in 0..8 {
                    let x = x_pos.wrapping_add(tile_x);
                    if x >= 160 {
                        continue;
                    }
                    
                    // Calculate which bit to read (handle X flip)
                    let bit_pos = if x_flip {
                        tile_x
                    } else {
                        7 - tile_x
                    };
                    
                    let color_low = (byte1 >> bit_pos) & 1;
                    let color_high = (byte2 >> bit_pos) & 1;
                    let color_id = (color_high << 1) | color_low;
                    
                    // Color 0 is transparent for sprites
                    if color_id == 0 {
                        continue;
                    }
                    
                    // TODO: Apply sprite palette (OBP0/OBP1) instead of BG palette
                    // For now, use same color mapping
                    let color = Color::from_id(color_id);
                    let pixel_index = (y as usize) * SCREEN_WIDTH + (x as usize);
                    self.framebuffer[pixel_index] = color.to_u32();
                }
            }
        }
    }
}
