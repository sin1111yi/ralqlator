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

//! Calculator module - expression calculation core
//!
//! Provides calculation functions for both standard and bitwise modes.
//!
//! Note: This module uses the legacy shunting_yard pipeline.
//! For the new AST-based evaluation, use the parser module directly.

use crate::evaluator::{eval_postfix, eval_postfix_bitwise};
use crate::functions::UserFunctions;
use crate::shunting_yard::{infix_to_postfix, infix_to_postfix_bitwise};
use crate::token::{is_reserved_constant_name, tokenize, Token};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Legacy function to resolve constants and prefixed numbers (for backward compatibility)
/// This is a temporary shim during the refactoring transition
fn resolve_constants_legacy(tokens: &[String]) -> Result<Vec<String>, String> {
    use crate::token::is_constant_identifier;

    let mut result = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        let token = &tokens[i];

        // Check for unary minus followed by prefixed number
        if token == "-" && i + 1 < tokens.len() {
            let next_token = &tokens[i + 1];
            if next_token.starts_with("0b")
                || next_token.starts_with("0o")
                || next_token.starts_with("0x")
            {
                return Err(format!(
                    "Negative non-decimal numbers are not supported. Use parentheses: -({})",
                    next_token
                ));
            }
        }

        // Check for built-in constants
        if is_constant_identifier(token) {
            let value = match token.as_str() {
                "C_PI" | "C_pi" => std::f64::consts::PI.to_string(),
                "C_E" | "C_e" => std::f64::consts::E.to_string(),
                _ => token.clone(),
            };
            result.push(value);
        } else if let Some(parse_result) = parse_prefixed_number_legacy(token) {
            result.push(parse_result?.to_string());
        } else {
            result.push(token.clone());
        }

        i += 1;
    }

    Ok(result)
}

/// Legacy function to parse prefixed numbers
fn parse_prefixed_number_legacy(token: &str) -> Option<Result<f64, String>> {
    let (prefix_len, radix) = match token {
        s if s.starts_with("0b") => Some((2, 2)),
        s if s.starts_with("0o") => Some((2, 8)),
        s if s.starts_with("0x") => Some((2, 16)),
        _ => None,
    }?;

    if token.starts_with('-') {
        return Some(Err(format!(
            "Negative non-decimal numbers are not supported. Use unary minus instead: -(0x{:})",
            &token[prefix_len..]
        )));
    }

    let num_str = &token[prefix_len..];
    Some(Ok(i64::from_str_radix(num_str, radix)
        .ok()
        .map(|v| v as f64)?))
}

/// User-defined constants storage
/// Maps constant name -> value
pub type UserConstants = Arc<Mutex<HashMap<String, f64>>>;

/// Calculate expression in standard mode using AST-based rational evaluation
/// This provides exact rational arithmetic for expressions like 1/2 + 1/3
pub fn calculate(expression: &str) -> Result<f64, String> {
    // Use AST-based evaluation for exact rational arithmetic
    let user_functions = Arc::new(Mutex::new(HashMap::new()));
    let user_constants = Arc::new(Mutex::new(HashMap::new()));

    // Import parser locally to avoid circular dependency
    use crate::parser;

    match parser::parse_and_eval(expression, false, &user_functions, &user_constants) {
        Ok(value) => Ok(value.to_float()),
        Err(_e) => {
            // Fall back to legacy evaluation if AST evaluation fails
            calculate_legacy(expression)
        }
    }
}

/// Legacy calculation using shunting_yard pipeline (for backward compatibility)
fn calculate_legacy(expression: &str) -> Result<f64, String> {
    let tokens = tokenize(expression, false);

    // Convert Token Vec<String> for legacy shunting_yard
    let string_tokens: Vec<String> = tokens
        .iter()
        .filter_map(|t| match t {
            Token::Number(s) => Some(s.clone()),
            Token::Identifier(s) => Some(s.clone()),
            Token::Operator(s) => Some(s.clone()),
            Token::LParen => Some("(".to_string()),
            Token::RParen => Some(")".to_string()),
            Token::Comma => Some(",".to_string()),
            Token::Eof => None,
        })
        .collect();

    // Resolve constants and prefixed numbers
    let tokens_resolved = resolve_constants_legacy(&string_tokens)?;
    let postfix = infix_to_postfix(tokens_resolved);
    eval_postfix(postfix)
}

/// Calculate expression with user-defined functions and constants
pub fn calculate_with_functions(
    expression: &str,
    user_functions: &UserFunctions,
    user_constants: &UserConstants,
) -> Result<f64, String> {
    // First, expand user-defined constants in the expression
    let expanded = expand_user_constants(expression, user_constants)?;
    // Then expand user-defined functions
    let expanded = expand_user_functions(&expanded, user_functions)?;
    calculate(&expanded)
}

/// Validate function/sequence name and parameters
/// Returns error if name conflicts with reserved constant format (C_xx)
fn validate_definition(name: &str, params: &[String]) -> Result<(), String> {
    // Check if name conflicts with reserved constant format
    if is_reserved_constant_name(name) {
        return Err(format!(
            "Name '{}' is reserved for constants (C_xx format), cannot be used for function/sequence",
            name
        ));
    }

    // Check if any parameter name conflicts with reserved constant format
    for param in params {
        if is_reserved_constant_name(param) {
            return Err(format!(
                "Parameter name '{}' is reserved for constants (C_xx format)",
                param
            ));
        }
    }

    Ok(())
}

/// Expand user-defined constants in expression
/// Replaces constant names with their values
pub fn expand_user_constants(
    expression: &str,
    user_constants: &UserConstants,
) -> Result<String, String> {
    let constants = user_constants.lock().unwrap();
    let mut result = expression.to_string();

    // Sort constants by name length (longest first) to avoid partial replacements
    let mut sorted_names: Vec<&String> = constants.keys().collect();
    sorted_names.sort_by_key(|b| std::cmp::Reverse(b.len()));

    for name in sorted_names {
        let value = constants.get(name).unwrap();
        // Simple word boundary replacement
        result = replace_word_boundary(&result, name, &value.to_string());
    }

    Ok(result)
}

/// Replace a word with another respecting word boundaries
fn replace_word_boundary(text: &str, word: &str, replacement: &str) -> String {
    let mut result = String::new();
    let mut last_end = 0;

    for (start, end) in find_word_occurrences(text, word) {
        // Check if this is a standalone word (not part of a larger identifier)
        let before_ok =
            start == 0 || !text[..start].ends_with(|c: char| c.is_alphanumeric() || c == '_');
        let after_ok = end >= text.len()
            || !text[end..].starts_with(|c: char| c.is_alphanumeric() || c == '_');

        result.push_str(&text[last_end..start]);
        if before_ok && after_ok {
            result.push_str(replacement);
        } else {
            result.push_str(word);
        }
        last_end = end;
    }
    result.push_str(&text[last_end..]);
    result
}

/// Find all occurrences of a word in text
fn find_word_occurrences(text: &str, word: &str) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    let mut start = 0;

    while let Some(pos) = text[start..].find(word) {
        let abs_start = start + pos;
        let abs_end = abs_start + word.len();
        positions.push((abs_start, abs_end));
        start = abs_end;
    }

    positions
}

/// Create a user-defined constant with validation
pub fn create_user_constant(
    name: &str,
    value: f64,
    user_constants: &UserConstants,
) -> Result<(), String> {
    // Check if name conflicts with reserved constant format
    if is_reserved_constant_name(name) {
        return Err(format!(
            "Name '{}' is reserved for built-in constants (C_xx format)",
            name
        ));
    }

    // Check if name looks like a function (contains parentheses)
    if name.contains('(') || name.contains(')') {
        return Err("Constant name cannot contain parentheses".to_string());
    }

    let mut constants = user_constants.lock().unwrap();
    constants.insert(name.to_string(), value);
    Ok(())
}

/// Expand user-defined functions in expression
/// Iteratively expands until no more changes (supports nested function calls)
pub fn expand_user_functions(
    expression: &str,
    user_functions: &UserFunctions,
) -> Result<String, String> {
    let funcs = user_functions.lock().unwrap();
    let mut result = expression.to_string();

    // First, handle suma(sequence, begin, end) special function
    result = expand_suma(&result, &funcs)?;

    // Iteratively expand until no more changes
    let mut changed = true;
    let max_iterations = 100; // Prevent infinite loops
    let mut iterations = 0;

    while changed && iterations < max_iterations {
        changed = false;
        iterations += 1;

        for (name, (params, expr)) in funcs.iter() {
            // Find pattern: name(...)
            while let Some(start) = result.find(&format!("{}(", name)) {
                // Find matching closing paren
                let arg_start = start + name.len() + 1;
                let mut paren_count = 1;
                let mut end = arg_start;

                while end < result.len() && paren_count > 0 {
                    match result.chars().nth(end) {
                        Some('(') => paren_count += 1,
                        Some(')') => paren_count -= 1,
                        _ => {}
                    }
                    if paren_count > 0 {
                        end += 1;
                    }
                }

                if paren_count != 0 {
                    return Err(format!("Unmatched parenthesis in function call: {}", name));
                }

                // Extract arguments
                let args_str = &result[arg_start..end];
                let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

                if args.len() != params.len() {
                    return Err(format!(
                        "Function '{}' expects {} arguments, got {}",
                        name,
                        params.len(),
                        args.len()
                    ));
                }

                // Replace parameters with arguments
                let mut expanded_expr = expr.clone();
                for (param, arg) in params.iter().zip(args.iter()) {
                    expanded_expr = expanded_expr.replace(param, arg);
                }

                // Wrap expanded expression in parentheses to preserve order of operations
                result.replace_range(start..=end, &format!("({})", expanded_expr));
                changed = true;
            }
        }
    }

    if iterations >= max_iterations {
        return Err(
            "Function expansion exceeded maximum iterations, possible circular reference"
                .to_string(),
        );
    }

    Ok(result)
}

/// Expand suma(sequence, begin, end) to sum of sequence terms
fn expand_suma(
    expression: &str,
    user_functions: &HashMap<String, (Vec<String>, String)>,
) -> Result<String, String> {
    let mut result = expression.to_string();

    // Find pattern: suma(name, begin, end)
    while let Some(start) = result.find("suma(") {
        let arg_start = start + 5;
        let mut paren_count = 1;
        let mut paren_end = arg_start;

        while paren_end < result.len() && paren_count > 0 {
            match result.chars().nth(paren_end) {
                Some('(') => paren_count += 1,
                Some(')') => paren_count -= 1,
                _ => {}
            }
            if paren_count > 0 {
                paren_end += 1;
            }
        }

        if paren_count != 0 {
            return Err("Unmatched parenthesis in suma() call".to_string());
        }

        // Extract arguments: seq_name, begin, end
        let args_str = &result[arg_start..paren_end];
        let args: Vec<&str> = args_str.split(',').map(|s| s.trim()).collect();

        if args.len() != 3 {
            return Err(format!(
                "suma: requires 3 arguments (sequence, begin, end), got {}",
                args.len()
            ));
        }

        let seq_name = args[0];
        let begin: i64 = args[1]
            .parse()
            .map_err(|_| format!("suma: begin must be an integer, got '{}'", args[1]))?;
        let seq_end: i64 = args[2]
            .parse()
            .map_err(|_| format!("suma: end must be an integer, got '{}'", args[2]))?;

        // Validate indices
        if begin < 1 {
            return Err(format!("suma: begin must be >= 1, got {}", begin));
        }
        if seq_end < 1 {
            return Err(format!("suma: end must be >= 1, got {}", seq_end));
        }
        if begin > seq_end {
            return Err(format!(
                "suma: begin ({}) must be <= end ({})",
                begin, seq_end
            ));
        }
        if seq_end > 1000 {
            return Err(format!("suma: end ({}) is too large (max 1000)", seq_end));
        }

        // Get sequence formula
        let (params, expr) = user_functions
            .get(seq_name)
            .ok_or_else(|| format!("suma: sequence '{}' not found", seq_name))?;

        if params.len() != 1 {
            return Err(format!(
                "suma: sequence '{}' must have exactly 1 parameter, got {}",
                seq_name,
                params.len()
            ));
        }

        let param = &params[0];

        // Expand to: seq(begin) + seq(begin+1) + ... + seq(end)
        let mut sum_parts = Vec::new();
        for i in begin..=seq_end {
            let mut term_expr = expr.clone();
            term_expr = term_expr.replace(param, &i.to_string());
            sum_parts.push(format!("({})", term_expr));
        }

        let sum_expr = sum_parts.join(" + ");
        result.replace_range(start..=paren_end, &sum_expr);
    }

    Ok(result)
}

/// Create a user-defined function with validation
pub fn create_user_function(
    name: &str,
    params: Vec<String>,
    expr: String,
    user_functions: &UserFunctions,
) -> Result<(), String> {
    validate_definition(name, &params)?;

    let mut funcs = user_functions.lock().unwrap();
    funcs.insert(name.to_string(), (params, expr));
    Ok(())
}

/// Create a user-defined sequence with validation
pub fn create_user_sequence(
    name: &str,
    param: String,
    expr: String,
    user_functions: &UserFunctions,
) -> Result<(), String> {
    validate_definition(name, std::slice::from_ref(&param))?;

    let mut funcs = user_functions.lock().unwrap();
    funcs.insert(name.to_string(), (vec![param], expr));
    Ok(())
}

/// Calculate expression in bitwise mode (integer)
pub fn calculate_bitwise(expression: &str) -> Result<i64, String> {
    let tokens = tokenize(expression, true);

    // Pre-check: reject floating point numbers and non-bitwise operators
    for token in &tokens {
        match token {
            Token::Number(s) => {
                // Reject floating point numbers
                if s.contains('.') {
                    return Err(format!(
                        "Bitwise mode only supports integers, got: {}",
                        s
                    ));
                }
                // Reject scientific notation
                if s.contains('e') || s.contains('E') {
                    return Err(format!(
                        "Bitwise mode does not support scientific notation, got: {}",
                        s
                    ));
                }
            }
            Token::Identifier(s) => {
                // Reject function calls and constants
                return Err(format!(
                    "Bitwise mode does not support functions or constants, got: {}",
                    s
                ));
            }
            Token::Operator(s) => {
                // Allow only bitwise operators
                let allowed_ops = ["&", "|", "^", "~", "<<", ">>"];
                if !allowed_ops.contains(&s.as_str()) {
                    return Err(format!(
                        "Operator '{}' is not supported in bitwise mode. Use: & | ^ ~ << >>",
                        s
                    ));
                }
            }
            _ => {}
        }
    }

    // Convert Token Vec<String> for legacy shunting_yard
    let string_tokens: Vec<String> = tokens
        .iter()
        .filter_map(|t| match t {
            Token::Number(s) => Some(s.clone()),
            Token::Identifier(s) => Some(s.clone()),
            Token::Operator(s) => Some(s.clone()),
            Token::LParen => Some("(".to_string()),
            Token::RParen => Some(")".to_string()),
            Token::Comma => Some(",".to_string()),
            Token::Eof => None,
        })
        .collect();

    let tokens_resolved = resolve_constants_legacy(&string_tokens)?;
    let postfix = infix_to_postfix_bitwise(tokens_resolved);
    eval_postfix_bitwise(postfix)
}
