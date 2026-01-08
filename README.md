# Game Boy Emulator

A cross-platform Game Boy emulator written in Rust that runs the DMG boot ROM and displays output in a native window.

## Features

- ✅ CPU emulation with essential Z80-like instructions
- ✅ Memory management (Boot ROM, ROM, VRAM, WRAM, I/O registers)
- ✅ GPU rendering with tile-based graphics
- ✅ Cross-platform windowing (160x144 native resolution, 4x scaling)
- ✅ Keyboard input support
- ✅ Boot ROM (dmg_boot.bin) execution

## Building

```bash
cargo build --release
```

## Running

Make sure `dmg_boot.bin` is in the project root directory, then:

```bash
cargo run --release
```

## Controls

| Game Boy | Keyboard Primary | Keyboard Alternative |
|----------|-----------------|---------------------|
| D-Pad    | Arrow Keys      | W/A/S/D            |
| A Button | Z               | J                  |
| B Button | X               | K                  |
| Start    | Enter           | I                  |
| Select   | Backspace       | U                  |
| Exit     | ESC             | -                  |

## Architecture

- **CPU**: Implements essential Game Boy CPU instructions
- **Memory**: Full memory map with boot ROM support
- **GPU**: Tile-based rendering to framebuffer
- **Input**: Joypad register emulation with keyboard mapping
- **Window**: Native window using minifb (cross-platform)

## Dependencies

The project uses minimal dependencies:
- `minifb` - Cross-platform framebuffer window library
- `serde` / `serde_json` - For opcode data (currently unused in runtime)

## Limitations

- Only implements essential CPU instructions (enough for boot ROM)
- No audio emulation
- No sprite rendering (only background)
- No cartridge memory bank controllers (MBC)
- No save states
- No Game Boy Color support

## Technical Details

### Display
- Resolution: 160x144 pixels (Game Boy native)
- Window Scale: 4x (640x576)
- Refresh Rate: ~60 FPS
- Colors: 4 shades of gray

### Timing
- Cycles per frame: 70,224 (~59.7 Hz)
- Clock speed: 4.194304 MHz (emulated)

## Project Structure

```
src/
├── main.rs      - Main emulator loop and window management
├── cpu.rs       - CPU implementation and instruction execution
├── memory.rs    - Memory management and addressing
├── gpu.rs       - Graphics rendering and framebuffer
├── input.rs     - Input handling and joypad emulation
├── flags.rs     - CPU flags register
├── clock.rs     - Timing utilities (legacy)
├── opcodes/     - Opcode definitions (legacy)
└── tests.rs     - Unit tests
```

## License

This project is provided as-is for educational purposes.
