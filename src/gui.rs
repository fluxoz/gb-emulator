use crate::cpu::CPU;
use crate::gpu::{GPU, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::input::Input;
use minifb::{Window, WindowOptions, Key};
use std::time::{Duration, Instant};

// Constants for timing
const TARGET_FRAME_TIME_MICROS: u64 = 16666; // ~60 FPS (1/60 second in microseconds)
const CYCLES_PER_FRAME: u128 = 69905; // Game Boy runs at ~4.194 MHz, at 60 FPS that's about 69905 cycles per frame

pub fn run_gui(mut cpu: CPU) {
    println!("\nEmulator started!");
    println!("Controls:");
    println!("  Arrow Keys / WASD - D-Pad");
    println!("  Z / J - A Button");
    println!("  X / K - B Button");
    println!("  Enter / I - Start");
    println!("  Backspace / U - Select");
    println!("  ESC - Quit\n");
    
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
        eprintln!("Error: Unable to create window: {:?}", e);
        
        // Provide detailed troubleshooting based on the error and environment
        let display_var = std::env::var("DISPLAY").ok();
        let wayland_display = std::env::var("WAYLAND_DISPLAY").ok();
        let xdg_session_type = std::env::var("XDG_SESSION_TYPE").ok();
        
        eprintln!("\nTroubleshooting Information:");
        eprintln!("  DISPLAY: {}", display_var.as_deref().unwrap_or("(not set)"));
        eprintln!("  WAYLAND_DISPLAY: {}", wayland_display.as_deref().unwrap_or("(not set)"));
        eprintln!("  XDG_SESSION_TYPE: {}", xdg_session_type.as_deref().unwrap_or("(not set)"));
        
        eprintln!("\nPossible Solutions:");
        
        if xdg_session_type.as_deref() == Some("wayland") || wayland_display.is_some() {
            eprintln!("  1. You're running on Wayland. The emulator needs X11 or XWayland support.");
            eprintln!("     - Ensure XWayland is installed: sudo apt install xwayland (Debian/Ubuntu)");
            eprintln!("     - Try setting DISPLAY if XWayland is running: export DISPLAY=:0");
            eprintln!("     - Or force X11 backend with: GDK_BACKEND=x11 cargo run");
            if display_var.is_none() {
                eprintln!("     - Note: DISPLAY variable is not set, which is needed for X11/XWayland");
            }
        } else if display_var.is_none() && wayland_display.is_none() {
            eprintln!("  1. No display environment detected. You may be running in a headless environment.");
            eprintln!("     - For headless/CI: Use a virtual display with: xvfb-run cargo run");
            eprintln!("     - For SSH: Enable X11 forwarding with: ssh -X user@host");
            eprintln!("     - If you have a desktop environment, ensure DISPLAY is set: export DISPLAY=:0");
        } else if display_var.is_some() {
            eprintln!("  1. DISPLAY is set but window creation failed. This could mean:");
            eprintln!("     - X server is not running or not accessible");
            eprintln!("     - Permission issues: Try 'xhost +local:' or 'xhost +SI:localuser:$(whoami)'");
            eprintln!("     - X11 libraries missing: sudo apt install libx11-dev libxrandr-dev");
            eprintln!("     - Display server crashed or isn't responding");
        }
        
        eprintln!("\n  General tips:");
        eprintln!("    - Check if you can run other GUI applications (e.g., xterm, xeyes)");
        eprintln!("    - Verify your display manager is running");
        eprintln!("    - Review system logs for display/graphics errors");
        
        std::process::exit(1);
    });
    
    // Limit to 60 FPS (approximately Game Boy refresh rate)
    window.set_target_fps(60);
    
    let mut last_frame_time = Instant::now();
    let target_frame_time = Duration::from_micros(TARGET_FRAME_TIME_MICROS);
    
    // Main emulation loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update input state
        let keys = window.get_keys();
        input.update_from_keys(&keys);
        
        // Run CPU for one frame's worth of cycles
        let start_cycles = cpu.get_ticks();
        
        while cpu.get_ticks() - start_cycles < CYCLES_PER_FRAME {
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
