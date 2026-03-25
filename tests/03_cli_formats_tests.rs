// Ralqlator - CLI and Number Format Tests
// Combines: cli_tests, number_formats_tests, comprehensive_cli_tests (CLI parts)

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
fn test_positional_expression() {
    let (stdout, _, success) = run_cli(&["1 + 2"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_row_shorthand() {
    let (stdout, _, success) = run_cli(&["-r", "3 * 4"]);
    assert!(success);
    assert!(stdout.contains("12"));
}

// ==================== Hex Output Tests ====================

#[test]
fn test_hex_output() {
    let (stdout, _, success) = run_cli(&["-x", "255"]);
    assert!(success);
    assert_eq!(stdout.trim(), "0xFF");
}

#[test]
fn test_hex_output_with_expression() {
    let (stdout, _, success) = run_cli(&["-x", "16 * 16"]);
    assert!(success);
    assert!(stdout.contains("0x"));
}

// ==================== Octal Output Tests ====================

#[test]
fn test_octal_output() {
    let (stdout, _, success) = run_cli(&["-o", "255"]);
    assert!(success);
    assert!(stdout.contains("0o377"));
}

// ==================== Binary Output Tests ====================

#[test]
fn test_binary_output() {
    let (stdout, _, success) = run_cli(&["-b", "15"]);
    assert!(success);
    assert!(stdout.contains("0b1111"));
}

// ==================== Bitwise Mode Tests ====================

#[test]
fn test_bitwise_mode() {
    let (stdout, _, success) = run_cli(&["-B", "12 & 10"]);
    assert!(success);
    assert!(stdout.contains("8"));
}

#[test]
fn test_bitwise_with_hex_output() {
    let (stdout, _, success) = run_cli(&["-Bx", "0xFF & 0x0F"]);
    assert!(success);
    assert!(stdout.contains("0xF"));
}

// ==================== Binary Input Tests ====================

#[test]
fn test_binary_input() {
    let (stdout, _, success) = run_cli(&["0b1010"]);
    assert!(success);
    assert!(stdout.contains("10"));
}

#[test]
fn test_binary_input_with_operation() {
    let (stdout, _, success) = run_cli(&["0b1010 + 0b0101"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_binary_negative() {
    let (stdout, _, success) = run_cli(&["--", "-0b1010"]);
    assert!(success);
    assert!(stdout.contains("-10"));
}

// ==================== Octal Input Tests ====================

#[test]
fn test_octal_input() {
    let (stdout, _, success) = run_cli(&["0o755"]);
    assert!(success);
    assert!(stdout.contains("493"));
}

#[test]
fn test_octal_negative() {
    let (stdout, _, success) = run_cli(&["--", "-0o755"]);
    assert!(success);
    assert!(stdout.contains("-493"));
}

// ==================== Hexadecimal Input Tests ====================

#[test]
fn test_hex_input() {
    let (stdout, _, success) = run_cli(&["0xFF"]);
    assert!(success);
    assert!(stdout.contains("255"));
}

#[test]
fn test_hex_input_1a() {
    let (stdout, _, success) = run_cli(&["0x1A"]);
    assert!(success);
    assert!(stdout.contains("26"));
}

#[test]
fn test_hex_negative() {
    let (stdout, _, success) = run_cli(&["--", "-0xFF"]);
    assert!(success);
    assert!(stdout.contains("-255"));
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
    assert!(stdout.contains("36"));
}

// ==================== Scientific Notation Tests ====================

#[test]
fn test_scientific_input() {
    let (stdout, _, success) = run_cli(&["1e3"]);
    assert!(success);
    assert!(stdout.contains("1000"));
}

#[test]
fn test_scientific_small() {
    let (stdout, _, success) = run_cli(&["1e-3"]);
    assert!(success);
    assert!(stdout.contains("0.001"));
}

#[test]
fn test_scientific_arithmetic() {
    let (stdout, _, success) = run_cli(&["1e3 * 2"]);
    assert!(success);
    assert!(stdout.contains("2000"));
}

#[test]
fn test_scientific_shows_scientific_output() {
    let (stdout, _, success) = run_cli(&["1e3"]);
    assert!(success);
    assert!(stdout.contains("e"));
}

// ==================== Compact Scientific Notation Tests ====================

#[test]
fn test_scientific_positive_exponent() {
    let (stdout, _, success) = run_cli(&["1e+3"]);
    assert!(success);
    assert!(stdout.contains("1000"));
}

#[test]
fn test_scientific_negative_exponent() {
    let (stdout, _, success) = run_cli(&["1e-3"]);
    assert!(success);
    assert!(stdout.contains("0.001"));
}

#[test]
fn test_scientific_compact_addition() {
    let (stdout, _, success) = run_cli(&["1e+3+1e-3"]);
    assert!(success);
    assert!(stdout.contains("1000.001"));
}

#[test]
fn test_scientific_compact_multiplication() {
    let (stdout, _, success) = run_cli(&["1e+2*1e+2"]);
    assert!(success);
    assert!(stdout.contains("10000"));
}

#[test]
fn test_scientific_with_decimal() {
    let (stdout, _, success) = run_cli(&["1.5e+2"]);
    assert!(success);
    assert!(stdout.contains("150"));
}

#[test]
fn test_scientific_compact_mixed() {
    let (stdout, _, success) = run_cli(&["2.5e-2+1"]);
    assert!(success);
    assert!(stdout.contains("1.025"));
}

// ==================== Help Flag Tests ====================

#[test]
fn test_help_flag() {
    let (stdout, _, success) = run_cli(&["--help"]);
    assert!(success);
    assert!(stdout.contains("Usage"));
}

#[test]
fn test_functions_help() {
    let (stdout, _, success) = run_cli(&["-F"]);
    assert!(success);
    assert!(stdout.contains("Function"));
}

#[test]
fn test_operators_help() {
    let (stdout, _, success) = run_cli(&["-O"]);
    assert!(success);
    assert!(stdout.contains("Operator"));
}

// ==================== Subcommand Tests ====================

#[test]
fn test_functions_subcommand() {
    let (stdout, _, success) = run_cli(&["functions"]);
    assert!(success);
    assert!(stdout.contains("Function"));
}

#[test]
fn test_operators_subcommand() {
    let (stdout, _, success) = run_cli(&["operators"]);
    assert!(success);
    assert!(stdout.contains("Operator"));
}

#[test]
fn test_formats_subcommand() {
    let (stdout, _, success) = run_cli(&["formats"]);
    assert!(success);
    assert!(stdout.contains("Format"));
}

#[test]
fn test_constants_subcommand() {
    let (stdout, _, success) = run_cli(&["constants"]);
    assert!(success);
    assert!(stdout.contains("Constant"));
}

// ==================== Error Handling Tests ====================

#[test]
fn test_hex_non_integer_error() {
    let (_, stderr, success) = run_cli(&["-x", "3.14"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_invalid_expression() {
    let (_, stderr, success) = run_cli(&["sin()"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_undefined_function() {
    let (_, stderr, success) = run_cli(&["undefined_func(1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}
