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

//! Tokenizer module - split input string into tokens
//!
//! Supports:
//! - Scientific notation (e.g., 1e3, 2.5e-3)
//! - Number prefixes: 0b (binary), 0o (octal), 0x (hex)
//! - Constants: pi, e
//! - Bitwise operators in bitwise mode

use crate::operator::{is_function, is_operator};

/// Tokenize input string into numbers and operators
pub fn tokenize(input: &str, bitwise_mode: bool) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            c if c.is_whitespace() => push_current(&mut tokens, &mut current),
            '(' | ')' | ',' => {
                push_current(&mut tokens, &mut current);
                tokens.push(c.to_string());
            }
            _ if bitwise_mode => {
                if handle_bitwise_char(c, &mut chars, &mut tokens, &mut current) {
                    continue;
                }
                handle_standard_char(c, &mut tokens, &mut current);
            }
            _ => handle_standard_char(c, &mut tokens, &mut current),
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

/// Push current token if not empty
#[inline]
fn push_current(tokens: &mut Vec<String>, current: &mut String) {
    if !current.is_empty() {
        tokens.push(std::mem::take(current));
    }
}

/// Handle bitwise mode specific characters
/// Returns true if the character was handled
fn handle_bitwise_char(
    c: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    tokens: &mut Vec<String>,
    current: &mut String,
) -> bool {
    match c {
        '<' | '>' => {
            if chars.peek() == Some(&c) {
                chars.next();
                push_current(tokens, current);
                tokens.push(format!("{}{}", c, c));
                return true;
            }
        }
        '~' | '&' | '|' | '^' => {
            push_current(tokens, current);
            tokens.push(c.to_string());
            return true;
        }
        _ => {}
    }
    false
}

/// Handle standard character processing
fn handle_standard_char(
    c: char,
    tokens: &mut Vec<String>,
    current: &mut String,
) {
    match c {
        'e' | 'E' => {
            if is_scientific_notation(current) {
                current.push(c);
            } else {
                push_current(tokens, current);
                tokens.push(c.to_string());
            }
        }
        '+' | '-' | '!' => {
            if c == '!' {
                // Factorial operator
                push_current(tokens, current);
                tokens.push(c.to_string());
            } else if is_scientific_sign(current) {
                current.push(c);
            } else if c == '-' && is_unary_context(tokens, current) {
                push_current(tokens, current);
                current.push(c);
            } else {
                push_current(tokens, current);
                tokens.push(c.to_string());
            }
        }
        '.' => current.push(c),
        c if c.is_ascii_digit() => current.push(c),
        'b' | 'B' | 'o' | 'O' | 'x' | 'X' => {
            if current == "0" || current == "-0" {
                current.push(c.to_ascii_lowercase());
            } else if current.is_empty() || current.chars().all(|ch| ch.is_alphabetic()) {
                current.push(c);
            } else {
                push_current(tokens, current);
                tokens.push(c.to_string());
            }
        }
        c if c.is_alphabetic() => current.push(c),
        c if is_operator(&c.to_string()) => {
            push_current(tokens, current);
            tokens.push(c.to_string());
        }
        _ => { push_current(tokens, current); }
    }
}

/// Check if 'e' or 'E' is part of scientific notation
#[inline]
fn is_scientific_notation(current: &str) -> bool {
    !current.is_empty() && current.chars().all(|ch| ch.is_ascii_digit() || ch == '.')
}

/// Check if '+' or '-' is part of scientific notation sign
#[inline]
fn is_scientific_sign(current: &str) -> bool {
    !current.is_empty() && matches!(current.chars().last(), Some('e' | 'E'))
}

/// Check if minus sign is in unary context
fn is_unary_context(tokens: &[String], current: &str) -> bool {
    current.is_empty()
        || tokens.last().is_none_or(|last| {
            is_operator(last) || last == "(" || is_function(last)
        })
}

/// Convert special constants and prefixed numbers to their values
pub fn resolve_constants(tokens: Vec<String>) -> Vec<String> {
    tokens.into_iter().map(|token| {
        if token.eq_ignore_ascii_case("pi") {
            std::f64::consts::PI.to_string()
        } else if token.eq_ignore_ascii_case("e") && token.len() == 1 {
            std::f64::consts::E.to_string()
        } else {
            parse_prefixed_number(&token).map_or(token, |v| v.to_string())
        }
    }).collect()
}

/// Parse numbers with prefixes: 0b (binary), 0o (octal), 0x (hex)
fn parse_prefixed_number(token: &str) -> Option<f64> {
    let (prefix_len, radix) = get_prefix_info(token)?;
    let is_negative = token.starts_with('-');
    let num_str = &token[prefix_len..];

    i64::from_str_radix(num_str, radix).ok().map(|v| {
        if is_negative { -(v as f64) } else { v as f64 }
    })
}

/// Get prefix length and radix for prefixed numbers
fn get_prefix_info(token: &str) -> Option<(usize, u32)> {
    if token.starts_with("0b") || token.starts_with("-0b") {
        Some((if token.starts_with('-') { 3 } else { 2 }, 2))
    } else if token.starts_with("0o") || token.starts_with("-0o") {
        Some((if token.starts_with('-') { 3 } else { 2 }, 8))
    } else if token.starts_with("0x") || token.starts_with("-0x") {
        Some((if token.starts_with('-') { 3 } else { 2 }, 16))
    } else {
        None
    }
}
