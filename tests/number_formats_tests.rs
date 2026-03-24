// Number Formats Tests
// Tests for number format input/output: decimal, binary, octal, hexadecimal, scientific notation

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

// ==================== Binary Input Tests ====================

#[test]
fn test_binary_input_basic() {
    let (stdout, _, success) = run_cli(&["0b1010"]);
    assert!(success);
    assert!(stdout.contains("10"));
}

#[test]
fn test_binary_input_255() {
    let (stdout, _, success) = run_cli(&["0b11111111"]);
    assert!(success);
    assert!(stdout.contains("255"));
}

#[test]
fn test_binary_input_with_operation() {
    let (stdout, _, success) = run_cli(&["0b1010 + 0b0101"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_binary_negative_error() {
    let (_, stderr, success) = run_cli(&["--", "-0b1010"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Octal Input Tests ====================

#[test]
fn test_octal_input_basic() {
    let (stdout, _, success) = run_cli(&["0o755"]);
    assert!(success);
    assert!(stdout.contains("493"));
}

#[test]
fn test_octal_input_10() {
    let (stdout, _, success) = run_cli(&["0o10"]);
    assert!(success);
    assert!(stdout.contains("8"));
}

#[test]
fn test_octal_input_377() {
    let (stdout, _, success) = run_cli(&["0o377"]);
    assert!(success);
    assert!(stdout.contains("255"));
}

#[test]
fn test_octal_negative_error() {
    let (_, stderr, success) = run_cli(&["--", "-0o755"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Hexadecimal Input Tests ====================

#[test]
fn test_hex_input_basic() {
    let (stdout, _, success) = run_cli(&["0xFF"]);
    assert!(success);
    assert!(stdout.contains("255"));
}

#[test]
fn test_hex_input_10() {
    let (stdout, _, success) = run_cli(&["0x10"]);
    assert!(success);
    assert!(stdout.contains("16"));
}

#[test]
fn test_hex_input_1a() {
    let (stdout, _, success) = run_cli(&["0x1A"]);
    assert!(success);
    assert!(stdout.contains("26"));
}

#[test]
fn test_hex_negative_error() {
    let (_, stderr, success) = run_cli(&["--", "-0xFF"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Mixed Format Tests ====================

#[test]
fn test_mixed_decimal_hex() {
    let (stdout, _, success) = run_cli(&["10 + 0x10"]);
    assert!(success);
    assert!(stdout.contains("26"));
}

#[test]
fn test_mixed_binary_octal() {
    let (stdout, _, success) = run_cli(&["0b1010 + 0o10"]);
    assert!(success);
    assert!(stdout.contains("18"));
}

#[test]
fn test_mixed_all_formats() {
    let (stdout, _, success) = run_cli(&["10 + 0b10 + 0o10 + 0x10"]);
    assert!(success);
    // 10 + 2 + 8 + 16 = 36
    assert!(stdout.contains("36"));
}

// ==================== Scientific Notation Tests ====================

#[test]
fn test_scientific_1e3() {
    let (stdout, _, success) = run_cli(&["1e3"]);
    assert!(success);
    assert!(stdout.contains("1000"));
}

#[test]
fn test_scientific_negative_exponent() {
    let (stdout, _, success) = run_cli(&["2.5e-3"]);
    assert!(success);
    assert!(stdout.contains("0.0025"));
}

#[test]
fn test_scientific_positive_exponent() {
    let (stdout, _, success) = run_cli(&["1.23E+10"]);
    assert!(success);
    assert!(stdout.contains("12300000000"));
}

#[test]
fn test_scientific_addition() {
    let (stdout, _, success) = run_cli(&["1e3 + 1e3"]);
    assert!(success);
    assert!(stdout.contains("2000"));
}

#[test]
fn test_scientific_multiplication() {
    let (stdout, _, success) = run_cli(&["1e3 * 1e2"]);
    assert!(success);
    assert!(stdout.contains("100000"));
}

#[test]
fn test_scientific_division() {
    let (stdout, _, success) = run_cli(&["1e6 / 1e3"]);
    assert!(success);
    assert!(stdout.contains("1000"));
}

#[test]
fn test_scientific_with_decimal() {
    let (stdout, _, success) = run_cli(&["2.5e2"]);
    assert!(success);
    assert!(stdout.contains("250"));
}

#[test]
fn test_scientific_uppercase_e() {
    let (stdout, _, success) = run_cli(&["3E2"]);
    assert!(success);
    assert!(stdout.contains("300"));
}

// ==================== Scientific Notation Output Tests ====================

#[test]
fn test_scientific_input_shows_scientific_output() {
    let (stdout, _, success) = run_cli(&["1e3 * 2"]);
    assert!(success);
    assert!(stdout.contains("2000"));
    assert!(stdout.contains("scientific"));
}

#[test]
fn test_scientific_negative_exponent_output() {
    let (stdout, _, success) = run_cli(&["2.5e-3 * 2"]);
    assert!(success);
    assert!(stdout.contains("scientific"));
}

#[test]
fn test_regular_input_no_scientific_output() {
    let (stdout, _, success) = run_cli(&["1000 + 2000"]);
    assert!(success);
    assert!(stdout.contains("3000"));
    assert!(!stdout.contains("scientific"));
}

#[test]
fn test_uppercase_e_scientific_output() {
    let (stdout, _, success) = run_cli(&["1E3 * 2"]);
    assert!(success);
    assert!(stdout.contains("scientific"));
}

// ==================== Output Format Tests ====================

#[test]
fn test_hex_output() {
    let (stdout, _, success) = run_cli(&["-x", "255"]);
    assert!(success);
    assert!(stdout.contains("0xFF"));
}

#[test]
fn test_hex_output_16() {
    let (stdout, _, success) = run_cli(&["-x", "16"]);
    assert!(success);
    assert!(stdout.contains("0x10"));
}

#[test]
fn test_octal_output() {
    let (stdout, _, success) = run_cli(&["-o", "64"]);
    assert!(success);
    assert!(stdout.contains("0o100"));
}

#[test]
fn test_binary_output() {
    let (stdout, _, success) = run_cli(&["-b", "8"]);
    assert!(success);
    assert!(stdout.contains("0b1000"));
}

#[test]
fn test_binary_output_255() {
    let (stdout, _, success) = run_cli(&["-b", "255"]);
    assert!(success);
    assert!(stdout.contains("0b11111111"));
}

// ==================== Large/Small Number Tests ====================

#[test]
fn test_very_large_numbers() {
    let (stdout, _, success) = run_cli(&["1e100 * 1e100"]);
    assert!(success);
    assert!(stdout.contains("100000") || stdout.contains("1e200"));
}

#[test]
fn test_very_small_numbers() {
    let (stdout, _, success) = run_cli(&["1e-100 * 1e-100"]);
    assert!(success);
    assert!(stdout.contains("1e-200") || stdout.contains("0"));
}
