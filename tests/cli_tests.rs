// CLI Arguments Tests
// Tests for command-line arguments and options

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

// ==================== Positional Argument Tests ====================

#[test]
fn test_cli_positional_basic() {
    let (stdout, _, success) = run_cli(&["1 + 2"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_cli_positional_complex() {
    let (stdout, _, success) = run_cli(&["(10 + 5) * 2"]);
    assert!(success);
    assert!(stdout.contains("30"));
}

// ==================== Row Shorthand Tests ====================

#[test]
fn test_cli_row_shorthand() {
    let (stdout, _, success) = run_cli(&["-r", "5 + 5"]);
    assert!(success);
    assert!(stdout.contains("10"));
}

#[test]
fn test_cli_row_complex() {
    let (stdout, _, success) = run_cli(&["-r", "2 ^ 10"]);
    assert!(success);
    assert!(stdout.contains("1024"));
}

// ==================== Hex Output Tests ====================

#[test]
fn test_cli_hex_basic() {
    let (stdout, _, success) = run_cli(&["-x", "255"]);
    assert!(success);
    assert!(stdout.contains("0xFF"));
}

#[test]
fn test_cli_hex_16() {
    let (stdout, _, success) = run_cli(&["-x", "16"]);
    assert!(success);
    assert!(stdout.contains("0x10"));
}

#[test]
fn test_cli_hex_65535() {
    let (stdout, _, success) = run_cli(&["-x", "65535"]);
    assert!(success);
    assert!(stdout.contains("0xFFFF"));
}

#[test]
fn test_cli_hex_zero() {
    let (stdout, _, success) = run_cli(&["-x", "0"]);
    assert!(success);
    assert!(stdout.contains("0x0"));
}

#[test]
fn test_cli_hex_non_integer_error() {
    let (_, stderr, success) = run_cli(&["-x", "10 / 3"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Octal Output Tests ====================

#[test]
fn test_cli_octal_basic() {
    let (stdout, _, success) = run_cli(&["-o", "64"]);
    assert!(success);
    assert!(stdout.contains("0o100"));
}

#[test]
fn test_cli_octal_512() {
    let (stdout, _, success) = run_cli(&["-o", "512"]);
    assert!(success);
    assert!(stdout.contains("0o1000"));
}

#[test]
fn test_cli_octal_zero() {
    let (stdout, _, success) = run_cli(&["-o", "0"]);
    assert!(success);
    assert!(stdout.contains("0o0"));
}

// ==================== Binary Output Tests ====================

#[test]
fn test_cli_binary_basic() {
    let (stdout, _, success) = run_cli(&["-b", "8"]);
    assert!(success);
    assert!(stdout.contains("0b1000"));
}

#[test]
fn test_cli_binary_255() {
    let (stdout, _, success) = run_cli(&["-b", "255"]);
    assert!(success);
    assert!(stdout.contains("0b11111111"));
}

#[test]
fn test_cli_binary_zero() {
    let (stdout, _, success) = run_cli(&["-b", "0"]);
    assert!(success);
    assert!(stdout.contains("0b0"));
}

// ==================== Bitwise Mode Tests ====================

#[test]
fn test_cli_bitwise_flag() {
    let (stdout, _, success) = run_cli(&["-B", "12 & 10"]);
    assert!(success);
    assert!(stdout.contains("8"));
}

#[test]
fn test_cli_bitwise_long_flag() {
    let (stdout, _, success) = run_cli(&["--bits", "8 | 4"]);
    assert!(success);
    assert!(stdout.contains("12"));
}

#[test]
fn test_cli_bitwise_xor() {
    let (stdout, _, success) = run_cli(&["-B", "12 ^ 10"]);
    assert!(success);
    assert!(stdout.contains("6"));
}

#[test]
fn test_cli_bitwise_not() {
    let (stdout, _, success) = run_cli(&["-B", "~0"]);
    assert!(success);
    assert!(stdout.contains("-1"));
}

#[test]
fn test_cli_bitwise_left_shift() {
    let (stdout, _, success) = run_cli(&["-B", "8 << 2"]);
    assert!(success);
    assert!(stdout.contains("32"));
}

#[test]
fn test_cli_bitwise_right_shift() {
    let (stdout, _, success) = run_cli(&["-B", "8 >> 2"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

// ==================== Combined Flags Tests ====================

#[test]
fn test_cli_bitwise_hex() {
    let (stdout, _, success) = run_cli(&["-Bx", "0xFF & 0x0F"]);
    assert!(success);
    assert!(stdout.contains("0xF"));
}

#[test]
fn test_cli_bitwise_binary() {
    let (stdout, _, success) = run_cli(&["-Bb", "8 << 2"]);
    assert!(success);
    assert!(stdout.contains("0b100000"));
}

#[test]
fn test_cli_bitwise_octal() {
    let (stdout, _, success) = run_cli(&["-Bo", "64"]);
    assert!(success);
    assert!(stdout.contains("0o100"));
}

// ==================== Help Flag Tests ====================

#[test]
fn test_cli_help_flag() {
    let (stdout, _, success) = run_cli(&["--help"]);
    assert!(success);
    assert!(stdout.contains("Usage"));
    assert!(stdout.contains("Options"));
}

#[test]
fn test_cli_short_help_flag() {
    let (stdout, _, success) = run_cli(&["-h"]);
    assert!(success);
    assert!(stdout.contains("Usage"));
}

#[test]
fn test_cli_functions_flag() {
    let (stdout, _, success) = run_cli(&["-F"]);
    assert!(success);
    assert!(stdout.contains("lg"));
}

#[test]
fn test_cli_operators_flag() {
    let (stdout, _, success) = run_cli(&["-O"]);
    assert!(success);
    assert!(stdout.contains("Addition"));
}

#[test]
fn test_cli_formats_flag() {
    let (stdout, _, success) = run_cli(&["-N"]);
    assert!(success);
    assert!(stdout.contains("Decimal"));
}

#[test]
fn test_cli_constants_flag() {
    let (stdout, _, success) = run_cli(&["-C"]);
    assert!(success);
    assert!(stdout.contains("C_PI"));
}

// ==================== Subcommand Tests ====================

#[test]
fn test_cli_functions_subcommand() {
    let (stdout, _, success) = run_cli(&["functions"]);
    assert!(success);
    assert!(stdout.contains("lg"));
}

#[test]
fn test_cli_operators_subcommand() {
    let (stdout, _, success) = run_cli(&["operators"]);
    assert!(success);
    assert!(stdout.contains("Addition"));
}

#[test]
fn test_cli_formats_subcommand() {
    let (stdout, _, success) = run_cli(&["formats"]);
    assert!(success);
    assert!(stdout.contains("Decimal"));
}

#[test]
fn test_cli_constants_subcommand() {
    let (stdout, _, success) = run_cli(&["constants"]);
    assert!(success);
    assert!(stdout.contains("C_PI"));
}

#[test]
fn test_cli_info_subcommand() {
    let (stdout, _, success) = run_cli(&["info"]);
    assert!(success);
    assert!(stdout.contains("Operators"));
    assert!(stdout.contains("Functions"));
}

// ==================== Error Handling Tests ====================

#[test]
fn test_cli_invalid_expression() {
    let (_, stderr, success) = run_cli(&["invalid"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_cli_division_by_zero() {
    let (_, stderr, success) = run_cli(&["10 / 0"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}
