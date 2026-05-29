# Phantom Click

> **Cross-platform auto-clicker with global hotkeys, TUI, GUI, and MCP server for AI assistants.**

Click anywhere on screen — hotkeys work system-wide. Available as a terminal app
with matrix-rain UI, a native desktop GUI, a Rust library, and an MCP server
that AI assistants (Claude, Cursor) can control directly.

## Quick Start

```bash
# One-line install (macOS / Linux)
curl -fsSL https://raw.githubusercontent.com/halimbahae/phantom-click/main/install.sh | bash

# Or via Cargo
cargo install phantom-click-cli
phantom-click 20

# Desktop GUI
cargo install phantom-click-gui
phantom-click-gui

# MCP server (for AI assistants)
cargo install phantom-click-mcp
phantom-click-mcp
```

## Features

| Feature | CLI | GUI | MCP | Library |
|---------|:---:|:---:|:---:|:-------:|
| Global hotkeys | ✅ | ✅ | — | — |
| Matrix-rain TUI | ✅ | — | — | — |
| Native window | — | ✅ | — | — |
| CPS slider | — | ✅ | ✅ | — |
| Live cursor position | ✅ | ✅ | ✅ | ✅ |
| Click once | ✅ | ✅ | ✅ | ✅ |
| Click for duration | — | ✅ | ✅ | ✅ |
| Auto-clicker (toggle) | ✅ | ✅ | — | ✅ |
| Move mouse | — | — | ✅ | ✅ |
| AI assistant control | — | — | ✅ | — |
| Cross-platform | ✅ | ✅ | ✅ | ✅ |

## Install

### One-line Install (macOS / Linux)

```bash
curl -fsSL https://raw.githubusercontent.com/halimbahae/phantom-click/main/install.sh | bash
```

### Cargo Install

```bash
# Terminal auto-clicker (installed as `phantom-click`)
cargo install phantom-click-cli

# Desktop GUI
cargo install phantom-click-gui

# MCP server
cargo install phantom-click-mcp

# Library (for your Rust projects)
cargo add phantom-click
```

### Build from Source

```bash
git clone https://github.com/halimbahae/phantom-click.git
cd phantom-click
cargo build --release

# Binaries are in target/release/
#   phantom-click         CLI TUI
#   phantom-click-gui     Desktop GUI
#   phantom-click-mcp     MCP server
```

## Usage

### Terminal UI (CLI)

```bash
phantom-click [CPS]
```

| Key | Action |
|-----|--------|
| `C` | Toggle clicking ON/OFF |
| `Z` | +1 CPS (speed up) |
| `S` | -1 CPS (slow down) |
| `H` | Toggle help overlay |
| `Q` / `Esc` | Quit |

### Desktop GUI

```bash
phantom-click-gui
```

Features:
- Start/Stop toggle
- CPS slider (1-200)
- Live cursor position
- One-click button

### MCP Server (AI Assistants)

```bash
phantom-click-mcp [OPTIONS]
```

Options:
- `-t, --timeout <SECONDS>` – Global timeout (auto-exit after N seconds)
- `-s, --silent` – Suppress stderr (default for stdio MCP)
- `-v, --verbose` – Enable stderr logging

Available tools: `get_cursor_position`, `move_mouse`, `click`,
`click_for_duration`, `browser_click`, `get_status`.

See [docs/mcp-integration.md](docs/mcp-integration.md) for AI assistant config.

## Crates

| Crate | Description | `cargo` |
|-------|-------------|---------|
| [`phantom-click`](https://crates.io/crates/phantom-click) | Core library (cursor, clicker) | `cargo add phantom-click` |
| [`phantom-click-cli`](https://crates.io/crates/phantom-click-cli) | Terminal auto-clicker with TUI | `cargo install phantom-click-cli` |
| [`phantom-click-gui`](https://crates.io/crates/phantom-click-gui) | Native desktop GUI (egui) | `cargo install phantom-click-gui` |
| [`phantom-click-mcp`](https://crates.io/crates/phantom-click-mcp) | MCP server for AI assistants | `cargo install phantom-click-mcp` |

## SDK / Library

```rust
use phantom_click::{cursor, clicker};

// Get cursor position
let (x, y) = cursor::position();

// Move cursor (macOS, Linux, Windows)
cursor::move_to(500.0, 300.0);

// Single click at current position
clicker::click_once()?;

// Click for duration (synchronous)
use std::sync::atomic::{AtomicBool, Ordering};
let stop = AtomicBool::new(false);
let count = clicker::click_for_duration(50, 10.0, &stop);

// Background auto-clicker
use std::sync::Arc;
clicker::spawn_clicker(
    Arc::new(AtomicBool::new(true)),
    Arc::new(AtomicU64::new(10)),
);
```

## Browser Automation

Phantom Click can automate click-speed test websites like
[arealme.com](https://www.arealme.com/click-speed-test/fr/).

**Important:** Browsers check `event.isTrusted` — OS-level mouse events from
`enigo` are NOT trusted. Use the `browser_click` MCP tool or Playwright's CDP
to generate trusted events.

### Method 1: MCP `browser_click` (recommended)

Start Chrome with remote debugging, then use the MCP server:

```bash
phantom-click-mcp

# Then call browser_click:
# {"name": "browser_click", "arguments": {"cdp_url": "ws://...", "x": 341, "y": 684, "cps": 100, "duration_secs": 5}}
```

### Method 2: Playwright + CDP

```js
const cdp = await page.context().newCDPSession(page);
for (let i = 0; i < 500; i++) {
  await cdp.send('Input.dispatchMouseEvent', {
    type: 'mousePressed', x: cx, y: cy, button: 'left', clickCount: 1
  });
  await cdp.send('Input.dispatchMouseEvent', {
    type: 'mouseReleased', x: cx, y: cy, button: 'left', clickCount: 1
  });
  await page.waitForTimeout(10);
}
```

See [docs/browser-automation.md](docs/browser-automation.md) for the full guide.

## Permission

**macOS:** Requires **Accessibility** permission for OS-level clicking.
`System Settings → Privacy & Security → Accessibility → Add Terminal.app`

For **browser automation**, use Playwright's CDP instead (no permissions needed).

**Linux:** Requires X11 development libraries.
```bash
sudo apt install libx11-dev libxi-dev libxtst-dev libxdo-dev pkg-config
```

## Prebuilt Binaries

Download from the [Releases page](https://github.com/halimbahae/phantom-click/releases).

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
│   ├── src/cursor.rs   #   Cross-platform cursor position & movement
│   ├── src/clicker.rs  #   Click simulation & auto-clicker
│   └── src/lib.rs      #   Public API
├── cli/                # TUI binary (crossterm + rdev)
│   └── src/main.rs     #   Matrix-rain terminal UI
├── gui/                # GUI binary (eframe/egui)
│   └── src/main.rs     #   Native desktop window
└── mcp/                # MCP server binary
    └── src/main.rs     #   JSON-RPC stdio server
docs/
├── browser-automation.md  # Playwright + CDP click test guide
└── mcp-integration.md     # AI assistant MCP setup
install.sh              # One-line installer
```

## Future Improvements

- **Windows CLI** – Fix cross-compilation from Docker
- **Network MCP** – TCP/WebSocket transport in addition to stdio
- **browser_click MCP tool** – Native CDP-based trusted click tool (bundled
  Playwright)
- **Custom hotkeys** – User-configurable key bindings in TUI/GUI
- **Click jitter** – Per-click timing jitter to mimic human clicking
- **Screenshare mode** – Stream click visualization overlay
- **Profile saving** – Save/load CPS profiles per game/website

## Contributing

Contributions welcome! To collaborate:

1. Fork the repo
2. Create a feature branch (`git checkout -b feature/awesome`)
3. Commit (`git commit -m 'Add awesome feature'`)
4. Push (`git push origin feature/awesome`)
5. Open a Pull Request

Please ensure `cargo build --release` passes without warnings and code follows
existing conventions.

## License

MIT — see [LICENSE](LICENSE).

## Author

**Bahae Eddine Halim** — [@halimbahae](https://github.com/halimbahae)

Want to collaborate? Reach out at **cto@xai.ma**
