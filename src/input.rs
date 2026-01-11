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

    #[cfg(feature = "tui")]
    pub fn update_from_key_event(&mut self, key_event: crossterm::event::KeyEvent) {
        let pressed = key_event.kind == crossterm::event::KeyEventKind::Press;
        
        match key_event.code {
            crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('d') | crossterm::event::KeyCode::Char('D') => self.right = pressed,
            crossterm::event::KeyCode::Left | crossterm::event::KeyCode::Char('a') | crossterm::event::KeyCode::Char('A') => self.left = pressed,
            crossterm::event::KeyCode::Up | crossterm::event::KeyCode::Char('w') | crossterm::event::KeyCode::Char('W') => self.up = pressed,
            crossterm::event::KeyCode::Down | crossterm::event::KeyCode::Char('s') | crossterm::event::KeyCode::Char('S') => self.down = pressed,
            crossterm::event::KeyCode::Char('z') | crossterm::event::KeyCode::Char('Z') | crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Char('J') => self.a = pressed,
            crossterm::event::KeyCode::Char('x') | crossterm::event::KeyCode::Char('X') | crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Char('K') => self.b = pressed,
            crossterm::event::KeyCode::Backspace | crossterm::event::KeyCode::Char('u') | crossterm::event::KeyCode::Char('U') => self.select = pressed,
            crossterm::event::KeyCode::Enter | crossterm::event::KeyCode::Char('i') | crossterm::event::KeyCode::Char('I') => self.start = pressed,
            _ => {}
        }
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
