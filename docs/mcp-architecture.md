# ralqlator MCP Server — Architecture Design

## 1. Overview

The ralqlator MCP (Model Context Protocol) server is a **stdio-based JSON-RPC 2.0
server** that exposes ralqlator's calculation capabilities as MCP tools. It runs as
a separate binary target `ralqlator-mcp` in the same Cargo workspace, reusing the
ralqlator library crate.

The server speaks the **Model Context Protocol** over stdin/stdout using
newline-delimited JSON (one JSON-RPC message per line). It responds to:

- `initialize` — capability negotiation
- `tools/list` — available tool definitions with JSON Schema inputs
- `tools/call` — tool execution

## 2. Module Structure

```
ralqlator/
  src/
    lib.rs              ← existing: export mcp module + new helper fns
    mcp.rs              ← NEW: MCP protocol types + handler
    bin/
      ralqlator_mcp.rs  ← NEW: binary entry point for MCP server
    ...
```

### 2.1 `src/mcp.rs` — MCP Protocol Layer

This module is divided into three sections:

**A. JSON-RPC 2.0 Types** (`McpRequest`, `McpResponse`, `McpError`)
- Generic JSON-RPC message envelope
- All-strings approach for `id` to bridge JavaScript/JSON Number semantics
- Error codes per JSON-RPC spec + MCP extensions

**B. MCP Tool Definitions** (`ToolDefinition`, `ToolInputSchema`, etc.)
- Statically defined tool descriptors
- JSON Schema (Draft-07 subset) for each tool's input parameters

**C. Request Handler** (`handle_request`)
- Dispatches `initialize`, `tools/list`, `tools/call` methods
- For `tools/call`: dispatches to calculator-specific functions
- Generic enough that adding future MCP methods (resources, prompts, etc.)
  requires only extending the match arms

### 2.2 `src/bin/ralqlator_mcp.rs` — Binary Entry Point

Minimal binary:
1. Read stdin line by line (BufReader)
2. Parse each line as JSON-RPC request
3. Call `mcp::handle_request(...)` 
4. Write JSON-RPC response to stdout
5. Exit cleanly on EOF

## 3. Tools and Their JSON Schemas

### 3.1 `calculate`

Evaluate a mathematical expression using ralqlator's AST-based rational arithmetic.

```
Input Schema:
{
  "type": "object",
  "properties": {
    "expression": {
      "type": "string",
      "description": "Mathematical expression to evaluate"
    }
  },
  "required": ["expression"]
}

Output:
{
  "content": [
    {
      "type": "text",
      "text": "result string (e.g. '7/3', '42', 'true')"
    }
  ]
}

On error: returns MCP error with code -32000 (calculation error)
         and descriptive message (e.g., "Division by zero")
```

Implementation: calls `parse_and_eval(expr, false, &user_functions, &user_constants)`
which returns `CalcResult<Value>`. Displays the Value using its `Display` impl.
Also returns `numerical_value` as f64 approximation in the result for programmatic use.

### 3.2 `calculate_bitwise`

Evaluate a bitwise expression (integer operations only).

```
Input Schema:
{
  "type": "object",
  "properties": {
    "expression": {
      "type": "string",
      "description": "Bitwise expression to evaluate"
    }
  },
  "required": ["expression"]
}
```

Implementation: calls `calculate_bitwise(expr)` which returns `Result<i64, String>`.

### 3.3 `list_functions`

Return all available mathematical functions with descriptions.

```
Input Schema: {}  (no parameters)
```

Returns a structured list of function names, their argument counts, and brief
descriptions. Sources: the `FUNCTIONS` constant in `operator.rs` + the built-in
rational functions from `parser.rs` (num/den/frac/rational/float/gcd/lcm).

### 3.4 `list_constants`

Return all built-in + user-defined mathematical constants.

```
Input Schema: {}  (no parameters)
```

Built-in constants: `C_PI` (pi, 3.14159...), `C_E` (e, 2.71828...).
User-defined constants are loaded from `~/.ralqlator` via `load_user_data()`.

### 3.5 `list_user_definitions`

Return all user-defined functions, sequences, and constants.

```
Input Schema: {}  (no parameters)
```

Loads from `~/.ralqlator` and returns structured lists of:
- Functions: name, parameter names, expression
- Sequences: name, parameter, expression
- Constants: name, value

## 4. Integration with Existing ralqlator lib API

### 4.1 Direct Reuse (no changes needed)

The MCP server directly calls these existing public APIs:

| MCP Tool | Library Function | Returns |
|----------|-----------------|---------|
| `calculate` | `parse_and_eval()` | `CalcResult<Value>` |
| `calculate_bitwise` | `calculator::calculate_bitwise()` | `Result<i64, String>` |
| `list_user_definitions` | `storage::load_user_data()` + lock maps | `Result<usize, String>` |
| `create_user_*` (future) | `calculator::create_user_function()` / `create_user_constant()` | `Result<(), String>` |

### 4.2 New Library Exports Needed

These items need to be made pub in lib.rs:

1. **`operator::FUNCTIONS`** — the static array of built-in function names
   (currently private to operator.rs). Needed by `list_functions`.

2. **`operator::is_function()`** — already public, but lib.rs doesn't export it.
   Export via `pub use operator::is_function`.

3. **`parser::is_builtin_constant()`** — already pub in parser.rs but not
   re-exported from lib.rs. Export it.

4. A new helper function **`list_builtin_functions() -> Vec<FunctionInfo>`** —
   to return structured function descriptions. Could live in a new helper section
   of the library or directly in the mcp module.

5. **`get_storage_path_string()`** — already exported from storage.rs.

### 4.3 User Data Loading Pattern

The MCP server maintains its own thread-local or per-request instances of
`UserFunctions` and `UserConstants`, loading from `load_user_data()`. This
matches the pattern used by the CLI binary in `main.rs`.

## 5. Key Implementation Decisions

### 5.1 Dependencies

**Minimal new dependency: only `serde_json`.**

- `serde` is already a dependency (used for TOML storage)
- `serde_json` is the standard JSON library for the Rust ecosystem
- No MCP SDK or client library is needed — the protocol is simple enough to
  implement in ~200 lines of pure JSON-RPC

Estimated `Cargo.toml` addition:
```toml
serde_json = "1.0"
```

### 5.2 Transport: Newline-Delimited JSON (NDJSON) over stdio

Each line on stdin is a complete JSON-RPC request. Each response is written as
a single JSON line to stdout.

Pros:
- Simplest possible transport for MCP
- No framing issues (newline is the delimiter)
- Works with any MCP client (Claude Desktop, VS Code, etc.)

### 5.3 Error Handling

| Error Condition | JSON-RPC Error Code | MCP Error Code |
|----------------|---------------------|----------------|
| Invalid JSON | -32700 (Parse Error) | — |
| Invalid request format | -32600 (Invalid Request) | — |
| Unknown method | -32601 (Method not found) | — |
| Invalid params | -32602 (Invalid params) | — |
| Calculation error | -32000 (Server error, application) | — |
| Internal error | -32603 (Internal error) | — |

### 5.4 JSON-RPC ID Handling

`id` is always a non-null value (string or number). We preserve the type from
the request. For string IDs, we round-trip without parsing (store as `serde_json::Value`).

### 5.5 Thread Safety

Since ralqlator's `UserFunctions` and `UserConstants` are `Arc<Mutex<...>>`,
they are thread-safe. The MCP server currently processes one request at a time
(synchronous read-process-write loop), so no concurrent access issues arise.

For future multi-threaded use, each request can clone the Arc and acquire the
lock independently.

### 5.6 Startup Behavior

- Reads and loads user data from `~/.ralqlator` on first request (not on startup)
- No persistent state between requests beyond what's loaded from disk
- Clean exit when stdin is closed (EOF)

## 6. What Needs to Change in lib.rs

### 6.1 New Module Declaration

```rust
pub mod mcp;
```

### 6.2 New Re-exports

```rust
// For MCP function listing
pub use operator::{FUNCTIONS as BUILTIN_FUNCTIONS};
pub use parser::is_builtin_constant;

// For MCP calculation
pub use calculator::{UserConstants, create_user_function, create_user_constant};
pub use storage::get_storage_path_string;
```

### 6.3 New Helper Function (optional but recommended)

```rust
/// Information about a built-in function
#[derive(Clone, Debug, serde::Serialize)]
pub struct FunctionInfo {
    pub name: &'static str,
    pub description: &'static str,
    pub min_args: usize,
    pub max_args: usize,
}

/// Return list of all built-in functions with descriptions
pub fn get_builtin_functions() -> Vec<FunctionInfo> { ... }
```

## 7. Example Session

```
--> {"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}
<-- {"jsonrpc":"2.0","id":1,"result":{"protocolVersion":"2024-11-05","serverInfo":{"name":"ralqlator-mcp","version":"0.5.1"},"capabilities":{"tools":{}}}}

--> {"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}
<-- {"jsonrpc":"2.0","id":2,"result":{"tools":[{"name":"calculate","description":"Evaluate a mathematical expression with exact rational arithmetic","inputSchema":{"type":"object","properties":{"expression":{"type":"string"}},"required":["expression"]}}, ...]}}

--> {"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"calculate","arguments":{"expression":"1/2 + 1/3"}}}
<-- {"jsonrpc":"2.0","id":3,"result":{"content":[{"type":"text","text":"5/6"}]}}
```

## 8. File Summary

| File | Action | Lines (est.) |
|------|--------|--------------|
| `src/mcp.rs` | NEW | ~250 lines |
| `src/bin/ralqlator_mcp.rs` | NEW | ~40 lines |
| `Cargo.toml` | ADD `serde_json` dep + `[[bin]]` target | +3 lines |
| `src/lib.rs` | Add `pub mod mcp` + re-exports | +8 lines |
| `docs/mcp-architecture.md` | NEW (this doc!) | — |
