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
        let wayland_display = std::env::var("WAYLAND_DISPLAY").ok();
        let xdg_runtime_dir = std::env::var("XDG_RUNTIME_DIR").ok();
        let xdg_session_type = std::env::var("XDG_SESSION_TYPE").ok();
        
        eprintln!("\nTroubleshooting Information:");
        eprintln!("  WAYLAND_DISPLAY: {}", wayland_display.as_deref().unwrap_or("(not set)"));
        eprintln!("  XDG_RUNTIME_DIR: {}", xdg_runtime_dir.as_deref().unwrap_or("(not set)"));
        eprintln!("  XDG_SESSION_TYPE: {}", xdg_session_type.as_deref().unwrap_or("(not set)"));
        
        eprintln!("\nNote: This emulator requires Wayland support.");
        eprintln!("\nPossible Solutions:");
        
        if wayland_display.is_none() && xdg_session_type.as_deref() != Some("wayland") {
            eprintln!("  1. You don't appear to be running Wayland. This emulator requires Wayland.");
            eprintln!("     - Check if you're in a Wayland session: echo $XDG_SESSION_TYPE");
            eprintln!("     - If using GNOME/KDE, log out and select 'Wayland' session at login screen");
            eprintln!("     - Alternatively, start a Wayland compositor like:");
            eprintln!("       * GNOME (Wayland): Log out and select 'GNOME' (not 'GNOME on Xorg')");
            eprintln!("       * KDE Plasma: Log out and select 'Plasma (Wayland)'");
            eprintln!("       * Sway: Start with 'sway' command");
        } else if wayland_display.is_some() {
            eprintln!("  1. Wayland is detected, but connection failed. Possible causes:");
            eprintln!("     - Wayland runtime libraries are missing. Install them:");
            eprintln!("       * Debian/Ubuntu: sudo apt install libwayland-client0 libxkbcommon0");
            eprintln!("       * Fedora/RHEL: sudo dnf install wayland libwayland-client libxkbcommon");
            eprintln!("       * Arch: sudo pacman -S wayland libxkbcommon");
            if let Some(runtime_dir) = xdg_runtime_dir.as_ref() {
                if let Some(wl_display) = wayland_display.as_ref() {
                    let socket_path = format!("{}/{}", runtime_dir, wl_display);
                    eprintln!("     - Verify Wayland socket exists: ls -la {}", socket_path);
                    eprintln!("       If it doesn't exist, your Wayland compositor may not be running properly");
                }
            } else {
                eprintln!("     - XDG_RUNTIME_DIR is not set, which is required for Wayland");
                eprintln!("       This should be set automatically by your login manager");
            }
            eprintln!("     - Your Wayland compositor may have crashed or isn't responding");
            eprintln!("     - Try restarting your Wayland session");
        } else if xdg_runtime_dir.is_none() {
            eprintln!("  1. XDG_RUNTIME_DIR is not set, which is required for Wayland.");
            eprintln!("     - This should be set automatically by your login manager");
            eprintln!("     - Try logging out and back in");
        } else {
            eprintln!("  1. No display environment detected. You may be running in a headless environment.");
            eprintln!("     - For headless/CI: Wayland requires a compositor; consider using weston-headless or similar");
            eprintln!("     - For SSH: Wayland doesn't support remote forwarding like X11");
        }
        
        eprintln!("\n  General tips:");
        eprintln!("    - Check if you can run other Wayland applications (e.g., wayland-info, weston-info)");
        eprintln!("    - Verify your Wayland compositor is running: ps aux | grep -E '(gnome-shell|kwin|sway|weston)'");
        eprintln!("    - Review system logs for compositor/graphics errors: journalctl -xe");
        
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
