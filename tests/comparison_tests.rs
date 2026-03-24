// Comparison Operators Tests
// Tests for comparison operators: <, >, =, ==

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

// ==================== Less Than Tests ====================

#[test]
fn test_comparison_less_than_true() {
    let (stdout, _, success) = run_cli(&["3 < 5"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_less_than_false() {
    let (stdout, _, success) = run_cli(&["5 < 3"]);
    assert!(success);
    assert!(stdout.contains("false"));
}

#[test]
fn test_comparison_less_than_equal() {
    let (stdout, _, success) = run_cli(&["5 < 5"]);
    assert!(success);
    assert!(stdout.contains("false"));
}

#[test]
fn test_comparison_less_than_floats() {
    let (stdout, _, success) = run_cli(&["3.14 < 3.15"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_less_than_expression() {
    let (stdout, _, success) = run_cli(&["2 + 3 < 6"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

// ==================== Greater Than Tests ====================

#[test]
fn test_comparison_greater_than_true() {
    let (stdout, _, success) = run_cli(&["5 > 3"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_greater_than_false() {
    let (stdout, _, success) = run_cli(&["3 > 5"]);
    assert!(success);
    assert!(stdout.contains("false"));
}

#[test]
fn test_comparison_greater_than_equal() {
    let (stdout, _, success) = run_cli(&["5 > 5"]);
    assert!(success);
    assert!(stdout.contains("false"));
}

#[test]
fn test_comparison_greater_than_floats() {
    let (stdout, _, success) = run_cli(&["3.15 > 3.14"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_greater_than_expression() {
    let (stdout, _, success) = run_cli(&["2 * 3 > 5"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

// ==================== Single Equal Tests ====================

#[test]
fn test_comparison_single_equal_yes() {
    let (stdout, _, success) = run_cli(&["5 = 5"]);
    assert!(success);
    assert!(stdout.contains("yes"));
}

#[test]
fn test_comparison_single_equal_no() {
    let (stdout, _, success) = run_cli(&["5 = 3"]);
    assert!(success);
    assert!(stdout.contains("no"));
}

#[test]
fn test_comparison_single_equal_floats() {
    let (stdout, _, success) = run_cli(&["3.14 = 3.14"]);
    assert!(success);
    assert!(stdout.contains("yes"));
}

#[test]
fn test_comparison_single_equal_expression() {
    let (stdout, _, success) = run_cli(&["10 / 2 = 5"]);
    assert!(success);
    assert!(stdout.contains("yes"));
}

#[test]
fn test_comparison_single_equal_precedence() {
    let (stdout, _, success) = run_cli(&["10 - 5 = 5"]);
    assert!(success);
    assert!(stdout.contains("yes"));
}

// ==================== Double Equal Tests ====================

#[test]
fn test_comparison_double_equal_true() {
    let (stdout, _, success) = run_cli(&["5 == 5"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_double_equal_false() {
    let (stdout, _, success) = run_cli(&["5 == 3"]);
    assert!(success);
    assert!(stdout.contains("false"));
}

#[test]
fn test_comparison_double_equal_floats() {
    let (stdout, _, success) = run_cli(&["3.14 == 3.14"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_double_equal_expression() {
    let (stdout, _, success) = run_cli(&["2 + 3 == 5"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

// ==================== Comparison Precedence Tests ====================

#[test]
fn test_comparison_precedence_1() {
    let (stdout, _, success) = run_cli(&["2 + 3 > 4 + 1"]);
    assert!(success);
    // (2+3) > (4+1) = 5 > 5 = false
    assert!(stdout.contains("false"));
}

#[test]
fn test_comparison_precedence_2() {
    let (stdout, _, success) = run_cli(&["2 + 3 > 3 + 1"]);
    assert!(success);
    // (2+3) > (3+1) = 5 > 4 = true
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_with_multiplication() {
    let (stdout, _, success) = run_cli(&["2 * 3 < 10"]);
    assert!(success);
    assert!(stdout.contains("true"));
}
