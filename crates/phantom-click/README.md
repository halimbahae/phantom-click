# phantom-click

Cross-platform mouse cursor position, movement, and click simulation library.

Part of the [Phantom Click](https://github.com/halimbahae/phantom-click) project.

```rust
use phantom_click::{cursor, clicker};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

// Get cursor position
let (x, y) = cursor::position();

// Move cursor
cursor::move_to(500.0, 300.0);

// Single click
clicker::click_once()?;

// Click for duration (synchronous, returns click count)
let stop = AtomicBool::new(false);
let count = clicker::click_for_duration(50, 10.0, &stop);

// Background auto-clicker (toggle via AtomicBool)
clicker::spawn_clicker(
    Arc::new(AtomicBool::new(true)),
    Arc::new(AtomicU64::new(10)),
);
```
