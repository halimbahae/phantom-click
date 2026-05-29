# Phantom Click

macOS auto-clicker with global hotkeys and a matrix-rain terminal UI.

Click anywhere on screen — the hotkeys work system-wide, not just in the terminal.

## Usage

```bash
cargo run --release
# or with a specific CPS (default 10):
cargo run --release -- 20
```

## Controls

| Key  | Action                 |
|------|------------------------|
| `C`  | Toggle clicking ON/OFF |
| `Z`  | +1 CPS (speed up)      |
| `S`  | -1 CPS (slow down)     |
| `Q`  | Quit                   |
| `Esc`| Quit                   |

All keys work globally in any application.

## Permission

Clicking requires **Accessibility** permission:

`System Settings → Privacy & Security → Accessibility → Add Terminal.app (or your binary)`

## Build

```bash
cargo build --release
```

The compiled binary is at `target/release/phantom-click`.
