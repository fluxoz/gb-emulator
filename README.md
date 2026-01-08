# Game Boy Emulator (Rust Implementation)

A cycle-accurate Game Boy emulator implemented in Rust, focusing on precise CPU emulation with the CPU module as the hot path.

## Features

- ✅ **Complete CPU Implementation**: All 256 unprefixed and 256 CB-prefixed Sharp LR35902 instructions
- ✅ **Cycle-Accurate Timing**: Precise cycle counting matching original Game Boy hardware (4.194304 MHz)
- ✅ **Memory Management Unit**: Complete memory map implementation with proper address decoding
- ✅ **Hot Path Architecture**: CPU instruction stepping is the main execution path
- ✅ **Comprehensive Testing**: 42 unit tests covering instruction execution and timing

## Architecture

### CPU Module (Hot Path)

The CPU module (`src/cpu.rs`) is the core of the emulator where all instruction execution happens:

```
Execution Flow:
1. Fetch opcode from memory at PC
2. Decode opcode using match statement dispatch
3. Execute instruction with precise cycle count
4. Update CPU state (registers, flags, memory)
5. Advance PC and track clock cycles
```

The `step()` method is the hot path - it's called repeatedly to execute instructions one at a time with accurate timing.

### Memory Management

The MMU (`src/memory.rs`) implements the complete Game Boy memory map:

- **0x0000-0x00FF**: Boot ROM (switchable)
- **0x0000-0x3FFF**: ROM Bank 0
- **0x4000-0x7FFF**: ROM Bank 1-N (switchable)
- **0x8000-0x9FFF**: VRAM (8KB)
- **0xA000-0xBFFF**: External RAM
- **0xC000-0xDFFF**: Work RAM (8KB)
- **0xE000-0xFDFF**: Echo RAM
- **0xFE00-0xFE9F**: OAM (Sprite Attributes)
- **0xFEA0-0xFEFF**: Not usable
- **0xFF00-0xFF7F**: I/O Registers
- **0xFF80-0xFFFE**: High RAM
- **0xFFFF**: Interrupt Enable Register

### Clock System

The clock module (`src/clock.rs`) tracks CPU cycles with nanosecond precision:
- Maintains total tick count for cycle-accurate emulation
- Integrates with CPU stepping for precise timing
- Clock speed: 4.194304 MHz (original Game Boy)

## Building and Running

### Prerequisites

- Rust 2024 edition or later
- No external dependencies except `serde` and `serde_json` (already in project)

### Build

```bash
cargo build --release
```

### Run Demo

```bash
cargo run
```

The demo program executes a test ROM showing various instruction types with cycle-accurate timing.

### Run Tests

```bash
cargo test
```

All 42 tests should pass, covering:
- Flag register operations
- Instruction execution (NOP, LD, INC/DEC, arithmetic, logic)
- 16-bit operations
- Stack operations (PUSH/POP)
- Jump and branch instructions
- CB-prefixed bit operations
- Timing accuracy

## Implementation Details

### Registers

- **8-bit registers**: A (accumulator), B, C, D, E, H, L, F (flags)
- **16-bit registers**: SP (stack pointer), PC (program counter)
- **Register pairs**: BC, DE, HL (can be accessed as 16-bit values)

### Flags (F Register)

- **Z**: Zero flag (bit 7)
- **N**: Subtract flag (bit 6)
- **H**: Half-carry flag (bit 5)
- **C**: Carry flag (bit 4)

### Instruction Set

All Game Boy CPU instructions are implemented:

- **Load/Store**: LD, LDH, PUSH, POP
- **Arithmetic**: ADD, ADC, SUB, SBC, INC, DEC
- **Logic**: AND, OR, XOR, CP
- **Rotate/Shift**: RLCA, RLA, RRCA, RRA, RLC, RL, RRC, RR, SLA, SRA, SRL
- **Bit Operations**: BIT, SET, RES, SWAP
- **Jumps**: JP, JR, CALL, RET, RST
- **Control**: NOP, HALT, STOP, DI, EI, CCF, SCF, DAA, CPL

### Timing

Each instruction executes with cycle-accurate timing:
- Single-byte operations: 4 cycles
- 8-bit loads/ALU: 8 cycles
- 16-bit loads: 12 cycles
- Memory operations: 8-12 cycles
- Jumps/calls: 12-24 cycles
- CB-prefixed: 8-16 cycles

## Project Structure

```
src/
├── main.rs         - Demo program and module declarations
├── cpu.rs          - CPU implementation (hot path)
├── memory.rs       - Memory management unit
├── clock.rs        - Clock and timing system
├── flags.rs        - Flags register implementation
├── opcodes/        - Opcode definitions
│   ├── mod.rs      - Opcode loader
│   ├── unprefixed.json
│   └── cbprefixed.json
├── tests.rs        - Unit tests
└── gpu.rs          - GPU skeleton (future implementation)
```

## Design Philosophy

1. **CPU as Hot Path**: All instruction execution flows through the CPU module's `step()` method
2. **Cycle Accuracy**: Every instruction tracks its exact cycle count to match original hardware
3. **Minimal Dependencies**: Only stdlib plus serde/serde_json for opcode data
4. **Clean Architecture**: Separation of concerns between CPU, memory, clock, and peripherals
5. **No Reorganization**: Implementation built on existing project structure

## Future Enhancements

The current implementation provides a solid foundation for:
- GPU/PPU implementation for graphics rendering
- Sound Processing Unit (APU) for audio
- Joypad input handling
- Interrupt handling system
- Save state functionality
- Debugger interface

## Performance

The emulator is designed for accuracy over speed, but the hot path architecture ensures efficient instruction execution. The CPU `step()` method is optimized for:
- Direct opcode dispatch via match statements (no table lookups)
- Inline register access
- Minimal memory allocations
- Precise cycle counting without overhead

## License

See project license file for details.

## Acknowledgments

- Opcode data sourced from Game Boy opcode reference
- CPU architecture based on Sharp LR35902 documentation
- Timing information from Pan Docs and other Game Boy technical references
