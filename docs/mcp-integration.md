# MCP Server Integration

Phantom Click implements the **Model Context Protocol (MCP)** — a JSON-RPC 2.0
server over stdio — so AI assistants can control the mouse cursor and perform
clicks.

## Installation

```bash
# Via cargo
cargo install phantom-click-mcp

# Via install script
curl -fsSL https://raw.githubusercontent.com/halimbahae/phantom-click/main/install.sh | bash

# Or download from GitHub Releases
```

## Usage

```bash
# Default (silent stdio mode)
phantom-click-mcp

# With global timeout (auto-exit after 30 seconds)
phantom-click-mcp --timeout 30

# Verbose mode (stderr logging)
phantom-click-mcp --verbose
```

## Tools

| Tool | Description |
|------|-------------|
| `get_cursor_position` | Returns current mouse coordinates |
| `move_mouse` | Moves cursor to absolute `(x, y)` screen position |
| `click` | Single left-click at current cursor position |
| `click_for_duration` | Clicks repeatedly at `cps` for `duration_secs` |
| `get_status` | Returns server info, cursor position, and click status |
| `browser_click` | Click inside a browser via CDP (trusted events) |

## Protocol

The server communicates over **stdin/stdout** using JSON-RPC 2.0.

### Initialize

```json
{"id": 1, "method": "initialize"}
```

### List tools

```json
{"id": 2, "method": "tools/list"}
```

### Call a tool

```json
{
  "id": 3,
  "method": "tools/call",
  "params": {
    "name": "click_for_duration",
    "arguments": {
      "cps": 50,
      "duration_secs": 10
    }
  }
}
```

## AI Assistant Configuration

### Claude Desktop

Add to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "phantom-click": {
      "name": "Phantom Click",
      "transport": "stdio",
      "command": "phantom-click-mcp",
      "args": ["--timeout", "60"]
    }
  }
}
```

### Cursor / Windsurf

Add to your MCP configuration:

```json
{
  "phantom-click": {
    "command": "phantom-click-mcp",
    "args": ["--timeout", "60"]
  }
}
```

## Options

| Flag | Description |
|------|-------------|
| `-t, --timeout <SECONDS>` | Global timeout – server auto-exits after N seconds |
| `-s, --silent` | Suppress all stderr output (default for stdio) |
| `-v, --verbose` | Enable stderr logging |
| `-h, --help` | Print usage information |

## Global Timeout

The `--timeout` flag sets a hard limit on execution time. When triggered, it:
- Interrupts any running `click_for_duration` operation
- Prevents new requests from being processed
- Exits the server cleanly

This is especially useful when connecting from AI assistants that may lose
track of the server's state.

## Browser Click

The `browser_click` tool generates **trusted** browser events via CDP.

1. Start Chrome with `--remote-debugging-port=9222 --remote-allow-origins=*`
2. Get the page WebSocket URL from `http://127.0.0.1:9222/json`
3. Call `browser_click` with `cdp_url`, `x`, `y`, `cps`, `duration_secs`

See [docs/browser-automation.md](browser-automation.md) for full details.

## Version

Current version: `0.4.0`
