# Browser Automation with Phantom Click

The `browser_click` tool in `phantom-click-mcp` generates **trusted browser
events** via Chrome DevTools Protocol (CDP). Unlike OS-level clicks (which fail
`event.isTrusted` in the browser), CDP events pass all browser security checks.

## How It Works

`browser_click` connects to Chrome's CDP WebSocket endpoint and dispatches
`Input.dispatchMouseEvent` commands directly. These are the same trusted events
that Chrome receives when you physically click.

## Prerequisites

Start Chrome with remote debugging enabled:

```bash
# macOS
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --remote-debugging-port=9222 \
  --remote-allow-origins=* \
  --user-data-dir=/tmp/chrome-cdp \
  "https://www.arealme.com/click-speed-test/fr/"
```

Get the page WebSocket URL:

```bash
# Find the page WS URL
curl -s http://127.0.0.1:9222/json | python3 -c "
import sys, json
pages = json.load(sys.stdin)
for p in pages:
    if 'arealme' in p.get('url', ''):
        print(p['webSocketDebuggerUrl'])
"
```

This returns something like:
`ws://127.0.0.1:9222/devtools/page/XXXXXXXXXXXX`

## Using browser_click

With the MCP server running:

```json
{
  "name": "browser_click",
  "arguments": {
    "cdp_url": "ws://127.0.0.1:9222/devtools/page/XXXXXXXXXXXX",
    "x": 341,
    "y": 684,
    "cps": 100,
    "duration_secs": 4
  }
}
```

Parameters:
| Param | Required | Description |
|-------|----------|-------------|
| `cdp_url` | ✅ | Chrome DevTools Protocol WS URL |
| `x` | ✅ | X coordinate on the page |
| `y` | ✅ | Y coordinate on the page |
| `cps` | ❌ | Clicks per second (1-200, default 10) |
| `duration_secs` | ❌ | Duration in seconds (default 5) |

## Example: Click Speed Test

1. Start Chrome with CDP and navigate to the test
2. Find the click button position:
   ```js
   // In Chrome DevTools console:
   const b = document.getElementById('clickarena');
   const r = b.getBoundingClientRect();
   console.log(`Center: (${r.left + r.width/2}, ${r.top + r.height/2})`);
   ```
3. Send browser_click with `{x, y, cps: 100, duration_secs: 5}`
4. Watch the click counter go up in real-time!

## Performance

| Target CPS | Actual CPS (tested) | Notes |
|-----------|-------------------|-------|
| 50 | ~41 | Conservative, reliable |
| 100 | ~82 | Good performance |
| 200 | ~140+ | Near-max rate |

Actual CPS is slightly lower than target due to WebSocket round-trip overhead.

## Limitations

- Requires Chrome with `--remote-debugging-port` and `--remote-allow-origins=*`
- Only works with Chromium-based browsers (Chrome, Edge, Brave)
- Coordinates are page-relative (not screen-absolute)
- Doesn't move the physical mouse cursor

## Full Workflow Script

```bash
#!/usr/bin/env bash
# 1. Start Chrome
"/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
  --remote-debugging-port=9222 \
  --remote-allow-origins=* \
  --user-data-dir=/tmp/chrome-cdp \
  "https://www.arealme.com/click-speed-test/fr/" &>/dev/null &

sleep 4

# 2. Get page WS URL
PAGE_WS=$(curl -s http://127.0.0.1:9222/json | python3 -c "
import sys, json
pages = json.load(sys.stdin)
for p in pages:
    if 'arealme' in p.get('url', ''):
        print(p['webSocketDebuggerUrl'])
")

# 3. Click at 100 CPS for 5 seconds
printf '{"id":1,"method":"initialize"}\n{"id":2,"method":"tools/call","params":{"name":"browser_click","arguments":{"cdp_url":"%s","x":341,"y":684,"cps":100,"duration_secs":5}}}\n' "$PAGE_WS" | \
  phantom-click-mcp --timeout 10

# 4. Check result
echo "Result:"
curl -s http://127.0.0.1:9222/json | python3 -c "
import sys, json
pages = json.load(sys.stdin)
for p in pages:
    if 'arealme' in p.get('url', ''):
        print(f\"  {p['title']}\")
"
```
