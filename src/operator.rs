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

/// Check if token is an operator
pub fn is_operator(token: &str) -> bool {
    matches!(
        token,
        "+" | "-" | "*" | "/" | "%" | "^" | "!" | "&" | "|" | "<<" | ">>"
    )
}

/// Check if token is a bitwise operator
pub fn is_bitwise_operator(token: &str) -> bool {
    matches!(token, "&" | "|" | "^" | "<<" | ">>" | "~")
}

/// Check if token is a unary operator
pub fn is_unary_operator(token: &str) -> bool {
    matches!(token, "~" | "!")
}

/// Check if token is a postfix unary operator
pub fn is_postfix_unary_operator(token: &str) -> bool {
    token == "!"
}

/// Check if token is a function
pub fn is_function(token: &str) -> bool {
    matches!(
        token,
        "lg" | "log" | "ln" | "sqrt" | "pow" | "sin" | "cos" | "tan"
            | "asin" | "acos" | "atan" | "mod" | "factorial" | "sum"
    )
}

/// Operator precedence (standard mode)
pub fn precedence(op: &str) -> u8 {
    match op {
        "+" | "-" => 1,
        "*" | "/" | "%" => 2,
        "^" => 3,
        "!" => 4, // Postfix unary has highest precedence
        _ => 0,
    }
}

/// Operator precedence (bitwise mode)
pub fn bitwise_precedence(op: &str) -> u8 {
    match op {
        "|" => 1, // lowest
        "^" => 2,
        "&" => 3,
        "<<" | ">>" => 4,
        "~" => 5, // unary, highest
        _ => 0,
    }
}

/// Check operator associativity (true for left-associative, false for right-associative)
pub fn is_left_associative(op: &str) -> bool {
    op != "^" // Exponentiation is right-associative
}
