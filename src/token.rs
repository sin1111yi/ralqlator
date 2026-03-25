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
//! Pure lexical analysis only. Number parsing and constant resolution
//! are handled by the parser module.

use crate::operator::is_operator;

/// Token types for lexical analysis
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Number literal (raw string, parsing deferred to parser)
    Number(String),
    /// Identifier (function name, constant name, variable)
    Identifier(String),
    /// Operator (+, -, *, /, etc.)
    Operator(String),
    /// Left parenthesis
    LParen,
    /// Right parenthesis
    RParen,
    /// Comma (function argument separator)
    Comma,
    /// End of input
    Eof,
}

/// Tokenize input string into tokens
/// 
/// Supports:
/// - Scientific notation (e.g., 1e3, 2.5e-3)
/// - Number prefixes: 0b (binary), 0o (octal), 0x (hex)
/// - Identifiers: C_PI, C_E, function names
/// - Bitwise operators in bitwise mode
pub fn tokenize(input: &str, bitwise_mode: bool) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            c if c.is_whitespace() => push_current(&mut tokens, &mut current),
            '(' => {
                push_current(&mut tokens, &mut current);
                tokens.push(Token::LParen);
            }
            ')' => {
                push_current(&mut tokens, &mut current);
                tokens.push(Token::RParen);
            }
            ',' => {
                push_current(&mut tokens, &mut current);
                tokens.push(Token::Comma);
            }
            _ if bitwise_mode => {
                if handle_bitwise_char(c, &mut chars, &mut tokens, &mut current) {
                    continue;
                }
                handle_standard_char(c, &mut chars, &mut tokens, &mut current);
            }
            _ => handle_standard_char(c, &mut chars, &mut tokens, &mut current),
        }
    }

    push_current(&mut tokens, &mut current);
    tokens.push(Token::Eof);
    tokens
}

/// Push current token if not empty
#[inline]
fn push_current(tokens: &mut Vec<Token>, current: &mut String) {
    if !current.is_empty() {
        let token_str = std::mem::take(current);
        let token = classify_token(&token_str);
        tokens.push(token);
    }
}

/// Classify a token string into appropriate token type
fn classify_token(s: &str) -> Token {
    // Check if it's a number (starts with digit or minus followed by digit)
    if is_number_literal(s) {
        Token::Number(s.to_string())
    } else if is_operator(s) {
        Token::Operator(s.to_string())
    } else {
        Token::Identifier(s.to_string())
    }
}

/// Check if a string looks like a number literal
fn is_number_literal(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    
    // Check for prefixed numbers: 0b, 0o, 0x
    if s.starts_with("0b") || s.starts_with("0o") || s.starts_with("0x") {
        return true;
    }
    
    // Check for negative prefixed numbers
    if s.starts_with("-0b") || s.starts_with("-0o") || s.starts_with("-0x") {
        return true;
    }
    
    // Check if starts with digit or minus followed by digit
    let first_char = s.chars().next().unwrap();
    if first_char.is_ascii_digit() {
        return true;
    }
    if first_char == '-' && s.len() > 1 && s.chars().nth(1).unwrap().is_ascii_digit() {
        return true;
    }
    
    false
}

/// Handle bitwise mode specific characters
/// Returns true if the character was handled
fn handle_bitwise_char(
    c: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    tokens: &mut Vec<Token>,
    current: &mut String,
) -> bool {
    match c {
        '<' | '>' => {
            if chars.peek() == Some(&c) {
                chars.next();
                push_current(tokens, current);
                tokens.push(Token::Operator(format!("{}{}", c, c)));
                return true;
            } else {
                // Single < or > is comparison operator
                push_current(tokens, current);
                tokens.push(Token::Operator(c.to_string()));
                return true;
            }
        }
        '~' | '&' | '|' | '^' => {
            push_current(tokens, current);
            tokens.push(Token::Operator(c.to_string()));
            return true;
        }
        _ => {}
    }
    false
}

/// Handle standard character processing
fn handle_standard_char(
    c: char,
    chars: &mut std::iter::Peekable<std::str::Chars>,
    tokens: &mut Vec<Token>,
    current: &mut String,
) {
    match c {
        'e' | 'E' => {
            // Check if this is part of C_E constant (C_ followed by e/E) or scientific notation
            // or part of an identifier like erf, erfc, etc.
            if current == "C_" || is_scientific_notation(current)
               || current.is_empty() || current.chars().all(|ch| ch.is_alphabetic() || ch == '_') {
                current.push(c);
            } else {
                push_current(tokens, current);
                tokens.push(Token::Operator(c.to_string()));
            }
        }
        '+' | '-' | '!' => {
            if c == '!' {
                // Factorial operator - always an operator
                push_current(tokens, current);
                tokens.push(Token::Operator(c.to_string()));
            } else if is_scientific_sign(current) {
                // Plus or minus in scientific notation (e.g., 1e+3, 1e-3)
                current.push(c);
            } else if c == '+' {
                // Plus sign (not in scientific notation) - always an operator
                push_current(tokens, current);
                tokens.push(Token::Operator(c.to_string()));
            } else if c == '-' && is_unary_context(tokens, current) {
                // Unary minus - push as operator
                push_current(tokens, current);
                tokens.push(Token::Operator("-".to_string()));
            } else {
                // Binary minus - push as operator
                push_current(tokens, current);
                tokens.push(Token::Operator("-".to_string()));
            }
        }
        '<' | '>' => {
            // Comparison operators - push as operator
            push_current(tokens, current);
            tokens.push(Token::Operator(c.to_string()));
        }
        '=' => {
            // Check for == (double equals)
            if chars.peek() == Some(&'=') {
                chars.next();
                push_current(tokens, current);
                tokens.push(Token::Operator("==".to_string()));
            } else {
                // Single = for equality check
                push_current(tokens, current);
                tokens.push(Token::Operator("=".to_string()));
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
                tokens.push(Token::Operator(c.to_string()));
            }
        }
        c if c.is_alphabetic() || c == '_' => current.push(c),
        c if is_operator(&c.to_string()) => {
            push_current(tokens, current);
            tokens.push(Token::Operator(c.to_string()));
        }
        _ => {
            push_current(tokens, current);
        }
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
/// Unary context means the minus is for negation, not subtraction
fn is_unary_context(tokens: &[Token], current: &str) -> bool {
    // If current is not empty and contains a number, minus is binary (subtraction)
    // Exception: scientific notation like 1e-3 is handled separately by is_scientific_sign
    if !current.is_empty() {
        return false;
    }
    // If current is empty, check if we're after an operator, opening paren, comma, or function
    tokens.last().is_none_or(|last| {
        matches!(last, Token::Operator(_) | Token::LParen | Token::Comma)
    })
}

/// Check if a token is a reserved constant identifier (C_xx format)
pub fn is_constant_identifier(token: &str) -> bool {
    token.starts_with("C_") && token.len() > 2
}

/// Check if a name conflicts with reserved constant format (C_xx)
/// All C_ prefixed names are reserved for potential future constants
pub fn is_reserved_constant_name(name: &str) -> bool {
    is_constant_identifier(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_simple() {
        let tokens = tokenize("1 + 2", false);
        assert!(matches!(tokens[0], Token::Number(_)));
        assert!(matches!(tokens[1], Token::Operator(_)));
        assert!(matches!(tokens[2], Token::Number(_)));
        assert!(matches!(tokens[3], Token::Eof));
    }

    #[test]
    fn test_tokenize_function_call() {
        let tokens = tokenize("sin(x)", false);
        assert!(matches!(tokens[0], Token::Identifier(_)));
        assert!(matches!(tokens[1], Token::LParen));
        assert!(matches!(tokens[2], Token::Identifier(_)));
        assert!(matches!(tokens[3], Token::RParen));
    }

    #[test]
    fn test_tokenize_prefixed_number() {
        let tokens = tokenize("0xFF", false);
        assert!(matches!(tokens[0], Token::Number(_)));
    }

    #[test]
    fn test_tokenize_scientific_notation() {
        let tokens = tokenize("1e3", false);
        assert!(matches!(tokens[0], Token::Number(_)));
    }

    #[test]
    fn test_tokenize_constant() {
        let tokens = tokenize("C_PI", false);
        assert!(matches!(tokens[0], Token::Identifier(_)));
    }
}
