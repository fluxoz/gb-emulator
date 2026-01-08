mod clock;
mod cpu;
mod flags;
mod gpu;
mod input;
mod memory;
mod opcodes;
mod tests;

use cpu::CPU;
use gpu::{GPU, SCREEN_WIDTH, SCREEN_HEIGHT};
use input::Input;
use minifb::{Window, WindowOptions, Key};
use std::env;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

fn main() {
    println!("Game Boy Emulator");
    println!("==================\n");
    
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let rom_path = if args.len() > 1 {
        args[1].clone()
    } else {
        // Default to boot ROM if no argument provided
        "dmg_boot.bin".to_string()
    };
    
    // Create a CPU instance
    let mut cpu = CPU::new();
    
    // Load ROM file
    if Path::new(&rom_path).exists() {
        match fs::read(&rom_path) {
            Ok(rom_data) => {
                println!("Loading ROM from {}...", rom_path);
                
                // Check if this is a boot ROM (256 bytes) or a game ROM
                if rom_data.len() == 256 {
                    cpu.load_boot_rom(&rom_data);
                    println!("Boot ROM loaded successfully ({} bytes)", rom_data.len());
                } else {
                    cpu.load_rom(rom_data.clone());
                    println!("Game ROM loaded successfully ({} bytes)", rom_data.len());
                }
            }
            Err(e) => {
                eprintln!("Error reading ROM file {}: {}", rom_path, e);
                eprintln!("Usage: cargo run -- <path_to_rom.gb>");
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("ROM file not found: {}", rom_path);
        eprintln!("Usage: cargo run -- <path_to_rom.gb>");
        std::process::exit(1);
    }
    
    // Create GPU
    let mut gpu = GPU::new();
    
    // Create input handler
    let mut input = Input::new();
    
    // Create window
    let mut window = Window::new(
        "Game Boy Emulator",
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WindowOptions {
            resize: false,
            scale: minifb::Scale::X4,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        eprintln!("Error: Unable to create window: {}", e);
        eprintln!("\nThis emulator requires a graphical display environment to run.");
        eprintln!("If you're running in a headless environment (CI, SSH without X11, etc.),");
        eprintln!("you'll need to set up a virtual display (Xvfb) or run with a display server.\n");
        eprintln!("For Linux with Xvfb:");
        eprintln!("  xvfb-run cargo run");
        eprintln!("\nFor SSH with X11 forwarding:");
        eprintln!("  ssh -X user@host");
        eprintln!("  export DISPLAY=:0");
        std::process::exit(1);
    });
    
    // Limit to 60 FPS (approximately Game Boy refresh rate)
    window.set_target_fps(60);
    
    println!("\nEmulator started!");
    println!("Controls:");
    println!("  Arrow Keys / WASD - D-Pad");
    println!("  Z / J - A Button");
    println!("  X / K - B Button");
    println!("  Enter / I - Start");
    println!("  Backspace / U - Select");
    println!("  ESC - Quit\n");
    
    let mut last_frame_time = Instant::now();
    let target_frame_time = Duration::from_micros(16666); // ~60 FPS
    
    // Main emulation loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update input state
        let keys = window.get_keys();
        input.update_from_keys(&keys);
        
        // Run CPU for one frame's worth of cycles
        // Game Boy runs at ~4.194 MHz, at 60 FPS that's about 69905 cycles per frame
        let target_cycles = 69905;
        let start_cycles = cpu.get_ticks();
        
        while cpu.get_ticks() - start_cycles < target_cycles {
            let cycles = cpu.step();
            gpu.step(cycles, cpu.get_memory());
        }
        
        // Update window with framebuffer
        window
            .update_with_buffer(&gpu.framebuffer, SCREEN_WIDTH, SCREEN_HEIGHT)
            .unwrap();
        
        // Frame timing
        let elapsed = last_frame_time.elapsed();
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
        last_frame_time = Instant::now();
    }
    
    println!("\nEmulator closed.");
    println!("Total CPU cycles: {}", cpu.get_ticks());
}
