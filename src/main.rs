use std::io::{stdout, Write};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use crossterm::{execute, queue};
use enigo::{Button, Direction, Enigo, Mouse, Settings};

mod mac {
    use std::ffi::c_void;

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct CGPoint {
        pub x: f64,
        pub y: f64,
    }

    #[link(name = "CoreGraphics", kind = "framework")]
    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        fn CGEventCreate(source: *const c_void) -> *mut c_void;
        fn CGEventGetLocation(event: *mut c_void) -> CGPoint;
        fn CFRelease(event: *mut c_void);
    }

    pub fn cursor_pos() -> (f64, f64) {
        unsafe {
            let e = CGEventCreate(std::ptr::null());
            if e.is_null() {
                return (0.0, 0.0);
            }
            let p = CGEventGetLocation(e);
            CFRelease(e);
            (p.x, p.y)
        }
    }
}

const MATRIX_CHARS: &[u8] = b"0123456789ABCDEF";

fn main() {
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

    // Click thread
    let click_flag = Arc::clone(&clicking);
    let cps_click = Arc::clone(&cps_val);
    std::thread::spawn(move || {
        let Ok(mut enigo) = Enigo::new(&Settings::default()) else { return };
        loop {
            if click_flag.load(Ordering::Relaxed) {
                let _ = <Enigo as Mouse>::button(&mut enigo, Button::Left, Direction::Click);
                let c = cps_click.load(Ordering::Relaxed);
                std::thread::sleep(Duration::from_micros(1_000_000 / c.max(1)));
            } else {
                std::thread::sleep(Duration::from_millis(50));
            }
        }
    });

    // Global key listener thread (rdev)
    let listen_clicking = Arc::clone(&clicking);
    let listen_cps = Arc::clone(&cps_val);
    let listen_quit = Arc::clone(&should_quit);
    std::thread::spawn(move || {
        if let Err(e) = rdev::listen(move |event| {
            use rdev::Key::*;
            if let rdev::EventType::KeyPress(key) = event.event_type {
                match key {
                    KeyC => {
                        listen_clicking.fetch_xor(true, Ordering::SeqCst);
                    }
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
                    KeyQ => {
                        listen_quit.store(true, Ordering::SeqCst);
                    }
                    Escape => {
                        listen_quit.store(true, Ordering::SeqCst);
                    }
                    _ => {}
                }
            }
        }) {
            eprintln!("rdev error: {:?}", e);
        }
    });

    crossterm::queue!(
        stdout(),
        crossterm::style::SetForegroundColor(crossterm::style::Color::Rgb {
            r: 0, g: 255, b: 64
        }),
    )
    .ok();

    let mut tick: u64 = 0;
    while !should_quit.load(Ordering::Relaxed) {
        let active = clicking.load(Ordering::Relaxed);
        let cps = cps_val.load(Ordering::Relaxed);
        let (mx, my) = mac::cursor_pos();

        draw_ui(cps, active, mx as u64, my as u64, tick);

        std::thread::sleep(Duration::from_millis(100));
        tick = tick.wrapping_add(1);
    }

    clicking.store(false, Ordering::SeqCst);
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), crossterm::terminal::LeaveAlternateScreen);
}

fn draw_ui(cps: u64, active: bool, mx: u64, my: u64, tick: u64) {
    let state = if active { "ON " } else { "OFF" };

    let _ = queue!(
        stdout(),
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
             ║  [Q][Esc] quit                   ║\r\n\
             ║  (global keys)                   ║\r\n\
             ╚══════════════════════════════════╝\r\n",
            cps, state, mx, my
        )),
    )
    .ok();

    let (w, h) = size().unwrap_or((80, 24));
    let start_row = 12;
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
                let _ = queue!(
                    stdout(),
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
