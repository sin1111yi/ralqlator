// Internal Unit Tests
// Tests for internal modules via CLI integration tests

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

// ==================== Tokenizer Tests ====================

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
fn test_tokenize_negative_prefixed_error() {
    let (_, stderr, success) = run_cli(&["--", "-0xFF"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
    assert!(stderr.contains("Negative non-decimal numbers are not supported"));
}

#[test]
fn test_tokenize_binary_negative_error() {
    let (_, stderr, success) = run_cli(&["--", "-0b1010"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_tokenize_octal_negative_error() {
    let (_, stderr, success) = run_cli(&["--", "-0o755"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Constant Identifier Tests ====================

#[test]
fn test_c_pi_constant() {
    let (stdout, _, success) = run_cli(&["C_PI"]);
    assert!(success);
    assert!(stdout.contains("3.14159"));
}

#[test]
fn test_c_e_constant() {
    let (stdout, _, success) = run_cli(&["C_E"]);
    assert!(success);
    assert!(stdout.contains("2.71828"));
}

#[test]
fn test_c_pi_expression() {
    let (stdout, _, success) = run_cli(&["C_PI * 2"]);
    assert!(success);
    assert!(stdout.contains("6.28318"));
}

// ==================== Comparison Result Tests ====================

#[test]
fn test_comparison_less_than() {
    let (stdout, _, success) = run_cli(&["3 < 5"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_greater_than() {
    let (stdout, _, success) = run_cli(&["5 > 3"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_equal_yes() {
    let (stdout, _, success) = run_cli(&["5 = 5"]);
    assert!(success);
    assert!(stdout.contains("yes"));
}

#[test]
fn test_comparison_double_equals_true() {
    let (stdout, _, success) = run_cli(&["5 == 5"]);
    assert!(success);
    assert!(stdout.contains("true"));
}
