// Ralqlator - A command line calculator
//
// Copyright (C) 2026 Ralqlator Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! ralqlator MCP Server binary
//!
//! Runs a Model Context Protocol server over stdio transport.
//! Listens for JSON-RPC 2.0 requests on stdin and writes responses to stdout.
//!
//! Usage:
//!   ralqlator-mcp
//!
//! The server processes one request per line (newline-delimited JSON) and
//! writes one response per line. It exits cleanly when stdin is closed.

use std::io::{self, BufRead, Write};
use ralqlator::mcp;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    for line_result in stdin.lock().lines() {
        let line = match line_result {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading stdin: {}", e);
                break;
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Parse the JSON-RPC request
        let request = match mcp::parse_request(trimmed) {
            Ok(req) => req,
            Err((code, msg)) => {
                let error_response = mcp::build_error(None, code, msg);
                let output = serde_json::to_string(&error_response)
                    .unwrap_or_else(|_| "{}".to_string());
                let _ = writeln!(stdout_lock, "{}", output);
                let _ = stdout_lock.flush();
                continue;
            }
        };

        // Handle the request
        let response = mcp::handle_request(request);

        // Send response (handle_request returns None for notifications)
        if let Some(resp) = response {
            let output = serde_json::to_string(&resp)
                .unwrap_or_else(|_| "{}".to_string());
            let _ = writeln!(stdout_lock, "{}", output);
            let _ = stdout_lock.flush();
        }
    }
}
