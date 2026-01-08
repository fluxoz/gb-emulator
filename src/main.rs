mod clock;
mod cpu;
mod flags;
mod gpu;
mod opcodes;
mod memory;
mod input;

use cpu::CPU;
use gpu::{GPU, SCREEN_WIDTH, SCREEN_HEIGHT};
use memory::Memory;
use minifb::{Key, Window, WindowOptions};
use std::fs;
use std::time::{Duration, Instant};

const CYCLES_PER_FRAME: u64 = 70224; // ~59.7 Hz

struct GameBoy {
    cpu: CPU,
    gpu: GPU,
    memory: Memory,
}

impl GameBoy {
    fn new(boot_rom: Vec<u8>, rom: Option<Vec<u8>>) -> Self {
        Self {
            cpu: CPU::new(),
            gpu: GPU::new(),
            memory: Memory::new(boot_rom, rom),
        }
    }

    fn step(&mut self) {
        let cycles = self.cpu.step(&mut self.memory);
        self.gpu.step(cycles, &self.memory);
    }

    fn run_frame(&mut self) {
        let start_cycles = self.cpu.cycles;
        while self.cpu.cycles - start_cycles < CYCLES_PER_FRAME {
            self.step();
        }
    }
}

fn main() {
    println!("Game Boy Emulator Starting...");
    println!("=================================");
    println!("Controls:");
    println!("  Arrow Keys / WASD - D-Pad");
    println!("  Z / J             - A Button");
    println!("  X / K             - B Button");
    println!("  Enter / I         - Start");
    println!("  Backspace / U     - Select");
    println!("  ESC               - Exit");
    println!("=================================");

    // Load boot ROM
    let boot_rom = fs::read("dmg_boot.bin").expect("Failed to read dmg_boot.bin");
    println!("Loaded boot ROM: {} bytes", boot_rom.len());

    // Create emulator
    let mut gameboy = GameBoy::new(boot_rom, None);

    // Create window
    let mut window = Window::new(
        "Game Boy Emulator - Press ESC to exit",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .expect("Failed to create window");

    // Limit to ~60 fps
    window.set_target_fps(60);

    println!("Running emulator...");

    let mut last_time = Instant::now();
    let mut frame_count = 0;

    // Main loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update input
        let keys = window.get_keys();
        gameboy.memory.input.update_from_keys(&keys);

        // Run one frame worth of cycles
        gameboy.run_frame();

        // Update window
        window
            .update_with_buffer(&gameboy.gpu.framebuffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .expect("Failed to update window");

        // FPS counter
        frame_count += 1;
        let elapsed = last_time.elapsed();
        if elapsed >= Duration::from_secs(1) {
            println!("FPS: {} | PC: 0x{:04X} | Cycles: {}", 
                frame_count, gameboy.cpu.PC, gameboy.cpu.cycles);
            frame_count = 0;
            last_time = Instant::now();
        }
    }

    println!("Emulator stopped.");
}
