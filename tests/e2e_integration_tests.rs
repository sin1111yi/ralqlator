// Ralqlator - End-to-End Integration Tests
// Comprehensive tests that verify the complete system behavior

use std::process::Command;
use std::io::{Read, Write};
use std::process::{Command as ProcCommand, Stdio};

/// Run CLI command and capture output
fn run_cli(args: &[&str]) -> (String, String, bool) {
    let output = Command::new("cargo")
        .args(["run", "--release", "--quiet", "--"])
        .args(args)
        .output()
        .expect("Failed to execute command");
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.success())
}

/// Run REPL commands and capture output
fn run_repl_commands(commands: &[&str]) -> String {
    let mut child = ProcCommand::new("cargo")
        .args(&["run", "--release", "--quiet", "--"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start REPL");

    {
        let stdin = child.stdin.as_mut().unwrap();
        for cmd in commands {
            writeln!(stdin, "{}", cmd).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        writeln!(stdin, "quit").unwrap();
    }

    let mut output = String::new();
    child.stdout.as_mut().unwrap().read_to_string(&mut output).unwrap();
    child.wait().unwrap();
    output
}

// ==================== Complete Workflow Tests ====================

#[test]
fn test_complete_arithmetic_workflow() {
    // Test a complete calculation workflow
    let (stdout, _, success) = run_cli(&["1 + 2 * 3"]);
    assert!(success);
    assert!(stdout.contains("7"));

    let (stdout, _, success) = run_cli(&["(1 + 2) * 3"]);
    assert!(success);
    assert!(stdout.contains("9"));

    let (stdout, _, success) = run_cli(&["100 / 4 + 25"]);
    assert!(success);
    assert!(stdout.contains("50"));
}

#[test]
fn test_complete_function_workflow() {
    // Test function calculations
    let (stdout, _, success) = run_cli(&["sin(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));

    let (stdout, _, success) = run_cli(&["cos(0)"]);
    assert!(success);
    assert!(stdout.contains("1"));

    let (stdout, _, success) = run_cli(&["sqrt(144)"]);
    assert!(success);
    assert!(stdout.contains("12"));

    let (stdout, _, success) = run_cli(&["lg(1000)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_complete_bitwise_workflow() {
    // Test bitwise operations
    let (stdout, _, success) = run_cli(&["-B", "0xFF & 0x0F"]);
    assert!(success);
    assert!(stdout.contains("15"));

    let (stdout, _, success) = run_cli(&["-B", "0b1010 | 0b0101"]);
    assert!(success);
    assert!(stdout.contains("15"));

    let (stdout, _, success) = run_cli(&["-B", "8 << 2"]);
    assert!(success);
    assert!(stdout.contains("32"));
}

#[test]
fn test_complete_format_workflow() {
    // Test format conversions
    let (stdout, _, success) = run_cli(&["-x", "255"]);
    assert!(success);
    assert!(stdout.contains("0xFF"));

    let (stdout, _, success) = run_cli(&["-o", "64"]);
    assert!(success);
    assert!(stdout.contains("0o100"));

    let (stdout, _, success) = run_cli(&["-b", "15"]);
    assert!(success);
    assert!(stdout.contains("0b1111"));
}

#[test]
fn test_complete_bigint_workflow() {
    // Test BigInt calculations
    let (stdout, _, success) = run_cli(&["gcd(48, 18)"]);
    assert!(success);
    assert!(stdout.contains("6"));

    let (stdout, _, success) = run_cli(&["lcm(12, 18)"]);
    assert!(success);
    assert!(stdout.contains("36"));

    let (stdout, _, success) = run_cli(&["isprime(97)"]);
    assert!(success);
    assert!(stdout.contains("1"));

    let (stdout, _, success) = run_cli(&["nextprime(100)"]);
    assert!(success);
    assert!(stdout.contains("101"));
}

#[test]
fn test_complete_comparison_workflow() {
    // Test comparison operations
    let (stdout, _, success) = run_cli(&["5 > 3"]);
    assert!(success);
    assert!(stdout.contains("true"));

    let (stdout, _, success) = run_cli(&["5 < 3"]);
    assert!(success);
    assert!(stdout.contains("false"));

    let (stdout, _, success) = run_cli(&["5 = 5"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

// ==================== REPL Integration Tests ====================

#[test]
fn test_repl_complete_session() {
    // Test a complete REPL session
    let output = run_repl_commands(&[
        "1 + 2",
        "@ * 3",
        "hex",
        "oct",
        "bin",
    ]);
    
    assert!(output.contains("3"));
    assert!(output.contains("9"));
    assert!(output.contains("0x"));
    assert!(output.contains("0o"));
    assert!(output.contains("0b"));
}

#[test]
fn test_repl_function_definition() {
    // Test function definition and usage
    let output = run_repl_commands(&[
        "create func square(x) = x * x",
        "square(5)",
        "square(10)",
    ]);
    
    assert!(output.contains("25"));
    assert!(output.contains("100"));
}

#[test]
fn test_repl_constant_definition() {
    // Test constant definition and usage
    let output = run_repl_commands(&[
        "create const G 9.81",
        "G * 10",
        "G * 20",
    ]);
    
    assert!(output.contains("98.1"));
    assert!(output.contains("196.2"));
}

#[test]
fn test_repl_mode_switching() {
    // Test mode switching
    let output = run_repl_commands(&[
        "10 + 5",
        "mode bitwise",
        "10 & 5",
        "mode standard",
        "10 + 5",
    ]);
    
    assert!(output.contains("15"));
    assert!(output.contains("0"));
}

#[test]
fn test_repl_help_system() {
    // Test help system
    let output = run_repl_commands(&[
        "help",
        "help functions",
        "help operators",
    ]);
    
    assert!(output.contains("help") || output.contains("Commands"));
    assert!(output.contains("Function") || output.contains("lg"));
    assert!(output.contains("Operator") || output.contains("+"));
}

// ==================== Error Handling Integration Tests ====================

#[test]
fn test_error_division_by_zero() {
    let (_, stderr, success) = run_cli(&["1 / 0"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("zero"));
}

#[test]
fn test_error_invalid_expression() {
    let (_, stderr, success) = run_cli(&["sin()"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_error_undefined_function() {
    let (_, stderr, success) = run_cli(&["undefined_func(1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_error_domain_error() {
    let (_, stderr, success) = run_cli(&["sqrt(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Performance Integration Tests ====================

#[test]
fn test_performance_large_calculation() {
    // Test performance with large calculations
    let (stdout, _, success) = run_cli(&["bfactorial(100)"]);
    assert!(success);
    assert!(stdout.len() > 100);
}

#[test]
fn test_performance_complex_expression() {
    // Test complex expression evaluation
    let (stdout, _, success) = run_cli(&[
        "sin(C_PI / 4) + cos(C_PI / 4) + tan(C_PI / 4)"
    ]);
    assert!(success);
    assert!(stdout.len() > 0);
}

#[test]
fn test_performance_nested_functions() {
    // Test nested function calls
    let (stdout, _, success) = run_cli(&[
        "sqrt(pow(sin(1), 2) + pow(cos(1), 2))"
    ]);
    assert!(success);
    assert!(stdout.len() > 0);
}

// ==================== Edge Case Integration Tests ====================

#[test]
fn test_edge_very_long_expression() {
    // Test very long expression
    let expr = "1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10";
    let (stdout, _, success) = run_cli(&[expr]);
    assert!(success);
    assert!(stdout.contains("55"));
}

#[test]
fn test_edge_deeply_nested_parens() {
    // Test deeply nested parentheses
    let (stdout, _, success) = run_cli(&["((((1 + 2))))"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_edge_mixed_formats() {
    // Test mixed number formats: 10 + 0xA(10) + 0o10(8) + 0b10(2) = 30
    let (stdout, _, success) = run_cli(&["10 + 0xA + 0o10 + 0b10"]);
    assert!(success);
    assert!(stdout.contains("30"));
}

#[test]
fn test_edge_scientific_notation() {
    // Test scientific notation
    let (stdout, _, success) = run_cli(&["1e3 + 2.5e2"]);
    assert!(success);
    assert!(stdout.contains("1250"));
}

#[test]
fn test_edge_constants_in_functions() {
    // Test constants in functions
    let (stdout, _, success) = run_cli(&["sin(C_PI / 2)"]);
    assert!(success);
    assert!(stdout.contains("1"));

    let (stdout, _, success) = run_cli(&["ln(C_E)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

// ==================== Help System Integration Tests ====================

#[test]
fn test_help_command_line() {
    let (stdout, _, success) = run_cli(&["--help"]);
    assert!(success);
    assert!(stdout.contains("Usage"));
    assert!(stdout.contains("ralqlator"));
}

#[test]
fn test_help_functions_flag() {
    let (stdout, _, success) = run_cli(&["-F"]);
    assert!(success);
    assert!(stdout.contains("Function"));
}

#[test]
fn test_help_operators_flag() {
    let (stdout, _, success) = run_cli(&["-O"]);
    assert!(success);
    assert!(stdout.contains("Operator"));
}

#[test]
fn test_help_formats_flag() {
    let (stdout, _, success) = run_cli(&["-N"]);
    assert!(success);
    assert!(stdout.contains("Format"));
}

#[test]
fn test_help_constants_flag() {
    let (stdout, _, success) = run_cli(&["-C"]);
    assert!(success);
    assert!(stdout.contains("Constant"));
}

#[test]
fn test_help_subcommands() {
    let (stdout, _, success) = run_cli(&["functions"]);
    assert!(success);
    assert!(stdout.contains("Function"));

    let (stdout, _, success) = run_cli(&["operators"]);
    assert!(success);
    assert!(stdout.contains("Operator"));

    let (stdout, _, success) = run_cli(&["info"]);
    assert!(success);
    assert!(stdout.contains("Operator") || stdout.contains("Function"));
}

// ==================== Regression Tests ====================

#[test]
fn test_regression_issue_factorial_zero() {
    // Ensure 0! = 1
    let (stdout, _, success) = run_cli(&["0!"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_regression_issue_power_zero() {
    // Ensure x^0 = 1
    let (stdout, _, success) = run_cli(&["5 ^ 0"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_regression_issue_negative_times_negative() {
    // Ensure negative * negative = positive
    let (stdout, _, success) = run_cli(&["--", "-3 * -4"]);
    assert!(success);
    assert!(stdout.contains("12"));
}

#[test]
fn test_regression_issue_division_precision() {
    // Ensure division precision
    let (stdout, _, success) = run_cli(&["1 / 3"]);
    assert!(success);
    assert!(stdout.contains("0.333"));
}

#[test]
fn test_regression_issue_operator_precedence() {
    // Ensure correct operator precedence
    let (stdout, _, success) = run_cli(&["2 + 3 * 4"]);
    assert!(success);
    assert!(stdout.contains("14"));
}
