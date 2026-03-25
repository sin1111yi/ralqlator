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

//! Ralqlator Library
//!
//! A powerful command-line calculator with support for:
//! - Exact rational arithmetic
//! - Floating-point calculations
//! - Bitwise operations
//! - Mathematical functions
//! - User-defined functions and constants

pub mod error;
pub mod value;
pub mod rational;
pub mod parser;
pub mod token;
pub mod operator;
pub mod functions;
pub mod shunting_yard;
pub mod evaluator;
pub mod calculator;
pub mod cli;
pub mod repl;

// Re-export commonly used types
pub use error::{CalcError, CalcResult};
pub use value::Value;
pub use parser::{AstNode, parse_expression, eval_ast, parse_and_eval};
pub use token::{Token, tokenize};
