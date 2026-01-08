mod clock;
mod cpu;
mod flags;
mod gpu;
#[cfg(feature = "gui")]
mod gui;
mod input;
mod memory;
mod opcodes;
mod tests;

use cpu::CPU;
use std::env;
use std::fs;
use std::path::Path;

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
    
    // Run with GUI if feature is enabled
    #[cfg(feature = "gui")]
    {
        gui::run_gui(cpu);
    }
    
    // Run without GUI (for WASM or headless builds)
    #[cfg(not(feature = "gui"))]
    {
        println!("\nGUI feature is disabled. Running in headless mode...");
        println!("To enable GUI, build with: cargo build --features gui");
        println!("Or use default features: cargo build");
        
        // For headless mode, just run a few cycles as a demonstration
        let mut cycles_count = 0;
        for _ in 0..1000 {
            cycles_count += cpu.step();
        }
        println!("Executed 1000 instructions ({} cycles)", cycles_count);
        println!("Total CPU cycles: {}", cpu.get_ticks());
    }
}
