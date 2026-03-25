// Ralqlator - Core Functionality Tests
// Combines: arithmetic, bitwise, comparison, constants, edge_cases

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

// ==================== Arithmetic Tests ====================

#[test]
fn test_addition() {
    let (stdout, _, success) = run_cli(&["1 + 2"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_addition_large_numbers() {
    let (stdout, _, success) = run_cli(&["1000000 + 2000000"]);
    assert!(success);
    assert!(stdout.contains("3000000"));
}

#[test]
fn test_addition_negative() {
    let (stdout, _, success) = run_cli(&["5 + (-3)"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_addition_multiple() {
    let (stdout, _, success) = run_cli(&["1 + 2 + 3 + 4 + 5"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_subtraction() {
    let (stdout, _, success) = run_cli(&["5 - 3"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_subtraction_negative_result() {
    let (stdout, _, success) = run_cli(&["3 - 5"]);
    assert!(success);
    assert!(stdout.contains("-2"));
}

#[test]
fn test_subtraction_negative_numbers() {
    let (stdout, _, success) = run_cli(&["--", "-5 - (-3)"]);
    assert!(success);
    assert!(stdout.contains("-2"));
}

#[test]
fn test_multiplication() {
    let (stdout, _, success) = run_cli(&["3 * 4"]);
    assert!(success);
    assert!(stdout.contains("12"));
}

#[test]
fn test_multiplication_by_zero() {
    let (stdout, _, success) = run_cli(&["100 * 0"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_multiplication_negative() {
    let (stdout, _, success) = run_cli(&["--", "-3 * 4"]);
    assert!(success);
    assert!(stdout.contains("-12"));
}

#[test]
fn test_division() {
    let (stdout, _, success) = run_cli(&["10 / 2"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_division_decimal_result() {
    let (stdout, _, success) = run_cli(&["10 / 3"]);
    assert!(success);
    assert!(stdout.contains("3.33"));
}

#[test]
fn test_division_by_one() {
    let (stdout, _, success) = run_cli(&["42 / 1"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

#[test]
fn test_division_by_zero() {
    let (_, stderr, success) = run_cli(&["1 / 0"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("zero"));
}

#[test]
fn test_modulo() {
    let (stdout, _, success) = run_cli(&["10 % 3"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_modulo_zero() {
    let (_, stderr, success) = run_cli(&["10 % 0"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("zero"));
}

#[test]
fn test_exponentiation() {
    let (stdout, _, success) = run_cli(&["2 ^ 10"]);
    assert!(success);
    assert!(stdout.contains("1024"));
}

#[test]
fn test_exponentiation_zero() {
    let (stdout, _, success) = run_cli(&["5 ^ 0"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_exponentiation_one() {
    let (stdout, _, success) = run_cli(&["5 ^ 1"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_factorial() {
    let (stdout, _, success) = run_cli(&["5!"]);
    assert!(success);
    assert!(stdout.contains("120"));
}

#[test]
fn test_factorial_zero() {
    let (stdout, _, success) = run_cli(&["0!"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_factorial_one() {
    let (stdout, _, success) = run_cli(&["1!"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_factorial_large() {
    let (stdout, _, success) = run_cli(&["10!"]);
    assert!(success);
    assert!(stdout.contains("3628800"));
}

#[test]
fn test_operator_precedence() {
    let (stdout, _, success) = run_cli(&["2 + 3 * 4"]);
    assert!(success);
    assert!(stdout.contains("14"));
}

#[test]
fn test_operator_precedence_complex() {
    let (stdout, _, success) = run_cli(&["2 + 3 * 4 - 5"]);
    assert!(success);
    assert!(stdout.contains("9"));
}

#[test]
fn test_parentheses() {
    let (stdout, _, success) = run_cli(&["(2 + 3) * 4"]);
    assert!(success);
    assert!(stdout.contains("20"));
}

#[test]
fn test_nested_parentheses() {
    let (stdout, _, success) = run_cli(&["((1 + 2) * (3 + 4))"]);
    assert!(success);
    assert!(stdout.contains("21"));
}

#[test]
fn test_deeply_nested_parentheses() {
    let (stdout, _, success) = run_cli(&["(((1 + 2)))"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

// ==================== Bitwise Tests ====================

#[test]
fn test_bitwise_and() {
    let (stdout, _, success) = run_cli(&["-B", "12 & 10"]);
    assert!(success);
    assert!(stdout.contains("8"));
}

#[test]
fn test_bitwise_and_hex() {
    let (stdout, _, success) = run_cli(&["-B", "0xFF & 0x0F"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_bitwise_or() {
    let (stdout, _, success) = run_cli(&["-B", "12 | 10"]);
    assert!(success);
    assert!(stdout.contains("14"));
}

#[test]
fn test_bitwise_xor() {
    let (stdout, _, success) = run_cli(&["-B", "12 ^ 10"]);
    assert!(success);
    assert!(stdout.contains("6"));
}

#[test]
fn test_bitwise_not() {
    let (stdout, _, success) = run_cli(&["-B", "~0"]);
    assert!(success);
    assert!(stdout.contains("-1"));
}

#[test]
fn test_bitwise_not_value() {
    let (stdout, _, success) = run_cli(&["-B", "~12"]);
    assert!(success);
    assert!(stdout.contains("-13"));
}

#[test]
fn test_bitwise_shift_left() {
    let (stdout, _, success) = run_cli(&["-B", "8 << 2"]);
    assert!(success);
    assert!(stdout.contains("32"));
}

#[test]
fn test_bitwise_shift_right() {
    let (stdout, _, success) = run_cli(&["-B", "8 >> 2"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_bitwise_shift_zero() {
    let (stdout, _, success) = run_cli(&["-B", "8 << 0"]);
    assert!(success);
    assert!(stdout.contains("8"));
}

#[test]
fn test_bitwise_combined() {
    let (stdout, _, success) = run_cli(&["-B", "(12 & 10) | 1"]);
    assert!(success);
    assert!(stdout.contains("9"));
}

// ==================== Comparison Tests ====================

#[test]
fn test_less_than() {
    let (stdout, _, success) = run_cli(&["3 < 5"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_less_than_false() {
    let (stdout, _, success) = run_cli(&["5 < 3"]);
    assert!(success);
    assert!(stdout.contains("false"));
}

#[test]
fn test_less_than_equal() {
    let (stdout, _, success) = run_cli(&["3 < 3"]);
    assert!(success);
    assert!(stdout.contains("false"));
}

#[test]
fn test_greater_than() {
    let (stdout, _, success) = run_cli(&["5 > 3"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_greater_than_false() {
    let (stdout, _, success) = run_cli(&["3 > 5"]);
    assert!(success);
    assert!(stdout.contains("false"));
}

#[test]
fn test_equal() {
    let (stdout, _, success) = run_cli(&["5 = 5"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_not_equal() {
    let (stdout, _, success) = run_cli(&["5 = 3"]);
    assert!(success);
    assert!(stdout.contains("false"));
}

#[test]
fn test_double_equal() {
    let (stdout, _, success) = run_cli(&["5 == 5"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_with_expression() {
    let (stdout, _, success) = run_cli(&["2 + 3 > 4"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_chained() {
    let (_stdout, _, success) = run_cli(&["(3 < 5) = true"]);
    assert!(success);
}

// ==================== Constants Tests ====================

#[test]
fn test_c_pi() {
    let (stdout, _, success) = run_cli(&["C_PI"]);
    assert!(success);
    assert!(stdout.contains("3.14"));
}

#[test]
fn test_c_e() {
    let (stdout, _, success) = run_cli(&["C_E"]);
    assert!(success);
    assert!(stdout.contains("2.718"));
}

#[test]
fn test_c_pi_expression() {
    let (stdout, _, success) = run_cli(&["C_PI * 2"]);
    assert!(success);
    assert!(stdout.contains("6.28"));
}

#[test]
fn test_c_e_expression() {
    let (stdout, _, success) = run_cli(&["C_E ^ 2"]);
    assert!(success);
    assert!(stdout.contains("7.38"));
}

#[test]
fn test_c_pi_with_sin() {
    let (stdout, _, success) = run_cli(&["sin(C_PI / 2)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_c_e_with_ln() {
    let (stdout, _, success) = run_cli(&["ln(C_E)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

// ==================== Edge Cases Tests ====================

#[test]
fn test_zero_addition() {
    let (stdout, _, success) = run_cli(&["0 + 5"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_zero_multiplication() {
    let (stdout, _, success) = run_cli(&["0 * 100"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_zero_exponent() {
    let (stdout, _, success) = run_cli(&["5 ^ 0"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_negative_number() {
    let (stdout, _, success) = run_cli(&["--", "-5 + 3"]);
    assert!(success);
    assert!(stdout.contains("-2"));
}

#[test]
fn test_negative_multiplication() {
    let (stdout, _, success) = run_cli(&["--", "-3 * -4"]);
    assert!(success);
    assert!(stdout.contains("12"));
}

#[test]
fn test_decimal_numbers() {
    let (stdout, _, success) = run_cli(&["3.14 + 2.86"]);
    assert!(success);
    assert!(stdout.contains("6"));
}

#[test]
fn test_decimal_multiplication() {
    let (stdout, _, success) = run_cli(&["0.5 * 0.5"]);
    assert!(success);
    assert!(stdout.contains("0.25"));
}

#[test]
fn test_large_numbers() {
    let (stdout, _, success) = run_cli(&["1000000 * 1000000"]);
    assert!(success);
    assert!(stdout.contains("1000000000000"));
}

#[test]
fn test_chain_operations() {
    let (stdout, _, success) = run_cli(&["1 + 2 + 3 + 4 + 5"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_mixed_operations() {
    let (stdout, _, success) = run_cli(&["2 + 3 * 4 - 6 / 2"]);
    assert!(success);
    assert!(stdout.contains("11"));
}

#[test]
fn test_identity_addition() {
    let (stdout, _, success) = run_cli(&["42 + 0"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

#[test]
fn test_identity_multiplication() {
    let (stdout, _, success) = run_cli(&["42 * 1"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

// ==================== Compact Format Tests (No Spaces) ====================

#[test]
fn test_compact_addition() {
    let (stdout, _, success) = run_cli(&["1+1"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_compact_subtraction() {
    let (stdout, _, success) = run_cli(&["10-5"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_compact_multiplication() {
    let (stdout, _, success) = run_cli(&["3*4"]);
    assert!(success);
    assert!(stdout.contains("12"));
}

#[test]
fn test_compact_division() {
    let (stdout, _, success) = run_cli(&["10/2"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_compact_mixed_operations() {
    let (stdout, _, success) = run_cli(&["1+2*3"]);
    assert!(success);
    assert!(stdout.contains("7"));
}

#[test]
fn test_compact_complex_expression() {
    let (stdout, _, success) = run_cli(&["2*3+4*5"]);
    assert!(success);
    assert!(stdout.contains("26"));
}

#[test]
fn test_compact_with_parentheses() {
    let (stdout, _, success) = run_cli(&["(1+2)*(3+4)"]);
    assert!(success);
    assert!(stdout.contains("21"));
}

#[test]
fn test_compact_negative_numbers() {
    let (stdout, _, success) = run_cli(&["10--5"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_compact_chain_operations() {
    let (stdout, _, success) = run_cli(&["1+2+3+4+5"]);
    assert!(success);
    assert!(stdout.contains("15"));
}
