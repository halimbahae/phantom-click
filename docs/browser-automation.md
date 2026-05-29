# Browser Automation with Playwright + CDP

Phantom Click uses **Chrome DevTools Protocol (CDP)** to generate trusted mouse
events inside the browser. OS-level clicks (via `enigo`) are **not trusted** by
the browser and fail the `event.isTrusted` check.

This guide shows how to automate click-speed tests using Playwright.

## Prerequisites

```bash
# Install Playwright
npm init -y
npm install playwright
npx playwright install chromium

# Or use the Playwright MCP server
```

## Basic Usage

```js
import { chromium } from 'playwright';

const browser = await chromium.launch({ headless: false });
const page = await browser.newPage();
await page.goto('https://www.arealme.com/click-speed-test/fr/');
await page.waitForSelector('#clickarena');

// Get button position
const box = await page.locator('#clickarena').boundingBox();
const cx = box.x + box.width / 2;
const cy = box.y + box.height / 2;

// Get CDP session for trusted events
const cdp = await page.context().newCDPSession(page);

// Start test with trusted click
await cdp.send('Input.dispatchMouseEvent', {
  type: 'mousePressed', x: cx, y: cy, button: 'left', clickCount: 1
});
await cdp.send('Input.dispatchMouseEvent', {
  type: 'mouseReleased', x: cx, y: cy, button: 'left', clickCount: 1
});

await page.waitForTimeout(500);

// Click at 50 CPS (20ms per click) for 10 seconds
const clicks = 500;
for (let i = 0; i < clicks; i++) {
  await cdp.send('Input.dispatchMouseEvent', {
    type: 'mousePressed', x: cx, y: cy, button: 'left', clickCount: 1
  });
  await cdp.send('Input.dispatchMouseEvent', {
    type: 'mouseReleased', x: cx, y: cy, button: 'left', clickCount: 1
  });
  await page.waitForTimeout(10); // 100 CPS
}

// Wait for results
await page.waitForTimeout(3000);
const title = await page.title();
console.log('Result:', title);
```

## CPS Calibration

| `waitForTimeout` | Theoretical CPS | Real CPS (tested) |
|-----------------|-----------------|-------------------|
| 20ms | 50 | ~40 |
| 10ms | 100 | ~80 |
| 5ms | 200 | ~140 |

Real-world CPS is lower due to CDP round-trip overhead.

## Integration with Phantom Click MCP

The MCP server (`phantom-click-mcp`) provides OS-level clicking tools. For
browser automation, combine Playwright's CDP (trusted clicks) with the MCP:

1. Playwright starts the test with CDP
2. Wait for the countdown
3. Use MCP `click_for_duration` for sustained clicking (if your app has
   Accessibility permissions)
4. Read results from the page

## Troubleshooting

- **`event.isTrusted = false`**: Use CDP (`Input.dispatchMouseEvent`) instead
  of OS-level clicks. The browser does NOT trust software-generated mouse events
  from other processes.
- **macOS Permission**: Even with Accessibility access, enigo/ CGEventPost
  events are not trusted by browsers (Chrome, Safari, Firefox).
- **Click not registering**: Check that coordinates point to the correct element.
  Use `boundingBox()` to get precise positions.
