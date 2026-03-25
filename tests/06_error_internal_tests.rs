// Ralqlator - Error Handling and Internal Tests
// Combines: error_tests, error_handling_tests, internal_tests, value_tests

use std::process::Command;
use ralqlator::error::CalcError;

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

// ==================== Error Type Tests ====================

#[test]
fn test_error_display() {
    assert_eq!(CalcError::DivisionByZero.to_string(), "Division by zero");
    assert_eq!(
        CalcError::TypeError {
            expected: "Rational".to_string(),
            got: "Float".to_string(),
        }.to_string(),
        "Type error: expected Rational, got Float"
    );
    assert_eq!(
        CalcError::UndefinedFunction("foo".to_string()).to_string(),
        "Undefined function: foo"
    );
}

#[test]
fn test_error_to_string() {
    let err = CalcError::DivisionByZero;
    let s: String = err.into();
    assert_eq!(s, "Division by zero");
}

#[test]
fn test_string_to_error() {
    let msg = "test error".to_string();
    let err: CalcError = msg.into();
    assert!(matches!(err, CalcError::ParseError(_)));
}

// ==================== Division/Modulo by Zero Tests ====================

#[test]
fn test_division_by_zero() {
    let (_, stderr, success) = run_cli(&["1 / 0"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("zero"));
}

#[test]
fn test_division_by_zero_nested() {
    let (_, stderr, success) = run_cli(&["1 / (2 - 2)"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("zero"));
}

#[test]
fn test_modulo_by_zero() {
    let (_, stderr, success) = run_cli(&["1 % 0"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("zero"));
}

#[test]
fn test_division_by_zero_in_expression() {
    let (_, stderr, success) = run_cli(&["(1 + 2) / (5 - 5)"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("zero"));
}

// ==================== Domain Error Tests ====================

#[test]
fn test_sqrt_negative() {
    let (_, stderr, success) = run_cli(&["sqrt(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_lg_negative() {
    let (_, stderr, success) = run_cli(&["lg(-1)"]);
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
fn test_asin_out_of_range() {
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
fn test_acos_out_of_range() {
    let (_, stderr, success) = run_cli(&["acos(2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_acosh_error() {
    let (_, stderr, success) = run_cli(&["acosh(0)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_atanh_error() {
    let (_, stderr, success) = run_cli(&["atanh(2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Invalid Input Tests ====================

#[test]
fn test_empty_expression() {
    let (_, _, success) = run_cli(&[""]);
    assert!(!success);
}

#[test]
fn test_whitespace_only() {
    let (_, _, success) = run_cli(&["   "]);
    assert!(!success);
}

#[test]
fn test_only_operator() {
    let (_, _, success) = run_cli(&["+"]);
    assert!(!success);
}

#[test]
fn test_only_multiplication() {
    let (_, _, success) = run_cli(&["*"]);
    assert!(!success);
}

#[test]
fn test_invalid_number() {
    let (_, stderr, success) = run_cli(&["abc"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Syntax Error Tests ====================

#[test]
fn test_consecutive_operators() {
    let (_, stderr, success) = run_cli(&["sin(1, 2, 3)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_consecutive_multiplication() {
    let (_, _, success) = run_cli(&["1 *** 2"]);
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
    let (_, stderr, success) = run_cli(&["1 + (2"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("unmatched"));
}

#[test]
fn test_extra_close_paren() {
    let (stdout, _, success) = run_cli(&["((1 + 2)) + 3"]);
    assert!(success);
    assert!(stdout.contains("6"));
}

#[test]
fn test_empty_parentheses() {
    let (_, _, success) = run_cli(&["()"]);
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
fn test_function_too_many_args() {
    let (_, stderr, success) = run_cli(&["sin(1, 2, 3)"]);
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
fn test_invalid_function() {
    let (_, stderr, success) = run_cli(&["invalid_func(1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Bitwise Error Tests ====================

#[test]
fn test_bitwise_subtraction_not_supported() {
    let (_, stderr, success) = run_cli(&["-B", "5 - 3"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("subtraction"));
}

#[test]
fn test_bitwise_invalid_shift_amount() {
    let (_, stderr, success) = run_cli(&["-B", "8 << 100"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Factorial Error Tests ====================

#[test]
fn test_factorial_negative() {
    let (_, stderr, success) = run_cli(&["factorial(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_factorial_too_large() {
    let (_, stderr, success) = run_cli(&["factorial(200)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_factorial_non_integer() {
    let (_, stderr, success) = run_cli(&["factorial(3.5)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Value Type Tests ====================

#[test]
fn test_value_from_int() {
    let v = ralqlator::Value::from_int(42);
    assert_eq!(v.to_integer(), Some(42));
    assert!(v.is_exact());
}

#[test]
fn test_value_from_float() {
    let v = ralqlator::Value::from_float(3.14);
    assert_eq!(v.to_float(), 3.14);
    assert!(!v.is_exact());
}

#[test]
fn test_value_add() {
    let a = ralqlator::Value::from_int(1);
    let b = ralqlator::Value::from_int(2);
    assert_eq!(a.add(&b).unwrap(), ralqlator::Value::from_int(3));
}

#[test]
fn test_value_division() {
    let a = ralqlator::Value::from_int(1);
    let b = ralqlator::Value::from_int(3);
    let result = a.div(&b).unwrap();
    assert!(result.to_rational().is_some());
}

#[test]
fn test_value_division_by_zero() {
    let a = ralqlator::Value::from_int(1);
    let b = ralqlator::Value::from_int(0);
    assert!(matches!(a.div(&b), Err(CalcError::DivisionByZero)));
}

#[test]
fn test_value_pow() {
    let base = ralqlator::Value::from_int(2);
    let exp = ralqlator::Value::from_int(10);
    assert_eq!(base.pow(&exp).unwrap(), ralqlator::Value::from_int(1024));
}

// ==================== Tokenize Tests ====================

#[test]
fn test_tokenize_simple() {
    let (stdout, _, success) = run_cli(&["1 + 2"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_tokenize_prefixed_numbers() {
    let (stdout, _, success) = run_cli(&["0xFF + 0b1010"]);
    assert!(success);
    assert!(stdout.contains("265"));
}

#[test]
fn test_tokenize_negative_prefixed() {
    let (stdout, _, success) = run_cli(&["--", "-0xFF"]);
    assert!(success);
    assert!(stdout.contains("-255"));
}

#[test]
fn test_tokenize_binary_negative() {
    let (stdout, _, success) = run_cli(&["--", "-0b1010"]);
    assert!(success);
    assert!(stdout.contains("-10"));
}

#[test]
fn test_tokenize_octal_negative() {
    let (stdout, _, success) = run_cli(&["--", "-0o755"]);
    assert!(success);
    assert!(stdout.contains("-493"));
}
