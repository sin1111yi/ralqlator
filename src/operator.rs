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

//! Operator definitions and utility functions
//!
//! This module provides operator recognition, precedence, and associativity functions.

/// List of all standard operators
const STANDARD_OPERATORS: &[&str] = &[
    "+", "-", "*", "/", "%", "^", "!", "&", "|", "<<", ">>", "<", ">", "=", "==",
];

/// List of comparison operators
const COMPARISON_OPERATORS: &[&str] = &["<", ">", "=", "=="];

/// List of bitwise operators
const BITWISE_OPERATORS: &[&str] = &["&", "|", "^", "<<", ">>", "~"];

/// List of unary operators
const UNARY_OPERATORS: &[&str] = &["~", "!"];

/// List of supported functions
const FUNCTIONS: &[&str] = &[
    "lg", "log", "ln", "log2", "sqrt", "cbrt", "pow",
    "sin", "cos", "tan", "sec", "csc", "cot",
    "asin", "acos", "atan", "atan2",
    "sinh", "cosh", "tanh", "asinh", "acosh", "atanh",
    "mod", "factorial", "gamma",
    "sum", "prod",
    "abs", "floor", "ceil", "round",
    "erf", "erfc",
    "beta",
    // BigInt functions
    "bfactorial", "bpow", "comb", "perm", "gcd", "lcm", "isprime", "nextprime",
];

/// Check if token is an operator
#[inline]
pub fn is_operator(token: &str) -> bool {
    STANDARD_OPERATORS.contains(&token)
}

/// Check if token is a comparison operator
#[inline]
pub fn is_comparison_operator(token: &str) -> bool {
    COMPARISON_OPERATORS.contains(&token)
}

/// Check if token is a bitwise operator
#[inline]
pub fn is_bitwise_operator(token: &str) -> bool {
    BITWISE_OPERATORS.contains(&token)
}

/// Check if token is a unary operator
#[inline]
pub fn is_unary_operator(token: &str) -> bool {
    UNARY_OPERATORS.contains(&token)
}

/// Check if token is a postfix unary operator
#[inline]
pub fn is_postfix_unary_operator(token: &str) -> bool {
    token == "!"
}

/// Check if token is a function
#[inline]
pub fn is_function(token: &str) -> bool {
    FUNCTIONS.contains(&token)
}

/// Operator precedence (standard mode)
/// Higher value = higher precedence
#[inline]
pub fn precedence(op: &str) -> u8 {
    match op {
        "=" | "==" | "<" | ">" => 0, // Comparison operators (lowest)
        "+" | "-" => 1,
        "*" | "/" | "%" => 2,
        "^" => 3,
        "!" => 4, // Postfix unary (highest)
        _ => 0,
    }
}

/// Operator precedence (bitwise mode)
/// Higher value = higher precedence
#[inline]
pub fn bitwise_precedence(op: &str) -> u8 {
    match op {
        "|" => 1,                // lowest
        "^" => 2,
        "&" => 3,
        "<<" | ">>" => 4,
        "~" | "NEG" => 5,        // unary (highest)
        _ => 0,
    }
}

/// Check operator associativity
/// Returns `true` for left-associative, `false` for right-associative
#[inline]
pub fn is_left_associative(op: &str) -> bool {
    op != "^" // Exponentiation is right-associative
}
