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

//! Parser module - Abstract Syntax Tree (AST) definition and parsing
//!
//! This module provides AST node types for representing parsed expressions.

use num_rational::Ratio;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive};

use crate::error::{CalcError, CalcResult};
use crate::rational::parse_decimal;

/// Abstract Syntax Tree node types
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    /// Numeric literal (rational number)
    Number(Ratio<BigInt>),
    
    /// Constant reference (e.g., C_PI, C_E)
    Constant(String),
    
    /// Function or sequence call
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },
    
    /// Binary operation
    BinaryOp {
        op: BinaryOperator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
    
    /// Unary operation (prefix)
    UnaryOp {
        op: UnaryOperator,
        operand: Box<AstNode>,
    },
    
    /// Postfix operation (e.g., factorial)
    PostfixOp {
        op: PostfixOperator,
        operand: Box<AstNode>,
    },
    
    /// Comparison operation
    Comparison {
        op: ComparisonOperator,
        left: Box<AstNode>,
        right: Box<AstNode>,
    },
}

/// Binary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator {
    Add,       // +
    Subtract,  // -
    Multiply,  // *
    Divide,    // /
    Modulo,    // %
    Power,     // ^
    #[allow(dead_code)]
    BitAnd,    // & (bitwise mode)
    #[allow(dead_code)]
    BitOr,     // | (bitwise mode)
    #[allow(dead_code)]
    BitXor,    // ^ (bitwise mode)
    #[allow(dead_code)]
    ShiftLeft, // << (bitwise mode)
    #[allow(dead_code)]
    ShiftRight,// >> (bitwise mode)
}

/// Unary operators (prefix)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOperator {
    Negate,   // -
    BitNot,   // ~
}

/// Postfix operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PostfixOperator {
    Factorial, // !
}

/// Comparison operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ComparisonOperator {
    LessThan,        // <
    GreaterThan,     // >
    Equal,           // = (returns true/false)
    LogicalEqual,    // == (returns true/false)
}

impl AstNode {
    /// Create a number node from i64
    #[allow(dead_code)]
    pub fn number_int(n: i64) -> Self {
        AstNode::Number(Ratio::new(BigInt::from(n), BigInt::one()))
    }

    /// Create a number node from a ratio
    #[allow(dead_code)]
    pub fn number_ratio(r: Ratio<BigInt>) -> Self {
        AstNode::Number(r)
    }

    /// Create a binary operation node
    pub fn binary(op: BinaryOperator, left: AstNode, right: AstNode) -> Self {
        AstNode::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a unary operation node
    pub fn unary(op: UnaryOperator, operand: AstNode) -> Self {
        AstNode::UnaryOp {
            op,
            operand: Box::new(operand),
        }
    }

    /// Create a postfix operation node
    pub fn postfix(op: PostfixOperator, operand: AstNode) -> Self {
        AstNode::PostfixOp {
            op,
            operand: Box::new(operand),
        }
    }

    /// Create a comparison node
    pub fn comparison(op: ComparisonOperator, left: AstNode, right: AstNode) -> Self {
        AstNode::Comparison {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Create a function call node
    pub fn function_call(name: String, args: Vec<AstNode>) -> Self {
        AstNode::FunctionCall { name, args }
    }

    /// Create a constant reference node
    #[allow(dead_code)]
    pub fn constant(name: String) -> Self {
        AstNode::Constant(name)
    }
}

/// Parse a number string into a Ratio
pub fn parse_number(s: &str) -> CalcResult<Ratio<BigInt>> {
    // Check for prefixed numbers
    if let Some(num_str) = s.strip_prefix("0b") {
        let n = i64::from_str_radix(num_str, 2)
            .map_err(|_| CalcError::ParseError(format!("Invalid binary number: {}", s)))?;
        return Ok(Ratio::new(BigInt::from(n), BigInt::one()));
    }
    
    if let Some(num_str) = s.strip_prefix("0o") {
        let n = i64::from_str_radix(num_str, 8)
            .map_err(|_| CalcError::ParseError(format!("Invalid octal number: {}", s)))?;
        return Ok(Ratio::new(BigInt::from(n), BigInt::one()));
    }
    
    if let Some(num_str) = s.strip_prefix("0x") {
        let n = i64::from_str_radix(num_str, 16)
            .map_err(|_| CalcError::ParseError(format!("Invalid hexadecimal number: {}", s)))?;
        return Ok(Ratio::new(BigInt::from(n), BigInt::one()));
    }

    // Check for negative prefixed numbers
    if s.starts_with("-0b") || s.starts_with("-0o") || s.starts_with("-0x") {
        return Err(CalcError::ParseError(
            "Negative non-decimal numbers are not supported. Use parentheses: -(0xFF)".to_string()
        ));
    }

    // Handle scientific notation
    if let Some(e_pos) = s.find(['e', 'E']) {
        let mantissa_str = &s[..e_pos];
        let exponent_str = &s[e_pos + 1..];

        let mantissa = parse_decimal(mantissa_str)?;
        let exponent: i32 = exponent_str.parse()
            .map_err(|_| CalcError::ParseError(format!("Invalid exponent: {}", exponent_str)))?;

        // Multiply by 10^exponent
        let result = if exponent >= 0 {
            &mantissa * BigInt::from(10u32).pow(exponent as u32)
        } else {
            &mantissa / BigInt::from(10u32).pow(exponent.unsigned_abs())
        };
        Ok(result)
    } else {
        // Regular decimal number
        parse_decimal(s)
    }
}

/// Check if a string is a valid identifier
#[allow(dead_code)]
pub fn is_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    let first = chars.next().unwrap();
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_alphanumeric() || c == '_')
}

/// Check if identifier is a built-in constant
pub fn is_builtin_constant(name: &str) -> bool {
    matches!(name, "C_PI" | "C_pi" | "C_E" | "C_e")
}

/// Get built-in constant value
pub fn get_builtin_constant(name: &str) -> Option<Ratio<BigInt>> {
    match name {
        "C_PI" | "C_pi" => Some(Ratio::new(
            BigInt::from(3141592653589793u64),
            BigInt::from(1000000000000000u64)
        )),
        "C_E" | "C_e" => Some(Ratio::new(
            BigInt::from(2718281828459045u64),
            BigInt::from(1000000000000000u64)
        )),
        _ => None,
    }
}

// ==================== Recursive Descent Parser ====================

use crate::token::{Token, tokenize};

/// Parse an expression string into an AST
pub fn parse_expression(input: &str, bitwise_mode: bool) -> CalcResult<AstNode> {
    let tokens = tokenize(input, bitwise_mode);
    let mut parser = Parser::new(&tokens);
    parser.parse_expression()
}

/// Recursive descent parser for expressions
struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> &Token {
        self.pos += 1;
        self.current()
    }

    fn peek(&self) -> &Token {
        self.current()
    }

    fn peek_next(&self) -> &Token {
        self.tokens.get(self.pos + 1).unwrap_or(&Token::Eof)
    }

    /// Parse expression (lowest precedence: comparison)
    fn parse_expression(&mut self) -> CalcResult<AstNode> {
        self.parse_comparison()
    }

    /// Parse comparison operators (=, ==, <, >)
    fn parse_comparison(&mut self) -> CalcResult<AstNode> {
        let mut left = self.parse_additive()?;

        loop {
            let op = match self.peek() {
                Token::Operator(s) if s == "<" => Some(ComparisonOperator::LessThan),
                Token::Operator(s) if s == ">" => Some(ComparisonOperator::GreaterThan),
                Token::Operator(s) if s == "=" => Some(ComparisonOperator::Equal),
                Token::Operator(s) if s == "==" => Some(ComparisonOperator::LogicalEqual),
                _ => None,
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_additive()?;
                left = AstNode::comparison(op, left, right);
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse additive operators (+, -)
    fn parse_additive(&mut self) -> CalcResult<AstNode> {
        let mut left = self.parse_multiplicative()?;

        loop {
            let op = match self.peek() {
                Token::Operator(s) if s == "+" => Some(BinaryOperator::Add),
                Token::Operator(s) if s == "-" => Some(BinaryOperator::Subtract),
                _ => None,
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_multiplicative()?;
                left = AstNode::binary(op, left, right);
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse multiplicative operators (*, /, %)
    fn parse_multiplicative(&mut self) -> CalcResult<AstNode> {
        let mut left = self.parse_power()?;

        loop {
            let op = match self.peek() {
                Token::Operator(s) if s == "*" => Some(BinaryOperator::Multiply),
                Token::Operator(s) if s == "/" => Some(BinaryOperator::Divide),
                Token::Operator(s) if s == "%" => Some(BinaryOperator::Modulo),
                _ => None,
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_power()?;
                left = AstNode::binary(op, left, right);
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse power operator (^) - right associative
    fn parse_power(&mut self) -> CalcResult<AstNode> {
        let base = self.parse_unary()?;

        if let Token::Operator(s) = self.peek() {
            if s == "^" {
                self.advance();
                let exp = self.parse_power()?; // Right associative
                return Ok(AstNode::binary(BinaryOperator::Power, base, exp));
            }
        }

        Ok(base)
    }

    /// Parse unary operators (-, ~)
    fn parse_unary(&mut self) -> CalcResult<AstNode> {
        match self.peek() {
            Token::Operator(s) if s == "-" => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(AstNode::unary(UnaryOperator::Negate, operand))
            }
            Token::Operator(s) if s == "~" => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(AstNode::unary(UnaryOperator::BitNot, operand))
            }
            _ => self.parse_postfix(),
        }
    }

    /// Parse postfix operators (!)
    fn parse_postfix(&mut self) -> CalcResult<AstNode> {
        let mut operand = self.parse_primary()?;

        if let Token::Operator(s) = self.peek() {
            if s == "!" {
                self.advance();
                operand = AstNode::postfix(PostfixOperator::Factorial, operand);
            }
        }

        Ok(operand)
    }

    /// Parse primary expressions (numbers, identifiers, function calls, parentheses)
    fn parse_primary(&mut self) -> CalcResult<AstNode> {
        match self.peek() {
            Token::Number(s) => {
                let num_str = s.clone();
                self.advance();
                
                // Check for fraction syntax: num/den
                // Look ahead: if next token is '/' and next-next is a number, parse as fraction
                if let Token::Operator(op) = self.peek() {
                    if op == "/" {
                        if let Token::Number(den_str) = self.peek_next() {
                            // Parse as fraction: num / den
                            let num = parse_number(&num_str)?;
                            let den = parse_number(den_str)?;
                            
                            // Both num and den are already Ratio<BigInt>
                            // We need den to be an integer (denominator = 1)
                            if !den.denom().is_one() {
                                return Err(CalcError::ParseError("Denominator must be an integer".to_string()));
                            }
                            
                            let den_int = match den.numer().to_i64() {
                                Some(n) => n,
                                None => return Err(CalcError::ParseError("Denominator too large".to_string())),
                            };
                            
                            if den_int == 0 {
                                return Err(CalcError::DivisionByZero);
                            }
                            
                            self.advance(); // consume '/'
                            self.advance(); // consume denominator
                            
                            let result = &num / BigInt::from(den_int);
                            return Ok(AstNode::Number(result));
                        }
                    }
                }
                
                let num = parse_number(&num_str)?;
                Ok(AstNode::Number(num))
            }
            Token::Identifier(name) => {
                let name_clone = name.clone();
                self.advance();
                
                // Check for function call
                if let Token::LParen = self.peek() {
                    self.advance(); // consume '('
                    let args = self.parse_function_args()?;
                    return Ok(AstNode::function_call(name_clone, args));
                }
                
                // Check for constant
                if is_builtin_constant(&name_clone) {
                    Ok(AstNode::Constant(name_clone))
                } else {
                    // User-defined function or sequence
                    Ok(AstNode::Constant(name_clone))
                }
            }
            Token::LParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                if let Token::RParen = self.peek() {
                    self.advance(); // consume ')'
                    Ok(expr)
                } else {
                    Err(CalcError::ParseError("Expected ')'".to_string()))
                }
            }
            Token::Eof => Err(CalcError::ParseError("Unexpected end of input".to_string())),
            _ => Err(CalcError::ParseError(format!("Unexpected token: {:?}", self.peek()))),
        }
    }

    /// Parse function arguments
    fn parse_function_args(&mut self) -> CalcResult<Vec<AstNode>> {
        let mut args = Vec::new();

        if let Token::RParen = self.peek() {
            self.advance(); // consume ')'
            return Ok(args);
        }

        args.push(self.parse_expression()?);

        while let Token::Comma = self.peek() {
            self.advance(); // consume ','
            args.push(self.parse_expression()?);
        }

        if let Token::RParen = self.peek() {
            self.advance(); // consume ')'
        } else {
            return Err(CalcError::ParseError("Expected ')' or ','".to_string()));
        }

        Ok(args)
    }
}

// ==================== AST Evaluator ====================

use crate::value::Value;
use crate::functions::UserFunctions;
use crate::calculator::UserConstants;

/// Evaluate an AST node to a Value
pub fn eval_ast(ast: &AstNode, user_functions: &UserFunctions, user_constants: &UserConstants) -> CalcResult<Value> {
    match ast {
        AstNode::Number(ratio) => {
            // Check if it's an integer
            if ratio.denom().is_one() {
                if let Some(n) = ratio.numer().to_i64() {
                    Ok(Value::Integer(n))
                } else {
                    Ok(Value::Rational(ratio.clone()))
                }
            } else {
                Ok(Value::Rational(ratio.clone()))
            }
        }
        AstNode::Constant(name) => {
            // Check built-in constants first
            if let Some(ratio) = get_builtin_constant(name) {
                if ratio.denom().is_one() {
                    if let Some(n) = ratio.numer().to_i64() {
                        return Ok(Value::Integer(n));
                    }
                }
                return Ok(Value::Rational(ratio));
            }
            
            // Check user-defined constants
            let constants = user_constants.lock().unwrap();
            if let Some(&value) = constants.get(name) {
                // Convert f64 to Value (approximate as rational)
                use crate::rational::float_to_rational;
                let ratio = float_to_rational(value, 1e-15);
                Ok(Value::Rational(ratio))
            } else {
                Err(CalcError::UndefinedConstant(name.clone()))
            }
        }
        AstNode::FunctionCall { name, args } => {
            eval_function_call(name, args, user_functions, user_constants)
        }
        AstNode::BinaryOp { op, left, right } => {
            let left_val = eval_ast(left, user_functions, user_constants)?;
            let right_val = eval_ast(right, user_functions, user_constants)?;
            eval_binary_op(op, &left_val, &right_val)
        }
        AstNode::UnaryOp { op, operand } => {
            let val = eval_ast(operand, user_functions, user_constants)?;
            eval_unary_op(op, &val)
        }
        AstNode::PostfixOp { op, operand } => {
            let val = eval_ast(operand, user_functions, user_constants)?;
            eval_postfix_op(op, &val)
        }
        AstNode::Comparison { op, left, right } => {
            let left_val = eval_ast(left, user_functions, user_constants)?;
            let right_val = eval_ast(right, user_functions, user_constants)?;
            eval_comparison(op, &left_val, &right_val)
        }
    }
}

/// Evaluate a function call
fn eval_function_call(
    name: &str,
    args: &[AstNode],
    user_functions: &UserFunctions,
    user_constants: &UserConstants,
) -> CalcResult<Value> {
    // First check user-defined functions
    {
        let funcs = user_functions.lock().unwrap();
        if let Some((params, expr)) = funcs.get(name) {
            // Expand function with arguments
            let mut expanded_expr = expr.clone();
            for (param, arg) in params.iter().zip(args.iter()) {
                // Evaluate argument first
                let arg_val = eval_ast(arg, user_functions, user_constants)?;
                let arg_str = match arg_val {
                    Value::Rational(ref r) => format!("{}", r),
                    Value::Integer(n) => n.to_string(),
                    Value::Float(f) => f.to_string(),
                    Value::Boolean(b) => if b { "1" } else { "0" }.to_string(),
                };
                expanded_expr = expanded_expr.replace(param, &arg_str);
            }
            // Parse and evaluate expanded expression
            let expanded_ast = parse_expression(&expanded_expr, false)?;
            return eval_ast(&expanded_ast, user_functions, user_constants);
        }
    }

    // Check for built-in rational functions
    eval_builtin_function(name, args, user_functions, user_constants)
}

/// Evaluate built-in rational functions
fn eval_builtin_function(
    name: &str,
    args: &[AstNode],
    user_functions: &UserFunctions,
    user_constants: &UserConstants,
) -> CalcResult<Value> {
    match name {
        // Rational number functions
        "num" | "numerator" => {
            if args.len() != 1 {
                return Err(CalcError::ArgumentCountMismatch {
                    function: name.to_string(),
                    expected: 1,
                    got: args.len(),
                });
            }
            let val = eval_ast(&args[0], user_functions, user_constants)?;
            match val {
                Value::Rational(r) => {
                    let num = r.numer().clone();
                    if let Some(n) = num.to_i64() {
                        Ok(Value::Integer(n))
                    } else {
                        Ok(Value::Rational(ratio_from_bigint(num)))
                    }
                }
                Value::Integer(n) => Ok(Value::Integer(n)),
                _ => Err(CalcError::TypeError {
                    expected: "Rational or Integer".to_string(),
                    got: "Other".to_string(),
                }),
            }
        }
        "den" | "denominator" => {
            if args.len() != 1 {
                return Err(CalcError::ArgumentCountMismatch {
                    function: name.to_string(),
                    expected: 1,
                    got: args.len(),
                });
            }
            let val = eval_ast(&args[0], user_functions, user_constants)?;
            match val {
                Value::Rational(r) => {
                    let den = r.denom().clone();
                    if let Some(d) = den.to_i64() {
                        Ok(Value::Integer(d))
                    } else {
                        Ok(Value::Rational(ratio_from_bigint(den)))
                    }
                }
                Value::Integer(_) => Ok(Value::Integer(1)),
                _ => Err(CalcError::TypeError {
                    expected: "Rational or Integer".to_string(),
                    got: "Other".to_string(),
                }),
            }
        }
        "frac" | "fractional_part" => {
            if args.len() != 1 {
                return Err(CalcError::ArgumentCountMismatch {
                    function: name.to_string(),
                    expected: 1,
                    got: args.len(),
                });
            }
            let val = eval_ast(&args[0], user_functions, user_constants)?;
            match val {
                Value::Rational(r) => {
                    // floor = numer / denom (integer division)
                    let floor_val = r.numer() / r.denom();
                    let floor_ratio = ratio_from_bigint(floor_val);
                    let frac = &r - &floor_ratio;
                    Ok(Value::Rational(frac))
                }
                Value::Integer(_) => Ok(Value::Integer(0)),
                Value::Float(f) => Ok(Value::Float(f.fract())),
                _ => Err(CalcError::TypeError {
                    expected: "Numeric".to_string(),
                    got: "Other".to_string(),
                }),
            }
        }
        "cf" | "continued_fraction" => {
            if args.len() != 1 {
                return Err(CalcError::ArgumentCountMismatch {
                    function: name.to_string(),
                    expected: 1,
                    got: args.len(),
                });
            }
            let val = eval_ast(&args[0], user_functions, user_constants)?;
            match val {
                Value::Rational(r) => {
                    let cf = crate::rational::continued_fraction(&r);
                    // Return as string representation
                    let cf_str = crate::rational::format_continued_fraction(&cf);
                    Err(CalcError::ParseError(format!("Continued fraction: {}", cf_str)))
                }
                _ => Err(CalcError::TypeError {
                    expected: "Rational".to_string(),
                    got: "Other".to_string(),
                }),
            }
        }
        // Conversion functions
        "rational" | "to_rational" => {
            if args.len() == 1 {
                // rational(x) - convert decimal to rational
                let val = eval_ast(&args[0], user_functions, user_constants)?;
                match val {
                    Value::Float(f) => {
                        use crate::rational::float_to_rational;
                        Ok(Value::Rational(float_to_rational(f, 1e-15)))
                    }
                    Value::Integer(n) => Ok(Value::Rational(ratio_from_int(n))),
                    Value::Rational(_) => Ok(val),
                    _ => Err(CalcError::TypeError {
                        expected: "Numeric".to_string(),
                        got: "Other".to_string(),
                    }),
                }
            } else if args.len() == 2 {
                // rational(num, den) - create from numerator and denominator
                let num_val = eval_ast(&args[0], user_functions, user_constants)?;
                let den_val = eval_ast(&args[1], user_functions, user_constants)?;
                let num = num_val.to_integer().ok_or_else(|| CalcError::TypeError {
                    expected: "Integer".to_string(),
                    got: "Non-integer".to_string(),
                })?;
                let den = den_val.to_integer().ok_or_else(|| CalcError::TypeError {
                    expected: "Integer".to_string(),
                    got: "Non-integer".to_string(),
                })?;
                if den == 0 {
                    return Err(CalcError::DivisionByZero);
                }
                Ok(Value::Rational(ratio_from_ints(num, den)))
            } else {
                Err(CalcError::ArgumentCountMismatch {
                    function: name.to_string(),
                    expected: 1,
                    got: args.len(),
                })
            }
        }
        "float" | "to_float" => {
            if args.len() != 1 {
                return Err(CalcError::ArgumentCountMismatch {
                    function: name.to_string(),
                    expected: 1,
                    got: args.len(),
                });
            }
            let val = eval_ast(&args[0], user_functions, user_constants)?;
            Ok(Value::Float(val.to_float()))
        }
        // GCD and LCM
        "gcd" => {
            if args.len() != 2 {
                return Err(CalcError::ArgumentCountMismatch {
                    function: name.to_string(),
                    expected: 2,
                    got: args.len(),
                });
            }
            let a = eval_ast(&args[0], user_functions, user_constants)?;
            let b = eval_ast(&args[1], user_functions, user_constants)?;
            let a_int = a.to_integer().ok_or_else(|| CalcError::TypeError {
                expected: "Integer".to_string(),
                got: "Non-integer".to_string(),
            })?;
            let b_int = b.to_integer().ok_or_else(|| CalcError::TypeError {
                expected: "Integer".to_string(),
                got: "Non-integer".to_string(),
            })?;
            let result = crate::rational::gcd_bigint(a_int.abs(), b_int.abs());
            Ok(Value::Integer(result))
        }
        "lcm" => {
            if args.len() != 2 {
                return Err(CalcError::ArgumentCountMismatch {
                    function: name.to_string(),
                    expected: 2,
                    got: args.len(),
                });
            }
            let a = eval_ast(&args[0], user_functions, user_constants)?;
            let b = eval_ast(&args[1], user_functions, user_constants)?;
            let a_int = a.to_integer().ok_or_else(|| CalcError::TypeError {
                expected: "Integer".to_string(),
                got: "Non-integer".to_string(),
            })?;
            let b_int = b.to_integer().ok_or_else(|| CalcError::TypeError {
                expected: "Integer".to_string(),
                got: "Non-integer".to_string(),
            })?;
            let result = crate::rational::lcm_bigint(a_int.abs(), b_int.abs());
            Ok(Value::Integer(result))
        }
        _ => Err(CalcError::UndefinedFunction(name.to_string())),
    }
}

/// Helper to create Ratio from BigInt
fn ratio_from_bigint(n: BigInt) -> num_rational::Ratio<BigInt> {
    use num_rational::Ratio;
    use num_traits::One;
    Ratio::new(n, BigInt::one())
}

/// Helper to create Ratio from i64
fn ratio_from_int(n: i64) -> num_rational::Ratio<BigInt> {
    use num_rational::Ratio;
    use num_traits::One;
    Ratio::new(BigInt::from(n), BigInt::one())
}

/// Helper to create Ratio from two i64 values
fn ratio_from_ints(num: i64, den: i64) -> num_rational::Ratio<BigInt> {
    use num_rational::Ratio;
    Ratio::new(BigInt::from(num), BigInt::from(den))
}

/// Evaluate a binary operation
fn eval_binary_op(op: &BinaryOperator, left: &Value, right: &Value) -> CalcResult<Value> {
    match op {
        BinaryOperator::Add => left.add(right),
        BinaryOperator::Subtract => left.sub(right),
        BinaryOperator::Multiply => left.mul(right),
        BinaryOperator::Divide => left.div(right),
        BinaryOperator::Modulo => left.modulo(right),
        BinaryOperator::Power => left.pow(right),
        // Bitwise operators - convert to integer first
        BinaryOperator::BitAnd | BinaryOperator::BitOr | BinaryOperator::BitXor 
        | BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight => {
            let left_int = left.to_integer().ok_or_else(|| CalcError::TypeError {
                expected: "Integer".to_string(),
                got: "Non-integer".to_string(),
            })?;
            let right_int = right.to_integer().ok_or_else(|| CalcError::TypeError {
                expected: "Integer".to_string(),
                got: "Non-integer".to_string(),
            })?;
            let result = match op {
                BinaryOperator::BitAnd => left_int & right_int,
                BinaryOperator::BitOr => left_int | right_int,
                BinaryOperator::BitXor => left_int ^ right_int,
                BinaryOperator::ShiftLeft => left_int << right_int,
                BinaryOperator::ShiftRight => left_int >> right_int,
                _ => unreachable!(),
            };
            Ok(Value::Integer(result))
        }
    }
}

/// Evaluate a unary operation
fn eval_unary_op(op: &UnaryOperator, val: &Value) -> CalcResult<Value> {
    match op {
        UnaryOperator::Negate => val.neg(),
        UnaryOperator::BitNot => {
            let int_val = val.to_integer().ok_or_else(|| CalcError::TypeError {
                expected: "Integer".to_string(),
                got: "Non-integer".to_string(),
            })?;
            Ok(Value::Integer(!int_val))
        }
    }
}

/// Evaluate a postfix operation
fn eval_postfix_op(op: &PostfixOperator, val: &Value) -> CalcResult<Value> {
    match op {
        PostfixOperator::Factorial => {
            // Use existing factorial function from functions module
            let float_val = val.to_float();
            let result = crate::functions::eval_factorial(float_val)?;
            Ok(Value::Float(result))
        }
    }
}

/// Evaluate a comparison operation
/// All comparison operators return true/false
fn eval_comparison(op: &ComparisonOperator, left: &Value, right: &Value) -> CalcResult<Value> {
    use crate::evaluator::{COMPARISON_TRUE_RESULT, COMPARISON_FALSE_RESULT};
    
    let is_true = match op {
        ComparisonOperator::LessThan => left.lt(right),
        ComparisonOperator::GreaterThan => left.gt(right),
        ComparisonOperator::Equal => left.is_equal(right),
        ComparisonOperator::LogicalEqual => left.is_equal(right),
    };
    
    if is_true {
        Ok(Value::Float(COMPARISON_TRUE_RESULT))
    } else {
        Ok(Value::Float(COMPARISON_FALSE_RESULT))
    }
}

/// Convenience function to parse and evaluate an expression
pub fn parse_and_eval(
    input: &str,
    bitwise_mode: bool,
    user_functions: &UserFunctions,
    user_constants: &UserConstants,
) -> CalcResult<Value> {
    let ast = parse_expression(input, bitwise_mode)?;
    eval_ast(&ast, user_functions, user_constants)
}
