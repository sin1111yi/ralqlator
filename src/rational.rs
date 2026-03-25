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

//! Rational number utility functions
//!
//! This module provides helper functions for working with rational numbers,
//! including continued fractions, decimal expansion, and conversion utilities.

use num_rational::Ratio;
use num_bigint::{BigInt, BigUint};
use num_traits::{Zero, One, Signed, ToPrimitive};

use crate::error::{CalcError, CalcResult};
use crate::value::Value;

/// Parse a decimal string into a rational number
/// e.g., "3.14159" -> 314159/100000
pub fn parse_decimal(s: &str) -> CalcResult<Ratio<BigInt>> {
    if !s.contains('.') {
        return s.parse::<BigInt>()
            .map(|n| Ratio::new(n, BigInt::one()))
            .map_err(|_| CalcError::ParseError(format!("Invalid integer: {}", s)));
    }

    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() != 2 {
        return Err(CalcError::ParseError(format!("Invalid decimal: {}", s)));
    }

    let integer_part = parts[0];
    let fractional_part = parts[1];

    // Remove trailing zeros from fractional part
    let fractional_part = fractional_part.trim_end_matches('0');
    
    if fractional_part.is_empty() {
        // No fractional part
        return integer_part.parse::<BigInt>()
            .map(|n| Ratio::new(n, BigInt::one()))
            .map_err(|_| CalcError::ParseError(format!("Invalid decimal: {}", s)));
    }

    let denominator = BigUint::from(10u32).pow(fractional_part.len() as u32);
    let numerator_str = format!("{}{}", integer_part.trim_start_matches('-'), fractional_part);
    
    let numerator: BigInt = numerator_str.parse()
        .map_err(|_| CalcError::ParseError(format!("Invalid decimal: {}", s)))?;
    
    let mut ratio = Ratio::new(numerator, BigInt::from_biguint(num_bigint::Sign::Plus, denominator));
    
    if integer_part.starts_with('-') {
        ratio = -ratio;
    }

    Ok(ratio)
}

/// Parse a repeating decimal string into a rational number
/// e.g., "0.(3)" -> 1/3, "0.1(6)" -> 1/6
#[allow(dead_code)]
pub fn parse_repeating_decimal(s: &str) -> CalcResult<Ratio<BigInt>> {
    if !s.contains('(') || !s.contains(')') {
        return parse_decimal(s);
    }

    let open_paren = s.find('(').ok_or_else(|| {
        CalcError::ParseError("Missing '(' in repeating decimal".to_string())
    })?;
    let close_paren = s.find(')').ok_or_else(|| {
        CalcError::ParseError("Missing ')' in repeating decimal".to_string())
    })?;

    if open_paren >= close_paren {
        return Err(CalcError::ParseError("Invalid repeating decimal format".to_string()));
    }

    let non_repeating = &s[..open_paren];
    let repeating = &s[open_paren + 1..close_paren];

    // Algorithm: 
    // If x = 0.abcd(efg), then
    // x = abcd/10000 + efg/(10000 * 999)
    
    let non_rep_decimal = if non_repeating.is_empty() || non_repeating == "." {
        Ratio::zero()
    } else {
        parse_decimal(non_repeating)?
    };

    if repeating.is_empty() {
        return Ok(non_rep_decimal);
    }

    // Parse repeating part as integer
    let rep_int: BigInt = repeating.parse()
        .map_err(|_| CalcError::ParseError(format!("Invalid repeating part: {}", repeating)))?;
    
    // Denominator for repeating part: 10^n * (10^m - 1)
    // where n = length of non-repeating fractional part
    // and m = length of repeating part
    let non_rep_frac_len = if let Some(dot_pos) = non_repeating.find('.') {
        non_repeating.len() - dot_pos - 1
    } else {
        0
    };

    let ten = BigUint::from(10u32);
    let denom_non_rep = &ten.pow(non_rep_frac_len as u32);
    let denom_rep = &ten.pow(repeating.len() as u32) - BigUint::from(1u32);
    let denominator = BigInt::from_biguint(num_bigint::Sign::Plus, denom_non_rep * denom_rep);

    let repeating_ratio = Ratio::new(rep_int, denominator);
    
    Ok(non_rep_decimal + repeating_ratio)
}

/// Convert a float to a rational number using continued fraction approximation
/// Returns a rational that approximates the float within the given tolerance
pub fn float_to_rational(f: f64, tolerance: f64) -> Ratio<BigInt> {
    if f == 0.0 {
        return Ratio::zero();
    }

    let negative = f < 0.0;
    let mut x = f.abs();

    // Continued fraction expansion
    let mut h_prev: BigInt = BigInt::zero();
    let mut h_curr: BigInt = BigInt::one();
    let mut k_prev: BigInt = BigInt::one();
    let mut k_curr: BigInt = BigInt::zero();

    loop {
        let a = x.floor() as i64;
        let f_part = x - a as f64;

        let h_next = &h_curr * BigInt::from(a) + &h_prev;
        let k_next = &k_curr * BigInt::from(a) + &k_prev;

        // Check convergence before updating
        if k_next > BigInt::from(1_000_000_000_000i64) {
            // Prevent overflow
            break;
        }

        // Update for next iteration
        h_prev = h_curr;
        h_curr = h_next;
        k_prev = k_curr;
        k_curr = k_next;

        // Check if k_curr is zero (shouldn't happen, but be safe)
        if k_curr.is_zero() {
            break;
        }

        let approx = Ratio::new(h_curr.clone(), k_curr.clone());
        if (approx.to_f64().unwrap_or(0.0) - x).abs() < tolerance {
            break;
        }

        if f_part < tolerance {
            break;
        }

        x = 1.0 / f_part;
    }

    // Handle edge case where k_curr is still zero
    if k_curr.is_zero() {
        k_curr = BigInt::one();
    }

    let result = Ratio::new(h_curr, k_curr);
    if negative {
        -result
    } else {
        result
    }
}

/// Compute the continued fraction expansion of a rational number
/// Returns the sequence [a0; a1, a2, ..., an]
pub fn continued_fraction(r: &Ratio<BigInt>) -> Vec<BigInt> {
    let mut result = Vec::new();
    let mut current = r.clone();

    loop {
        // floor = numer / denom (integer division)
        let floor = current.numer() / current.denom();
        result.push(floor.clone());

        let floor_ratio: Ratio<BigInt> = Ratio::new(floor, BigInt::one());
        let frac = &current - &floor_ratio;
        
        if frac.is_zero() {
            break;
        }

        current = frac.recip();
        
        // Prevent infinite loops for irrational approximations
        if result.len() > 100 {
            break;
        }
    }

    result
}

/// Format continued fraction as string
/// e.g., [3; 7, 15, 1]
pub fn format_continued_fraction(cf: &[BigInt]) -> String {
    if cf.is_empty() {
        return "[]".to_string();
    }
    
    let mut result = format!("[{}", cf[0]);
    if cf.len() > 1 {
        result.push(';');
        for (i, term) in cf.iter().skip(1).enumerate() {
            if i > 0 {
                result.push(',');
            }
            result.push(' ');
            result.push_str(&term.to_string());
        }
    }
    result.push(']');
    result
}

/// Get the fractional part of a rational number
/// e.g., 7/3 -> 1/3
#[allow(dead_code)]
pub fn fractional_part(r: &Ratio<BigInt>) -> Ratio<BigInt> {
    r - r.floor()
}

/// Simplify a ratio (already done by Ratio, but explicit for clarity)
#[allow(dead_code)]
pub fn simplify(r: &Ratio<BigInt>) -> Ratio<BigInt> {
    r.clone()
}

/// Check if a rational number has a terminating decimal expansion
/// A rational has a terminating decimal iff its denominator (in lowest terms)
/// has only prime factors 2 and 5
#[allow(dead_code)]
pub fn has_terminating_decimal(r: &Ratio<BigInt>) -> bool {
    let denom = r.denom();
    let mut d = denom.clone();

    // Remove all factors of 2
    while &d % 2u32 == BigInt::zero() {
        d /= 2u32;
    }

    // Remove all factors of 5
    while &d % 5u32 == BigInt::zero() {
        d /= 5u32;
    }

    d.is_one()
}

/// Get the decimal expansion of a rational number
/// Returns (integer_part, fractional_digits, repeating_start)
/// If repeating_start is None, the decimal terminates
#[allow(dead_code)]
pub fn decimal_expansion(r: &Ratio<BigInt>, max_digits: usize) -> (String, String, Option<usize>) {
    let negative = r < &Ratio::zero();
    let abs_r = r.abs();
    
    // integer_part = numer / denom (integer division)
    let integer_part = abs_r.numer() / abs_r.denom();
    let floor_ratio: Ratio<BigInt> = Ratio::new(integer_part.clone(), BigInt::one());
    let frac = &abs_r - &floor_ratio;

    let mut fractional = String::new();
    let mut remainder = frac.numer().clone() * BigInt::from(10);
    let denom = frac.denom().clone();
    
    let mut seen_remainders: std::collections::HashMap<BigInt, usize> = std::collections::HashMap::new();
    let mut repeating_start = None;

    for i in 0..max_digits {
        if remainder.is_zero() {
            break;
        }

        if let Some(&start) = seen_remainders.get(&remainder) {
            repeating_start = Some(start);
            break;
        }

        seen_remainders.insert(remainder.clone(), i);

        let digit = &remainder / &denom;
        fractional.push_str(&digit.to_string());
        remainder = (&remainder % &denom) * BigInt::from(10);
    }

    let int_str = if negative {
        format!("-{}", integer_part)
    } else {
        integer_part.to_string()
    };

    (int_str, fractional, repeating_start)
}

/// Convert a Value to rational if possible
#[allow(dead_code)]
pub fn value_to_rational(v: &Value) -> Option<Ratio<BigInt>> {
    v.to_rational()
}

/// Create a rational from numerator and denominator
#[allow(dead_code)]
pub fn make_rational(num: BigInt, den: BigInt) -> CalcResult<Ratio<BigInt>> {
    if den.is_zero() {
        return Err(CalcError::DivisionByZero);
    }
    Ok(Ratio::new(num, den))
}

/// Create a rational from i64 values
#[allow(dead_code)]
pub fn make_rational_int(num: i64, den: i64) -> CalcResult<Ratio<BigInt>> {
    make_rational(BigInt::from(num), BigInt::from(den))
}

/// Calculate GCD of two BigInt values
pub fn gcd_bigint(a: i64, b: i64) -> i64 {
    use num_bigint::BigInt;
    use num_traits::Zero;
    
    let mut a = BigInt::from(a.abs());
    let mut b = BigInt::from(b.abs());
    
    while !b.is_zero() {
        let temp = b.clone();
        b = &a % &b;
        a = temp;
    }
    
    a.to_i64().unwrap_or(i64::MAX)
}

/// Calculate LCM of two BigInt values
pub fn lcm_bigint(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a.abs() / gcd_bigint(a, b)) * b.abs()
}
