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

//! Unified error handling for Ralqlator
//!
//! This module provides a comprehensive error type system for all calculation
//! and parsing operations.

use std::fmt;

/// Error codes for programmatic error handling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ErrorCode {
    ParseError = 1001,
    TypeError = 1002,
    DomainError = 1003,
    DivisionByZero = 1004,
    Overflow = 1005,
    UndefinedFunction = 1006,
    UndefinedConstant = 1007,
    UndefinedVariable = 1008,
    ArgumentCountMismatch = 1009,
    InvalidArgument = 1010,
    UnmatchedParenthesis = 1011,
    UnknownOperator = 1012,
    UnknownToken = 1013,
    InternalError = 1999,
}

/// Calculation error types
#[derive(Debug, Clone, PartialEq)]
pub enum CalcError {
    /// Parse error: invalid syntax or token
    ParseError(String),

    /// Type error: operation received incompatible types
    TypeError {
        expected: String,
        got: String,
    },

    /// Domain error: argument out of valid range
    DomainError(String),

    /// Division by zero
    DivisionByZero,

    /// Integer overflow
    Overflow(String),

    /// Undefined function
    UndefinedFunction(String),

    /// Undefined constant
    UndefinedConstant(String),

    /// Undefined variable or sequence
    UndefinedVariable(String),

    /// Argument count mismatch
    ArgumentCountMismatch {
        function: String,
        expected: usize,
        got: usize,
    },

    /// Invalid argument type
    InvalidArgument {
        function: String,
        message: String,
    },

    /// Unmatched parentheses
    UnmatchedParenthesis(String),

    /// Unknown operator
    UnknownOperator(String),

    /// Unknown token
    UnknownToken(String),

    /// Internal error (should not happen)
    InternalError(String),
}

impl CalcError {
    /// Get the error code for this error
    #[allow(dead_code)]
    pub fn code(&self) -> ErrorCode {
        match self {
            CalcError::ParseError(_) => ErrorCode::ParseError,
            CalcError::TypeError { .. } => ErrorCode::TypeError,
            CalcError::DomainError(_) => ErrorCode::DomainError,
            CalcError::DivisionByZero => ErrorCode::DivisionByZero,
            CalcError::Overflow(_) => ErrorCode::Overflow,
            CalcError::UndefinedFunction(_) => ErrorCode::UndefinedFunction,
            CalcError::UndefinedConstant(_) => ErrorCode::UndefinedConstant,
            CalcError::UndefinedVariable(_) => ErrorCode::UndefinedVariable,
            CalcError::ArgumentCountMismatch { .. } => ErrorCode::ArgumentCountMismatch,
            CalcError::InvalidArgument { .. } => ErrorCode::InvalidArgument,
            CalcError::UnmatchedParenthesis(_) => ErrorCode::UnmatchedParenthesis,
            CalcError::UnknownOperator(_) => ErrorCode::UnknownOperator,
            CalcError::UnknownToken(_) => ErrorCode::UnknownToken,
            CalcError::InternalError(_) => ErrorCode::InternalError,
        }
    }

    /// Create a parse error
    #[allow(dead_code)]
    pub fn parse(msg: impl Into<String>) -> Self {
        CalcError::ParseError(msg.into())
    }

    /// Create a type error
    #[allow(dead_code)]
    pub fn type_error(expected: impl Into<String>, got: impl Into<String>) -> Self {
        CalcError::TypeError {
            expected: expected.into(),
            got: got.into(),
        }
    }

    /// Create a domain error
    #[allow(dead_code)]
    pub fn domain(msg: impl Into<String>) -> Self {
        CalcError::DomainError(msg.into())
    }

    /// Create an undefined function error
    #[allow(dead_code)]
    pub fn undefined_function(name: impl Into<String>) -> Self {
        CalcError::UndefinedFunction(name.into())
    }

    /// Create an argument count mismatch error
    #[allow(dead_code)]
    pub fn arg_count_mismatch(
        function: impl Into<String>,
        expected: usize,
        got: usize,
    ) -> Self {
        CalcError::ArgumentCountMismatch {
            function: function.into(),
            expected,
            got,
        }
    }
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CalcError::TypeError { expected, got } => {
                write!(f, "Type error: expected {}, got {}", expected, got)
            }
            CalcError::DomainError(msg) => write!(f, "Domain error: {}", msg),
            CalcError::DivisionByZero => write!(f, "Division by zero"),
            CalcError::Overflow(msg) => write!(f, "Overflow: {}", msg),
            CalcError::UndefinedFunction(name) => write!(f, "Undefined function: {}", name),
            CalcError::UndefinedConstant(name) => write!(f, "Undefined constant: {}", name),
            CalcError::UndefinedVariable(name) => write!(f, "Undefined variable: {}", name),
            CalcError::ArgumentCountMismatch { function, expected, got } => {
                write!(
                    f,
                    "Function '{}': expected {} arguments, got {}",
                    function, expected, got
                )
            }
            CalcError::InvalidArgument { function, message } => {
                write!(f, "Invalid argument for '{}': {}", function, message)
            }
            CalcError::UnmatchedParenthesis(msg) => write!(f, "Unmatched parenthesis: {}", msg),
            CalcError::UnknownOperator(op) => write!(f, "Unknown operator: {}", op),
            CalcError::UnknownToken(token) => write!(f, "Unknown token: {}", token),
            CalcError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for CalcError {}

/// Result type alias for calculation operations
pub type CalcResult<T> = Result<T, CalcError>;

/// Convert CalcError to String (for compatibility with legacy code)
impl From<CalcError> for String {
    fn from(err: CalcError) -> Self {
        err.to_string()
    }
}

/// Convert String to CalcError (for compatibility with legacy code)
impl From<String> for CalcError {
    fn from(msg: String) -> Self {
        CalcError::ParseError(msg)
    }
}
