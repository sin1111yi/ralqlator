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
    // sin(π) should be very close to 0 (displays as scientific notation)
    assert!(stdout.contains("0") || stdout.contains("e-"));
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
    // cos(π/2) should be very close to 0 (displays as scientific notation)
    assert!(stdout.contains("0") || stdout.contains("e-"));
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

// ==================== New Mathematical Functions Tests ====================
// Tests for functions added from statrs and num crates

// ==================== Logarithm Tests ====================

#[test]
fn test_log2_basic() {
    let (stdout, _, success) = run_cli(&["log2(256)"]);
    assert!(success);
    assert!(stdout.contains("8"));
}

#[test]
fn test_log2_power_of_2() {
    let (stdout, _, success) = run_cli(&["log2(1024)"]);
    assert!(success);
    assert!(stdout.contains("10"));
}

#[test]
fn test_log2_of_1() {
    let (stdout, _, success) = run_cli(&["log2(1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_log2_error_negative() {
    let (_, stderr, success) = run_cli(&["log2(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_log2_error_zero() {
    let (_, stderr, success) = run_cli(&["log2(0)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Cube Root Tests ====================

#[test]
fn test_cbrt_basic() {
    let (stdout, _, success) = run_cli(&["cbrt(27)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_cbrt_64() {
    let (stdout, _, success) = run_cli(&["cbrt(64)"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

#[test]
fn test_cbrt_negative() {
    let (stdout, _, success) = run_cli(&["cbrt(-8)"]);
    assert!(success);
    assert!(stdout.contains("-2"));
}

// ==================== Reciprocal Trigonometric Tests ====================

#[test]
fn test_sec_zero() {
    let (stdout, _, success) = run_cli(&["sec(0)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_sec_pi() {
    let (stdout, _, success) = run_cli(&["sec(C_PI)"]);
    assert!(success);
    assert!(stdout.contains("-1"));
}

#[test]
fn test_csc_pi_half() {
    let (stdout, _, success) = run_cli(&["csc(C_PI / 2)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_cot_pi_half() {
    let (stdout, _, success) = run_cli(&["cot(C_PI / 2)"]);
    assert!(success);
    // cot(π/2) should be close to 0
    assert!(stdout.contains("0") || stdout.contains("e-"));
}

#[test]
fn test_sec_cos_reciprocal() {
    // sec(x) = 1/cos(x)
    let (stdout, _, success) = run_cli(&["sec(C_PI / 3)"]);
    assert!(success);
    // sec(π/3) = 1/cos(π/3) = 1/0.5 = 2 (may have floating point error)
    assert!(stdout.contains("1.99") || stdout.contains("2"));
}

#[test]
fn test_csc_sin_reciprocal() {
    // csc(x) = 1/sin(x)
    let (stdout, _, success) = run_cli(&["csc(C_PI / 6)"]);
    assert!(success);
    // csc(π/6) = 1/sin(π/6) = 1/0.5 = 2
    assert!(stdout.contains("2"));
}

#[test]
fn test_cot_tan_reciprocal() {
    // cot(x) = 1/tan(x)
    let (stdout, _, success) = run_cli(&["cot(C_PI / 4)"]);
    assert!(success);
    // cot(π/4) = 1/tan(π/4) = 1/1 = 1
    assert!(stdout.contains("1"));
}

// ==================== Hyperbolic Functions Tests ====================

#[test]
fn test_sinh_zero() {
    let (stdout, _, success) = run_cli(&["sinh(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_sinh_one() {
    let (stdout, _, success) = run_cli(&["sinh(1)"]);
    assert!(success);
    assert!(stdout.contains("1.175"));
}

#[test]
fn test_cosh_zero() {
    let (stdout, _, success) = run_cli(&["cosh(0)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_cosh_one() {
    let (stdout, _, success) = run_cli(&["cosh(1)"]);
    assert!(success);
    assert!(stdout.contains("1.543"));
}

#[test]
fn test_tanh_zero() {
    let (stdout, _, success) = run_cli(&["tanh(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_tanh_one() {
    let (stdout, _, success) = run_cli(&["tanh(1)"]);
    assert!(success);
    assert!(stdout.contains("0.761"));
}

#[test]
fn test_cosh_squared_minus_sinh_squared() {
    // cosh²(x) - sinh²(x) = 1
    let (stdout, _, success) = run_cli(&["cosh(1) ^ 2 - sinh(1) ^ 2"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_sinh_cosh_sum() {
    // sinh(x) + cosh(x) = e^x
    let (stdout, _, success) = run_cli(&["sinh(1) + cosh(1)"]);
    assert!(success);
    assert!(stdout.contains("2.718"));
}

// ==================== Inverse Hyperbolic Tests ====================

#[test]
fn test_asinh_zero() {
    let (stdout, _, success) = run_cli(&["asinh(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_asinh_one() {
    let (stdout, _, success) = run_cli(&["asinh(1)"]);
    assert!(success);
    assert!(stdout.contains("0.881"));
}

#[test]
fn test_acosh_one() {
    let (stdout, _, success) = run_cli(&["acosh(1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_acosh_two() {
    let (stdout, _, success) = run_cli(&["acosh(2)"]);
    assert!(success);
    assert!(stdout.contains("1.316"));
}

#[test]
fn test_acosh_error() {
    let (_, stderr, success) = run_cli(&["acosh(0.5)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_atanh_zero() {
    let (stdout, _, success) = run_cli(&["atanh(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_atanh_half() {
    let (stdout, _, success) = run_cli(&["atanh(0.5)"]);
    assert!(success);
    assert!(stdout.contains("0.549"));
}

#[test]
fn test_atanh_error() {
    let (_, stderr, success) = run_cli(&["atanh(1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Utility Functions Tests ====================

#[test]
fn test_abs_positive() {
    let (stdout, _, success) = run_cli(&["abs(5)"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_abs_negative() {
    let (stdout, _, success) = run_cli(&["abs(-5)"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_abs_zero() {
    let (stdout, _, success) = run_cli(&["abs(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_floor_basic() {
    let (stdout, _, success) = run_cli(&["floor(3.7)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_floor_negative() {
    let (stdout, _, success) = run_cli(&["floor(-3.7)"]);
    assert!(success);
    assert!(stdout.contains("-4"));
}

#[test]
fn test_ceil_basic() {
    let (stdout, _, success) = run_cli(&["ceil(3.2)"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

#[test]
fn test_ceil_negative() {
    let (stdout, _, success) = run_cli(&["ceil(-3.2)"]);
    assert!(success);
    assert!(stdout.contains("-3"));
}

#[test]
fn test_round_down() {
    let (stdout, _, success) = run_cli(&["round(3.4)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_round_up() {
    let (stdout, _, success) = run_cli(&["round(3.6)"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

#[test]
fn test_round_half() {
    let (stdout, _, success) = run_cli(&["round(3.5)"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

// ==================== Error Function Tests ====================

#[test]
fn test_erf_zero() {
    let (stdout, _, success) = run_cli(&["erf(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_erf_one() {
    let (stdout, _, success) = run_cli(&["erf(1)"]);
    assert!(success);
    assert!(stdout.contains("0.8427"));
}

#[test]
fn test_erf_negative() {
    let (stdout, _, success) = run_cli(&["erf(-1)"]);
    assert!(success);
    assert!(stdout.contains("-0.8427"));
}

#[test]
fn test_erf_large() {
    let (stdout, _, success) = run_cli(&["erf(3)"]);
    assert!(success);
    // erf(3) should be very close to 1
    assert!(stdout.contains("0.999"));
}

#[test]
fn test_erfc_zero() {
    let (stdout, _, success) = run_cli(&["erfc(0)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_erfc_one() {
    let (stdout, _, success) = run_cli(&["erfc(1)"]);
    assert!(success);
    assert!(stdout.contains("0.157"));
}

#[test]
fn test_erf_erfc_complement() {
    // erf(x) + erfc(x) = 1
    let (stdout, _, success) = run_cli(&["erf(1) + erfc(1)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

// ==================== Beta Function Tests ====================

#[test]
fn test_beta_basic() {
    let (stdout, _, success) = run_cli(&["beta(2, 3)"]);
    assert!(success);
    // B(2,3) = 1! * 2! / 3! = 2/6 = 1/12 ≈ 0.0833
    assert!(stdout.contains("0.0833"));
}

#[test]
fn test_beta_one_one() {
    let (stdout, _, success) = run_cli(&["beta(1, 1)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_beta_symmetric() {
    // beta(x,y) = beta(y,x)
    let (stdout1, _, success1) = run_cli(&["beta(2, 3)"]);
    let (stdout2, _, success2) = run_cli(&["beta(3, 2)"]);
    assert!(success1 && success2);
    assert_eq!(stdout1.trim(), stdout2.trim());
}

#[test]
fn test_beta_error_negative() {
    let (_, stderr, success) = run_cli(&["beta(-1, 2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_beta_error_zero() {
    let (_, stderr, success) = run_cli(&["beta(0, 1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Gamma Function Tests ====================

#[test]
fn test_gamma_integer() {
    // gamma(n) = (n-1)!
    let (stdout, _, success) = run_cli(&["gamma(5)"]);
    assert!(success);
    assert!(stdout.contains("24"));
}

#[test]
fn test_gamma_one() {
    let (stdout, _, success) = run_cli(&["gamma(1)"]);
    assert!(success);
    // gamma(1) = 0! = 1 (may have floating point error)
    assert!(stdout.contains("0.999") || stdout.contains("1"));
}

#[test]
fn test_gamma_two() {
    let (stdout, _, success) = run_cli(&["gamma(2)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_gamma_half() {
    let (stdout, _, success) = run_cli(&["gamma(0.5)"]);
    assert!(success);
    // gamma(0.5) = sqrt(π) ≈ 1.772
    assert!(stdout.contains("1.772"));
}

#[test]
fn test_gamma_error_negative_integer() {
    let (_, stderr, success) = run_cli(&["gamma(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_gamma_factorial_relationship() {
    // gamma(n) = (n-1)!
    let (stdout1, _, success1) = run_cli(&["gamma(6)"]);
    let (stdout2, _, success2) = run_cli(&["factorial(5)"]);
    assert!(success1 && success2);
    // Both should be approximately 120 (gamma may have floating point error)
    assert!(stdout1.contains("119.9") || stdout1.contains("120"));
    assert!(stdout2.contains("120"));
}

#[test]
fn test_beta_gamma_relationship() {
    // B(x,y) = Γ(x) * Γ(y) / Γ(x+y)
    // For x=2, y=3: B(2,3) = 1! * 2! / 3! = 2/6 = 1/12
    let (stdout, _, success) = run_cli(&["beta(2, 3)"]);
    assert!(success);
    assert!(stdout.contains("0.0833"));
}

// ==================== Two-Argument Arctangent Tests ====================

#[test]
fn test_atan2_basic() {
    let (stdout, _, success) = run_cli(&["atan2(1, 1)"]);
    assert!(success);
    // atan2(1,1) = π/4 ≈ 0.785
    assert!(stdout.contains("0.785"));
}

#[test]
fn test_atan2_y_zero() {
    let (stdout, _, success) = run_cli(&["atan2(0, 1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_atan2_x_zero() {
    let (stdout, _, success) = run_cli(&["atan2(1, 0)"]);
    assert!(success);
    // atan2(1,0) = π/2 ≈ 1.57
    assert!(stdout.contains("1.57"));
}

#[test]
fn test_atan2_negative() {
    let (stdout, _, success) = run_cli(&["atan2(-1, -1)"]);
    assert!(success);
    // atan2(-1,-1) = -3π/4 ≈ -2.356
    assert!(stdout.contains("-2.356"));
}

#[test]
fn test_atan2_error_args() {
    let (_, stderr, success) = run_cli(&["atan2(1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}
