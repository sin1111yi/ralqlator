// Error Handling Tests
// Tests for error conditions and invalid input

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

// ==================== Division/Modulo by Zero Tests ====================

#[test]
fn test_division_by_zero() {
    let (_, stderr, success) = run_cli(&["10 / 0"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
    assert!(stderr.contains("zero"));
}

#[test]
fn test_modulo_by_zero() {
    let (_, stderr, success) = run_cli(&["10 % 0"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_division_by_zero_in_expression() {
    let (_, stderr, success) = run_cli(&["1 / 0 + 5"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_division_by_zero_nested() {
    let (_, stderr, success) = run_cli(&["10 / (5 - 5)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_modulo_by_zero_in_expression() {
    let (_, stderr, success) = run_cli(&["10 % (3 - 3)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Domain Error Tests ====================

#[test]
fn test_sqrt_negative() {
    let (_, stderr, success) = run_cli(&["sqrt(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_ln_negative() {
    let (_, stderr, success) = run_cli(&["ln(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_ln_zero() {
    let (_, stderr, success) = run_cli(&["ln(0)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_lg_negative() {
    let (_, stderr, success) = run_cli(&["lg(-10)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_lg_zero() {
    let (_, stderr, success) = run_cli(&["lg(0)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_asin_out_of_range_positive() {
    let (_, stderr, success) = run_cli(&["asin(2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_asin_out_of_range_negative() {
    let (_, stderr, success) = run_cli(&["asin(-2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_acos_out_of_range_positive() {
    let (_, stderr, success) = run_cli(&["acos(2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_acos_out_of_range_negative() {
    let (_, stderr, success) = run_cli(&["acos(-2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_log_base_1() {
    let (_, stderr, success) = run_cli(&["lg(10, 1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_log_base_zero() {
    let (_, stderr, success) = run_cli(&["lg(10, 0)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_log_base_negative() {
    let (_, stderr, success) = run_cli(&["lg(10, -2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Invalid Input Tests ====================

#[test]
fn test_invalid_number() {
    let (_, stderr, success) = run_cli(&["abc"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_invalid_expression() {
    let (_, stderr, success) = run_cli(&["1 + abc"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_invalid_function() {
    let (_, stderr, success) = run_cli(&["invalid_func(1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_empty_expression() {
    let (_, stderr, success) = run_cli(&[""]);
    assert!(!success);
}

#[test]
fn test_whitespace_only() {
    let (_, stderr, success) = run_cli(&["   "]);
    assert!(!success);
}

#[test]
fn test_only_operator() {
    let (_, stderr, success) = run_cli(&["+"]);
    assert!(!success);
}

#[test]
fn test_only_multiplication() {
    let (_, stderr, success) = run_cli(&["*"]);
    assert!(!success);
}

// ==================== Syntax Error Tests ====================

#[test]
fn test_consecutive_operators() {
    let (_, stderr, success) = run_cli(&["1 ++ 2"]);
    assert!(!success);
}

#[test]
fn test_consecutive_multiplication() {
    let (_, stderr, success) = run_cli(&["1 ** 2"]);
    assert!(!success);
}

#[test]
fn test_unmatched_open_paren() {
    let (_, stderr, success) = run_cli(&["(1 + 2"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("unmatched"));
}

#[test]
fn test_unmatched_close_paren() {
    let (_, stderr, success) = run_cli(&["1 + 2)"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("unmatched"));
}

#[test]
fn test_extra_close_paren() {
    let (_, stderr, success) = run_cli(&["((1 + 2)) + 3)"]);
    assert!(!success);
}

#[test]
fn test_empty_parentheses() {
    let (_, stderr, success) = run_cli(&["()"]);
    assert!(!success);
}

// ==================== Function Argument Error Tests ====================

#[test]
fn test_function_no_args() {
    let (_, stderr, success) = run_cli(&["sin()"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_function_wrong_arg_count() {
    let (_, stderr, success) = run_cli(&["pow(2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_function_too_many_args() {
    let (_, stderr, success) = run_cli(&["pow(2, 3, 4)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_sum_empty() {
    let (_, stderr, success) = run_cli(&["sum()"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_prod_empty() {
    let (_, stderr, success) = run_cli(&["prod()"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Bitwise Error Tests ====================

#[test]
fn test_bitwise_invalid_shift_positive() {
    let (_, stderr, success) = run_cli(&["-B", "8 << 64"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_bitwise_invalid_shift_negative() {
    let (_, stderr, success) = run_cli(&["-B", "8 >> -1"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_bitwise_subtraction_not_supported() {
    let (_, stderr, success) = run_cli(&["-B", "10 - 3"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
    assert!(stderr.contains("subtraction"));
}

// ==================== Factorial Error Tests ====================

#[test]
fn test_factorial_negative() {
    let (_, stderr, success) = run_cli(&["factorial(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_factorial_non_integer() {
    let (_, stderr, success) = run_cli(&["factorial(3.5)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_factorial_too_large() {
    let (_, stderr, success) = run_cli(&["factorial(171)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}
