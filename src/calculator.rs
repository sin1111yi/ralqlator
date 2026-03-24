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

use crate::evaluator::{eval_postfix, eval_postfix_bitwise};
use crate::functions::UserFunctions;
use crate::shunting_yard::{infix_to_postfix, infix_to_postfix_bitwise};
use crate::token::{resolve_constants, tokenize};

use std::collections::HashMap;

/// Calculate expression in standard mode (floating point)
pub fn calculate(expression: &str) -> Result<f64, String> {
    let tokens = tokenize(expression, false);
    let tokens = resolve_constants(tokens);
    let postfix = infix_to_postfix(tokens);
    eval_postfix(postfix)
}

/// Calculate expression with user-defined functions
pub fn calculate_with_functions(expression: &str, user_functions: &UserFunctions) -> Result<f64, String> {
    // First, expand user-defined functions in the expression
    let expanded = expand_user_functions(expression, user_functions)?;
    calculate(&expanded)
}

/// Expand user-defined functions in expression
fn expand_user_functions(expression: &str, user_functions: &UserFunctions) -> Result<String, String> {
    let funcs = user_functions.lock().unwrap();
    let mut result = expression.to_string();
    
    // First, handle suma(sequence, begin, end) special function
    result = expand_suma(&result, &funcs)?;
    
    // Find all function calls like name(args)
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
                return Err(format!("Function '{}' expects {} arguments, got {}", 
                    name, params.len(), args.len()));
            }
            
            // Replace parameters with arguments
            let mut expanded_expr = expr.clone();
            for (param, arg) in params.iter().zip(args.iter()) {
                expanded_expr = expanded_expr.replace(param, arg);
            }
            
            // Replace function call with expanded expression
            result.replace_range(start..=end, &expanded_expr);
        }
    }
    
    Ok(result)
}

/// Expand suma(sequence, begin, end) to sum of sequence terms
fn expand_suma(expression: &str, user_functions: &HashMap<String, (Vec<String>, String)>) -> Result<String, String> {
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
            return Err(format!("suma: requires 3 arguments (sequence, begin, end), got {}", args.len()));
        }

        let seq_name = args[0];
        let begin: i64 = args[1].parse()
            .map_err(|_| format!("suma: begin must be an integer, got '{}'", args[1]))?;
        let seq_end: i64 = args[2].parse()
            .map_err(|_| format!("suma: end must be an integer, got '{}'", args[2]))?;

        // Validate indices
        if begin < 1 {
            return Err(format!("suma: begin must be >= 1, got {}", begin));
        }
        if seq_end < 1 {
            return Err(format!("suma: end must be >= 1, got {}", seq_end));
        }
        if begin > seq_end {
            return Err(format!("suma: begin ({}) must be <= end ({})", begin, seq_end));
        }
        if seq_end > 1000 {
            return Err(format!("suma: end ({}) is too large (max 1000)", seq_end));
        }

        // Get sequence formula
        let (params, expr) = user_functions.get(seq_name)
            .ok_or_else(|| format!("suma: sequence '{}' not found", seq_name))?;

        if params.len() != 1 {
            return Err(format!("suma: sequence '{}' must have exactly 1 parameter, got {}",
                seq_name, params.len()));
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

/// Calculate expression in bitwise mode (integer)
pub fn calculate_bitwise(expression: &str) -> Result<i64, String> {
    let tokens = tokenize(expression, true);
    let tokens = resolve_constants(tokens);
    let postfix = infix_to_postfix_bitwise(tokens);
    eval_postfix_bitwise(postfix)
}
