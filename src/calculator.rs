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

/// Calculate expression in bitwise mode (integer)
pub fn calculate_bitwise(expression: &str) -> Result<i64, String> {
    let tokens = tokenize(expression, true);
    let tokens = resolve_constants(tokens);
    let postfix = infix_to_postfix_bitwise(tokens);
    eval_postfix_bitwise(postfix)
}
