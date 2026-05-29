use std::io::{self, BufRead, Write};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use phantom_click::{clicker, cursor};

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

fn handle_request(req: RpcRequest) {
    match req.method.as_str() {
        "initialize" => {
            respond(req.id, json!({
                "protocolVersion": "1.0",
                "capabilities": {
                    "tools": {}
                },
                "serverInfo": {
                    "name": "phantom-click-mcp",
                    "version": "0.1.0"
                }
            }));
        }
        "tools/list" => {
            respond(req.id, json!({
                "tools": [
                    {
                        "name": "get_cursor_position",
                        "description": "Get the current mouse cursor position",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
                    },
                    {
                        "name": "click",
                        "description": "Perform a single mouse click at the current cursor position",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
                    },
                    {
                        "name": "get_status",
                        "description": "Get phantom-click status info",
                        "inputSchema": {
                            "type": "object",
                            "properties": {}
                        }
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
                "click" => {
                    match clicker::click_once() {
                        Ok(()) => respond(req.id, json!({
                            "content": [{
                                "type": "text",
                                "text": "Click performed successfully"
                            }]
                        })),
                        Err(e) => respond(req.id, json!({
                            "content": [{
                                "type": "text",
                                "text": format!("Click failed: {}", e)
                            }]
                        })),
                    }
                }
                "get_status" => {
                    let (x, y) = cursor::position();
                    respond(req.id, json!({
                        "content": [{
                            "type": "text",
                            "text": format!("Phantom Click MCP Server\nCursor: ({:.0}, {:.0})", x, y)
                        }]
                    }));
                }
                _ => respond(req.id, json!({
                    "content": [{
                        "type": "text",
                        "text": format!("Unknown tool: {}", name)
                    }]
                })),
            }
        }
        _ => respond(req.id, json!({ "status": "ok" })),
    }
}

fn main() {
    let stdin = io::stdin().lock();
    for line in stdin.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };
        if line.trim().is_empty() { continue; }
        if let Ok(req) = serde_json::from_str::<RpcRequest>(&line) {
            handle_request(req);
        }
    }
}
