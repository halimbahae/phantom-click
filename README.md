# Phantom Click ⚡

> **A hacker-terminal CPS test with keyboard shortcuts — built in Rust.**

Measure your clicks per second across 10 timed modes with real-time feedback, animal-based rankings, and persistent local high scores. All in a green-on-black terminal aesthetic.

[![Rust](https://img.shields.io/badge/Rust-1.96%2B-dea584?logo=rust)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![GitHub release](https://img.shields.io/badge/release-v0.1.0--alpha-blue)](https://github.com/halimbahae/phantom-click/releases)

---

## Screenshot

```
╔══════════════════════════════════════╗
║  ██████  ██  █████  ███    ███      ║
║  ██   ██ ██ ██   ██ ████  ████      ║
║  ██████  ██ ███████ ██ ████ ██      ║
║  ██      ██ ██   ██ ██  ██  ██      ║
║  ██      ██ ██   ██ ██      ██      ║
║  >>> TERMINAL CPS HACK SUITE v1.0 << ║
║  ═══════════════════════════════════  ║
║      SELECT DURATION                 ║
║   [1s]  [3s]  [5s]  [10s]  [15s]    ║
║  [30s] [60s] [100s] [180s] [900s]   ║
╚══════════════════════════════════════╝
```

---

## Features

- **10 Time Durations** — 1s, 3s, 5s, 10s, 15s, 30s, 60s, 100s, 180s, 900s (Marathon)
- **Keyboard-Driven** — No mouse required. Press `Space` to register clicks.
- **Real-Time Feedback** — Live CPS counter, progress bar, elapsed/remaining time, current animal rank
- **Animal Ranking System** — Each score maps to an animal based on CPS:
  | CPS Range | Animal | Emoji | Speed |
  |---|---|---|---|
  | 0 – 2.9 | Tortue de mer | 🐢 | 22 km/h |
  | 3 – 4.9 | Panda | 🐼 | 32 km/h |
  | 5 – 6.9 | Lapin | 🐰 | 45 km/h |
  | 7 – 8.9 | Guépard | 🐆 | 58 km/h |
  | 9 – 10.9 | Faucon | 🦅 | 72 km/h |
  | 11 – 12.9 | Éclair | ⚡ | 88 km/h |
  | 13+ | Légende | 👑 | 100 km/h |
- **Persistent High Scores** — Top 5 scores per duration stored in `~/.config/phantom-click/scores.json`
- **Countdown Start** — 3-2-1-GO! before each test for consistent timing
- **Hacker Aesthetic** — Green-on-black terminal UI with matrix rain background effect
- **3-Second Countdown** — Prepares you before each test

---

## Package Managers

| Package | Command | Status |
|---------|---------|--------|
| **Cargo (crates.io)** | `cargo install phantom-click` | Coming soon |
| **Homebrew** | `brew install halimbahae/tap/phantom-click` | Coming soon |
| **Arch Linux (AUR)** | `yay -S phantom-click-bin` | Coming soon |

---

## Installation

### From Source (Recommended)

```bash
# Prerequisites: Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone & build
git clone https://github.com/halimbahae/phantom-click.git
cd phantom-click
cargo build --release

# Run
./target/release/phantom-click
```

### Prebuilt Binaries

Download the latest binary for your platform from the [Releases page](https://github.com/halimbahae/phantom-click/releases).

| Platform | Arch | Link |
|----------|------|------|
| macOS | arm64 (Apple Silicon) | `phantom-click-macos-arm64` |
| macOS | x86_64 (Intel) | `phantom-click-macos-x86_64` |
| Linux | x86_64 | `phantom-click-linux-x86_64` |
| Linux | aarch64 | `phantom-click-linux-aarch64` |
| Windows | x86_64 | `phantom-click-windows-x86_64.exe` |

---

## Usage

```bash
cargo run --release
```

### Controls

| Key | Context | Action |
|-----|---------|--------|
| `1` – `9` | Menu | Quick-select duration (1s – 180s) |
| `0` | Menu | Select 900s Marathon |
| `↑` `↓` `←` `→` | Menu | Navigate duration grid |
| `j` `k` `h` `l` | Menu | Vim-style navigation |
| `Enter` | Menu / Ready | Confirm & start |
| `Space` | Running | Register a click |
| `Space` | Ready | Start test |
| `R` | Ready / Result | Return to menu / Retry |
| `Esc` | Ready | Go back to menu |
| `Q` | Anywhere | Quit |

### Scoring Formula

```
CPS = Total Clicks ÷ Time Elapsed (seconds)
```

Example: 65 clicks in 10 seconds = **6.5 CPS**

---

## Project Structure

```
src/
├── main.rs      # Entry point, terminal setup, event loop
├── app.rs       # State machine & key event dispatch
├── ui.rs        # TUI rendering (hacker aesthetic)
├── test.rs      # CPS test engine & animal ranking
└── storage.rs   # Score persistence (JSON)
```

### Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `ratatui` | 0.29 | Terminal UI framework |
| `crossterm` | 0.28 | Terminal backend & event handling |
| `serde` | 1.x | Serialization |
| `serde_json` | 1.x | JSON persistence |
| `chrono` | 0.4 | Timestamps |
| `rand` | 0.8 | Matrix rain generation |

---

## Roadmap

- [ ] Custom duration input
- [ ] Mouse click support
- [ ] Real-time CPS graph
- [ ] World record display
- [ ] Online leaderboard
- [ ] `cargo publish` to crates.io
- [ ] Homebrew tap
- [ ] CI/CD with GitHub Actions
- [ ] Windows support improvements

---

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please ensure your code:
- Follows existing code style and conventions
- Passes `cargo build` without warnings
- Is well-structured and commented where necessary

---

## Releases

See the [Releases page](https://github.com/halimbahae/phantom-click/releases) for changelogs and downloads.

Current version: **v0.1.0-alpha**

---

## License

Distributed under the **MIT License**. See [LICENSE](LICENSE) for more information.

Copyright © 2026 [Bahae Eddine Halim](https://github.com/halimbahae)

---

## Contact

- **Author:** Bahae Eddine Halim
- **GitHub:** [@halimbahae](https://github.com/halimbahae)
- **Project:** [github.com/halimbahae/phantom-click](https://github.com/halimbahae/phantom-click)

---

## Acknowledgments

- Inspired by [CPS Test](https://cpstest.org) and similar click-speed benchmarks
- Built with [Ratatui](https://ratatui.rs/) — the Rust TUI framework
- ASCII art title generated with [patorjk.com](https://patorjk.com/software/taag/)
