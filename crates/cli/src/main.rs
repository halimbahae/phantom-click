use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use crossterm::{execute, queue};
use phantom_click::{clicker, cursor};

const MATRIX_CHARS: &[u8] = b"0123456789ABCDEF";

fn print_usage() {
    eprintln!("Phantom Click - auto-clicker with global hotkeys");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("  phantom-click [CPS]");
    eprintln!();
    eprintln!("ARGS:");
    eprintln!("  <CPS>    Initial clicks per second (1-200, default: 10)");
    eprintln!();
    eprintln!("CONTROLS (work globally in any app):");
    eprintln!("  C       Toggle clicking ON/OFF");
    eprintln!("  Z       +1 CPS (speed up)");
    eprintln!("  S       -1 CPS (slow down)");
    eprintln!("  H       Toggle help overlay");
    eprintln!("  Q/Esc   Quit");
    eprintln!();
    eprintln!("PERMISSION:");
    eprintln!("  Requires Accessibility access on macOS.");
    eprintln!("  System Settings -> Privacy & Security -> Accessibility");
}

fn main() {
    if std::env::args().any(|a| a == "--help" || a == "-h") {
        print_usage();
        return;
    }

    let _ = enable_raw_mode();
    let _ = execute!(stdout(), crossterm::terminal::EnterAlternateScreen);

    let initial_cps: u64 = std::env::args()
        .skip(1)
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10)
        .max(1)
        .min(200);

    let clicking = Arc::new(AtomicBool::new(false));
    let cps_val = Arc::new(AtomicU64::new(initial_cps));
    let should_quit = Arc::new(AtomicBool::new(false));
    let show_help = Arc::new(AtomicBool::new(false));

    clicker::spawn_clicker(Arc::clone(&clicking), Arc::clone(&cps_val));

    let listen_clicking = Arc::clone(&clicking);
    let listen_cps = Arc::clone(&cps_val);
    let listen_quit = Arc::clone(&should_quit);
    let listen_help = Arc::clone(&show_help);
    std::thread::spawn(move || {
        if let Err(e) = rdev::listen(move |event| {
            use rdev::Key::*;
            if let rdev::EventType::KeyPress(key) = event.event_type {
                match key {
                    KeyC => { listen_clicking.fetch_xor(true, Ordering::SeqCst); }
                    KeyZ => {
                        let _ = listen_cps.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |c| {
                            Some((c + 1).min(200))
                        });
                    }
                    KeyS => {
                        let _ = listen_cps.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |c| {
                            Some(c.saturating_sub(1).max(1))
                        });
                    }
                    KeyH => { listen_help.fetch_xor(true, Ordering::SeqCst); }
                    KeyQ | Escape => { listen_quit.store(true, Ordering::SeqCst); }
                    _ => {}
                }
            }
        }) {
            eprintln!("rdev error: {:?}", e);
        }
    });

    crossterm::queue!(stdout(), crossterm::style::SetForegroundColor(
        crossterm::style::Color::Rgb { r: 0, g: 255, b: 64 },
    )).ok();

    let mut tick: u64 = 0;
    while !should_quit.load(Ordering::Relaxed) {
        let active = clicking.load(Ordering::Relaxed);
        let cps = cps_val.load(Ordering::Relaxed);
        let (mx, my) = cursor::position();
        let help = show_help.load(Ordering::Relaxed);
        draw_ui(cps, active, mx as u64, my as u64, tick, help);
        std::thread::sleep(Duration::from_millis(100));
        tick = tick.wrapping_add(1);
    }

    clicking.store(false, Ordering::SeqCst);
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), crossterm::terminal::LeaveAlternateScreen);
}

fn draw_ui(cps: u64, active: bool, mx: u64, my: u64, tick: u64, help: bool) {
    let state = if active { "ON " } else { "OFF" };
    let _ = queue!(stdout(),
        crossterm::cursor::MoveTo(0, 0),
        crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb { r: 0, g: 255, b: 64 }),
        crossterm::style::Print(format!(
            "╔══════════════════════════════════╗\r\n\
             ║     PHANTOM CLICK  AUTO-CLICKER  ║\r\n\
             ╠══════════════════════════════════╣\r\n\
             ║  CPS: {:>3}                        ║\r\n\
             ║  Status: [{}]                    ║\r\n\
             ║  Mouse: {:>4},{:<4}                ║\r\n\
             ╠══════════════════════════════════╣\r\n\
             ║  [C] toggle  [Z]+CPS  [S]-CPS    ║\r\n\
             ║  [H] help    [Q][Esc] quit       ║\r\n\
             ║  (global keys)                   ║\r\n\
             ╚══════════════════════════════════╝\r\n",
            cps, state, mx, my
        )),
    ).ok();

    if help {
        let help_lines = [
            "╔══════ HELP ═══════════════════╗",
            "║                               ║",
            "║  C  Toggle clicking on/off    ║",
            "║  Z  Increase CPS (+1)         ║",
            "║  S  Decrease CPS (-1)         ║",
            "║  H  Toggle this help          ║",
            "║  Q  Quit                      ║",
            "║  Esc Quit                     ║",
            "║                               ║",
            "║  CPS: clicks per second       ║",
            "║  Range: 1 - 200               ║",
            "║                               ║",
            "╚════════════════════════════════╝",
        ];
        for (i, line) in help_lines.iter().enumerate() {
            let _ = queue!(stdout(),
                crossterm::cursor::MoveTo(0, 11 + i as u16),
                crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
                    r: 0, g: 255, b: 100
                }),
                crossterm::style::Print(line),
            ).ok();
        }
    }

    let (w, h) = size().unwrap_or((80, 24));
    let start_row = if help { 25 } else { 12 };
    if h > start_row {
        let fill_h = (h - start_row) as u16;
        for y in 0..fill_h {
            let row = start_row + y;
            let cols = (w as u64 / 2).min(60);
            for cx in 0..cols {
                let idx = (tick.wrapping_add(cx * 7).wrapping_add(y as u64 * 13)) as usize
                    % MATRIX_CHARS.len();
                let ch = MATRIX_CHARS[idx] as char;
                let bright = (tick.wrapping_add(cx * 3).wrapping_add(y as u64 * 5)) % 7 > 2;
                let _ = queue!(stdout(),
                    crossterm::cursor::MoveTo((cx as u16) * 2, row),
                    crossterm::style::SetForegroundColor(if bright {
                        crossterm::style::Color::Rgb { r: 0, g: 180, b: 40 }
                    } else {
                        crossterm::style::Color::Rgb { r: 0, g: 60, b: 12 }
                    }),
                    crossterm::style::Print(ch),
                );
            }
        }
    }
    let _ = stdout().flush();
}
