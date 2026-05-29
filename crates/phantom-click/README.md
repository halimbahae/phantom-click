# phantom-click

Cross-platform mouse cursor position and click simulation library.

Part of the [Phantom Click](https://github.com/halimbahae/phantom-click) project.

```rust
use phantom_click::{cursor, clicker};

// Get cursor position
let (x, y) = cursor::position();

// Single click
clicker::click_once()?;

// Background auto-clicker
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
clicker::spawn_clicker(
    Arc::new(AtomicBool::new(true)),
    Arc::new(AtomicU64::new(10)),
);
```
