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

use crate::functions::{
    eval_acos, eval_asin, eval_atan, eval_cos, eval_factorial, eval_lg, eval_ln, eval_log_base,
    eval_mod, eval_pow, eval_sin, eval_sqrt, eval_sum, eval_tan,
};
use crate::operator::{is_function, is_operator, is_postfix_unary_operator};

/// Evaluate postfix expression
pub fn eval_postfix(postfix: Vec<String>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();
    let mut arg_stack: Vec<Vec<f64>> = Vec::new();

    for token in postfix {
        if token == "," {
            // Comma: move current stack top to arg_stack as a completed argument
            if let Some(val) = stack.pop() {
                arg_stack.push(vec![val]);
            }
        } else if is_postfix_unary_operator(&token) {
            // Postfix unary operator (e.g., !)
            let a = stack.pop().ok_or("Stack is empty, invalid expression")?;
            let result = match token.as_str() {
                "!" => eval_factorial(a)?,
                _ => return Err(format!("Unknown postfix operator: {}", token)),
            };
            stack.push(result);
        } else if is_operator(&token) {
            let b = stack.pop().ok_or("Stack is empty, invalid expression")?;
            let a = stack.pop().ok_or("Stack is empty, invalid expression")?;
            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => {
                    if b == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    a / b
                }
                "%" => {
                    if b == 0.0 {
                        return Err("Modulo by zero".to_string());
                    }
                    a % b
                }
                "^" => a.powf(b),
                _ => return Err(format!("Unknown operator: {}", token)),
            };
            stack.push(result);
        } else if is_function(&token) {
            // Collect arguments: last arg is on stack, previous args are in arg_stack
            let mut args: Vec<f64> = Vec::new();

            // Get the last argument from stack
            if let Some(val) = stack.pop() {
                args.push(val);
            }

            // Get previous arguments from arg_stack (in reverse order)
            while let Some(mut arg_list) = arg_stack.pop() {
                args.append(&mut arg_list);
            }

            // Reverse to get correct order: args were collected from last to first
            args.reverse();

            let result = match token.as_str() {
                "lg" => match args.len() {
                    1 => eval_lg(args[0]),
                    2 => eval_log_base(args[1], args[0]),
                    _ => Err(format!("lg: requires 1 or 2 arguments, got {}", args.len())),
                },
                "log" => match args.len() {
                    2 => eval_log_base(args[1], args[0]),
                    _ => Err(format!(
                        "log: requires 2 arguments (x, base), got {}",
                        args.len()
                    )),
                },
                "ln" => match args.len() {
                    1 => eval_ln(args[0]),
                    _ => Err(format!("ln: requires 1 argument, got {}", args.len())),
                },
                "sqrt" => match args.len() {
                    1 => eval_sqrt(args[0]),
                    _ => Err(format!("sqrt: requires 1 argument, got {}", args.len())),
                },
                "pow" => match args.len() {
                    2 => eval_pow(args[0], args[1]),
                    _ => Err(format!(
                        "pow: requires 2 arguments (base, exp), got {}",
                        args.len()
                    )),
                },
                "sin" => match args.len() {
                    1 => Ok(eval_sin(args[0])),
                    _ => Err(format!("sin: requires 1 argument, got {}", args.len())),
                },
                "cos" => match args.len() {
                    1 => Ok(eval_cos(args[0])),
                    _ => Err(format!("cos: requires 1 argument, got {}", args.len())),
                },
                "tan" => match args.len() {
                    1 => eval_tan(args[0]),
                    _ => Err(format!("tan: requires 1 argument, got {}", args.len())),
                },
                "asin" => match args.len() {
                    1 => eval_asin(args[0]),
                    _ => Err(format!("asin: requires 1 argument, got {}", args.len())),
                },
                "acos" => match args.len() {
                    1 => eval_acos(args[0]),
                    _ => Err(format!("acos: requires 1 argument, got {}", args.len())),
                },
                "atan" => match args.len() {
                    1 => Ok(eval_atan(args[0])),
                    _ => Err(format!("atan: requires 1 argument, got {}", args.len())),
                },
                "factorial" => match args.len() {
                    1 => eval_factorial(args[0]),
                    _ => Err(format!("factorial: requires 1 argument, got {}", args.len())),
                },
                "sum" => match args.len() {
                    1 => eval_sum(args[0]),
                    _ => Err(format!("sum: requires 1 argument, got {}", args.len())),
                },
                "mod" => match args.len() {
                    2 => eval_mod(args[0], args[1]),
                    _ => Err(format!(
                        "mod: requires 2 arguments (a, b), got {}",
                        args.len()
                    )),
                },
                _ => return Err(format!("Unknown function: {}", token)),
            }?;
            stack.push(result);
        } else {
            let num: f64 = token
                .parse()
                .map_err(|_| format!("Invalid number: {}", token))?;
            stack.push(num);
        }
    }

    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err("Invalid expression: multiple values remaining in stack".to_string())
    }
}

/// Evaluate postfix expression (bitwise mode)
/// All values are integers, operators: & | ^ ~ << >>
pub fn eval_postfix_bitwise(postfix: Vec<String>) -> Result<i64, String> {
    use crate::operator::is_unary_operator;

    let mut stack: Vec<i64> = Vec::new();

    for token in postfix {
        if is_unary_operator(&token) {
            let a = stack.pop().ok_or("Stack is empty, invalid expression")?;
            let result = match token.as_str() {
                "~" => !a,
                _ => return Err(format!("Unknown unary operator: {}", token)),
            };
            stack.push(result);
        } else if crate::operator::is_bitwise_operator(&token) {
            let b = stack.pop().ok_or("Stack is empty, invalid expression")?;
            let a = stack.pop().ok_or("Stack is empty, invalid expression")?;
            let result = match token.as_str() {
                "&" => a & b,
                "|" => a | b,
                "^" => a ^ b,
                "<<" => {
                    if !(0..=63).contains(&b) {
                        return Err(format!("Invalid shift amount: {}", b));
                    }
                    a << b
                }
                ">>" => {
                    if !(0..=63).contains(&b) {
                        return Err(format!("Invalid shift amount: {}", b));
                    }
                    a >> b
                }
                _ => return Err(format!("Unknown bitwise operator: {}", token)),
            };
            stack.push(result);
        } else {
            // Parse as integer
            let num: i64 = token
                .parse()
                .map_err(|_| format!("Invalid integer: {}", token))?;
            stack.push(num);
        }
    }

    if stack.len() == 1 {
        Ok(stack[0])
    } else {
        Err("Invalid expression: multiple values remaining in stack".to_string())
    }
}
