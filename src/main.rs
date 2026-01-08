mod clock;
mod cpu;
mod flags;
mod gpu;
mod memory;
mod opcodes;
mod tests;

use cpu::CPU;
use std::fs;
use std::path::Path;

// Demo configuration constants
const MAX_DEMO_STEPS: usize = 10000;

fn main() {
    println!("Game Boy Emulator - Boot ROM Demo");
    println!("===================================\n");
    
    // Create a CPU instance
    let mut cpu = CPU::new();
    
    // Try to load boot ROM
    let boot_rom_path = "dmg_boot.bin";
    if Path::new(boot_rom_path).exists() {
        match fs::read(boot_rom_path) {
            Ok(boot_data) => {
                println!("Loading boot ROM from {}...", boot_rom_path);
                cpu.load_boot_rom(&boot_data);
                println!("Boot ROM loaded successfully ({} bytes)\n", boot_data.len());
            }
            Err(e) => {
                eprintln!("Error reading boot ROM: {}", e);
                println!("Continuing without boot ROM\n");
            }
        }
    } else {
        println!("Boot ROM not found at {}", boot_rom_path);
        println!("Continuing with test program\n");
        
        // Create a test ROM demonstrating various instruction types
        let mut test_rom = vec![0; 0x100]; // Header area
        test_rom.extend_from_slice(&[
            // Demonstrate 8-bit loads
            0x3E, 0x42,       // LD A, $42
            0x06, 0x10,       // LD B, $10
            0x0E, 0x20,       // LD C, $20
            
            // Demonstrate arithmetic
            0x80,             // ADD A, B (A = $42 + $10 = $52)
            0x91,             // SUB C (A = $52 - $20 = $32)
            
            // Demonstrate 16-bit operations
            0x01, 0x34, 0x12, // LD BC, $1234
            0x21, 0x00, 0xC0, // LD HL, $C000
            
            // Demonstrate increment/decrement
            0x04,             // INC B
            0x0D,             // DEC C
            
            // Demonstrate jump
            0x18, 0x02,       // JR +2 (skip next 2 bytes)
            0x00, 0x00,       // NOPs (skipped)
            
            // Demonstrate stack operations
            0xC5,             // PUSH BC
            0xC1,             // POP BC
            
            // Demonstrate CB prefix (bit operations)
            0xCB, 0x47,       // BIT 0, A
            
            0x76,             // HALT
        ]);
        
        cpu.load_rom(test_rom);
    }
    
    println!("Executing program...\n");
    println!("PC       Cycles  Total Ticks  Instruction");
    println!("-------------------------------------------");
    
    let mut step = 0;
    loop {
        let pc = cpu.get_pc();
        let cycles = cpu.step();
        let total = cpu.get_ticks();
        
        step += 1;
        
        // Show first 20 and last few instructions
        if step <= 20 || step > MAX_DEMO_STEPS - 5 {
            // Determine instruction type from cycles (rough approximation)
            let instr_type = match cycles {
                4 => "Single byte/HALT",
                8 => "8-bit load/ALU",
                12 => "16-bit load/JP",
                16 => "Stack/CB prefix",
                _ => "Complex op",
            };
            
            println!("0x{:04X}   {:2}      {:5}        {}", pc, cycles, total, instr_type);
        } else if step == 21 {
            println!("... (continuing execution) ...");
        }
        
        if step > MAX_DEMO_STEPS || (cycles == 4 && step > 10) {
            // Stop after HALT or max steps
            println!("\n[Execution stopped after {} steps]", step);
            break;
        }
    }
    
    println!("\n===================================");
    println!("Execution complete!");
    println!("Total CPU cycles: {}", cpu.get_ticks());
    println!("Final PC: 0x{:04X}", cpu.get_pc());
    println!("\nThe CPU executes with cycle-accurate timing");
    println!("matching the original Game Boy hardware.");
}
