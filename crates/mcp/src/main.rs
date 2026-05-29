use std::io::{self, BufRead, Write};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use phantom_click::{clicker, cursor};

const VERSION: &str = "0.3.0";

#[derive(Deserialize)]
struct RpcRequest {
    id: u64,
    method: String,
    #[serde(default)]
    params: Value,
}

#[derive(Serialize)]
struct RpcResponse {
    jsonrpc: &'static str,
    id: u64,
    result: Value,
}

fn respond(id: u64, result: Value) {
    let resp = RpcResponse { jsonrpc: "2.0", id, result };
    let line = serde_json::to_string(&resp).unwrap();
    let mut out = io::stdout().lock();
    let _ = writeln!(out, "{}", line);
    let _ = out.flush();
}

fn handle_request(req: RpcRequest, clicking: &Arc<AtomicBool>, cps_val: &Arc<AtomicU64>, global_timeout: &Arc<AtomicBool>, _silent: bool) {
    match req.method.as_str() {
        "initialize" => {
            respond(req.id, json!({
                "protocolVersion": "1.0",
                "capabilities": { "tools": {} },
                "serverInfo": {
                    "name": "phantom-click-mcp",
                    "version": VERSION
                }
            }));
        }
        "tools/list" => {
            respond(req.id, json!({
                "tools": [
                    {
                        "name": "get_cursor_position",
                        "description": "Get the current mouse cursor position",
                        "inputSchema": { "type": "object", "properties": {} }
                    },
                    {
                        "name": "move_mouse",
                        "description": "Move the mouse cursor to absolute screen coordinates",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "x": { "type": "number", "description": "X coordinate" },
                                "y": { "type": "number", "description": "Y coordinate" }
                            },
                            "required": ["x", "y"]
                        }
                    },
                    {
                        "name": "click",
                        "description": "Perform a single mouse click at the current cursor position",
                        "inputSchema": { "type": "object", "properties": {} }
                    },
                    {
                        "name": "click_for_duration",
                        "description": "Click repeatedly at a given CPS for a specified duration, then stop",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "cps": {
                                    "type": "number",
                                    "description": "Clicks per second (1-200, default: 10)",
                                    "default": 10
                                },
                                "duration_secs": {
                                    "type": "number",
                                    "description": "How many seconds to click for (default: 5)",
                                    "default": 5
                                }
                            }
                        }
                    },
                    {
                        "name": "get_status",
                        "description": "Get phantom-click status info",
                        "inputSchema": { "type": "object", "properties": {} }
                    }
                ]
            }));
        }
        "tools/call" => {
            let name = req.params["name"].as_str().unwrap_or("");
            match name {
                "get_cursor_position" => {
                    let (x, y) = cursor::position();
                    respond(req.id, json!({
                        "content": [{
                            "type": "text",
                            "text": format!("Cursor position: ({:.0}, {:.0})", x, y)
                        }]
                    }));
                }
                "move_mouse" => {
                    let x = req.params["arguments"]["x"].as_f64().unwrap_or(0.0);
                    let y = req.params["arguments"]["y"].as_f64().unwrap_or(0.0);
                    cursor::move_to(x, y);
                    respond(req.id, json!({
                        "content": [{
                            "type": "text",
                            "text": format!("Moved cursor to ({:.0}, {:.0})", x, y)
                        }]
                    }));
                }
                "click" => {
                    match clicker::click_once() {
                        Ok(()) => respond(req.id, json!({
                            "content": [{ "type": "text", "text": "Click performed successfully" }]
                        })),
                        Err(e) => respond(req.id, json!({
                            "content": [{ "type": "text", "text": format!("Click failed: {}", e) }]
                        })),
                    }
                }
                "click_for_duration" => {
                    let cps = req.params["arguments"]["cps"].as_f64().unwrap_or(10.0).max(1.0).min(200.0) as u64;
                    let duration = req.params["arguments"]["duration_secs"].as_f64().unwrap_or(5.0).max(0.1).min(3600.0);

                    clicking.store(true, Ordering::SeqCst);
                    cps_val.store(cps, Ordering::SeqCst);

                    let start = Instant::now();
                    let count = clicker::click_for_duration(cps, duration, global_timeout);
                    let elapsed = start.elapsed().as_secs_f64();

                    clicking.store(false, Ordering::SeqCst);

                    let reason = if global_timeout.load(Ordering::SeqCst) {
                        format!(" (stopped by global timeout after {:.1}s)", elapsed)
                    } else {
                        String::new()
                    };

                    respond(req.id, json!({
                        "content": [{
                            "type": "text",
                            "text": format!("Clicked {} times in {:.1}s at {} CPS{}",
                                count, duration.min(elapsed), cps, reason)
                        }]
                    }));
                }
                "get_status" => {
                    let (x, y) = cursor::position();
                    let active = clicking.load(Ordering::Relaxed);
                    let cps_val = cps_val.load(Ordering::Relaxed);
                    respond(req.id, json!({
                        "content": [{
                            "type": "text",
                            "text": format!(
                                "Phantom Click MCP Server v{}\n\
                                 Cursor: ({:.0}, {:.0})\n\
                                 Status: {}\n\
                                 CPS: {}",
                                VERSION, x, y,
                                if active { "Clicking" } else { "Idle" },
                                cps_val
                            )
                        }]
                    }));
                }
                _ => respond(req.id, json!({
                    "content": [{ "type": "text", "text": format!("Unknown tool: {}", name) }]
                })),
            }
        }
        _ => respond(req.id, json!({ "status": "ok" })),
    }
}

fn print_usage() {
    eprintln!("Phantom Click MCP Server v{VERSION}");
    eprintln!();
    eprintln!("USAGE:");
    eprintln!("  phantom-click-mcp [OPTIONS]");
    eprintln!();
    eprintln!("OPTIONS:");
    eprintln!("  -t, --timeout <SECONDS>   Global timeout - server auto-exits after N seconds");
    eprintln!("  -s, --silent              Suppress stderr output (default: enabled for stdio MCP)");
    eprintln!("  -h, --help                Print this help");
    eprintln!();
    eprintln!("TOOLS:");
    eprintln!("  get_cursor_position       Get mouse position");
    eprintln!("  move_mouse                Move cursor to (x, y)");
    eprintln!("  click                     Single click at current position");
    eprintln!("  click_for_duration        Click at CPS for N seconds");
    eprintln!("  get_status                Get server status");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut timeout_secs: Option<f64> = None;
    let mut silent = true;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-t" | "--timeout" => {
                i += 1;
                if i < args.len() {
                    timeout_secs = Some(args[i].parse().unwrap_or(10.0));
                }
            }
            "-s" | "--silent" => silent = true,
            "-v" | "--verbose" => silent = false,
            "-h" | "--help" => {
                print_usage();
                return;
            }
            _ => {}
        }
        i += 1;
    }

    if !silent {
        eprintln!("[phantom-click-mcp v{VERSION}] Starting (timeout: {:?})", timeout_secs);
    }

    let clicking = Arc::new(AtomicBool::new(false));
    let cps_val = Arc::new(AtomicU64::new(10));
    let global_timeout = Arc::new(AtomicBool::new(false));

    clicker::spawn_clicker(Arc::clone(&clicking), Arc::clone(&cps_val));

    if let Some(timeout) = timeout_secs {
        let gt = Arc::clone(&global_timeout);
        let click_stop = Arc::clone(&clicking);
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs_f64(timeout));
            gt.store(true, Ordering::SeqCst);
            click_stop.store(false, Ordering::SeqCst);
        });
    }

    let stdin = io::stdin().lock();
    for line in stdin.lines() {
        if global_timeout.load(Ordering::Relaxed) { break; }
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() { continue; }
        if let Ok(req) = serde_json::from_str::<RpcRequest>(&line) {
            handle_request(req, &clicking, &cps_val, &global_timeout, silent);
        }
    }

    clicking.store(false, Ordering::SeqCst);

    if !silent {
        eprintln!("[phantom-click-mcp v{VERSION}] Exiting");
    }
}
