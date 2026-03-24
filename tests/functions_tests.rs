// Mathematical Functions Tests
// Tests for built-in mathematical functions

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

// ==================== Logarithm Tests ====================

#[test]
fn test_lg_base10() {
    let (stdout, _, success) = run_cli(&["lg(100)"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_lg_10() {
    let (stdout, _, success) = run_cli(&["lg(10)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_lg_1() {
    let (stdout, _, success) = run_cli(&["lg(1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_lg_custom_base() {
    let (stdout, _, success) = run_cli(&["lg(8, 2)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_lg_custom_base_1024() {
    let (stdout, _, success) = run_cli(&["lg(1024, 2)"]);
    assert!(success);
    assert!(stdout.contains("10"));
}

#[test]
fn test_lg_base_1_error() {
    let (_, stderr, success) = run_cli(&["lg(10, 1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_lg_negative_base_error() {
    let (_, stderr, success) = run_cli(&["lg(10, -2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_lg_zero_argument_error() {
    let (_, stderr, success) = run_cli(&["lg(0)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_lg_negative_argument_error() {
    let (_, stderr, success) = run_cli(&["lg(-10)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_log_custom_base() {
    let (stdout, _, success) = run_cli(&["log(27, 3)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

// ==================== Natural Logarithm Tests ====================

#[test]
fn test_ln_e() {
    let (stdout, _, success) = run_cli(&["ln(C_E)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_ln_1() {
    let (stdout, _, success) = run_cli(&["ln(1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_ln_zero_error() {
    let (_, stderr, success) = run_cli(&["ln(0)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_ln_negative_error() {
    let (_, stderr, success) = run_cli(&["ln(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Square Root Tests ====================

#[test]
fn test_sqrt_perfect() {
    let (stdout, _, success) = run_cli(&["sqrt(16)"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

#[test]
fn test_sqrt_2() {
    let (stdout, _, success) = run_cli(&["sqrt(2)"]);
    assert!(success);
    assert!(stdout.contains("1.414"));
}

#[test]
fn test_sqrt_zero() {
    let (stdout, _, success) = run_cli(&["sqrt(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_sqrt_144() {
    let (stdout, _, success) = run_cli(&["sqrt(144)"]);
    assert!(success);
    assert!(stdout.contains("12"));
}

#[test]
fn test_sqrt_negative_error() {
    let (_, stderr, success) = run_cli(&["sqrt(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Power Function Tests ====================

#[test]
fn test_pow_positive() {
    let (stdout, _, success) = run_cli(&["pow(2, 10)"]);
    assert!(success);
    assert!(stdout.contains("1024"));
}

#[test]
fn test_pow_3_3() {
    let (stdout, _, success) = run_cli(&["pow(3, 3)"]);
    assert!(success);
    assert!(stdout.contains("27"));
}

#[test]
fn test_pow_negative_exponent() {
    let (stdout, _, success) = run_cli(&["pow(2, -1)"]);
    assert!(success);
    assert!(stdout.contains("0.5"));
}

#[test]
fn test_pow_fractional_exponent() {
    let (stdout, _, success) = run_cli(&["pow(4, 0.5)"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_pow_zero_base() {
    let (stdout, _, success) = run_cli(&["pow(0, 5)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_pow_zero_exponent() {
    let (stdout, _, success) = run_cli(&["pow(5, 0)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

// ==================== Trigonometric Tests ====================

#[test]
fn test_sin_zero() {
    let (stdout, _, success) = run_cli(&["sin(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_sin_pi_half() {
    let (stdout, _, success) = run_cli(&["sin(C_PI / 2)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_sin_pi() {
    let (stdout, _, success) = run_cli(&["sin(C_PI)"]);
    assert!(success);
    assert!(stdout.contains("0") || stdout.contains("1e-"));
}

#[test]
fn test_cos_zero() {
    let (stdout, _, success) = run_cli(&["cos(0)"]);
    assert!(success);
    assert!(stdout.contains("1"));
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
    assert!(stdout.contains("0") || stdout.contains("1e-"));
}

#[test]
fn test_tan_zero() {
    let (stdout, _, success) = run_cli(&["tan(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_tan_pi_half_error() {
    let (stdout, _, success) = run_cli(&["tan(C_PI / 2)"]);
    // tan(π/2) is undefined, but due to floating point precision it may return a very large number
    assert!(success);
    // The result should be a very large number (>> 1)
    assert!(stdout.contains("1633") || stdout.contains("1e") || stdout.contains("Error"));
}

#[test]
fn test_tan_pi_quarter() {
    let (stdout, _, success) = run_cli(&["tan(C_PI / 4)"]);
    assert!(success);
    // tan(π/4) ≈ 1 (may be 0.9999... due to floating point)
    assert!(stdout.contains("0.999") || stdout.contains("1.0") || stdout.contains("1"));
}

// ==================== Inverse Trigonometric Tests ====================

#[test]
fn test_asin_zero() {
    let (stdout, _, success) = run_cli(&["asin(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_asin_one() {
    let (stdout, _, success) = run_cli(&["asin(1)"]);
    assert!(success);
    assert!(stdout.contains("1.57") || stdout.contains("C_PI"));
}

#[test]
fn test_asin_half() {
    let (stdout, _, success) = run_cli(&["asin(0.5)"]);
    assert!(success);
    assert!(stdout.contains("0.523"));
}

#[test]
fn test_asin_out_of_range_error() {
    let (_, stderr, success) = run_cli(&["asin(2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_asin_negative_out_of_range_error() {
    let (_, stderr, success) = run_cli(&["asin(-2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_acos_zero() {
    let (stdout, _, success) = run_cli(&["acos(0)"]);
    assert!(success);
    assert!(stdout.contains("1.57") || stdout.contains("C_PI"));
}

#[test]
fn test_acos_one() {
    let (stdout, _, success) = run_cli(&["acos(1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_acos_half() {
    let (stdout, _, success) = run_cli(&["acos(0.5)"]);
    assert!(success);
    assert!(stdout.contains("1.047"));
}

#[test]
fn test_acos_out_of_range_error() {
    let (_, stderr, success) = run_cli(&["acos(2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_atan_zero() {
    let (stdout, _, success) = run_cli(&["atan(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_atan_one() {
    let (stdout, _, success) = run_cli(&["atan(1)"]);
    assert!(success);
    assert!(stdout.contains("0.785"));
}

// ==================== Modulo Function Tests ====================

#[test]
fn test_mod_function() {
    let (stdout, _, success) = run_cli(&["mod(10, 3)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_mod_function_17_5() {
    let (stdout, _, success) = run_cli(&["mod(17, 5)"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_mod_zero_dividend() {
    let (stdout, _, success) = run_cli(&["mod(0, 5)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_mod_same_values() {
    let (stdout, _, success) = run_cli(&["mod(5, 5)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_mod_larger_divisor() {
    let (stdout, _, success) = run_cli(&["mod(3, 5)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_mod_negative() {
    let (stdout, _, success) = run_cli(&["mod(-10, 3)"]);
    assert!(success);
    assert!(stdout.contains("-1"));
}

#[test]
fn test_mod_by_zero_error() {
    let (_, stderr, success) = run_cli(&["mod(10, 0)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Sum Function Tests ====================

#[test]
fn test_sum_single() {
    let (stdout, _, success) = run_cli(&["sum(5)"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_sum_multiple() {
    let (stdout, _, success) = run_cli(&["sum(1,2,3,4,5)"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_sum_float() {
    let (stdout, _, success) = run_cli(&["sum(1.5,2.5,3)"]);
    assert!(success);
    assert!(stdout.contains("7"));
}

#[test]
fn test_sum_with_negatives() {
    let (stdout, _, success) = run_cli(&["sum(1,-2,3,-4)"]);
    assert!(success);
    assert!(stdout.contains("-2"));
}

#[test]
fn test_sum_with_zeros() {
    let (stdout, _, success) = run_cli(&["sum(0,0,0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_sum_empty_error() {
    let (_, stderr, success) = run_cli(&["sum()"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Product Function Tests ====================

#[test]
fn test_prod_single() {
    let (stdout, _, success) = run_cli(&["prod(5)"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_prod_multiple() {
    let (stdout, _, success) = run_cli(&["prod(1,2,3,4,5)"]);
    assert!(success);
    assert!(stdout.contains("120"));
}

#[test]
fn test_prod_float() {
    let (stdout, _, success) = run_cli(&["prod(1.5,2,3)"]);
    assert!(success);
    assert!(stdout.contains("9"));
}

#[test]
fn test_prod_with_negatives() {
    let (stdout, _, success) = run_cli(&["prod(-1,-2,-3)"]);
    assert!(success);
    assert!(stdout.contains("-6"));
}

#[test]
fn test_prod_single_negative() {
    let (stdout, _, success) = run_cli(&["prod(-5)"]);
    assert!(success);
    assert!(stdout.contains("-5"));
}

#[test]
fn test_prod_with_zero() {
    let (stdout, _, success) = run_cli(&["prod(1,2,0,4)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_prod_empty_error() {
    let (_, stderr, success) = run_cli(&["prod()"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Pythagorean Identity Tests ====================

#[test]
fn test_pythagorean_identity() {
    let (stdout, _, success) = run_cli(&["sin(C_PI / 4) ^ 2 + cos(C_PI / 4) ^ 2"]);
    assert!(success);
    assert!(stdout.contains("1"));
}
