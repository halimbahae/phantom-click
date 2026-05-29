use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

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
