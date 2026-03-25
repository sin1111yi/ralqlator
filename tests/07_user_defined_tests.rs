// Ralqlator - User Defined and Constants Tests
// Combines: user_defined_tests, constants_tests

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

// ==================== C_PI Tests ====================

#[test]
fn test_c_pi_constant() {
    let (stdout, _, success) = run_cli(&["C_PI"]);
    assert!(success);
    assert!(stdout.contains("3.14"));
}

#[test]
fn test_c_pi_expression() {
    let (stdout, _, success) = run_cli(&["C_PI * 2"]);
    assert!(success);
    assert!(stdout.contains("6.28"));
}

#[test]
fn test_c_pi_with_function() {
    let (stdout, _, success) = run_cli(&["sin(C_PI / 2)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

// ==================== C_E Tests ====================

#[test]
fn test_c_e_constant() {
    let (stdout, _, success) = run_cli(&["C_E"]);
    assert!(success);
    assert!(stdout.contains("2.718"));
}

#[test]
fn test_c_e_expression() {
    let (stdout, _, success) = run_cli(&["C_E ^ 2"]);
    assert!(success);
    assert!(stdout.contains("7.38"));
}

#[test]
fn test_c_e_with_ln() {
    let (stdout, _, success) = run_cli(&["ln(C_E)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

// ==================== Constants in Expressions ====================

#[test]
fn test_constants_in_complex_expression() {
    let (stdout, _, success) = run_cli(&["C_PI * C_E"]);
    assert!(success);
    assert!(stdout.contains("8.5"));
}

#[test]
fn test_constants_with_comparison() {
    let (stdout, _, success) = run_cli(&["C_PI > 3"]);
    assert!(success);
    assert!(stdout.contains("true"));
}
