use crate::cpu::CPU;
use crate::gpu::{GPU, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::input::Input;
use minifb::{Window, WindowOptions, Key};
use std::time::{Duration, Instant};

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
        eprintln!("Error: Unable to create window: {}", e);
        eprintln!("\nThis emulator requires a graphical display environment to run.");
        eprintln!("If you're running in a headless environment (CI, SSH without X11, etc.),");
        eprintln!("you'll need to set up a virtual display or configure display permissions.\n");
        eprintln!("Common solutions:");
        eprintln!("  - Linux with Xvfb: xvfb-run cargo run");
        eprintln!("  - SSH with X11 forwarding: ssh -X user@host (DISPLAY set automatically)");
        eprintln!("  - Check display permissions: xhost +local:");
        std::process::exit(1);
    });
    
    // Limit to 60 FPS (approximately Game Boy refresh rate)
    window.set_target_fps(60);
    
    let mut last_frame_time = Instant::now();
    let target_frame_time = Duration::from_micros(16666); // ~60 FPS
    
    // Main emulation loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update input state
        let keys = window.get_keys();
        input.update_from_keys(&keys);
        
        // Run CPU for one frame's worth of cycles
        // Game Boy runs at ~4.194 MHz, at 60 FPS that's about 69905 cycles per frame
        let target_cycles = 69905;
        let start_cycles = cpu.get_ticks();
        
        while cpu.get_ticks() - start_cycles < target_cycles {
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
