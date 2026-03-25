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

//! Unified value type for Ralqlator
//!
//! This module provides the core Value enum that can represent different
//! numeric types used in calculations.

use num_rational::Ratio;
use num_bigint::BigInt;
use num_traits::{Zero, One, Signed, ToPrimitive};

use crate::error::{CalcError, CalcResult};

/// Unified value type that can represent different numeric types
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Exact rational number (default for most calculations)
    Rational(Ratio<BigInt>),
    
    /// Floating point number (for transcendental functions)
    Float(f64),
    
    /// Integer (for bitwise operations and small integers)
    Integer(i64),
    
    /// Boolean (for comparison results)
    #[allow(dead_code)]
    Boolean(bool),
}

impl Value {
    /// Create a Value from an integer
    pub fn from_int(n: i64) -> Self {
        Value::Integer(n)
    }

    /// Create a Value from a BigInt
    pub fn from_bigint(n: BigInt) -> Self {
        if let Some(n) = n.to_i64() {
            Value::Integer(n)
        } else {
            Value::Rational(Ratio::new(n, BigInt::one()))
        }
    }

    /// Create a Value from a ratio
    pub fn from_ratio(r: Ratio<BigInt>) -> Self {
        // Optimize: if denominator is 1, use Integer
        if r.denom().is_one() {
            if let Some(n) = r.numer().to_i64() {
                return Value::Integer(n);
            }
        }
        Value::Rational(r)
    }

    /// Create a Value from a float
    pub fn from_float(f: f64) -> Self {
        Value::Float(f)
    }

    /// Create a Value from a boolean
    #[allow(dead_code)]
    pub fn from_bool(b: bool) -> Self {
        Value::Boolean(b)
    }

    /// Create a rational from two integers
    #[allow(dead_code)]
    pub fn from_ratio_int(num: i64, den: i64) -> CalcResult<Self> {
        if den == 0 {
            return Err(CalcError::DivisionByZero);
        }
        Ok(Value::Rational(Ratio::new(BigInt::from(num), BigInt::from(den))))
    }

    /// Convert to rational number
    #[allow(dead_code)]
    pub fn to_rational(&self) -> Option<Ratio<BigInt>> {
        match self {
            Value::Rational(r) => Some(r.clone()),
            Value::Integer(n) => Some(Ratio::new(BigInt::from(*n), BigInt::one())),
            Value::Float(_) => None,
            Value::Boolean(_) => None,
        }
    }

    /// Convert to float (may lose precision for rationals)
    pub fn to_float(&self) -> f64 {
        match self {
            Value::Rational(r) => r.to_f64().unwrap_or(f64::NAN),
            Value::Float(f) => *f,
            Value::Integer(n) => *n as f64,
            Value::Boolean(b) => if *b { 1.0 } else { 0.0 },
        }
    }

    /// Convert to integer (only if exact)
    pub fn to_integer(&self) -> Option<i64> {
        match self {
            Value::Integer(n) => Some(*n),
            Value::Rational(r) => {
                if r.is_integer() {
                    r.numer().to_i64()
                } else {
                    None
                }
            }
            Value::Float(f) => {
                if f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 {
                    Some(*f as i64)
                } else {
                    None
                }
            }
            Value::Boolean(_) => None,
        }
    }

    /// Convert to BigInt (only if exact integer)
    #[allow(dead_code)]
    pub fn to_bigint(&self) -> Option<BigInt> {
        match self {
            Value::Integer(n) => Some(BigInt::from(*n)),
            Value::Rational(r) => {
                if r.is_integer() {
                    Some(r.numer().clone())
                } else {
                    None
                }
            }
            Value::Float(_) => None,
            Value::Boolean(_) => None,
        }
    }

    /// Convert to boolean
    #[allow(dead_code)]
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Check if this value is exact (not a float approximation)
    #[allow(dead_code)]
    pub fn is_exact(&self) -> bool {
        !matches!(self, Value::Float(_))
    }

    /// Check if this value is zero
    pub fn is_zero(&self) -> bool {
        match self {
            Value::Integer(n) => *n == 0,
            Value::Rational(r) => r.is_zero(),
            Value::Float(f) => *f == 0.0,
            Value::Boolean(_) => false,
        }
    }

    /// Get the numerator (for rational numbers)
    #[allow(dead_code)]
    pub fn numerator(&self) -> Option<BigInt> {
        match self {
            Value::Rational(r) => Some(r.numer().clone()),
            Value::Integer(n) => Some(BigInt::from(*n)),
            _ => None,
        }
    }

    /// Get the denominator (for rational numbers)
    #[allow(dead_code)]
    pub fn denominator(&self) -> Option<BigInt> {
        match self {
            Value::Rational(r) => Some(r.denom().clone()),
            Value::Integer(_) => Some(BigInt::one()),
            _ => None,
        }
    }

    /// Arithmetic addition
    pub fn add(&self, other: &Value) -> CalcResult<Value> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                Ok(Value::from_int(a.checked_add(*b)
                    .ok_or_else(|| CalcError::Overflow("integer addition".to_string()))?))
            }
            (Value::Rational(a), Value::Rational(b)) => {
                Ok(Value::from_ratio(a + b))
            }
            (Value::Integer(a), Value::Rational(b)) => {
                Ok(Value::from_ratio(Ratio::new(BigInt::from(*a), BigInt::one()) + b))
            }
            (Value::Rational(a), Value::Integer(b)) => {
                Ok(Value::from_ratio(a + Ratio::new(BigInt::from(*b), BigInt::one())))
            }
            (Value::Float(a), Value::Float(b)) => {
                Ok(Value::from_float(a + b))
            }
            // Promote to float if either operand is float
            _ => Ok(Value::from_float(self.to_float() + other.to_float())),
        }
    }

    /// Arithmetic subtraction
    pub fn sub(&self, other: &Value) -> CalcResult<Value> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                Ok(Value::from_int(a.checked_sub(*b)
                    .ok_or_else(|| CalcError::Overflow("integer subtraction".to_string()))?))
            }
            (Value::Rational(a), Value::Rational(b)) => {
                Ok(Value::from_ratio(a - b))
            }
            (Value::Integer(a), Value::Rational(b)) => {
                Ok(Value::from_ratio(Ratio::new(BigInt::from(*a), BigInt::one()) - b))
            }
            (Value::Rational(a), Value::Integer(b)) => {
                Ok(Value::from_ratio(a - Ratio::new(BigInt::from(*b), BigInt::one())))
            }
            (Value::Float(a), Value::Float(b)) => {
                Ok(Value::from_float(a - b))
            }
            _ => Ok(Value::from_float(self.to_float() - other.to_float())),
        }
    }

    /// Arithmetic multiplication
    pub fn mul(&self, other: &Value) -> CalcResult<Value> {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                Ok(Value::from_int(a.checked_mul(*b)
                    .ok_or_else(|| CalcError::Overflow("integer multiplication".to_string()))?))
            }
            (Value::Rational(a), Value::Rational(b)) => {
                Ok(Value::from_ratio(a * b))
            }
            (Value::Integer(a), Value::Rational(b)) => {
                Ok(Value::from_ratio(Ratio::new(BigInt::from(*a), BigInt::one()) * b))
            }
            (Value::Rational(a), Value::Integer(b)) => {
                Ok(Value::from_ratio(a * Ratio::new(BigInt::from(*b), BigInt::one())))
            }
            (Value::Float(a), Value::Float(b)) => {
                Ok(Value::from_float(a * b))
            }
            _ => Ok(Value::from_float(self.to_float() * other.to_float())),
        }
    }

    /// Arithmetic division
    pub fn div(&self, other: &Value) -> CalcResult<Value> {
        if other.is_zero() {
            return Err(CalcError::DivisionByZero);
        }
        
        match (self, other) {
            (Value::Rational(a), Value::Rational(b)) => {
                Ok(Value::from_ratio(a / b))
            }
            (Value::Integer(a), Value::Integer(b)) => {
                // Promote to rational for exact division
                let rat_a = Ratio::new(BigInt::from(*a), BigInt::one());
                let rat_b = Ratio::new(BigInt::from(*b), BigInt::one());
                Ok(Value::from_ratio(rat_a / rat_b))
            }
            (Value::Integer(a), Value::Rational(b)) => {
                Ok(Value::from_ratio(Ratio::new(BigInt::from(*a), BigInt::one()) / b))
            }
            (Value::Rational(a), Value::Integer(b)) => {
                Ok(Value::from_ratio(a / Ratio::new(BigInt::from(*b), BigInt::one())))
            }
            (Value::Float(a), Value::Float(b)) => {
                Ok(Value::from_float(a / b))
            }
            _ => Ok(Value::from_float(self.to_float() / other.to_float())),
        }
    }

    /// Modulo operation
    pub fn modulo(&self, other: &Value) -> CalcResult<Value> {
        if other.is_zero() {
            return Err(CalcError::DivisionByZero);
        }
        
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => {
                Ok(Value::from_int(a % b))
            }
            (Value::Rational(a), Value::Rational(b)) => {
                // For rationals: a % b = a - b * floor(a/b)
                let div = a / b;
                let floor_div = div.floor();
                let result = a - b * &floor_div;
                Ok(Value::from_ratio(result))
            }
            _ => {
                let a = self.to_float();
                let b = other.to_float();
                Ok(Value::from_float(a % b))
            }
        }
    }

    /// Exponentiation
    pub fn pow(&self, exp: &Value) -> CalcResult<Value> {
        match (self, exp) {
            (Value::Integer(base), Value::Integer(exp)) => {
                if *exp >= 0 && *exp <= 30 {
                    // Small integer exponent - compute exactly
                    let result = BigInt::from(*base).pow(*exp as u32);
                    Ok(Value::from_bigint(result))
                } else {
                    // Large exponent - use float
                    Ok(Value::from_float((*base as f64).powf(*exp as f64)))
                }
            }
            (Value::Rational(base), Value::Integer(exp)) => {
                if *exp >= 0 {
                    Ok(Value::from_ratio(base.pow(*exp as i32)))
                } else {
                    // Negative exponent: invert and raise
                    let inv = base.recip();
                    Ok(Value::from_ratio(inv.pow((-exp) as i32)))
                }
            }
            _ => {
                Ok(Value::from_float(self.to_float().powf(exp.to_float())))
            }
        }
    }

    /// Negation
    pub fn neg(&self) -> CalcResult<Value> {
        match self {
            Value::Integer(n) => Ok(Value::from_int(
                n.checked_neg()
                    .ok_or_else(|| CalcError::Overflow("integer negation".to_string()))?
            )),
            Value::Rational(r) => Ok(Value::from_ratio(-r)),
            Value::Float(f) => Ok(Value::from_float(-f)),
            Value::Boolean(_) => Err(CalcError::TypeError {
                expected: "numeric type".to_string(),
                got: "Boolean".to_string(),
            }),
        }
    }

    /// Absolute value
    #[allow(dead_code)]
    pub fn abs(&self) -> Value {
        match self {
            Value::Integer(n) => Value::from_int(n.abs()),
            Value::Rational(r) => Value::from_ratio(r.abs()),
            Value::Float(f) => Value::from_float(f.abs()),
            Value::Boolean(_) => self.clone(),
        }
    }

    /// Floor function
    #[allow(dead_code)]
    pub fn floor(&self) -> Value {
        match self {
            Value::Integer(_) => self.clone(),
            Value::Rational(r) => {
                // floor(r) = numer / denom (integer division)
                let floor_val = r.numer() / r.denom();
                Value::from_bigint(floor_val)
            }
            Value::Float(f) => Value::from_float(f.floor()),
            Value::Boolean(_) => self.clone(),
        }
    }

    /// Ceiling function
    #[allow(dead_code)]
    pub fn ceil(&self) -> Value {
        match self {
            Value::Integer(_) => self.clone(),
            Value::Rational(r) => {
                // ceil(r) = (numer + denom - 1) / denom for positive
                // For negative: ceil(r) = numer / denom
                let numer = r.numer();
                let denom = r.denom();
                let ceil_val = if numer >= &BigInt::zero() {
                    (numer + denom - BigInt::one()) / denom
                } else {
                    numer / denom
                };
                Value::from_bigint(ceil_val)
            }
            Value::Float(f) => Value::from_float(f.ceil()),
            Value::Boolean(_) => self.clone(),
        }
    }

    /// Round to nearest integer
    #[allow(dead_code)]
    pub fn round(&self) -> Value {
        match self {
            Value::Integer(_) => self.clone(),
            Value::Rational(r) => {
                // round(r) = (numer * 2 + denom) / (2 * denom) for positive
                // Simplified: add denom/2 to numer then divide
                let numer = r.numer();
                let denom = r.denom();
                let half_denom = denom / BigInt::from(2u32);
                let round_val = if numer >= &BigInt::zero() {
                    (numer + half_denom) / denom
                } else {
                    (numer - half_denom) / denom
                };
                Value::from_bigint(round_val)
            }
            Value::Float(f) => Value::from_float(f.round()),
            Value::Boolean(_) => self.clone(),
        }
    }

    /// Comparison: less than
    pub fn lt(&self, other: &Value) -> bool {
        self.to_float() < other.to_float()
    }

    /// Comparison: greater than
    pub fn gt(&self, other: &Value) -> bool {
        self.to_float() > other.to_float()
    }

    /// Comparison: equal (with tolerance for floats)
    pub fn is_equal(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Boolean(a), Value::Boolean(b)) => *a == *b,
            _ => {
                let a = self.to_float();
                let b = other.to_float();
                (a - b).abs() < 1e-10
            }
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Rational(r) => {
                if r.denom().is_one() {
                    write!(f, "{}", r.numer())
                } else {
                    write!(f, "{}", r)
                }
            }
            Value::Integer(n) => write!(f, "{}", n),
            Value::Float(x) => {
                if x.fract() == 0.0 && x.abs() < 1e15 {
                    write!(f, "{}", *x as i64)
                } else {
                    write!(f, "{}", x)
                }
            }
            Value::Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
        }
    }
}
