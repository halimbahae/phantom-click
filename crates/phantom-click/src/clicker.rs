use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use enigo::{Button, Direction, Enigo, Mouse, Settings};

pub fn click_once() -> Result<(), String> {
    let mut enigo = Enigo::new(&Settings::default()).map_err(|e| format!("{}", e))?;
    <Enigo as Mouse>::button(&mut enigo, Button::Left, Direction::Click)
        .map_err(|_| "click failed".to_string())
}

pub fn spawn_clicker(active: Arc<AtomicBool>, cps: Arc<AtomicU64>) {
    thread::spawn(move || {
        let Ok(mut enigo) = Enigo::new(&Settings::default()) else { return };
        loop {
            if active.load(Ordering::Relaxed) {
                let _ = <Enigo as Mouse>::button(&mut enigo, Button::Left, Direction::Click);
                let c = cps.load(Ordering::Relaxed);
                thread::sleep(Duration::from_micros(1_000_000 / c.max(1)));
            } else {
                thread::sleep(Duration::from_millis(50));
            }
        }
    });
}

pub fn click_for_duration(cps: u64, duration_secs: f64, should_stop: &AtomicBool) -> u64 {
    let mut count = 0;
    let start = Instant::now();
    let interval_us = 1_000_000 / cps.max(1);
    let mut enigo = match Enigo::new(&Settings::default()) {
        Ok(e) => e,
        Err(_) => return 0,
    };
    loop {
        if should_stop.load(Ordering::Relaxed) { break; }
        if start.elapsed().as_secs_f64() >= duration_secs { break; }
        let _ = <Enigo as Mouse>::button(&mut enigo, Button::Left, Direction::Click);
        count += 1;
        thread::sleep(Duration::from_micros(interval_us));
    }
    count
}
