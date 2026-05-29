# Phantom Click ⚡️

A brutally fast, low-level terminal autoclicker built in Rust. Designed for precision, speed, and zero GUI bloat.

## Features
* **Global Hotkeys:** Start and stop the clicker instantly from anywhere, regardless of window focus.
* **Custom CPS (Clicks Per Second):** Finely tune your click rate to bypass anti-cheat thresholds.
* **TUI Interface:** A clean, "hacker aesthetic" terminal UI to monitor active status and current CPS configuration.
* **Low Overhead:** Direct system-level API hooks for zero latency.

## Prerequisites
* [Rust toolchain](https://rustup.rs/) installed.

## Build & Run
1. Clone the repository:
   ```bash
   git clone [https://github.com/halimbahae/phantom-click.git](https://github.com/halimbahae/phantom-click.git)
   cd phantom-click
```

2. Build for release (critical for speed):
```bash
cargo build --release

```


3. Run the executable:
```bash
cargo run --release

```



## Controls

* `F8` - Toggle Autoclicker ON/OFF
* `Up/Down Arrows` - Adjust target Clicks Per Second
* `Ctrl + C` - Exit Terminal

```

---

### Git Push Instructions (Do this yourself)

Once we write the code, you will run this in your terminal to get it on GitHub:

```bash
git init
git add .
git commit -m "Initial commit: Core autoclicker architecture"
git branch -M main
git remote add origin https://github.com/halimbahae/phantom-click.git
git push -u origin main

```
