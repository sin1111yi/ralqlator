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

//! Mathematical function implementations
//!
//! This module provides implementations for all built-in mathematical functions,
//! leveraging Rust's standard library and the statrs crate for special functions.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// Import statrs for special mathematical functions

/// User-defined function storage
/// Maps function name -> (parameter names, expression)
pub type UserFunctions = Arc<Mutex<HashMap<String, (Vec<String>, String)>>>;

/// Calculate factorial: n! = n * (n-1) * ... * 1
/// Uses statrs::function::factorial for better performance and accuracy
pub fn eval_factorial(n: f64) -> Result<f64, String> {
    if n < 0.0 {
        return Err("factorial: argument must be non-negative".to_string());
    }
    if n.fract() != 0.0 {
        return Err("factorial: argument must be an integer".to_string());
    }
    if n > 170.0 {
        return Err("factorial: argument too large (max 170)".to_string());
    }
    let n = n as u64;
    Ok(statrs::function::factorial::factorial(n))
}

/// Calculate gamma function: Γ(n) = (n-1)! for positive integers
/// Extension of factorial to real and complex numbers
pub fn eval_gamma(n: f64) -> Result<f64, String> {
    if n <= 0.0 && n.fract() == 0.0 {
        return Err("gamma: undefined for non-positive integers".to_string());
    }
    Ok(statrs::function::gamma::gamma(n))
}

/// Calculate sum of multiple arguments: sum(a, b, c, ...) = a + b + c + ...
pub fn eval_sum(args: &[f64]) -> Result<f64, String> {
    if args.is_empty() {
        return Err("sum: requires at least 1 argument".to_string());
    }
    Ok(args.iter().sum())
}

/// Calculate product of multiple arguments: prod(a, b, c, ...) = a * b * c * ...
pub fn eval_product(args: &[f64]) -> Result<f64, String> {
    if args.is_empty() {
        return Err("prod: requires at least 1 argument".to_string());
    }
    Ok(args.iter().product())
}

/// Calculate logarithm with custom base: log_base(base, x) = ln(x) / ln(base)
pub fn eval_log_base(base: f64, x: f64) -> Result<f64, String> {
    if base <= 0.0 || base == 1.0 {
        return Err("log: base must be positive and not equal to 1".to_string());
    }
    if x <= 0.0 {
        return Err("log: argument must be positive".to_string());
    }
    Ok(x.ln() / base.ln())
}

/// Evaluate lg (base 10 logarithm)
pub fn eval_lg(x: f64) -> Result<f64, String> {
    if x <= 0.0 {
        return Err("lg: argument must be positive".to_string());
    }
    Ok(x.log10())
}

/// Evaluate ln (natural logarithm)
pub fn eval_ln(x: f64) -> Result<f64, String> {
    if x <= 0.0 {
        return Err("ln: argument must be positive".to_string());
    }
    Ok(x.ln())
}

/// Evaluate log2 (base 2 logarithm)
pub fn eval_log2(x: f64) -> Result<f64, String> {
    if x <= 0.0 {
        return Err("log2: argument must be positive".to_string());
    }
    Ok(x.log2())
}

/// Evaluate sqrt (square root)
pub fn eval_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        return Err("sqrt: argument must be non-negative".to_string());
    }
    Ok(x.sqrt())
}

/// Evaluate cbrt (cube root)
pub fn eval_cbrt(x: f64) -> f64 {
    x.cbrt()
}

/// Evaluate pow (power function)
pub fn eval_pow(base: f64, exp: f64) -> Result<f64, String> {
    Ok(base.powf(exp))
}

/// Evaluate sin (sine function, input in radians)
pub fn eval_sin(x: f64) -> f64 {
    x.sin()
}

/// Evaluate cos (cosine function, input in radians)
pub fn eval_cos(x: f64) -> f64 {
    x.cos()
}

/// Evaluate tan (tangent function, input in radians)
pub fn eval_tan(x: f64) -> Result<f64, String> {
    // Check for undefined values (cos(x) == 0)
    if x.cos() == 0.0 {
        return Err("tan: undefined at this value".to_string());
    }
    Ok(x.tan())
}

/// Evaluate sec (secant function, input in radians)
pub fn eval_sec(x: f64) -> Result<f64, String> {
    let cos_val = x.cos();
    if cos_val == 0.0 {
        return Err("sec: undefined at this value".to_string());
    }
    Ok(1.0 / cos_val)
}

/// Evaluate csc (cosecant function, input in radians)
pub fn eval_csc(x: f64) -> Result<f64, String> {
    let sin_val = x.sin();
    if sin_val == 0.0 {
        return Err("csc: undefined at this value".to_string());
    }
    Ok(1.0 / sin_val)
}

/// Evaluate cot (cotangent function, input in radians)
pub fn eval_cot(x: f64) -> Result<f64, String> {
    let sin_val = x.sin();
    if sin_val == 0.0 {
        return Err("cot: undefined at this value".to_string());
    }
    Ok(x.cos() / sin_val)
}

/// Evaluate asin (inverse sine function, output in radians)
pub fn eval_asin(x: f64) -> Result<f64, String> {
    if !(-1.0..=1.0).contains(&x) {
        return Err("asin: argument must be in range [-1, 1]".to_string());
    }
    Ok(x.asin())
}

/// Evaluate acos (inverse cosine function, output in radians)
pub fn eval_acos(x: f64) -> Result<f64, String> {
    if !(-1.0..=1.0).contains(&x) {
        return Err("acos: argument must be in range [-1, 1]".to_string());
    }
    Ok(x.acos())
}

/// Evaluate atan (inverse tangent function, output in radians)
pub fn eval_atan(x: f64) -> f64 {
    x.atan()
}

/// Evaluate atan2 (two-argument inverse tangent, output in radians)
pub fn eval_atan2(y: f64, x: f64) -> f64 {
    y.atan2(x)
}

/// Evaluate sinh (hyperbolic sine)
pub fn eval_sinh(x: f64) -> f64 {
    x.sinh()
}

/// Evaluate cosh (hyperbolic cosine)
pub fn eval_cosh(x: f64) -> f64 {
    x.cosh()
}

/// Evaluate tanh (hyperbolic tangent)
pub fn eval_tanh(x: f64) -> f64 {
    x.tanh()
}

/// Evaluate asinh (inverse hyperbolic sine)
pub fn eval_asinh(x: f64) -> f64 {
    x.asinh()
}

/// Evaluate acosh (inverse hyperbolic cosine)
pub fn eval_acosh(x: f64) -> Result<f64, String> {
    if x < 1.0 {
        return Err("acosh: argument must be >= 1".to_string());
    }
    Ok(x.acosh())
}

/// Evaluate atanh (inverse hyperbolic tangent)
pub fn eval_atanh(x: f64) -> Result<f64, String> {
    if x <= -1.0 || x >= 1.0 {
        return Err("atanh: argument must be in range (-1, 1)".to_string());
    }
    Ok(x.atanh())
}

/// Evaluate mod (modulo function)
pub fn eval_mod(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("mod: modulo by zero".to_string());
    }
    Ok(a % b)
}

/// Evaluate abs (absolute value)
pub fn eval_abs(x: f64) -> f64 {
    x.abs()
}

/// Evaluate floor (largest integer <= x)
pub fn eval_floor(x: f64) -> f64 {
    x.floor()
}

/// Evaluate ceil (smallest integer >= x)
pub fn eval_ceil(x: f64) -> f64 {
    x.ceil()
}

/// Evaluate round (nearest integer)
pub fn eval_round(x: f64) -> f64 {
    x.round()
}

/// Evaluate erf (error function)
pub fn eval_erf(x: f64) -> f64 {
    statrs::function::erf::erf(x)
}

/// Evaluate erfc (complementary error function)
pub fn eval_erfc(x: f64) -> f64 {
    statrs::function::erf::erfc(x)
}

/// Evaluate beta function: B(x, y) = Γ(x) * Γ(y) / Γ(x + y)
pub fn eval_beta(x: f64, y: f64) -> Result<f64, String> {
    if x <= 0.0 || y <= 0.0 {
        return Err("beta: arguments must be positive".to_string());
    }
    Ok(statrs::function::beta::beta(x, y))
}

// Elliptic integrals temporarily disabled due to statrs API changes
// /// Evaluate complete elliptic integral of the first kind: K(k)
// pub fn eval_elliptic_k(k: f64) -> Result<f64, String> {
//     if k < -1.0 || k > 1.0 {
//         return Err("elliptic_k: modulus must be in range [-1, 1]".to_string());
//     }
//     Ok(statrs::elliptic::K(k * k))
// }

// /// Evaluate complete elliptic integral of the second kind: E(k)
// pub fn eval_elliptic_e(k: f64) -> Result<f64, String> {
//     if k < -1.0 || k > 1.0 {
//         return Err("elliptic_e: modulus must be in range [-1, 1]".to_string());
//     }
//     Ok(statrs::elliptic::E(k * k))
// }

// ==================== BigInt Functions ====================

use num_bigint::{BigInt, BigUint};
use num_traits::{One, Zero};

/// Calculate factorial using BigInt for arbitrary precision
/// Can calculate factorials much larger than 170!
pub fn eval_factorial_bigint(n: u64) -> String {
    let mut result = BigUint::one();
    for i in 2..=n {
        result *= i;
    }
    result.to_string()
}

/// Calculate power using BigInt for arbitrary precision
/// Returns base^exp as a string
pub fn eval_pow_bigint(base: i64, exp: u32) -> String {
    let big_base = BigInt::from(base);
    big_base.pow(exp).to_string()
}

/// Calculate combinations: C(n, k) = n! / (k! * (n-k)!)
pub fn eval_combinations(n: u64, k: u64) -> Result<String, String> {
    if k > n {
        return Err("combinations: k must be <= n".to_string());
    }
    
    // Use the multiplicative formula for better efficiency
    // C(n, k) = n * (n-1) * ... * (n-k+1) / (k * (k-1) * ... * 1)
    let mut result = BigUint::one();
    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }
    Ok(result.to_string())
}

/// Calculate permutations: P(n, k) = n! / (n-k)!
pub fn eval_permutations(n: u64, k: u64) -> Result<String, String> {
    if k > n {
        return Err("permutations: k must be <= n".to_string());
    }
    
    // P(n, k) = n * (n-1) * ... * (n-k+1)
    let mut result = BigUint::one();
    for i in 0..k {
        result *= n - i;
    }
    Ok(result.to_string())
}

/// Calculate greatest common divisor using BigInt
pub fn eval_gcd(a: i64, b: i64) -> String {
    let big_a = BigInt::from(a.abs());
    let big_b = BigInt::from(b.abs());
    gcd(&big_a, &big_b).to_string()
}

/// Calculate least common multiple using BigInt
pub fn eval_lcm(a: i64, b: i64) -> String {
    let big_a = BigInt::from(a.abs());
    let big_b = BigInt::from(b.abs());
    lcm(&big_a, &big_b).to_string()
}

/// GCD helper function for BigInt
fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let mut a = a.clone();
    let mut b = b.clone();
    while !b.is_zero() {
        let temp = b.clone();
        b = &a % &b;
        a = temp;
    }
    a
}

/// LCM helper function for BigInt
fn lcm(a: &BigInt, b: &BigInt) -> BigInt {
    if a.is_zero() || b.is_zero() {
        return BigInt::zero();
    }
    (a * b) / gcd(a, b)
}

/// Check if a number is prime (simple trial division for demonstration)
pub fn eval_is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as u64;
    for i in (3..=sqrt_n).step_by(2) {
        if n.is_multiple_of(i) {
            return false;
        }
    }
    true
}

/// Get the next prime number greater than n
pub fn eval_next_prime(n: u64) -> String {
    let mut candidate = n + 1;
    while !eval_is_prime(candidate) {
        candidate += 1;
    }
    candidate.to_string()
}
