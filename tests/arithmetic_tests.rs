// Arithmetic Operations Tests
// Tests for basic arithmetic operations: +, -, *, /, %, ^, !

use std::process::Command;

fn run_cli(args: &[&str]) -> (String, String, bool) {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(args)
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.success())
}

// ==================== Addition Tests ====================

#[test]
fn test_addition_positive() {
    let (stdout, _, success) = run_cli(&["1 + 2"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_addition_large_numbers() {
    let (stdout, _, success) = run_cli(&["100 + 200"]);
    assert!(success);
    assert!(stdout.contains("300"));
}

#[test]
fn test_addition_negative_result() {
    let (stdout, _, success) = run_cli(&["--", "-5 + 3"]);
    assert!(success);
    assert!(stdout.contains("-2"));
}

#[test]
fn test_addition_multiple() {
    let (stdout, _, success) = run_cli(&["1 + 2 + 3 + 4 + 5"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

// ==================== Subtraction Tests ====================

#[test]
fn test_subtraction_positive() {
    let (stdout, _, success) = run_cli(&["10 - 3"]);
    assert!(success);
    assert!(stdout.contains("7"));
}

#[test]
fn test_subtraction_negative_result() {
    let (stdout, _, success) = run_cli(&["5 - 10"]);
    assert!(success);
    assert!(stdout.contains("-5"));
}

#[test]
fn test_subtraction_zero() {
    let (stdout, _, success) = run_cli(&["5 - 5"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_subtraction_multiple() {
    let (stdout, _, success) = run_cli(&["10 - 5 - 3"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

// ==================== Multiplication Tests ====================

#[test]
fn test_multiplication_positive() {
    let (stdout, _, success) = run_cli(&["3 * 4"]);
    assert!(success);
    assert!(stdout.contains("12"));
}

#[test]
fn test_multiplication_negative() {
    let (stdout, _, success) = run_cli(&["--", "-2 * 3"]);
    assert!(success);
    assert!(stdout.contains("-6"));
}

#[test]
fn test_multiplication_zero() {
    let (stdout, _, success) = run_cli(&["42 * 0"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_multiplication_multiple() {
    let (stdout, _, success) = run_cli(&["2 * 3 * 4"]);
    assert!(success);
    assert!(stdout.contains("24"));
}

// ==================== Division Tests ====================

#[test]
fn test_division_exact() {
    let (stdout, _, success) = run_cli(&["10 / 2"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_division_decimal() {
    let (stdout, _, success) = run_cli(&["7 / 2"]);
    assert!(success);
    assert!(stdout.contains("3.5"));
}

#[test]
fn test_division_zero() {
    let (stdout, _, success) = run_cli(&["1 / 3"]);
    assert!(success);
    assert!(stdout.contains("0.333"));
}

#[test]
fn test_division_by_zero_error() {
    let (_, stderr, success) = run_cli(&["10 / 0"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
    assert!(stderr.contains("zero"));
}

#[test]
fn test_division_chain() {
    let (stdout, _, success) = run_cli(&["100 / 5 / 2"]);
    assert!(success);
    assert!(stdout.contains("10"));
}

// ==================== Modulo Tests ====================

#[test]
fn test_modulo_positive() {
    let (stdout, _, success) = run_cli(&["10 % 3"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_modulo_multiple() {
    let (stdout, _, success) = run_cli(&["17 % 5"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_modulo_zero_dividend() {
    let (stdout, _, success) = run_cli(&["0 % 5"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_modulo_same_values() {
    let (stdout, _, success) = run_cli(&["5 % 5"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_modulo_larger_divisor() {
    let (stdout, _, success) = run_cli(&["3 % 5"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_modulo_by_zero_error() {
    let (_, stderr, success) = run_cli(&["10 % 0"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Exponentiation Tests ====================

#[test]
fn test_exponentiation_positive() {
    let (stdout, _, success) = run_cli(&["2 ^ 3"]);
    assert!(success);
    assert!(stdout.contains("8"));
}

#[test]
fn test_exponentiation_large() {
    let (stdout, _, success) = run_cli(&["2 ^ 10"]);
    assert!(success);
    assert!(stdout.contains("1024"));
}

#[test]
fn test_exponentiation_zero_power() {
    let (stdout, _, success) = run_cli(&["5 ^ 0"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_exponentiation_zero_base() {
    let (stdout, _, success) = run_cli(&["0 ^ 5"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_exponentiation_negative_base_even() {
    let (stdout, _, success) = run_cli(&["(-2) ^ 2"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

#[test]
fn test_exponentiation_negative_base_odd() {
    let (stdout, _, success) = run_cli(&["(-2) ^ 3"]);
    assert!(success);
    assert!(stdout.contains("-8"));
}

#[test]
fn test_exponentiation_negative_exponent() {
    let (stdout, _, success) = run_cli(&["pow(2, -1)"]);
    assert!(success);
    assert!(stdout.contains("0.5"));
}

#[test]
fn test_exponentiation_fractional_exponent() {
    let (stdout, _, success) = run_cli(&["pow(4, 0.5)"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

// ==================== Factorial Tests ====================

#[test]
fn test_factorial_zero() {
    let (stdout, _, success) = run_cli(&["factorial(0)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_factorial_one() {
    let (stdout, _, success) = run_cli(&["factorial(1)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_factorial_small() {
    let (stdout, _, success) = run_cli(&["factorial(5)"]);
    assert!(success);
    assert!(stdout.contains("120"));
}

#[test]
fn test_factorial_large() {
    let (stdout, _, success) = run_cli(&["factorial(10)"]);
    assert!(success);
    assert!(stdout.contains("3628800"));
}

#[test]
fn test_factorial_negative_error() {
    let (_, stderr, success) = run_cli(&["factorial(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Operator Precedence Tests ====================

#[test]
fn test_precedence_mul_add() {
    let (stdout, _, success) = run_cli(&["1 + 2 * 3"]);
    assert!(success);
    assert!(stdout.contains("7"));
}

#[test]
fn test_precedence_parentheses() {
    let (stdout, _, success) = run_cli(&["(1 + 2) * 3"]);
    assert!(success);
    assert!(stdout.contains("9"));
}

#[test]
fn test_precedence_complex() {
    let (stdout, _, success) = run_cli(&["2 + 3 * 4 - 5"]);
    assert!(success);
    assert!(stdout.contains("9"));
}

#[test]
fn test_precedence_mixed() {
    let (stdout, _, success) = run_cli(&["2 + 3 * 4 - 8 / 2"]);
    assert!(success);
    assert!(stdout.contains("10"));
}

#[test]
fn test_precedence_nested_parentheses() {
    let (stdout, _, success) = run_cli(&["((10 + 5) * 2 - 6) / 4"]);
    assert!(success);
    assert!(stdout.contains("6"));
}

#[test]
fn test_precedence_deeply_nested() {
    let (stdout, _, success) = run_cli(&["(((1 + 2)))"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

// ==================== Complex Expression Tests ====================

#[test]
fn test_complex_expression_1() {
    let (stdout, _, success) = run_cli(&["sqrt(pow(3, 2) + pow(4, 2))"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_complex_expression_2() {
    let (stdout, _, success) = run_cli(&["1 + 2 * 3 ^ 2"]);
    assert!(success);
    assert!(stdout.contains("19"));
}

#[test]
fn test_alternating_signs() {
    let (stdout, _, success) = run_cli(&["1 - 2 + 3 - 4 + 5"]);
    assert!(success);
    assert!(stdout.contains("3"));
}
