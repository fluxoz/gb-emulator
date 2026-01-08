# Game Boy Emulator (Rust Implementation)

A cycle-accurate Game Boy emulator implemented in Rust with full window-based display and input support.

## Features

- ✅ **Complete CPU Implementation**: All 256 unprefixed and 256 CB-prefixed Sharp LR35902 instructions
- ✅ **Cycle-Accurate Timing**: Precise cycle counting matching original Game Boy hardware (4.194304 MHz)
- ✅ **Cross-Platform GUI**: Real-time graphics rendering with minifb supporting Wayland and X11 out of the box
- ✅ **Full Input Support**: Keyboard controls for all Game Boy buttons
- ✅ **ROM Loading**: Load and run Game Boy ROM files from command line
- ✅ **Memory Management Unit**: Complete memory map implementation with proper address decoding
- ✅ **Hot Path Architecture**: CPU instruction stepping is the main execution path
- ✅ **Feature Flags**: Optional GUI module for WASM and headless environments
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
- Dependencies: `minifb` (optional, enabled by default), `serde`, and `serde_json` (automatically managed by Cargo)
- For Linux: The GUI supports both Wayland and X11 out of the box. On Wayland systems, you may need `libwayland-dev` and `libwayland-cursor0` packages installed.

### Build

Build with GUI (default):
```bash
cargo build --release
```

Build without GUI (for WASM or headless environments):
```bash
cargo build --release --no-default-features
```

Build with explicit features:
```bash
# With GUI
cargo build --release --features gui

# Without GUI (same as --no-default-features for now)
cargo build --release --no-default-features
```

### Running the Emulator

Run with the boot ROM (256 bytes) - GUI opens automatically:
```bash
cargo run
```

Run with a Game Boy ROM file:
```bash
cargo run -- path/to/game.gb
```

The emulator will open a window displaying the Game Boy screen at 4x scale, running at 60 FPS. The GUI supports both Wayland and X11 on Linux out of the box.

#### Running in Headless/CI Environments

If you're running in a headless environment (GitHub Actions, SSH without X11, Docker, etc.), you'll need a virtual display:

**Using Xvfb (X Virtual Framebuffer):**
```bash
# Install Xvfb (if not already installed)
sudo apt-get install xvfb  # Debian/Ubuntu
sudo yum install xorg-x11-server-Xvfb  # RHEL/CentOS

# Run with Xvfb
xvfb-run cargo run
```

**For GitHub Actions workflows:**
```yaml
- name: Run emulator
  run: xvfb-run cargo run
```

**Alternative: Build without GUI for headless environments:**
```bash
cargo run --no-default-features
```

This will build and run the emulator without the GUI, which is useful for automated testing or environments where a display is not available.

### GUI and Feature Flags

The GUI is implemented as an optional module that is enabled by default. This allows the emulator to be built for different environments:

**Default behavior (GUI enabled):**
```bash
cargo run
# or
cargo run --features gui
```

**Headless mode (for WASM or other non-GUI environments):**
```bash
cargo run --no-default-features
```

The feature flag architecture makes it possible to:
- Build for WASM without GUI dependencies
- Create headless builds for automated testing
- Reduce binary size when GUI is not needed

### Controls

- **Arrow Keys / WASD** - D-Pad
- **Z / J** - A Button
- **X / K** - B Button
- **Enter / I** - Start
- **Backspace / U** - Select
- **ESC** - Quit

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

## Troubleshooting

### "Unable to create window: Failed to create window"

This error occurs when running in a headless environment without a display server. Solutions:

1. **Use Xvfb** (recommended for CI/headless):
   ```bash
   xvfb-run cargo run
   ```

2. **Build without GUI**:
   ```bash
   cargo run --no-default-features
   ```

3. **SSH with X11 forwarding**:
   ```bash
   ssh -X user@host
   cargo run
   ```

4. **Check DISPLAY variable**:
   ```bash
   echo $DISPLAY  # Should output something like :0 or :1
   export DISPLAY=:0  # Set if needed
   ```

### Wayland vs X11 on Linux

The emulator's GUI (minifb) supports both Wayland and X11 automatically. If you experience issues:

- On Wayland systems, ensure `libwayland-dev` and `libwayland-cursor0` are installed
- On X11 systems, ensure `libx11-dev` and related X11 libraries are installed
- Use `echo $XDG_SESSION_TYPE` to check which display server you're using

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
├── gui.rs          - GUI module (optional, enabled by default)
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
3. **Real-Time Emulation**: Window updates at 60 FPS matching Game Boy refresh rate
4. **Modular GUI**: GUI is feature-flagged for flexibility in different environments (native, WASM, headless)
5. **Cross-Platform**: GUI supports Wayland and X11 on Linux out of the box
6. **Clean Architecture**: Separation of concerns between CPU, memory, GPU, clock, input, and GUI
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
