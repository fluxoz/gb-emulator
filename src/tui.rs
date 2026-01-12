use crate::cpu::CPU;
use crate::gpu::{GPU, SCREEN_WIDTH, SCREEN_HEIGHT};
use crate::input::Input;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use std::time::{Duration, Instant};

// Constants for timing
const TARGET_FRAME_TIME_MICROS: u64 = 16666; // ~60 FPS (1/60 second in microseconds)
const CYCLES_PER_FRAME: u128 = 69905; // Game Boy runs at ~4.194 MHz, at 60 FPS that's about 69905 cycles per frame

pub fn run_tui(mut cpu: CPU) -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create GPU
    let mut gpu = GPU::new();
    
    // Create input handler
    let mut input = Input::new();

    let mut last_frame_time = Instant::now();
    let target_frame_time = Duration::from_micros(TARGET_FRAME_TIME_MICROS);

    let mut running = true;
    let start_time = Instant::now();

    // Main emulation loop
    while running {
        // Run CPU for one frame's worth of cycles
        let start_cycles = cpu.get_ticks();
        let mut cycles_executed = 0;
        
        // Execute cycles in small chunks to remain responsive to input
        while cycles_executed < CYCLES_PER_FRAME {
            let cycles = cpu.step();
            gpu.step(cycles, cpu.get_memory_mut());
            cycles_executed = cpu.get_ticks() - start_cycles;
            
            // Check for input less frequently (every ~10000 cycles instead of 1000)
            // This reduces overhead significantly
            if cycles_executed % 10000 < 100 && event::poll(Duration::from_millis(0))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                running = false;
                                break;
                            }
                            _ => {}
                        }
                    }
                    input.update_from_key_event(key);
                }
            }
            
            // Yield CPU occasionally to prevent busy-wait hogging
            // This significantly reduces CPU usage
            if cycles_executed % 5000 == 0 {
                std::thread::yield_now();
            }
        }

        // Render to terminal
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(1),
                    Constraint::Length(3),
                ])
                .split(f.area());

            // Render the Game Boy screen
            render_screen(f, chunks[0], &gpu.framebuffer);

            // Render status bar
            let elapsed = start_time.elapsed();
            let status = format!(
                "GB Emulator | Cycles: {} | Time: {:.1}s | Controls: Arrow/WASD=D-Pad Z/J=A X/K=B Enter/I=Start Bksp/U=Select Q/ESC=Quit",
                cpu.get_ticks(),
                elapsed.as_secs_f32()
            );
            let status_paragraph = Paragraph::new(status)
                .style(Style::default().fg(Color::Cyan))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(status_paragraph, chunks[1]);
        })?;

        // Frame timing
        let elapsed = last_frame_time.elapsed();
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
        last_frame_time = Instant::now();
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Now print messages after terminal is restored
    eprintln!("\nEmulator closed.");
    eprintln!("Total CPU cycles: {}", cpu.get_ticks());

    Ok(())
}

fn render_screen(f: &mut ratatui::Frame, area: Rect, framebuffer: &[u32; SCREEN_WIDTH * SCREEN_HEIGHT]) {
    // Calculate how to fit the Game Boy screen into the terminal area
    // Game Boy is 160x144 pixels
    // We'll use Unicode block characters (▀ ▄ █) to represent 2 vertical pixels per character
    // This gives us 160x72 characters for the full screen
    
    let screen_height_chars = (SCREEN_HEIGHT / 2) as u16; // 72 rows (2 pixels per char)
    let screen_width_chars = SCREEN_WIDTH as u16; // 160 columns
    
    // Create a centered area for the screen
    let screen_area = centered_rect(
        screen_width_chars + 2,  // +2 for borders
        screen_height_chars + 2, // +2 for borders
        area
    );

    let block = Block::default()
        .title("Game Boy Screen")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));
    
    let inner = block.inner(screen_area);
    f.render_widget(block, screen_area);

    // Render the framebuffer using half-block characters
    let mut lines: Vec<Line> = Vec::new();
    
    for y in (0..SCREEN_HEIGHT).step_by(2) {
        let mut spans: Vec<Span> = Vec::new();
        
        for x in 0..SCREEN_WIDTH {
            let top_pixel = framebuffer[y * SCREEN_WIDTH + x];
            let bottom_pixel = if y + 1 < SCREEN_HEIGHT {
                framebuffer[(y + 1) * SCREEN_WIDTH + x]
            } else {
                top_pixel
            };
            
            let (ch, fg, bg) = get_half_block_char(top_pixel, bottom_pixel);
            spans.push(Span::styled(ch.to_string(), Style::default().fg(fg).bg(bg)));
        }
        
        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner);
}

fn get_half_block_char(top_pixel: u32, bottom_pixel: u32) -> (char, Color, Color) {
    let top_color = pixel_to_color(top_pixel);
    let bottom_color = pixel_to_color(bottom_pixel);
    
    // Use upper half block (▀) with fg=top, bg=bottom
    ('▀', top_color, bottom_color)
}

fn pixel_to_color(pixel: u32) -> Color {
    match pixel {
        0xFFFFFF => Color::White,      // White
        0xAAAAAA => Color::Gray,       // Light Gray
        0x555555 => Color::DarkGray,   // Dark Gray
        0x000000 => Color::Black,      // Black
        _ => Color::White,
    }
}

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Length((r.height.saturating_sub(height)) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width.saturating_sub(width)) / 2),
            Constraint::Length(width),
            Constraint::Length((r.width.saturating_sub(width)) / 2),
        ])
        .split(popup_layout[1])[1]
}
