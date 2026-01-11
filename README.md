# Game Boy Emulator (Rust Implementation)

A cycle-accurate Game Boy emulator implemented in Rust with terminal-based display and input support.

## Features

- ✅ **Complete CPU Implementation**: All 256 unprefixed and 256 CB-prefixed Sharp LR35902 instructions
- ✅ **Cycle-Accurate Timing**: Precise cycle counting matching original Game Boy hardware (4.194304 MHz)
- ✅ **Terminal-Based UI (TUI)**: Real-time graphics rendering directly in the terminal using ratatui
- ✅ **Full Input Support**: Keyboard controls for all Game Boy buttons
- ✅ **ROM Loading**: Load and run Game Boy ROM files from command line
- ✅ **Memory Management Unit**: Complete memory map implementation with proper address decoding
- ✅ **Hot Path Architecture**: CPU instruction stepping is the main execution path
- ✅ **Feature Flags**: Optional TUI module for WASM and headless environments
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
- Dependencies: `ratatui`, `crossterm` (enabled by default), `serde`, and `serde_json` (automatically managed by Cargo)
- A terminal emulator that supports:
  - Unicode characters (for half-block rendering: ▀)
  - 256 colors or true color support
  - Common terminals like iTerm2, Alacritty, Windows Terminal, or GNOME Terminal work well

### Build

Build with TUI (default):
```bash
cargo build --release
```

Build without TUI (for WASM or headless environments):
```bash
cargo build --release --no-default-features
```

Build with explicit features:
```bash
# With TUI
cargo build --release --features tui

# Without TUI (same as --no-default-features for now)
cargo build --release --no-default-features
```

### Running the Emulator

Run with the boot ROM (256 bytes) - TUI opens automatically:
```bash
cargo run
```

Run with a Game Boy ROM file:
```bash
cargo run -- path/to/game.gb
```

The emulator will render the Game Boy screen (160x144 pixels) directly in your terminal using Unicode half-block characters, running at 60 FPS.

### Controls

- **Arrow Keys / WASD** - D-Pad
- **Z / J** - A Button
- **X / K** - B Button
- **Enter / I** - Start
- **Backspace / U** - Select
- **Q / ESC** - Quit

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
├── main.rs         - Main entry point with feature flag support
├── tui.rs          - TUI module for terminal-based rendering (enabled by default)
├── cpu.rs          - CPU implementation (hot path)
├── memory.rs       - Memory management unit
├── clock.rs        - Clock and timing system
├── flags.rs        - Flags register implementation
├── gpu.rs          - GPU/PPU for graphics rendering
├── input.rs        - Input handling for Game Boy controls
├── opcodes/        - Opcode definitions
│   ├── mod.rs      - Opcode loader
│   ├── unprefixed.json
│   └── cbprefixed.json
└── tests.rs        - Unit tests
```

## Design Philosophy

1. **CPU as Hot Path**: All instruction execution flows through the CPU module's `step()` method
2. **Cycle Accuracy**: Every instruction tracks its exact cycle count to match original hardware
3. **Real-Time Emulation**: Terminal updates at 60 FPS matching Game Boy refresh rate
4. **Modular TUI**: TUI is feature-flagged for flexibility in different environments (native, WASM, headless)
5. **Terminal Rendering**: Screen is rendered directly in the terminal buffer using Unicode half-block characters
6. **Clean Architecture**: Separation of concerns between CPU, memory, GPU, clock, input, and TUI
7. **User-Friendly**: Simple command-line interface for loading ROMs

## Future Enhancements

The current implementation provides a solid foundation for:
- Enhanced GPU/PPU implementation for more accurate graphics rendering
- Sound Processing Unit (APU) for audio
- More sophisticated interrupt handling
- Save state functionality
- Debugger interface
- Game Boy Color support

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
