# Phantom Click

> **Cross-platform auto-clicker with global hotkeys, TUI, and GUI.**

Click anywhere on screen — hotkeys work system-wide. Available as a terminal app with matrix-rain UI, a native desktop GUI, and a Rust library.

## Quick Start

```bash
# Terminal UI (macOS)
cargo run --release -p phantom-click-cli

# Desktop GUI (all platforms)
cargo run --release -p phantom-click-gui

# With custom CPS
cargo run --release -p phantom-click-cli -- 20
```

Or install:
```bash
cargo install phantom-click-cli
phantom-click 20
```

## Crates

| Crate | Description | `cargo add` |
|-------|-------------|-------------|
| [`phantom-click`](https://crates.io/crates/phantom-click) | Core library (cursor, clicker) | `cargo add phantom-click` |
| [`phantom-click-cli`](https://crates.io/crates/phantom-click-cli) | Terminal auto-clicker with TUI | `cargo install phantom-click-cli` |
| [`phantom-click-gui`](https://crates.io/crates/phantom-click-gui) | Native desktop GUI (egui) | `cargo install phantom-click-gui` |
| [`phantom-click-mcp`](https://crates.io/crates/phantom-click-mcp) | MCP server for AI assistants | `cargo install phantom-click-mcp` |

## Controls (CLI)

All keys work globally in any application.

| Key | Action |
|-----|--------|
| `C` | Toggle clicking ON/OFF |
| `Z` | +1 CPS (speed up) |
| `S` | -1 CPS (slow down) |
| `H` | Toggle help overlay |
| `Q` / `Esc` | Quit |

## GUI

Launch a native desktop window with:
- Click toggle button
- CPS slider (1-200)
- Live cursor position
- One-click button

```bash
cargo run --release -p phantom-click-gui
```

## MCP Server

Use with any MCP-compatible AI assistant (Claude, etc.):

```bash
cargo run --release -p phantom-click-mcp
```

Exposes tools:
- `get_cursor_position` — current mouse position
- `click` — single click at cursor
- `get_status` — server status

## Library

```rust
use phantom_click::{cursor, clicker};

let (x, y) = cursor::position();
clicker::click_once()?;

// Background auto-clicker
use std::sync::atomic::{AtomicBool, AtomicU64};
use std::sync::Arc;
clicker::spawn_clicker(Arc::new(AtomicBool::new(true)),
                       Arc::new(AtomicU64::new(10)));
```

## Build

```bash
# All crates
cargo build --release

# Specific crate
cargo build --release -p phantom-click-cli
cargo build --release -p phantom-click-gui
cargo build --release -p phantom-click-mcp
```

## Permission

**macOS:** Requires **Accessibility** permission.
`System Settings → Privacy & Security → Accessibility → Add Terminal.app (or your binary)`

**Linux:** Requires X11 libraries.
```bash
sudo apt install libx11-dev libxi-dev libxtst-dev libxdo-dev pkg-config
```

## Releases

Prebuilt binaries are available on the [Releases page](https://github.com/halimbahae/phantom-click/releases).

| Platform | Arch | Binary |
|----------|------|--------|
| macOS | ARM64 (Apple Silicon) | `phantom-click-macos-arm64` |
| macOS | x86_64 (Intel) | `phantom-click-macos-x86_64` |
| Linux | x86_64 | `phantom-click-linux-x86_64` |
| Linux | aarch64 | `phantom-click-linux-aarch64` |
| Windows | x86_64 | `phantom-click-windows-x86_64.exe` |

## Project Structure

```
Cargo.toml              # Workspace root
crates/
├── phantom-click/      # Library (published to crates.io)
│   ├── src/cursor.rs   #   Cross-platform cursor position
│   ├── src/clicker.rs  #   Click simulation & auto-clicker
│   └── src/lib.rs      #   Public API
├── cli/                # TUI binary (crossterm + rdev)
│   └── src/main.rs     #   Matrix-rain terminal UI
├── gui/                # GUI binary (eframe/egui)
│   └── src/main.rs     #   Native desktop window
└── mcp/                # MCP server binary
    └── src/main.rs     #   JSON-RPC stdio server
```

## Contributing

Contributions are welcome!

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/awesome`)
3. Commit your changes (`git commit -m 'Add awesome feature'`)
4. Push (`git push origin feature/awesome`)
5. Open a Pull Request

Please ensure:
- `cargo build --release` passes without warnings
- Code follows existing style and conventions

## License

MIT License — see [LICENSE](LICENSE).

## Author

**Bahae Eddine Halim** — [@halimbahae](https://github.com/halimbahae)
