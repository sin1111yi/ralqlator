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

//! Command-line interface definitions for Ralqlator
//!
//! This module defines all CLI arguments, subcommands, and help messages.

use clap::{Parser, Subcommand};

/// Ralqlator - A powerful command-line calculator
///
/// Ralqlator supports:
/// - Basic arithmetic: +, -, *, /, %, ^ (exponentiation), ! (factorial)
/// - Bitwise operations: &, |, ^, ~, <<, >> (use -B flag)
/// - Mathematical functions: sin, cos, tan, log, sqrt, etc.
/// - BigInt functions: bfactorial, bpow, comb, perm, gcd, lcm, isprime, nextprime
/// - Comparison operators: <, >, =, ==
/// - Number formats: decimal, binary (0b), octal (0o), hexadecimal (0x)
/// - Scientific notation: 1e3, 2.5e-3
/// - User-defined functions, sequences, and constants
/// - Interactive REPL mode with history and Tab completion
#[derive(Parser)]
#[command(name = "ralqlator")]
#[command(author = "Ralqlator Contributors")]
#[command(about = "A powerful command-line calculator", long_about = None)]
#[command(after_help = "Examples:
  ralqlator \"1 + 2 * 3\"              Calculate expression (output: 7)
  ralqlator \"sin(C_PI / 2)\"          Use functions and constants (output: 1)
  ralqlator -B \"12 & 10\"             Bitwise AND (output: 8)
  ralqlator -x \"255\"                 Hexadecimal output (output: 0xFF)
  ralqlator                            Interactive REPL mode
  ralqlator -B                         Interactive REPL in bitwise mode
  ralqlator -c \"func f(x) = x * 2\"   Create function from command line
  ralqlator -d f                       Delete function 'f'
  ralqlator -L                         List all user definitions

For more information, visit: https://github.com/ralqlator/ralqlator")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Expression to calculate
    #[arg(
        value_name = "EXPR",
        help_heading = "Positional Arguments",
        allow_hyphen_values = true
    )]
    pub expression: Option<String>,

    /// Expression to calculate (shorthand for EXPR)
    #[arg(
        short = 'r',
        long = "row",
        value_name = "EXPR",
        help_heading = "Positional Arguments",
        allow_hyphen_values = true
    )]
    pub row: Option<String>,

    /// Output result in hexadecimal format (e.g., 0xFF)
    #[arg(short = 'x', long = "hex", help_heading = "Output Format Options")]
    pub hex: bool,

    /// Output result in octal format (e.g., 0o377)
    #[arg(short = 'o', long = "oct", help_heading = "Output Format Options")]
    pub oct: bool,

    /// Output result in binary format (e.g., 0b11111111)
    #[arg(short = 'b', long = "bin", help_heading = "Output Format Options")]
    pub bin: bool,

    /// Bitwise operation mode (integer operations only)
    ///
    /// In this mode, use: & (AND), | (OR), ^ (XOR), ~ (NOT), << (left shift), >> (right shift)
    #[arg(short = 'B', long = "bits", help_heading = "Mode Options")]
    pub bits: bool,

    /// Show help for mathematical functions
    #[arg(
        short = 'F',
        long = "functions",
        action = clap::ArgAction::SetTrue,
        help_heading = "Help Options"
    )]
    pub show_functions: bool,

    /// Show help for operators (standard and bitwise)
    #[arg(
        short = 'O',
        long = "operators",
        action = clap::ArgAction::SetTrue,
        help_heading = "Help Options"
    )]
    pub show_operators: bool,

    /// Show help for number formats (input/output)
    #[arg(
        short = 'N',
        long = "formats",
        action = clap::ArgAction::SetTrue,
        help_heading = "Help Options"
    )]
    pub show_formats: bool,

    /// Show help for mathematical constants
    #[arg(
        short = 'C',
        long = "constants",
        action = clap::ArgAction::SetTrue,
        help_heading = "Help Options"
    )]
    pub show_constants: bool,

    /// Create a user-defined function, sequence, or constant
    ///
    /// Format: --create func name(args) = expression
    ///         --create seq name(n) = expression
    ///         --create const NAME value
    #[arg(
        short = 'c',
        long = "create",
        value_name = "DEFINITION",
        help_heading = "User Definitions",
        num_args = 1
    )]
    pub create: Option<String>,

    /// Delete a user-defined function, sequence, or constant by name
    #[arg(
        short = 'd',
        long = "destroy",
        value_name = "NAME",
        help_heading = "User Definitions",
        num_args = 1
    )]
    pub destroy: Option<String>,

    /// List all user-defined functions, sequences, and constants
    #[arg(
        short = 'L',
        long = "list",
        action = clap::ArgAction::SetTrue,
        help_heading = "User Definitions"
    )]
    pub list: bool,
}

/// Ralqlator subcommands for displaying help information
#[derive(Subcommand)]
pub enum Commands {
    /// Show supported mathematical functions
    #[command(
        long_about = "Display all supported mathematical functions including:
        
        Logarithms: lg(x), lg(x, base), log(x, base), ln(x), log2(x)
        
        Roots & Powers: sqrt(x), cbrt(x), pow(x, y)
        
        Trigonometric: sin(x), cos(x), tan(x), sec(x), csc(x), cot(x)
        
        Inverse Trig: asin(x), acos(x), atan(x), atan2(y, x)
        
        Hyperbolic: sinh(x), cosh(x), tanh(x), asinh(x), acosh(x), atanh(x)
        
        Special: factorial(n), gamma(n), erf(x), erfc(x), beta(x, y)
        
        BigInt: bfactorial(n), bpow(base, exp), comb(n, k), perm(n, k), 
                gcd(a, b), lcm(a, b), isprime(n), nextprime(n)
        
        Utility: mod(a, b), sum(a,b,...), prod(a,b,...), abs(x), 
                 floor(x), ceil(x), round(x)
        
        Sequence: suma(seq, begin, end) - sum of sequence terms
        
        For interactive mode, use: help functions"
    )]
    Functions,

    /// Show supported operators (standard and bitwise)
    #[command(
        long_about = "Display all supported operators:
        
        Arithmetic Operators:
          +    Addition           3 + 2 = 5
          -    Subtraction        5 - 3 = 2
          *    Multiplication     3 * 4 = 12
          /    Division           10 / 2 = 5
          %    Modulo             10 % 3 = 1
          ^    Exponentiation     2 ^ 3 = 8
          !    Factorial          5! = 120
        
        Comparison Operators:
          <    Less than          3 < 5 → true
          >    Greater than       5 > 3 → true
          =    Equality           5 = 5 → true
          ==   Logical equality   5 == 5 → true
        
        Bitwise Operators (use -B mode):
          &    AND                12 & 10 = 8
          |    OR                 12 | 10 = 14
          ^    XOR                12 ^ 10 = 6
          ~    NOT                ~0 = -1
          <<   Left shift         8 << 2 = 32
          >>   Right shift        8 >> 2 = 2
        
        For interactive mode, use: help operators"
    )]
    Operators,

    /// Show supported number formats (input/output)
    #[command(
        long_about = "Display supported number formats:
        
        Input Formats:
          Decimal      255, -456, 3.14, 1e3
          Binary       0b1010, 0b11111111
          Octal        0o755, 0o377
          Hexadecimal  0xFF, 0x1A
          Scientific   1e3, 2.5e-3, 1.23E+10
        
        Output Format Options:
          -x, --hex    Hexadecimal output (e.g., 0xFF)
          -o, --oct    Octal output (e.g., 0o377)
          -b, --bin    Binary output (e.g., 0b11111111)
        
        Interactive Commands:
          hex          Show last result in hexadecimal
          oct          Show last result in octal
          bin          Show last result in binary
        
        For interactive mode, use: help formats"
    )]
    Formats,

    /// Show mathematical constants
    #[command(
        long_about = "Display built-in mathematical constants:
        
        Built-in Constants:
          C_PI    π ≈ 3.141592653589793
                  Ratio of circumference to diameter
          
          C_E     e ≈ 2.718281828459045
                  Euler's number, natural log base
        
        Note:
          Constants use C_ prefix format (C_PI, C_E).
          Names starting with C_ are reserved and cannot be used
          for user-defined functions or sequences.
        
        User-defined Constants:
          In interactive mode, use: create const NAME value
          Example: create const G 9.81
        
        For interactive mode, use: help constants"
    )]
    Constants,

    /// Show all help information
    #[command(
        long_about = "Display comprehensive help information including:
        
        - All supported operators (arithmetic, comparison, bitwise)
        - All mathematical functions (logarithms, trig, BigInt, etc.)
        - Number formats (input and output)
        - Mathematical constants
        
        For interactive mode help, start ralqlator and use:
          help                 Show all interactive commands
          help functions       Detailed function help
          help operators       Detailed operator help
          help formats         Detailed format help
          help constants       Detailed constant help
          help create          How to define functions/constants
          help mode            How to switch modes"
    )]
    Info,
}
