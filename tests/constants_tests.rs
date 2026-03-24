// Constants Tests
// Tests for mathematical constants: C_PI, C_E

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
    assert!(stdout.contains("3.14159"));
}

#[test]
fn test_c_pi_lowercase() {
    let (stdout, _, success) = run_cli(&["C_pi"]);
    assert!(success);
    assert!(stdout.contains("3.14159"));
}

#[test]
fn test_c_pi_expression() {
    let (stdout, _, success) = run_cli(&["C_PI * 2"]);
    assert!(success);
    assert!(stdout.contains("6.28318"));
}

#[test]
fn test_c_pi_trig() {
    let (stdout, _, success) = run_cli(&["sin(C_PI / 2)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_c_pi_trig_cos() {
    let (stdout, _, success) = run_cli(&["cos(C_PI)"]);
    assert!(success);
    assert!(stdout.contains("-1"));
}

// ==================== C_E Tests ====================

#[test]
fn test_c_e_constant() {
    let (stdout, _, success) = run_cli(&["C_E"]);
    assert!(success);
    assert!(stdout.contains("2.71828"));
}

#[test]
fn test_c_e_lowercase() {
    let (stdout, _, success) = run_cli(&["C_e"]);
    assert!(success);
    assert!(stdout.contains("2.71828"));
}

#[test]
fn test_c_e_ln() {
    let (stdout, _, success) = run_cli(&["ln(C_E)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_c_e_expression() {
    let (stdout, _, success) = run_cli(&["C_E ^ 2"]);
    assert!(success);
    assert!(stdout.contains("7.389"));
}

// ==================== Constants in Expressions ====================

#[test]
fn test_constants_mixed() {
    let (stdout, _, success) = run_cli(&["C_PI + C_E"]);
    assert!(success);
    assert!(stdout.contains("5.8598"));
}

#[test]
fn test_constants_in_function() {
    let (stdout, _, success) = run_cli(&["pow(C_PI, 2)"]);
    assert!(success);
    assert!(stdout.contains("9.8696"));
}

#[test]
fn test_log_with_pi() {
    let (stdout, _, success) = run_cli(&["lg(C_PI * 10)"]);
    assert!(success);
    assert!(stdout.contains("1.497"));
}
