// Game Boy input handling
// Button mapping:
// Bit 7 - Not used
// Bit 6 - Not used
// Bit 5 - P15 (Select Button Keys)
// Bit 4 - P14 (Select Direction Keys)
// Bit 3 - P13 (Down or Start)
// Bit 2 - P12 (Up or Select)
// Bit 1 - P11 (Left or B)
// Bit 0 - P10 (Right or A)

#[cfg(feature = "gui")]
use minifb::Key;

#[derive(Default)]
pub struct Input {
    // Direction keys
    pub right: bool,
    pub left: bool,
    pub up: bool,
    pub down: bool,
    // Button keys
    pub a: bool,
    pub b: bool,
    pub select: bool,
    pub start: bool,
}

impl Input {
    pub fn new() -> Self {
        Self::default()
    }

    #[cfg(feature = "gui")]
    pub fn update_from_keys(&mut self, keys: &[Key]) {
        self.right = keys.contains(&Key::Right) || keys.contains(&Key::D);
        self.left = keys.contains(&Key::Left) || keys.contains(&Key::A);
        self.up = keys.contains(&Key::Up) || keys.contains(&Key::W);
        self.down = keys.contains(&Key::Down) || keys.contains(&Key::S);
        
        self.a = keys.contains(&Key::Z) || keys.contains(&Key::J);
        self.b = keys.contains(&Key::X) || keys.contains(&Key::K);
        self.select = keys.contains(&Key::Backspace) || keys.contains(&Key::U);
        self.start = keys.contains(&Key::Enter) || keys.contains(&Key::I);
    }

    pub fn get_joypad_state(&self, joypad_register: u8) -> u8 {
        let mut result = 0xFF;
        
        // Check which button group is selected
        let select_buttons = (joypad_register & 0x20) == 0;
        let select_directions = (joypad_register & 0x10) == 0;
        
        if select_buttons {
            // Button keys (A, B, Select, Start)
            if self.start { result &= !0x08; }
            if self.select { result &= !0x04; }
            if self.b { result &= !0x02; }
            if self.a { result &= !0x01; }
        }
        
        if select_directions {
            // Direction keys (Down, Up, Left, Right)
            if self.down { result &= !0x08; }
            if self.up { result &= !0x04; }
            if self.left { result &= !0x02; }
            if self.right { result &= !0x01; }
        }
        
        // Preserve the selection bits
        result = (result & 0x0F) | (joypad_register & 0x30);
        
        result
    }
}
