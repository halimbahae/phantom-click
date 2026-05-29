#!/usr/bin/env node

/**
 * Phantom Click Browser Agent
 * Uses Playwright + CDP to generate TRUSTED browser click events.
 *
 * Usage:
 *   node playwright-click.mjs <cdp_url> <x> <y> [count] [interval_ms]
 *
 * Examples:
 *   node playwright-click.mjs ws://127.0.0.1:9222/... 341 684 1
 *   node playwright-click.mjs ws://127.0.0.1:9222/... 341 684 500 10
 *
 * Or get CDP URL from an existing browser:
 *   node playwright-click.mjs auto 341 684 200 20
 *   (launches a new browser automatically)
 */

import { chromium } from 'playwright';

const [cdpUrl, x, y, count = 1, intervalMs = 20] = process.argv.slice(2);
const cx = parseInt(x);
const cy = parseInt(y);
const totalClicks = parseInt(count);
const interval = parseInt(intervalMs);

async function clickViaCDP(cdpSessionUrl, targetX, targetY, clicks, ms) {
  let browser;
  let cdp;

  if (cdpSessionUrl === 'auto') {
    browser = await chromium.launch({ headless: false });
    const page = await browser.newPage();
    cdp = await page.context().newCDPSession(page);
  } else {
    browser = await chromium.connectOverCDP(cdpSessionUrl);
    const [page] = browser.contexts()[0]?.pages() || [];
    if (!page) { console.error('No page found'); process.exit(1); }
    cdp = await page.context().newCDPSession(page);
  }

  const press = (x, y) => cdp.send('Input.dispatchMouseEvent', {
    type: 'mousePressed', x, y, button: 'left', clickCount: 1
  });
  const release = (x, y) => cdp.send('Input.dispatchMouseEvent', {
    type: 'mouseReleased', x, y, button: 'left', clickCount: 1
  });

  const start = Date.now();
  for (let i = 0; i < clicks; i++) {
    await press(targetX, targetY);
    await release(targetX, targetY);
    if (ms > 0) await new Promise(r => setTimeout(r, ms));
  }
  const elapsed = ((Date.now() - start) / 1000).toFixed(1);

  console.log(`Clicked ${clicks}x at (${targetX},${targetY}) in ${elapsed}s (${(clicks / elapsed).toFixed(1)} CPS)`);

  if (cdpSessionUrl === 'auto') {
    await browser.close();
  }
}

clickViaCDP(cdpUrl, cx, cy, totalClicks, interval).catch(e => {
  console.error('Error:', e.message);
  process.exit(1);
});
