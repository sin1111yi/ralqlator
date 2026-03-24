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

use crate::operator::{
    bitwise_precedence, is_bitwise_operator, is_function, is_left_associative, is_operator,
    is_postfix_unary_operator, is_unary_operator, precedence,
};

/// Convert infix expression to postfix (Shunting Yard Algorithm)
/// Supports multi-argument functions like log(x, base)
pub fn infix_to_postfix(tokens: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut op_stack: Vec<String> = Vec::new();
    let mut paren_count = 0;

    for token in tokens {
        if is_postfix_unary_operator(&token) {
            // Postfix unary operators go directly to output
            output.push(token);
        } else if is_operator(&token) {
            while let Some(top) = op_stack.last() {
                if is_operator(top) {
                    let top_prec = precedence(top);
                    let curr_prec = precedence(&token);
                    if (is_left_associative(&token) && curr_prec <= top_prec)
                        || (!is_left_associative(&token) && curr_prec < top_prec)
                    {
                        output.push(op_stack.pop().unwrap());
                        continue;
                    }
                }
                break;
            }
            op_stack.push(token);
        } else if is_function(&token) || token == "(" {
            if token == "(" {
                paren_count += 1;
            }
            op_stack.push(token);
        } else if token == "," {
            // Comma: pop operators until we hit '('
            while let Some(top) = op_stack.last() {
                if top != "(" {
                    output.push(op_stack.pop().unwrap());
                } else {
                    break;
                }
            }
            // Add comma to output as argument separator
            output.push(token);
        } else if token == ")" {
            if paren_count > 0 {
                paren_count -= 1;
            }
            while let Some(top) = op_stack.pop() {
                if top == "(" {
                    break;
                }
                output.push(top);
            }
            // If there's a function on top of stack, pop it to output
            if let Some(top) = op_stack.last() {
                if is_function(top) {
                    let func = op_stack.pop().unwrap();
                    output.push(func);
                }
            }
        } else {
            // Number
            output.push(token);
        }
    }

    // Check for unmatched parentheses
    if paren_count != 0 {
        output.push("ERROR: unmatched parentheses".to_string());
    }

    while let Some(op) = op_stack.pop() {
        if op != "(" && op != ")" {
            output.push(op);
        }
    }

    output
}

/// Convert infix expression to postfix (Bitwise mode)
/// Supports bitwise operators: & | ^ ~ << >>
pub fn infix_to_postfix_bitwise(tokens: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut op_stack: Vec<String> = Vec::new();

    for token in tokens {
        if is_bitwise_operator(&token) {
            let is_unary = is_unary_operator(&token);
            let curr_prec = bitwise_precedence(&token);
            let is_left_assoc = !is_unary; // Unary operators are right-associative

            while let Some(top) = op_stack.last() {
                if is_bitwise_operator(top) {
                    let top_prec = bitwise_precedence(top);
                    if (is_left_assoc && curr_prec <= top_prec)
                        || (!is_left_assoc && curr_prec < top_prec)
                    {
                        output.push(op_stack.pop().unwrap());
                        continue;
                    }
                }
                break;
            }
            op_stack.push(token);
        } else if token == "(" {
            op_stack.push(token);
        } else if token == ")" {
            while let Some(top) = op_stack.pop() {
                if top == "(" {
                    break;
                }
                output.push(top);
            }
        } else {
            // Number
            output.push(token);
        }
    }

    while let Some(op) = op_stack.pop() {
        if op != "(" && op != ")" {
            output.push(op);
        }
    }

    output
}
