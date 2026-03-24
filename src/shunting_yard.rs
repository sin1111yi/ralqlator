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

//! Shunting Yard Algorithm implementation
//!
//! Converts infix expressions to postfix notation.

use crate::operator::{
    bitwise_precedence, is_bitwise_operator, is_function, is_left_associative, is_operator,
    is_postfix_unary_operator, precedence,
};

/// State for the shunting yard algorithm
struct ShuntingYard {
    output: Vec<String>,
    op_stack: Vec<String>,
    paren_count: usize,
    expect_operand: bool,
}

impl ShuntingYard {
    fn new() -> Self {
        Self {
            output: Vec::new(),
            op_stack: Vec::new(),
            paren_count: 0,
            expect_operand: true,
        }
    }

    fn into_postfix(mut self) -> Vec<String> {
        // Pop remaining operators
        while let Some(op) = self.op_stack.pop() {
            if op != "(" && op != ")" {
                self.output.push(op);
            }
        }

        // Check for unmatched parentheses
        if self.paren_count != 0 {
            self.output
                .push("ERROR: unmatched parentheses".to_string());
        }

        self.output
    }
}

/// Convert infix expression to postfix (Shunting Yard Algorithm)
///
/// Supports:
/// - Multi-argument functions like `log(x, base)`
/// - Unary minus: `-x` becomes `0 x -`
/// - Comparison operators: `<`, `>`, `=`, `==`
/// - Postfix factorial: `5!`
pub fn infix_to_postfix(tokens: Vec<String>) -> Vec<String> {
    let mut sy = ShuntingYard::new();

    for token in tokens {
        process_standard_token(token, &mut sy);
    }

    sy.into_postfix()
}

/// Process a single token in standard mode
fn process_standard_token(token: String, sy: &mut ShuntingYard) {
    if is_postfix_unary_operator(&token) {
        // Postfix unary operators go directly to output
        sy.output.push(token);
        sy.expect_operand = false;
    } else if token == "-" && sy.expect_operand {
        // Unary minus: convert to `0 - x`
        sy.output.push("0".to_string());
        sy.op_stack.push("-".to_string());
        sy.expect_operand = true;
    } else if is_operator(&token) {
        pop_higher_precedence(token.as_str(), precedence, is_operator, sy);
        sy.op_stack.push(token);
        sy.expect_operand = true;
    } else if is_function(&token) || token == "(" {
        if token == "(" {
            sy.paren_count += 1;
        }
        sy.op_stack.push(token);
        sy.expect_operand = true;
    } else if token == "," {
        // Comma: pop operators until '('
        while let Some(top) = sy.op_stack.last() {
            if top != "(" {
                sy.output.push(sy.op_stack.pop().unwrap());
            } else {
                break;
            }
        }
        sy.output.push(token);
        sy.expect_operand = true;
    } else if token == ")" {
        if sy.paren_count == 0 {
            // Extra closing parenthesis
            sy.output.push("ERROR: unmatched closing parenthesis".to_string());
            return;
        }
        sy.paren_count -= 1;
        // Pop until '('
        while let Some(top) = sy.op_stack.pop() {
            if top == "(" {
                break;
            }
            sy.output.push(top);
        }
        // Pop function if present
        if let Some(top) = sy.op_stack.last() {
            if is_function(top) {
                sy.output.push(sy.op_stack.pop().unwrap());
            }
        }
        sy.expect_operand = false;
    } else {
        // Number
        sy.output.push(token);
        sy.expect_operand = false;
    }
}

/// Pop operators with higher or equal precedence from the stack
fn pop_higher_precedence<F, P>(
    current_op: &str,
    precedence_fn: P,
    is_op_fn: F,
    sy: &mut ShuntingYard,
) where
    F: Fn(&str) -> bool,
    P: Fn(&str) -> u8,
{
    let curr_prec = precedence_fn(current_op);
    let is_left_assoc = is_left_associative(current_op);

    while let Some(top) = sy.op_stack.last() {
        if is_op_fn(top) {
            let top_prec = precedence_fn(top);
            if (is_left_assoc && curr_prec <= top_prec)
                || (!is_left_assoc && curr_prec < top_prec)
            {
                sy.output.push(sy.op_stack.pop().unwrap());
                continue;
            }
        }
        break;
    }
}

/// Convert infix expression to postfix (Bitwise mode)
///
/// Supports:
/// - Bitwise operators: `&`, `|`, `^`, `~`, `<<`, `>>`
/// - Unary minus: `-x` becomes `NEG x` (two's complement)
///
/// Does NOT support binary subtraction (`a - b`)
pub fn infix_to_postfix_bitwise(tokens: Vec<String>) -> Vec<String> {
    let mut sy = ShuntingYard::new();

    for token in tokens {
        process_bitwise_token(token, &mut sy);
    }

    sy.into_postfix()
}

/// Process a single token in bitwise mode
fn process_bitwise_token(token: String, sy: &mut ShuntingYard) {
    if token == "-" && sy.expect_operand {
        // Unary minus: use NEG operator (two's complement)
        // NEG is a prefix unary operator, so we push it to stack
        sy.op_stack.push("NEG".to_string());
        sy.expect_operand = true;
    } else if token == "-" {
        // Binary subtraction not supported
        sy.output.push(
            "ERROR: binary subtraction (-) not supported in bitwise mode, \
             use two's complement manually"
                .to_string(),
        );
        return;
    } else if is_bitwise_operator(&token) {
        // For bitwise operators, pop higher precedence operators
        // But NEG is a prefix unary operator, so it should stay on stack until we see an operand
        pop_higher_precedence(
            token.as_str(),
            bitwise_precedence,
            is_bitwise_operator,
            sy,
        );
        sy.op_stack.push(token);
        sy.expect_operand = true;
    } else if token == "(" {
        sy.op_stack.push(token);
        sy.expect_operand = true;
    } else if token == ")" {
        while let Some(top) = sy.op_stack.pop() {
            if top == "(" {
                break;
            }
            sy.output.push(top);
        }
        sy.expect_operand = false;
    } else {
        // Number - push to output, then check if there's a pending NEG
        sy.output.push(token);
        
        // If there's a pending NEG on top of stack, pop it to output
        if let Some(top) = sy.op_stack.last() {
            if top == "NEG" {
                sy.output.push(sy.op_stack.pop().unwrap());
            }
        }
        
        sy.expect_operand = false;
    }
}
