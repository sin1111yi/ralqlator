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
            // Whitespace: finalize current token
            c if c.is_whitespace() => {
                push_current(&mut tokens, &mut current);
            }

            // Parentheses and comma
            '(' | ')' | ',' => {
                push_current(&mut tokens, &mut current);
                tokens.push(c.to_string());
            }

            // Bitwise mode specific handling
            _ if bitwise_mode => {
                if handle_bitwise_char(c, &mut chars, &mut tokens, &mut current) {
                    continue;
                }
                handle_standard_char(c, &mut chars, &mut tokens, &mut current);
            }

            // Standard mode
            _ => handle_standard_char(c, &mut chars, &mut tokens, &mut current),
        }
    }

    // Push any remaining token
    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

/// Push current token if not empty
fn push_current(tokens: &mut Vec<String>, current: &mut String) {
    if !current.is_empty() {
        tokens.push(std::mem::take(current));
    }
}

/// Handle bitwise mode specific characters
fn handle_bitwise_char(
    c: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    tokens: &mut Vec<String>,
    current: &mut String,
) -> bool {
    match c {
        // Shift operators: << >>
        '<' | '>' => {
            if let Some(&next_c) = chars.peek() {
                if next_c == c {
                    chars.next();
                    push_current(tokens, current);
                    tokens.push(format!("{}{}", c, c));
                    return true;
                }
            }
        }

        // Bitwise operators: ~ & | ^
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
    _chars: &mut std::iter::Peekable<std::str::Chars>,
    tokens: &mut Vec<String>,
    current: &mut String,
) {
    match c {
        // Scientific notation
        'e' | 'E' => {
            if !current.is_empty() && current.chars().all(|ch| ch.is_ascii_digit() || ch == '.') {
                current.push(c);
            } else {
                push_current(tokens, current);
                tokens.push(c.to_string());
            }
        }

        // Sign handling (+ or -)
        '+' | '-' => {
            if !current.is_empty() && matches!(current.chars().last(), Some('e' | 'E')) {
                // Scientific notation sign
                current.push(c);
            } else if c == '-' && is_unary_context(tokens, current) {
                // Unary minus
                push_current(tokens, current);
                current.push(c);
            } else {
                // Binary operator
                push_current(tokens, current);
                tokens.push(c.to_string());
            }
        }

        // Decimal point
        '.' => current.push(c),

        // Digits
        c if c.is_ascii_digit() => current.push(c),

        // Number prefixes (0b, 0o, 0x) - only after "0" or "-0"
        'b' | 'B' | 'o' | 'O' | 'x' | 'X' => {
            if current == "0" || current == "-0" {
                current.push(c.to_ascii_lowercase());
            } else if current.is_empty() || current.chars().all(|ch| ch.is_alphabetic()) {
                // Part of a function name or constant
                current.push(c);
            } else {
                push_current(tokens, current);
                tokens.push(c.to_string());
            }
        }

        // Alphabetic (functions and constants)
        c if c.is_alphabetic() => current.push(c),

        // Standard operators
        c if is_operator(&c.to_string()) => {
            push_current(tokens, current);
            tokens.push(c.to_string());
        }

        // Unknown character: skip
        _ => {
            push_current(tokens, current);
        }
    }
}

/// Check if minus sign is in unary context
fn is_unary_context(tokens: &[String], current: &str) -> bool {
    current.is_empty()
        || tokens
            .last()
            .is_none_or(|last| is_operator(last) || last == "(" || is_function(last))
}

/// Convert special constants and prefixed numbers to their values
pub fn resolve_constants(tokens: Vec<String>) -> Vec<String> {
    tokens
        .into_iter()
        .map(|token| {
            if token.eq_ignore_ascii_case("pi") {
                std::f64::consts::PI.to_string()
            } else if token.eq_ignore_ascii_case("e") && token.len() == 1 {
                std::f64::consts::E.to_string()
            } else if let Some(val) = parse_prefixed_number(&token) {
                val.to_string()
            } else {
                token
            }
        })
        .collect()
}

/// Parse numbers with prefixes: 0b (binary), 0o (octal), 0x (hex)
fn parse_prefixed_number(token: &str) -> Option<f64> {
    let (prefix_len, radix) = get_prefix_info(token)?;

    let is_negative = token.starts_with('-');
    let num_str = &token[prefix_len..];

    i64::from_str_radix(num_str, radix).ok().map(
        |v| {
            if is_negative {
                -(v as f64)
            } else {
                v as f64
            }
        },
    )
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
