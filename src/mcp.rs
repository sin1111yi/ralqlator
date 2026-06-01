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

//! MCP (Model Context Protocol) server implementation
//!
//! Implements JSON-RPC 2.0 over stdio transport for the Model Context Protocol.
//! Exposes ralqlator's calculation capabilities as MCP tools.
//!
//! Supported MCP methods:
//! - `initialize` — capability negotiation
//! - `tools/list` — list available tools with JSON Schema inputs
//! - `tools/call` — execute a tool and return results

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

// ============================================================================
// JSON-RPC 2.0 Protocol Types
// ============================================================================

/// JSON-RPC 2.0 request
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    #[serde(default)]
    pub id: Option<JsonValue>,
    pub method: String,
    #[serde(default)]
    pub params: Option<JsonValue>,
}

/// JSON-RPC 2.0 response (success)
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: JsonValue,
    pub result: JsonValue,
}

/// JSON-RPC 2.0 response (error)
#[derive(Debug, Serialize)]
pub struct JsonRpcErrorResponse {
    pub jsonrpc: String,
    pub id: JsonValue,
    pub error: JsonRpcErrorDetail,
}

/// JSON-RPC 2.0 error detail
#[derive(Debug, Serialize)]
pub struct JsonRpcErrorDetail {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonValue>,
}

/// Standard JSON-RPC error codes
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;
pub const CALCULATION_ERROR: i32 = -32000;

// ============================================================================
// MCP Protocol Types
// ============================================================================

/// MCP tool definition (for `tools/list` response)
#[derive(Debug, Serialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<JsonValue>,
}

/// MCP tool result content item
#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum ToolContent {
    #[serde(rename = "text")]
    Text { text: String },
}

/// Tool call result
#[derive(Debug, Serialize)]
pub struct ToolResult {
    pub content: Vec<ToolContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

impl ToolResult {
    pub fn ok(text: String) -> Self {
        ToolResult {
            content: vec![ToolContent::Text { text }],
            is_error: None,
        }
    }

    pub fn error(text: String) -> Self {
        ToolResult {
            content: vec![ToolContent::Text { text }],
            is_error: Some(true),
        }
    }
}

// ============================================================================
// Tool Schemas (JSON Schema Draft-07 subset)
// ============================================================================

/// Schema for the `calculate` tool
fn calculate_schema() -> JsonValue {
    serde_json::json!({
        "type": "object",
        "properties": {
            "expression": {
                "type": "string",
                "description": "Mathematical expression to evaluate"
            },
            "format": {
                "type": "string",
                "enum": ["decimal", "hex", "oct", "bin"],
                "description": "Output format for integer results (default: decimal)"
            }
        },
        "required": ["expression"],
        "additionalProperties": false
    })
}

/// Schema for the `calculate_bitwise` tool
fn calculate_bitwise_schema() -> JsonValue {
    serde_json::json!({
        "type": "object",
        "properties": {
            "expression": {
                "type": "string",
                "description": "Bitwise expression to evaluate (integers only)"
            },
            "format": {
                "type": "string",
                "enum": ["decimal", "hex", "oct", "bin"],
                "description": "Output format for the result (default: decimal)"
            }
        },
        "required": ["expression"],
        "additionalProperties": false
    })
}

/// Schema for tools that take no parameters
fn empty_schema() -> JsonValue {
    serde_json::json!({
        "type": "object",
        "properties": {},
        "additionalProperties": false
    })
}

/// Schema for the `create_user_definition` tool
fn create_user_definition_schema() -> JsonValue {
    serde_json::json!({
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "description": "Name of the user definition"
            },
            "type": {
                "type": "string",
                "enum": ["function", "sequence", "constant"],
                "description": "Type of definition to create"
            },
            "expression_or_value": {
                "type": "string",
                "description": "For function/sequence: the expression. For constant: the numeric value"
            },
            "params": {
                "type": "array",
                "items": { "type": "string" },
                "description": "Parameter names (required for functions, single-element for sequences, omitted for constants)"
            }
        },
        "required": ["name", "type", "expression_or_value"],
        "additionalProperties": false
    })
}

/// Schema for the `delete_user_definition` tool
fn delete_user_definition_schema() -> JsonValue {
    serde_json::json!({
        "type": "object",
        "properties": {
            "name": {
                "type": "string",
                "description": "Name of the user definition to delete"
            }
        },
        "required": ["name"],
        "additionalProperties": false
    })
}

/// Format an integer value with the specified base prefix
/// Negative numbers are represented in two's complement form (no sign prefix)
fn format_int_base(val: i64, hex: bool, oct: bool, bin: bool) -> String {
    let unsigned_val = val as u64;
    if hex {
        format!("0x{:X}", unsigned_val)
    } else if oct {
        format!("0o{:o}", unsigned_val)
    } else if bin {
        format!("0b{:b}", unsigned_val)
    } else {
        val.to_string()
    }
}

/// Apply format string to a float result
fn apply_format(result: f64, format: &str) -> Result<String, String> {
    match format {
        "decimal" => Ok(result.to_string()),
        "hex" | "oct" | "bin" => {
            if result.fract() != 0.0 || result.abs() >= 1e15 {
                return Err(format!(
                    "Base output only supports integer results, got: {}",
                    result
                ));
            }
            Ok(format_int_base(
                result as i64,
                format == "hex",
                format == "oct",
                format == "bin",
            ))
        }
        _ => Ok(result.to_string()),
    }
}

// ============================================================================
// Built-in Function Descriptions
// ============================================================================

/// Information about a built-in function
#[derive(Clone, Debug)]
pub struct FunctionInfo {
    pub name: &'static str,
    pub description: &'static str,
}

/// Return all built-in function descriptions
pub fn builtin_functions() -> Vec<FunctionInfo> {
    vec![
        // Logarithms
        FunctionInfo { name: "lg", description: "Base-10 logarithm: lg(x) or lg(x, base)" },
        FunctionInfo { name: "log", description: "Custom base logarithm: log(x, base)" },
        FunctionInfo { name: "ln", description: "Natural logarithm (base e): ln(x)" },
        FunctionInfo { name: "log2", description: "Base-2 logarithm: log2(x)" },
        // Roots & Powers
        FunctionInfo { name: "sqrt", description: "Square root: sqrt(x)" },
        FunctionInfo { name: "cbrt", description: "Cube root: cbrt(x)" },
        FunctionInfo { name: "pow", description: "Power: pow(base, exp)" },
        // Trigonometric
        FunctionInfo { name: "sin", description: "Sine (radians): sin(x)" },
        FunctionInfo { name: "cos", description: "Cosine (radians): cos(x)" },
        FunctionInfo { name: "tan", description: "Tangent (radians): tan(x)" },
        FunctionInfo { name: "sec", description: "Secant (radians): sec(x)" },
        FunctionInfo { name: "csc", description: "Cosecant (radians): csc(x)" },
        FunctionInfo { name: "cot", description: "Cotangent (radians): cot(x)" },
        FunctionInfo { name: "asin", description: "Inverse sine: asin(x)" },
        FunctionInfo { name: "acos", description: "Inverse cosine: acos(x)" },
        FunctionInfo { name: "atan", description: "Inverse tangent: atan(x)" },
        FunctionInfo { name: "atan2", description: "Two-argument arctangent: atan2(y, x)" },
        // Hyperbolic
        FunctionInfo { name: "sinh", description: "Hyperbolic sine: sinh(x)" },
        FunctionInfo { name: "cosh", description: "Hyperbolic cosine: cosh(x)" },
        FunctionInfo { name: "tanh", description: "Hyperbolic tangent: tanh(x)" },
        FunctionInfo { name: "asinh", description: "Inverse hyperbolic sine: asinh(x)" },
        FunctionInfo { name: "acosh", description: "Inverse hyperbolic cosine: acosh(x)" },
        FunctionInfo { name: "atanh", description: "Inverse hyperbolic tangent: atanh(x)" },
        // Special Functions
        FunctionInfo { name: "factorial", description: "Factorial: factorial(n) (max 170!)" },
        FunctionInfo { name: "gamma", description: "Gamma function: gamma(n) (Γ(n))" },
        FunctionInfo { name: "erf", description: "Error function: erf(x)" },
        FunctionInfo { name: "erfc", description: "Complementary error function: erfc(x)" },
        FunctionInfo { name: "beta", description: "Beta function: beta(x, y)" },
        // BigInt Functions
        FunctionInfo { name: "bfactorial", description: "BigInteger factorial: bfactorial(n) (max 10000!)" },
        FunctionInfo { name: "bpow", description: "BigInteger power: bpow(base, exp)" },
        FunctionInfo { name: "comb", description: "Combinations: comb(n, k) = C(n,k)" },
        FunctionInfo { name: "perm", description: "Permutations: perm(n, k) = P(n,k)" },
        FunctionInfo { name: "gcd", description: "Greatest common divisor: gcd(a, b)" },
        FunctionInfo { name: "lcm", description: "Least common multiple: lcm(a, b)" },
        FunctionInfo { name: "isprime", description: "Primality test: isprime(n)" },
        FunctionInfo { name: "nextprime", description: "Next prime after n: nextprime(n)" },
        // Utility Functions
        FunctionInfo { name: "mod", description: "Modulo: mod(a, b)" },
        FunctionInfo { name: "sum", description: "Sum of multiple arguments: sum(a, b, ...)" },
        FunctionInfo { name: "prod", description: "Product of multiple arguments: prod(a, b, ...)" },
        FunctionInfo { name: "abs", description: "Absolute value: abs(x)" },
        FunctionInfo { name: "floor", description: "Floor: floor(x)" },
        FunctionInfo { name: "ceil", description: "Ceiling: ceil(x)" },
        FunctionInfo { name: "round", description: "Round to nearest integer: round(x)" },
        // Sequence Functions
        FunctionInfo { name: "suma", description: "Sum of sequence from b to e: suma(seq, b, e)" },
        // Rational Functions
        FunctionInfo { name: "num", description: "Numerator of rational: num(x)" },
        FunctionInfo { name: "den", description: "Denominator of rational: den(x)" },
        FunctionInfo { name: "frac", description: "Fractional part: frac(x)" },
        FunctionInfo { name: "rational", description: "Create rational: rational(n,d) or rational(x)" },
        FunctionInfo { name: "float", description: "Convert to float: float(x)" },
        FunctionInfo { name: "cf", description: "Continued fraction: cf(x)" },
    ]
}

// ============================================================================
// Request Handler
// ============================================================================

use crate::calculator::{self, UserConstants};
use crate::functions::UserFunctions;
use crate::storage;

/// Handle an incoming MCP JSON-RPC request and return a JSON response.
///
/// Returns `None` if the request is a notification (no id), meaning no response
/// should be sent.
pub fn handle_request(request: JsonRpcRequest) -> Option<JsonValue> {
    let id = match request.id {
        Some(ref id_val) => id_val.clone(),
        None => return None, // Notification — no response
    };

    // Helper to build a success response
    let success = |result: JsonValue| -> JsonValue {
        serde_json::to_value(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: id.clone(),
            result,
        })
        .unwrap_or(JsonValue::Null)
    };

    // Helper to build an error response
    let error = |code: i32, message: String| -> JsonValue {
        serde_json::to_value(JsonRpcErrorResponse {
            jsonrpc: "2.0".to_string(),
            id: id.clone(),
            error: JsonRpcErrorDetail {
                code,
                message,
                data: None,
            },
        })
        .unwrap_or(JsonValue::Null)
    };

    match request.method.as_str() {
        "initialize" => {
            let result = serde_json::json!({
                "serverInfo": {
                    "name": "ralqlator-mcp",
                    "version": "0.4.0"
                },
                "capabilities": {
                    "tools": {}
                }
            });
            Some(success(result))
        }

        "tools/list" => {
            let tools = vec![
                ToolDefinition {
                    name: "calculate".to_string(),
                    description: "Evaluate a mathematical expression with exact rational arithmetic. Supports +, -, *, /, %, ^, !, functions (sin, cos, sqrt, etc.), and constants (C_PI, C_E). Optional 'format' parameter for hex/oct/bin output.".to_string(),
                    input_schema: Some(calculate_schema()),
                },
                ToolDefinition {
                    name: "calculate_bitwise".to_string(),
                    description: "Evaluate a bitwise expression (integers only). Supports &, |, ^, ~, <<, >>, and NOT (~). Optional 'format' parameter for hex/oct/bin output.".to_string(),
                    input_schema: Some(calculate_bitwise_schema()),
                },
                ToolDefinition {
                    name: "list_functions".to_string(),
                    description: "List all available mathematical functions with descriptions.".to_string(),
                    input_schema: Some(empty_schema()),
                },
                ToolDefinition {
                    name: "list_constants".to_string(),
                    description: "List all built-in and user-defined mathematical constants.".to_string(),
                    input_schema: Some(empty_schema()),
                },
                ToolDefinition {
                    name: "list_user_definitions".to_string(),
                    description: "List all user-defined functions, sequences, and constants.".to_string(),
                    input_schema: Some(empty_schema()),
                },
                ToolDefinition {
                    name: "create_user_definition".to_string(),
                    description: "Create a user-defined function, sequence, or constant with validation and persistence.".to_string(),
                    input_schema: Some(create_user_definition_schema()),
                },
                ToolDefinition {
                    name: "delete_user_definition".to_string(),
                    description: "Delete a user-defined function, sequence, or constant by name.".to_string(),
                    input_schema: Some(delete_user_definition_schema()),
                },
                ToolDefinition {
                    name: "list_operators".to_string(),
                    description: "List all available operators with their symbols, types, precedence, and descriptions.".to_string(),
                    input_schema: Some(empty_schema()),
                },
            ];

            let result = serde_json::json!({ "tools": tools });
            Some(success(result))
        }

        "tools/call" => {
            let params = match &request.params {
                Some(p) => p,
                None => return Some(error(INVALID_PARAMS, "Missing params".to_string())),
            };

            let tool_name = match params.get("name").and_then(|v| v.as_str()) {
                Some(name) => name,
                None => return Some(error(INVALID_PARAMS, "Missing tool name".to_string())),
            };

            let arguments = params.get("arguments").and_then(|v| v.as_object());

            match tool_name {
                "calculate" => {
                    let expr = arguments
                        .and_then(|a| a.get("expression"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    if expr.is_empty() {
                        return Some(error(INVALID_PARAMS, "Missing or empty 'expression' argument".to_string()));
                    }

                    // Get optional format parameter
                    let format = arguments
                        .and_then(|a| a.get("format"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("decimal");

                    // Load user definitions
                    let user_functions: UserFunctions =
                        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
                    let user_constants: UserConstants =
                        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
                    let _ = storage::load_user_data(&user_functions, &user_constants);

                    match calculator::calculate_with_functions(expr, &user_functions, &user_constants) {
                        Ok(result) => {
                            // Check if it's a comparison result (0.0/1.0 encoded as boolean)
                            let result_str = if let Some(cmp_str) = crate::evaluator::format_comparison_result(result) {
                                cmp_str
                            } else if format != "decimal" {
                                match apply_format(result, format) {
                                    Ok(s) => s,
                                    Err(e) => return Some(error(CALCULATION_ERROR, e)),
                                }
                            } else {
                                result.to_string()
                            };

                            let result_obj = serde_json::json!({
                                "content": [
                                    {
                                        "type": "text",
                                        "text": result_str
                                    }
                                ]
                            });
                            Some(success(result_obj))
                        }
                        Err(e) => Some(error(CALCULATION_ERROR, e)),
                    }
                }

                "calculate_bitwise" => {
                    let expr = arguments
                        .and_then(|a| a.get("expression"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    if expr.is_empty() {
                        return Some(error(INVALID_PARAMS, "Missing or empty 'expression' argument".to_string()));
                    }

                    // Get optional format parameter
                    let format = arguments
                        .and_then(|a| a.get("format"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("decimal");

                    match calculator::calculate_bitwise(expr) {
                        Ok(result) => {
                            let result_str = if format != "decimal" {
                                match apply_format(result as f64, format) {
                                    Ok(s) => s,
                                    Err(e) => return Some(error(CALCULATION_ERROR, e)),
                                }
                            } else {
                                result.to_string()
                            };

                            let result_obj = serde_json::json!({
                                "content": [
                                    {
                                        "type": "text",
                                        "text": result_str
                                    }
                                ]
                            });
                            Some(success(result_obj))
                        }
                        Err(e) => Some(error(CALCULATION_ERROR, e)),
                    }
                }

                "list_functions" => {
                    let fns = builtin_functions();
                    let fn_list: Vec<JsonValue> = fns
                        .into_iter()
                        .map(|f| {
                            serde_json::json!({
                                "name": f.name,
                                "description": f.description
                            })
                        })
                        .collect();

                    let result_obj = serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": serde_json::to_string_pretty(&fn_list).unwrap_or_default()
                            }
                        ]
                    });
                    Some(success(result_obj))
                }

                "list_constants" => {
                    // Load user data
                    let user_functions: UserFunctions =
                        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
                    let user_constants: UserConstants =
                        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
                    let _ = storage::load_user_data(&user_functions, &user_constants);

                    // Built-in constants
                    let builtin_constants = vec![
                        serde_json::json!({
                            "name": "C_PI",
                            "value": 3.141592653589793,
                            "description": "π - Ratio of circumference to diameter"
                        }),
                        serde_json::json!({
                            "name": "C_E",
                            "value": 2.718281828459045,
                            "description": "e - Euler's number, base of natural logarithms"
                        }),
                    ];

                    // User-defined constants
                    let consts = user_constants.lock().unwrap();
                    let user_const_list: Vec<JsonValue> = consts
                        .iter()
                        .map(|(name, value)| {
                            serde_json::json!({
                                "name": name,
                                "value": value,
                                "source": "user-defined"
                            })
                        })
                        .collect();
                    drop(consts);

                    let all_constants: Vec<JsonValue> = builtin_constants
                        .into_iter()
                        .chain(user_const_list)
                        .collect();

                    let result_obj = serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": serde_json::to_string_pretty(&all_constants).unwrap_or_default()
                            }
                        ]
                    });
                    Some(success(result_obj))
                }

                "list_user_definitions" => {
                    // Load user data
                    let user_functions: UserFunctions =
                        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
                    let user_constants: UserConstants =
                        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
                    let count = storage::load_user_data(&user_functions, &user_constants)
                        .unwrap_or(0);

                    let funcs = user_functions.lock().unwrap();
                    let fn_list: Vec<JsonValue> = funcs
                        .iter()
                        .map(|(name, (params, expr))| {
                            serde_json::json!({
                                "name": name,
                                "parameters": params,
                                "expression": expr,
                                "type": if params.len() == 1 { "sequence" } else { "function" }
                            })
                        })
                        .collect();
                    drop(funcs);

                    let consts = user_constants.lock().unwrap();
                    let const_list: Vec<JsonValue> = consts
                        .iter()
                        .map(|(name, value)| {
                            serde_json::json!({
                                "name": name,
                                "value": value,
                                "type": "constant"
                            })
                        })
                        .collect();
                    drop(consts);

                    let result_obj = serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": serde_json::to_string_pretty(&serde_json::json!({
                                    "count": count,
                                    "functions": fn_list,
                                    "constants": const_list
                                })).unwrap_or_default()
                            }
                        ]
                    });
                    Some(success(result_obj))
                }

                "create_user_definition" => {
                    let name = arguments
                        .and_then(|a| a.get("name"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    let def_type = arguments
                        .and_then(|a| a.get("type"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    let expr_or_value = arguments
                        .and_then(|a| a.get("expression_or_value"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    if name.is_empty() || def_type.is_empty() || expr_or_value.is_empty() {
                        return Some(error(INVALID_PARAMS, "Missing required arguments: 'name', 'type', and 'expression_or_value' are all required".to_string()));
                    }

                    // Load existing user definitions
                    let user_functions: UserFunctions =
                        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
                    let user_constants: UserConstants =
                        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
                    let _ = storage::load_user_data(&user_functions, &user_constants);

                    match def_type {
                        "function" => {
                            let params: Vec<String> = arguments
                                .and_then(|a| a.get("params"))
                                .and_then(|v| v.as_array())
                                .map(|arr| {
                                    arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect()
                                })
                                .unwrap_or_default();

                            if params.is_empty() {
                                return Some(error(INVALID_PARAMS, "Functions require at least one parameter in 'params' array".to_string()));
                            }

                            match calculator::create_user_function(name, params, expr_or_value.to_string(), &user_functions) {
                                Ok(()) => match storage::save_user_data(&user_functions, &user_constants) {
                                    Ok(()) => {
                                        let result_obj = serde_json::json!({
                                            "content": [{"type": "text", "text": format!("Function '{}' created and saved", name)}]
                                        });
                                        Some(success(result_obj))
                                    }
                                    Err(e) => Some(error(INTERNAL_ERROR, format!("Function created but failed to save: {}", e))),
                                },
                                Err(e) => Some(error(CALCULATION_ERROR, e)),
                            }
                        }
                        "sequence" => {
                            let param = arguments
                                .and_then(|a| a.get("params"))
                                .and_then(|v| v.as_array())
                                .and_then(|arr| arr.first())
                                .and_then(|v| v.as_str())
                                .unwrap_or("n");

                            match calculator::create_user_sequence(name, param.to_string(), expr_or_value.to_string(), &user_functions) {
                                Ok(()) => match storage::save_user_data(&user_functions, &user_constants) {
                                    Ok(()) => {
                                        let result_obj = serde_json::json!({
                                            "content": [{"type": "text", "text": format!("Sequence '{}' created and saved", name)}]
                                        });
                                        Some(success(result_obj))
                                    }
                                    Err(e) => Some(error(INTERNAL_ERROR, format!("Sequence created but failed to save: {}", e))),
                                },
                                Err(e) => Some(error(CALCULATION_ERROR, e)),
                            }
                        }
                        "constant" => {
                            let value: f64 = match expr_or_value.parse() {
                                Ok(v) => v,
                                Err(_) => return Some(error(INVALID_PARAMS, format!("Invalid numeric value for constant: '{}'", expr_or_value))),
                            };

                            match calculator::create_user_constant(name, value, &user_constants) {
                                Ok(()) => match storage::save_user_data(&user_functions, &user_constants) {
                                    Ok(()) => {
                                        let result_obj = serde_json::json!({
                                            "content": [{"type": "text", "text": format!("Constant '{}' = {} created and saved", name, value)}]
                                        });
                                        Some(success(result_obj))
                                    }
                                    Err(e) => Some(error(INTERNAL_ERROR, format!("Constant created but failed to save: {}", e))),
                                },
                                Err(e) => Some(error(CALCULATION_ERROR, e)),
                            }
                        }
                        _ => Some(error(INVALID_PARAMS, format!("Invalid definition type: '{}'. Must be 'function', 'sequence', or 'constant'", def_type))),
                    }
                }

                "delete_user_definition" => {
                    let name = arguments
                        .and_then(|a| a.get("name"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("");

                    if name.is_empty() {
                        return Some(error(INVALID_PARAMS, "Missing required argument: 'name'".to_string()));
                    }

                    // Load existing user definitions
                    let user_functions: UserFunctions =
                        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
                    let user_constants: UserConstants =
                        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()));
                    let _ = storage::load_user_data(&user_functions, &user_constants);

                    match storage::delete_user_definition(name, &user_functions, &user_constants) {
                        Ok(true) => {
                            let result_obj = serde_json::json!({
                                "content": [{"type": "text", "text": format!("Definition '{}' deleted", name)}]
                            });
                            Some(success(result_obj))
                        }
                        Ok(false) => Some(error(CALCULATION_ERROR, format!("Definition '{}' not found", name))),
                        Err(e) => Some(error(CALCULATION_ERROR, e)),
                    }
                }

                "list_operators" => {
                    let operators = serde_json::json!({
                        "arithmetic": [
                            {"symbol": "+", "description": "Addition", "precedence": 1},
                            {"symbol": "-", "description": "Subtraction", "precedence": 1},
                            {"symbol": "*", "description": "Multiplication", "precedence": 2},
                            {"symbol": "/", "description": "Division", "precedence": 2},
                            {"symbol": "%", "description": "Modulo", "precedence": 2},
                            {"symbol": "^", "description": "Exponentiation (right-associative)", "precedence": 3},
                            {"symbol": "!", "description": "Factorial (postfix)", "precedence": 4}
                        ],
                        "comparison": [
                            {"symbol": "<", "description": "Less than", "precedence": 0},
                            {"symbol": ">", "description": "Greater than", "precedence": 0},
                            {"symbol": "=", "description": "Equal (assignment)", "precedence": 0},
                            {"symbol": "==", "description": "Equal (comparison)", "precedence": 0}
                        ],
                        "bitwise": [
                            {"symbol": "&", "description": "Bitwise AND", "precedence": 3},
                            {"symbol": "|", "description": "Bitwise OR", "precedence": 1},
                            {"symbol": "^", "description": "Bitwise XOR", "precedence": 2},
                            {"symbol": "~", "description": "Bitwise NOT (unary)", "precedence": 5},
                            {"symbol": "<<", "description": "Left shift", "precedence": 4},
                            {"symbol": ">>", "description": "Right shift", "precedence": 4}
                        ],
                        "precedence_info": "Higher precedence values bind tighter. Use parentheses to override."
                    });

                    let result_obj = serde_json::json!({
                        "content": [{"type": "text", "text": serde_json::to_string_pretty(&operators).unwrap_or_default()}]
                    });
                    Some(success(result_obj))
                }

                _ => Some(error(METHOD_NOT_FOUND, format!("Unknown tool: {}", tool_name))),
            }
        }

        // Respond to any unknown method
        _ => Some(error(METHOD_NOT_FOUND, format!("Unknown method: {}", request.method))),
    }
}

/// Parse a JSON line into a JsonRpcRequest
pub fn parse_request(line: &str) -> Result<JsonRpcRequest, (i32, String)> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err((PARSE_ERROR, "Empty input".to_string()));
    }

    let value: JsonValue = serde_json::from_str(trimmed)
        .map_err(|e| (PARSE_ERROR, format!("Invalid JSON: {}", e)))?;

    let obj = value.as_object()
        .ok_or_else(|| (PARSE_ERROR, "Request must be a JSON object".to_string()))?;

    // Validate jsonrpc field
    match obj.get("jsonrpc").and_then(|v| v.as_str()) {
        Some("2.0") => {}
        Some(other) => {
            return Err((INVALID_REQUEST, format!("Invalid jsonrpc version: '{}' (must be '2.0')", other)));
        }
        None => {
            return Err((INVALID_REQUEST, "Missing 'jsonrpc' field".to_string()));
        }
    }

    // Validate method field
    match obj.get("method").and_then(|v| v.as_str()) {
        Some(m) if !m.is_empty() => {}
        Some(_) => {
            return Err((INVALID_REQUEST, "'method' must be a non-empty string".to_string()));
        }
        None => {
            return Err((INVALID_REQUEST, "Missing 'method' field".to_string()));
        }
    }

    serde_json::from_value::<JsonRpcRequest>(value)
        .map_err(|e| (PARSE_ERROR, format!("Invalid request format: {}", e)))
}

/// Build a JSON-RPC error response
pub fn build_error(id: Option<JsonValue>, code: i32, message: String) -> JsonValue {
    let effective_id = id.unwrap_or(JsonValue::Null);

    serde_json::to_value(JsonRpcErrorResponse {
        jsonrpc: "2.0".to_string(),
        id: effective_id,
        error: JsonRpcErrorDetail {
            code,
            message,
            data: None,
        },
    })
    .unwrap_or(JsonValue::Null)
}
