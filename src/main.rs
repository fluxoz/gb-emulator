mod clock;
mod cpu;
mod flags;
mod gpu;
mod memory;
mod opcodes;
mod tests;

use cpu::CPU;

fn main() {
    println!("Game Boy Emulator - Starting CPU");
    
    // Create a CPU instance
    let mut cpu = CPU::new();
    
    // Create a test ROM with proper padding
    let mut test_rom = vec![0; 0x100]; // Start at 0x100 after header
    test_rom.extend_from_slice(&[
        0x00,       // NOP at 0x100
        0x3E, 0x42, // LD A, $42 at 0x101-0x102
        0x06, 0x10, // LD B, $10 at 0x103-0x104
        0x0E, 0x20, // LD C, $20 at 0x105-0x106
        0x04,       // INC B at 0x107
        0x05,       // DEC B at 0x108
        0x76,       // HALT at 0x109
    ]);
    
    cpu.load_rom(test_rom);
    
    // Execute instructions
    println!("Starting execution at PC: 0x{:04X}", cpu.get_pc());
    
    for i in 0..9 {
        let pc_before = cpu.get_pc();
        let cycles = cpu.step();
        println!("Step {}: PC was 0x{:04X}, executed with {} cycles, total ticks={}", 
                 i + 1, pc_before, cycles, cpu.get_ticks());
    }
    
    println!("Execution complete. Final PC: 0x{:04X}", cpu.get_pc());
}
