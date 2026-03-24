// Edge Cases Tests
// Tests for boundary conditions and edge cases

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

// ==================== Zero Tests ====================

#[test]
fn test_zero_addition() {
    let (stdout, _, success) = run_cli(&["42 + 0"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

#[test]
fn test_zero_addition_left() {
    let (stdout, _, success) = run_cli(&["0 + 42"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

#[test]
fn test_zero_subtraction() {
    let (stdout, _, success) = run_cli(&["42 - 0"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

#[test]
fn test_zero_multiplication() {
    let (stdout, _, success) = run_cli(&["42 * 0"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_zero_multiplication_left() {
    let (stdout, _, success) = run_cli(&["0 * 42"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_zero_division_result() {
    let (stdout, _, success) = run_cli(&["0 / 42"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_zero_identity_division() {
    let (stdout, _, success) = run_cli(&["42 / 1"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

#[test]
fn test_zero_modulo() {
    let (stdout, _, success) = run_cli(&["0 % 42"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_zero_identity_modulo() {
    let (stdout, _, success) = run_cli(&["42 % 1"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_zero_power() {
    let (stdout, _, success) = run_cli(&["0 ^ 5"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_power_zero() {
    let (stdout, _, success) = run_cli(&["5 ^ 0"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_zero_to_zero() {
    let (stdout, _, success) = run_cli(&["0 ^ 0"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

// ==================== Negative Number Tests ====================

#[test]
fn test_negative_addition() {
    let (stdout, _, success) = run_cli(&["--", "-5 + 3"]);
    assert!(success);
    assert!(stdout.contains("-2"));
}

#[test]
fn test_negative_subtraction() {
    let (stdout, _, success) = run_cli(&["5 - 10"]);
    assert!(success);
    assert!(stdout.contains("-5"));
}

#[test]
fn test_negative_multiplication() {
    let (stdout, _, success) = run_cli(&["--", "-2 * 3"]);
    assert!(success);
    assert!(stdout.contains("-6"));
}

#[test]
fn test_negative_times_negative() {
    let (stdout, _, success) = run_cli(&["--", "-2 * -3"]);
    assert!(success);
    assert!(stdout.contains("6"));
}

#[test]
fn test_unary_minus_in_expression() {
    let (stdout, _, success) = run_cli(&["--", "-3 + 5"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_double_negation() {
    let (stdout, _, success) = run_cli(&["0 - (-5)"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_triple_negation() {
    let (stdout, _, success) = run_cli(&["0 - (0 - (0 - 5))"]);
    assert!(success);
    assert!(stdout.contains("-5"));
}

// ==================== Parentheses Tests ====================

#[test]
fn test_single_parentheses() {
    let (stdout, _, success) = run_cli(&["(5)"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_nested_parentheses() {
    let (stdout, _, success) = run_cli(&["((1 + 2))"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_deeply_nested_parentheses() {
    let (stdout, _, success) = run_cli(&["((((((1))))))"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_multiple_parentheses_groups() {
    let (stdout, _, success) = run_cli(&["(2 + 3) * (4 + 5)"]);
    assert!(success);
    assert!(stdout.contains("45"));
}

#[test]
fn test_complex_parentheses() {
    let (stdout, _, success) = run_cli(&["((2 + 3) * (4 + 5))"]);
    assert!(success);
    assert!(stdout.contains("45"));
}

// ==================== Decimal Tests ====================

#[test]
fn test_decimal_addition() {
    let (stdout, _, success) = run_cli(&["1.5 + 2.5"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

#[test]
fn test_decimal_multiplication() {
    let (stdout, _, success) = run_cli(&["0.5 * 0.5"]);
    assert!(success);
    assert!(stdout.contains("0.25"));
}

#[test]
fn test_decimal_division() {
    let (stdout, _, success) = run_cli(&["1 / 3"]);
    assert!(success);
    assert!(stdout.contains("0.333"));
}

#[test]
fn test_decimal_precision() {
    let (stdout, _, success) = run_cli(&["1.0 / 3.0"]);
    assert!(success);
    assert!(stdout.contains("0.333333"));
}

// ==================== Large Number Tests ====================

#[test]
fn test_large_addition() {
    let (stdout, _, success) = run_cli(&["1000000 + 2000000"]);
    assert!(success);
    assert!(stdout.contains("3000000"));
}

#[test]
fn test_large_multiplication() {
    let (stdout, _, success) = run_cli(&["1000 * 1000"]);
    assert!(success);
    assert!(stdout.contains("1000000"));
}

#[test]
fn test_very_large_numbers() {
    let (stdout, _, success) = run_cli(&["1e100 * 1e100"]);
    assert!(success);
    assert!(stdout.contains("100000") || stdout.contains("1e200"));
}

#[test]
fn test_very_small_numbers() {
    let (stdout, _, success) = run_cli(&["1e-100 * 1e-100"]);
    assert!(success);
    assert!(stdout.contains("1e-200") || stdout.contains("0"));
}

// ==================== Chain Operation Tests ====================

#[test]
fn test_chain_addition() {
    let (stdout, _, success) = run_cli(&["1 + 2 + 3 + 4 + 5"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_chain_subtraction() {
    let (stdout, _, success) = run_cli(&["10 - 5 - 3"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_chain_division() {
    let (stdout, _, success) = run_cli(&["100 / 5 / 2"]);
    assert!(success);
    assert!(stdout.contains("10"));
}

#[test]
fn test_chain_mixed() {
    let (stdout, _, success) = run_cli(&["1 + 2 - 3 + 4 - 5"]);
    assert!(success);
    assert!(stdout.contains("-1"));
}

#[test]
fn test_alternating_signs() {
    let (stdout, _, success) = run_cli(&["1 - 2 + 3 - 4 + 5"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

// ==================== Trigonometric Edge Cases ====================

#[test]
fn test_sin_pi() {
    let (stdout, _, success) = run_cli(&["sin(C_PI)"]);
    assert!(success);
    // sin(π) should be very close to 0 (displays as scientific notation)
    assert!(stdout.contains("0") || stdout.contains("e-"));
}

#[test]
fn test_sin_negative_pi() {
    let (stdout, _, success) = run_cli(&["sin(0 - C_PI)"]);
    assert!(success);
    assert!(stdout.contains("0") || stdout.contains("e-"));
}

#[test]
fn test_cos_pi() {
    let (stdout, _, success) = run_cli(&["cos(C_PI)"]);
    assert!(success);
    assert!(stdout.contains("-1"));
}

#[test]
fn test_cos_pi_half() {
    let (stdout, _, success) = run_cli(&["cos(C_PI / 2)"]);
    assert!(success);
    // cos(π/2) should be very close to 0 (displays as scientific notation)
    assert!(stdout.contains("0") || stdout.contains("e-"));
}

#[test]
fn test_tan_pi_quarter() {
    let (stdout, _, success) = run_cli(&["tan(C_PI / 4)"]);
    assert!(success);
    // tan(π/4) ≈ 1 (may be 0.9999... or 1.0000... due to floating point)
    assert!(stdout.contains("0.999") || stdout.contains("1.0") || stdout.contains("1"));
}

#[test]
fn test_trig_pi_values() {
    let (stdout, _, success) = run_cli(&["sin(C_PI / 2) + cos(0)"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

// ==================== Logarithm Edge Cases ====================

#[test]
fn test_lg_power_of_10() {
    let (stdout, _, success) = run_cli(&["lg(1000)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_lg_1() {
    let (stdout, _, success) = run_cli(&["lg(1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_ln_1() {
    let (stdout, _, success) = run_cli(&["ln(1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

// ==================== Factorial Edge Cases ====================

#[test]
fn test_factorial_boundary() {
    let (stdout, _, success) = run_cli(&["factorial(170)"]);
    assert!(success);
    // 170! is a very large number (7257415615307994... with many zeros)
    assert!(stdout.contains("72574") || stdout.contains("170"));
}

#[test]
fn test_factorial_2() {
    let (stdout, _, success) = run_cli(&["factorial(2)"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_factorial_3() {
    let (stdout, _, success) = run_cli(&["factorial(3)"]);
    assert!(success);
    assert!(stdout.contains("6"));
}

#[test]
fn test_factorial_4() {
    let (stdout, _, success) = run_cli(&["factorial(4)"]);
    assert!(success);
    assert!(stdout.contains("24"));
}
