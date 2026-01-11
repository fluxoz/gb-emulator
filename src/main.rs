mod clock;
mod cpu;
mod flags;
mod gpu;
#[cfg(feature = "tui")]
mod tui;
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
    
    // Create a CPU instance
    let mut cpu = CPU::new();
    
    // Always try to load the boot ROM first for proper initialization
    let boot_rom_path = "dmg_boot.bin";
    if Path::new(boot_rom_path).exists() {
        match fs::read(boot_rom_path) {
            Ok(boot_data) => {
                if boot_data.len() == 256 {
                    cpu.load_boot_rom(&boot_data);
                    println!("Boot ROM loaded successfully ({} bytes)", boot_data.len());
                } else {
                    eprintln!("Warning: Boot ROM file is not 256 bytes, skipping boot ROM");
                }
            }
            Err(e) => {
                eprintln!("Warning: Could not read boot ROM ({}): {}", boot_rom_path, e);
                eprintln!("Continuing without boot ROM...");
            }
        }
    } else {
        eprintln!("Warning: Boot ROM not found at {}", boot_rom_path);
        eprintln!("Continuing without boot ROM...");
    }
    
    // Load game ROM if provided
    if args.len() > 1 {
        let rom_path = &args[1];
        if Path::new(rom_path).exists() {
            match fs::read(rom_path) {
                Ok(rom_data) => {
                    println!("Loading game ROM from {}...", rom_path);
                    
                    // Game ROMs should be at least 32KB
                    if rom_data.len() >= 32768 {
                        cpu.load_rom(rom_data.clone());
                        println!("Game ROM loaded successfully ({} bytes)", rom_data.len());
                    } else {
                        eprintln!("Error: ROM file is too small ({} bytes). Game ROMs should be at least 32KB.", rom_data.len());
                        std::process::exit(1);
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
    } else {
        println!("No game ROM specified. Running boot ROM only.");
        println!("Usage: cargo run -- <path_to_rom.gb>");
    }
    
    // Run with TUI if feature is enabled
    #[cfg(feature = "tui")]
    {
        if let Err(e) = tui::run_tui(cpu) {
            eprintln!("TUI Error: {}", e);
            std::process::exit(1);
        }
    }
    
    // Run without TUI (for WASM or headless builds)
    #[cfg(not(feature = "tui"))]
    {
        println!("\nTUI feature is disabled. Build completed successfully!");
        println!("To enable TUI, build with: cargo build --features tui");
        println!("Or use default features: cargo build");
        println!("\nThis headless build is suitable for WASM or other non-TUI environments.");
        println!("Total CPU cycles initialized: {}", cpu.get_ticks());
    }
}
