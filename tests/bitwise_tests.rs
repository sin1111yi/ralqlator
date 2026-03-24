// Bitwise Operations Tests
// Tests for bitwise operators: &, |, ^, ~, <<, >>

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

// ==================== AND Tests ====================

#[test]
fn test_bitwise_and_basic() {
    let (stdout, _, success) = run_cli(&["-B", "12 & 10"]);
    assert!(success);
    assert!(stdout.contains("8"));
}

#[test]
fn test_bitwise_and_hex() {
    let (stdout, _, success) = run_cli(&["-B", "0xFF & 0x0F"]);
    assert!(success);
    assert!(stdout.contains("15") || stdout.contains("0xF"));
}

#[test]
fn test_bitwise_and_with_zero() {
    let (stdout, _, success) = run_cli(&["-B", "255 & 0"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_bitwise_and_all_ones() {
    let (stdout, _, success) = run_cli(&["-B", "255 & 255"]);
    assert!(success);
    assert!(stdout.contains("255"));
}

// ==================== OR Tests ====================

#[test]
fn test_bitwise_or_basic() {
    let (stdout, _, success) = run_cli(&["-B", "12 | 10"]);
    assert!(success);
    assert!(stdout.contains("14"));
}

#[test]
fn test_bitwise_or_hex() {
    let (stdout, _, success) = run_cli(&["-B", "0xF0 | 0x0F"]);
    assert!(success);
    assert!(stdout.contains("255") || stdout.contains("0xFF"));
}

#[test]
fn test_bitwise_or_with_zero() {
    let (stdout, _, success) = run_cli(&["-B", "128 | 0"]);
    assert!(success);
    assert!(stdout.contains("128"));
}

#[test]
fn test_bitwise_or_all_zeros() {
    let (stdout, _, success) = run_cli(&["-B", "0 | 0"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

// ==================== XOR Tests ====================

#[test]
fn test_bitwise_xor_basic() {
    let (stdout, _, success) = run_cli(&["-B", "12 ^ 10"]);
    assert!(success);
    assert!(stdout.contains("6"));
}

#[test]
fn test_bitwise_xor_hex() {
    let (stdout, _, success) = run_cli(&["-B", "0xFF ^ 0x0F"]);
    assert!(success);
    assert!(stdout.contains("240") || stdout.contains("0xF0"));
}

#[test]
fn test_bitwise_xor_with_self() {
    let (stdout, _, success) = run_cli(&["-B", "42 ^ 42"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_bitwise_xor_with_zero() {
    let (stdout, _, success) = run_cli(&["-B", "42 ^ 0"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

// ==================== NOT Tests ====================

#[test]
fn test_bitwise_not_zero() {
    let (stdout, _, success) = run_cli(&["-B", "~0"]);
    assert!(success);
    assert!(stdout.contains("-1"));
}

#[test]
fn test_bitwise_not_12() {
    let (stdout, _, success) = run_cli(&["-B", "~12"]);
    assert!(success);
    assert!(stdout.contains("-13"));
}

#[test]
fn test_bitwise_not_negative() {
    let (stdout, _, success) = run_cli(&["-B", "~(-1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_bitwise_not_negative_one() {
    let (stdout, _, success) = run_cli(&["-B", "~(-1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

// ==================== Left Shift Tests ====================

#[test]
fn test_bitwise_left_shift_basic() {
    let (stdout, _, success) = run_cli(&["-B", "8 << 2"]);
    assert!(success);
    assert!(stdout.contains("32"));
}

#[test]
fn test_bitwise_left_shift_large() {
    let (stdout, _, success) = run_cli(&["-B", "1 << 10"]);
    assert!(success);
    assert!(stdout.contains("1024"));
}

#[test]
fn test_bitwise_left_shift_by_zero() {
    let (stdout, _, success) = run_cli(&["-B", "42 << 0"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

#[test]
fn test_bitwise_left_shift_invalid_positive() {
    let (_, stderr, success) = run_cli(&["-B", "8 << 64"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_bitwise_left_shift_invalid_negative() {
    let (_, stderr, success) = run_cli(&["-B", "8 << -1"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Right Shift Tests ====================

#[test]
fn test_bitwise_right_shift_basic() {
    let (stdout, _, success) = run_cli(&["-B", "8 >> 2"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_bitwise_right_shift_large() {
    let (stdout, _, success) = run_cli(&["-B", "1024 >> 10"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_bitwise_right_shift_by_zero() {
    let (stdout, _, success) = run_cli(&["-B", "42 >> 0"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

#[test]
fn test_bitwise_right_shift_invalid() {
    let (_, stderr, success) = run_cli(&["-B", "8 >> -1"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Mixed Format Tests ====================

#[test]
fn test_bitwise_mixed_binary_hex() {
    let (stdout, _, success) = run_cli(&["-B", "0b1010 & 0xFF"]);
    assert!(success);
    assert!(stdout.contains("10"));
}

#[test]
fn test_bitwise_mixed_octal_hex() {
    let (stdout, _, success) = run_cli(&["-B", "0o10 | 0x0F"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

// ==================== Complex Expression Tests ====================

#[test]
fn test_bitwise_complex_1() {
    let (stdout, _, success) = run_cli(&["-B", "(0xFF & 0xF0) | (0x0F & 0x0F)"]);
    assert!(success);
    assert!(stdout.contains("255") || stdout.contains("0xFF"));
}

#[test]
fn test_bitwise_complex_2() {
    let (stdout, _, success) = run_cli(&["-B", "((8 | 4) & 12) ^ 0"]);
    assert!(success);
    assert!(stdout.contains("12"));
}

#[test]
fn test_bitwise_complex_3() {
    let (stdout, _, success) = run_cli(&["-B", "(12 & 10) | (8 ^ 4)"]);
    assert!(success);
    assert!(stdout.contains("12"));
}

// ==================== Negative Number Tests ====================

#[test]
fn test_bitwise_negative_unary() {
    let (stdout, _, success) = run_cli(&["-B", "--", "-1 & 255"]);
    assert!(success);
    assert!(stdout.contains("255"));
}

#[test]
fn test_bitwise_negative_unary_2() {
    let (stdout, _, success) = run_cli(&["-B", "--", "-255 & 255"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

// ==================== Subtraction Error Test ====================

#[test]
fn test_bitwise_subtraction_not_supported() {
    let (_, stderr, success) = run_cli(&["-B", "10 - 3"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
    assert!(stderr.contains("subtraction"));
}

// ==================== Combined Flags Tests ====================

#[test]
fn test_bitwise_with_hex_output() {
    let (stdout, _, success) = run_cli(&["-Bx", "0xFF & 0x0F"]);
    assert!(success);
    assert!(stdout.contains("0xF"));
}

#[test]
fn test_bitwise_with_binary_output() {
    let (stdout, _, success) = run_cli(&["-Bb", "8 << 2"]);
    assert!(success);
    assert!(stdout.contains("0b100000"));
}

#[test]
fn test_bitwise_with_octal_output() {
    let (stdout, _, success) = run_cli(&["-Bo", "64"]);
    assert!(success);
    assert!(stdout.contains("0o100"));
}

// ==================== Precedence Tests ====================

#[test]
fn test_bitwise_precedence_and_or() {
    let (stdout, _, success) = run_cli(&["-B", "12 | 10 & 8"]);
    assert!(success);
    // & has higher precedence: 12 | (10 & 8) = 12 | 8 = 12
    assert!(stdout.contains("12"));
}

#[test]
fn test_bitwise_precedence_shift_and() {
    let (stdout, _, success) = run_cli(&["-B", "8 << 2 & 15"]);
    assert!(success);
    // << has higher precedence: (8 << 2) & 15 = 32 & 15 = 0
    assert!(stdout.contains("0"));
}
