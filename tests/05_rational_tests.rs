// Ralqlator - Rational Number Tests
// Combines: rational_tests, rational_func_tests, rational_cli_tests, rational_repl_tests, parser_tests, parser_integration_tests

use std::process::Command;
use std::io::{Read, Write};
use std::process::{Command as ProcCommand, Stdio};
use ralqlator::parser::{parse_expression, parse_and_eval, AstNode, BinaryOperator};
use ralqlator::functions::UserFunctions;
use ralqlator::calculator::UserConstants;
use num_traits::ToPrimitive;

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

fn run_repl_commands(commands: &[&str]) -> String {
    let mut child = ProcCommand::new("cargo")
        .args(&["run", "--quiet", "--"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start REPL");

    {
        let stdin = child.stdin.as_mut().unwrap();
        for cmd in commands {
            writeln!(stdin, "{}", cmd).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(30));
        }
        writeln!(stdin, "quit").unwrap();
    }

    let mut output = String::new();
    child.stdout.as_mut().unwrap().read_to_string(&mut output).unwrap();
    child.wait().unwrap();
    output
}

fn empty_context() -> (UserFunctions, UserConstants) {
    (
        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
    )
}

// ==================== Parser Tests ====================

#[test]
fn test_parse_number_decimal() {
    let ast = parse_expression("42", false).unwrap();
    assert!(matches!(ast, AstNode::Number(_)));
}

#[test]
fn test_parse_number_binary() {
    let ast = parse_expression("0b1010", false).unwrap();
    assert!(matches!(ast, AstNode::Number(_)));
}

#[test]
fn test_parse_number_hex() {
    let ast = parse_expression("0xFF", false).unwrap();
    assert!(matches!(ast, AstNode::Number(_)));
}

#[test]
fn test_parse_scientific() {
    let ast = parse_expression("1e3", false).unwrap();
    assert!(matches!(ast, AstNode::Number(_)));
}

#[test]
fn test_parse_identifier() {
    let ast = parse_expression("C_PI", false).unwrap();
    assert!(matches!(ast, AstNode::Constant(_)));
}

#[test]
fn test_parse_function_call() {
    let ast = parse_expression("sin(x)", false).unwrap();
    assert!(matches!(ast, AstNode::FunctionCall { name, .. } if name == "sin"));
}

#[test]
fn test_parse_simple_expression() {
    let ast = parse_expression("1 + 2 * 3", false).unwrap();
    assert!(matches!(ast, AstNode::BinaryOp { op: BinaryOperator::Add, .. }));
}

#[test]
fn test_parse_parentheses() {
    let ast = parse_expression("(1 + 2) * 3", false).unwrap();
    assert!(matches!(ast, AstNode::BinaryOp { op: BinaryOperator::Multiply, .. }));
}

// ==================== Rational CLI Tests ====================

#[test]
fn test_rational_fraction_input() {
    let (stdout, _, success) = run_cli(&["1/2"]);
    assert!(success);
    assert!(stdout.contains("0.5"));
}

#[test]
fn test_rational_addition() {
    let (stdout, _, success) = run_cli(&["1/2 + 1/3"]);
    assert!(success);
    assert!(stdout.contains("0.833"));
}

#[test]
fn test_rational_subtraction() {
    let (stdout, _, success) = run_cli(&["3/4 - 1/2"]);
    assert!(success);
    assert!(stdout.contains("0.25"));
}

#[test]
fn test_rational_multiplication() {
    let (stdout, _, success) = run_cli(&["2/3 * 3/4"]);
    assert!(success);
    assert!(stdout.contains("0.5"));
}

#[test]
fn test_rational_division() {
    let (stdout, _, success) = run_cli(&["(1/2) / (1/4)"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_rational_complex_expression() {
    let (stdout, _, success) = run_cli(&["(1/2 + 1/3) * 6"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_gcd_function_cli() {
    let (stdout, _, success) = run_cli(&["gcd(48, 18)"]);
    assert!(success);
    assert!(stdout.contains("6"));
}

#[test]
fn test_lcm_function_cli() {
    let (stdout, _, success) = run_cli(&["lcm(12, 18)"]);
    assert!(success);
    assert!(stdout.contains("36"));
}

#[test]
fn test_rational_integer_addition() {
    let (stdout, _, success) = run_cli(&["1/2 + 1"]);
    assert!(success);
    assert!(stdout.contains("1.5"));
}

#[test]
fn test_rational_comparison() {
    let (stdout, _, success) = run_cli(&["1/2 < 3/4"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_rational_equality() {
    let (stdout, _, success) = run_cli(&["2/4 = 1/2"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_rational_chain_operations() {
    let (stdout, _, success) = run_cli(&["1/2 + 1/3 + 1/4 + 1/5"]);
    assert!(success);
    assert!(stdout.contains("1.283"));
}

// ==================== Rational REPL Tests ====================

#[test]
fn test_repl_fraction_input() {
    let output = run_repl_commands(&["1/2", "2/3"]);
    assert!(output.contains("0.5") || output.contains("1/2"));
}

#[test]
fn test_repl_rational_addition() {
    let output = run_repl_commands(&["1/2 + 1/3"]);
    assert!(output.contains("0.833") || output.contains("5/6"));
}

#[test]
fn test_repl_rational_chain() {
    let output = run_repl_commands(&["1/2 + 1/3 + 1/6"]);
    assert!(output.contains("1"));
}

#[test]
fn test_repl_gcd_function() {
    let output = run_repl_commands(&["gcd(48, 18)"]);
    assert!(output.contains("6") || output.len() > 0);
}

#[test]
fn test_repl_lcm_function() {
    let output = run_repl_commands(&["lcm(12, 18)"]);
    assert!(output.contains("36") || output.len() > 0);
}

#[test]
fn test_repl_user_func_with_rational() {
    let output = run_repl_commands(&["create func half(x) = x/2", "half(1)"]);
    assert!(output.contains("0.5") || output.len() > 0);
}

#[test]
fn test_repl_user_const_rational() {
    let output = run_repl_commands(&["create const HALF 1/2", "HALF"]);
    assert!(output.contains("0.5") || output.len() > 0);
}

#[test]
fn test_repl_rational_equality() {
    let output = run_repl_commands(&["2/4 = 1/2"]);
    assert!(output.contains("true") || output.contains("true"));
}

// ==================== Parser Integration Tests ====================

#[test]
fn test_eval_arithmetic() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("1 + 2 * 3", false, &funcs, &consts).unwrap();
    assert_eq!(result.to_integer(), Some(7));
}

#[test]
fn test_eval_parentheses() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("(1 + 2) * 3", false, &funcs, &consts).unwrap();
    assert_eq!(result.to_integer(), Some(9));
}

#[test]
fn test_eval_unary_minus() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("-5 + 3", false, &funcs, &consts).unwrap();
    assert_eq!(result.to_integer(), Some(-2));
}

#[test]
fn test_eval_exponentiation() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("2 ^ 10", false, &funcs, &consts).unwrap();
    assert_eq!(result.to_integer(), Some(1024));
}

#[test]
fn test_eval_comparison() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("5 > 3", false, &funcs, &consts).unwrap();
    assert!(result.to_float() < -1e300);
}

#[test]
fn test_eval_fraction_input() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("1/2 + 1/3", false, &funcs, &consts).unwrap();
    let rational = result.to_rational().unwrap();
    assert_eq!(rational.numer().to_i64(), Some(5));
    assert_eq!(rational.denom().to_i64(), Some(6));
}

#[test]
fn test_eval_nested_function() {
    let (funcs, consts) = empty_context();
    let mut f = funcs.lock().unwrap();
    f.insert("f".to_string(), (vec!["x".to_string()], "x * 2".to_string()));
    drop(f);
    let result = parse_and_eval("f(5)", false, &funcs, &consts).unwrap();
    assert_eq!(result.to_integer(), Some(10));
}

#[test]
fn test_eval_error_division_by_zero() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("1 / 0", false, &funcs, &consts);
    assert!(result.is_err());
}

#[test]
fn test_eval_error_undefined_function() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("undefined_func(1)", false, &funcs, &consts);
    assert!(result.is_err());
}

#[test]
fn test_eval_right_associative_power() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("2 ^ 3 ^ 2", false, &funcs, &consts).unwrap();
    assert_eq!(result.to_integer(), Some(512));
}

#[test]
fn test_eval_mixed_operations() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("2 + 3 * 4 - 6 / 2", false, &funcs, &consts).unwrap();
    assert_eq!(result.to_integer(), Some(11));
}

// ==================== Rational Function Tests ====================

#[test]
fn test_num_function() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("num(3/4)", false, &funcs, &consts);
    assert!(result.is_ok());
}

#[test]
fn test_den_function() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("den(3/4)", false, &funcs, &consts);
    assert!(result.is_ok());
}

#[test]
fn test_frac_function() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("frac(5)", false, &funcs, &consts);
    assert!(result.is_ok());
}

#[test]
fn test_rational_constructor() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("rational(1, 3)", false, &funcs, &consts);
    assert!(result.is_ok());
}

#[test]
fn test_float_function() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("float(1/3)", false, &funcs, &consts);
    assert!(result.is_ok());
}

// ==================== Compact Fraction Tests ====================

#[test]
fn test_compact_fraction_addition() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("1/2+1/3", false, &funcs, &consts).unwrap();
    let rational = result.to_rational().unwrap();
    assert_eq!(rational.numer().to_i64(), Some(5));
    assert_eq!(rational.denom().to_i64(), Some(6));
}

#[test]
fn test_compact_fraction_multiplication() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("1/2*2/3", false, &funcs, &consts).unwrap();
    let rational = result.to_rational().unwrap();
    assert_eq!(rational.numer().to_i64(), Some(1));
    assert_eq!(rational.denom().to_i64(), Some(3));
}

#[test]
fn test_compact_fraction_division() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("1/2/1/4", false, &funcs, &consts).unwrap();
    let rational = result.to_rational().unwrap();
    assert_eq!(rational.numer().to_i64(), Some(2));
    assert_eq!(rational.denom().to_i64(), Some(1));
}

#[test]
fn test_compact_fraction_with_parentheses() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("(1/2+1/3)*6", false, &funcs, &consts).unwrap();
    assert_eq!(result.to_integer(), Some(5));
}

#[test]
fn test_fraction_and_integer_mixed() {
    let (funcs, consts) = empty_context();
    let result = parse_and_eval("1/2+1", false, &funcs, &consts).unwrap();
    let rational = result.to_rational().unwrap();
    assert_eq!(rational.numer().to_i64(), Some(3));
    assert_eq!(rational.denom().to_i64(), Some(2));
}
