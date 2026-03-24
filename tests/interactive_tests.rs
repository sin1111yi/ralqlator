// Interactive Mode Tests
// Tests for REPL interactive mode features

use std::process::Command;
use std::io::Write;

fn run_interactive(commands: &[&str]) -> String {
    let mut child = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Failed to start interactive mode");
    
    {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        for cmd in commands {
            stdin.write_all(format!("{}\n", cmd).as_bytes())
                .expect("Failed to write to stdin");
        }
    }
    
    let output = child.wait_with_output().expect("Failed to read output");
    format!("{}{}", 
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr))
}

// ==================== Basic Calculation Tests ====================

#[test]
fn test_interactive_calc_addition() {
    let stdout = run_interactive(&["100 + 200", "q"]);
    assert!(stdout.contains("300"));
}

#[test]
fn test_interactive_calc_subtraction() {
    let stdout = run_interactive(&["100 - 30", "q"]);
    assert!(stdout.contains("70"));
}

#[test]
fn test_interactive_calc_multiplication() {
    let stdout = run_interactive(&["12 * 12", "q"]);
    assert!(stdout.contains("144"));
}

#[test]
fn test_interactive_calc_division() {
    let stdout = run_interactive(&["144 / 12", "q"]);
    assert!(stdout.contains("12"));
}

#[test]
fn test_interactive_calc_exponentiation() {
    let stdout = run_interactive(&["2 ^ 10", "q"]);
    assert!(stdout.contains("1024"));
}

// ==================== Last Result (@) Tests ====================

#[test]
fn test_interactive_at_addition() {
    let stdout = run_interactive(&["10", "@ + 5", "q"]);
    assert!(stdout.contains("15"));
}

#[test]
fn test_interactive_at_multiplication() {
    let stdout = run_interactive(&["7", "@ * 7", "q"]);
    assert!(stdout.contains("49"));
}

#[test]
fn test_interactive_at_power() {
    let stdout = run_interactive(&["2", "@ ^ 10", "q"]);
    assert!(stdout.contains("1024"));
}

#[test]
fn test_interactive_at_chain() {
    let stdout = run_interactive(&["5", "@ + 5", "@ * 2", "q"]);
    assert!(stdout.contains("20"));
}

#[test]
fn test_interactive_at_with_format() {
    let stdout = run_interactive(&["255", "@", "hex", "q"]);
    assert!(stdout.contains("255"));
    assert!(stdout.contains("0xFF"));
}

// ==================== Format Conversion Tests ====================

#[test]
fn test_interactive_hex_positive() {
    let stdout = run_interactive(&["255", "hex", "q"]);
    assert!(stdout.contains("0xFF"));
}

#[test]
fn test_interactive_hex_zero() {
    let stdout = run_interactive(&["0", "hex", "q"]);
    assert!(stdout.contains("0x0"));
}

#[test]
fn test_interactive_hex_large() {
    let stdout = run_interactive(&["65535", "hex", "q"]);
    assert!(stdout.contains("0xFFFF"));
}

#[test]
fn test_interactive_oct_positive() {
    let stdout = run_interactive(&["64", "oct", "q"]);
    assert!(stdout.contains("0o100"));
}

#[test]
fn test_interactive_oct_zero() {
    let stdout = run_interactive(&["0", "oct", "q"]);
    assert!(stdout.contains("0o0"));
}

#[test]
fn test_interactive_bin_positive() {
    let stdout = run_interactive(&["8", "bin", "q"]);
    assert!(stdout.contains("0b1000"));
}

#[test]
fn test_interactive_bin_zero() {
    let stdout = run_interactive(&["0", "bin", "q"]);
    assert!(stdout.contains("0b0"));
}

#[test]
fn test_interactive_bin_255() {
    let stdout = run_interactive(&["255", "bin", "q"]);
    assert!(stdout.contains("0b11111111"));
}

#[test]
fn test_interactive_multiple_format_conversions() {
    let stdout = run_interactive(&["42", "hex", "oct", "bin", "q"]);
    assert!(stdout.contains("0x2A"));
    assert!(stdout.contains("0o52"));
    assert!(stdout.contains("0b101010"));
}

// ==================== Mode Switching Tests ====================

#[test]
fn test_interactive_mode_toggle() {
    let stdout = run_interactive(&["mode", "q"]);
    assert!(stdout.contains("Bitwise") || stdout.contains("Standard"));
}

#[test]
fn test_interactive_mode_set_standard() {
    let stdout = run_interactive(&["mode standard", "10 + 10", "q"]);
    assert!(stdout.contains("20"));
}

#[test]
fn test_interactive_mode_set_bitwise() {
    let stdout = run_interactive(&["mode bitwise", "12 & 10", "q"]);
    assert!(stdout.contains("8"));
}

#[test]
fn test_interactive_mode_switch_back() {
    let stdout = run_interactive(&["mode bitwise", "mode standard", "10 + 10", "q"]);
    assert!(stdout.contains("20"));
}

#[test]
fn test_interactive_mode_std_shorthand() {
    let stdout = run_interactive(&["mode std", "5 + 5", "q"]);
    assert!(stdout.contains("10"));
}

#[test]
fn test_interactive_mode_bit_shorthand() {
    let stdout = run_interactive(&["mode bit", "8 | 4", "q"]);
    assert!(stdout.contains("12"));
}

// ==================== Help Command Tests ====================

#[test]
fn test_interactive_help_basic() {
    let stdout = run_interactive(&["help", "q"]);
    assert!(stdout.contains("Commands"));
}

#[test]
fn test_interactive_help_functions() {
    let stdout = run_interactive(&["help functions", "q"]);
    assert!(stdout.contains("lg"));
    assert!(stdout.contains("sin"));
}

#[test]
fn test_interactive_help_operators() {
    let stdout = run_interactive(&["help operators", "q"]);
    assert!(stdout.contains("Addition"));
    assert!(stdout.contains("Comparison"));
}

#[test]
fn test_interactive_help_formats() {
    let stdout = run_interactive(&["help formats", "q"]);
    assert!(stdout.contains("Decimal"));
    assert!(stdout.contains("Binary"));
}

#[test]
fn test_interactive_help_constants() {
    let stdout = run_interactive(&["help constants", "q"]);
    assert!(stdout.contains("C_PI"));
}

#[test]
fn test_interactive_help_mode() {
    let stdout = run_interactive(&["help mode", "q"]);
    assert!(stdout.contains("Mode"));
}

#[test]
fn test_interactive_help_create() {
    let stdout = run_interactive(&["help create", "q"]);
    assert!(stdout.contains("create"));
}

#[test]
fn test_interactive_help_standard() {
    let stdout = run_interactive(&["help standard", "q"]);
    assert!(stdout.contains("Arithmetic"));
}

#[test]
fn test_interactive_help_bitwise() {
    let stdout = run_interactive(&["help bitwise", "q"]);
    assert!(stdout.contains("Bitwise"));
}

// ==================== Functions Command Tests ====================

#[test]
fn test_interactive_functions_basic() {
    let stdout = run_interactive(&["functions", "q"]);
    assert!(stdout.contains("lg"));
}

#[test]
fn test_interactive_functions_with_user_defined() {
    let stdout = run_interactive(&["create func f(x) = x + 1", "functions", "q"]);
    assert!(stdout.contains("lg"));
    assert!(stdout.contains("f"));
}

// ==================== Operators Command Tests ====================

#[test]
fn test_interactive_operators_basic() {
    let stdout = run_interactive(&["operators", "q"]);
    assert!(stdout.contains("Addition"));
}

#[test]
fn test_interactive_operators_standard() {
    let stdout = run_interactive(&["operators standard", "q"]);
    assert!(stdout.contains("Addition"));
}

#[test]
fn test_interactive_operators_bitwise() {
    let stdout = run_interactive(&["operators bitwise", "q"]);
    assert!(stdout.contains("AND"));
}

// ==================== Formats Command Tests ====================

#[test]
fn test_interactive_formats_basic() {
    let stdout = run_interactive(&["formats", "q"]);
    assert!(stdout.contains("Decimal"));
}

// ==================== Constants Command Tests ====================

#[test]
fn test_interactive_constants_basic() {
    let stdout = run_interactive(&["constants", "q"]);
    assert!(stdout.contains("C_PI"));
}

#[test]
fn test_interactive_constants_use() {
    let stdout = run_interactive(&["constants", "C_PI * 2", "q"]);
    assert!(stdout.contains("C_PI"));
    assert!(stdout.contains("6.28"));
}

// ==================== Empty Input Tests ====================

#[test]
fn test_interactive_empty_line_shows_help() {
    let stdout = run_interactive(&["", "q"]);
    assert!(stdout.contains("Commands"));
}

#[test]
fn test_interactive_whitespace_only() {
    let stdout = run_interactive(&["   ", "q"]);
    assert!(stdout.contains("Commands") || stdout.is_empty());
}

// ==================== Quit Command Tests ====================

#[test]
fn test_interactive_quit_q() {
    let output = run_interactive(&["q"]);
    assert!(output.len() > 0);
}

#[test]
fn test_interactive_quit_quit() {
    let output = run_interactive(&["quit"]);
    assert!(output.len() > 0);
}

// ==================== Error Handling Tests ====================

#[test]
fn test_interactive_division_by_zero() {
    let stdout = run_interactive(&["10 / 0", "q"]);
    assert!(stdout.contains("Error"));
}

#[test]
fn test_interactive_invalid_expression() {
    let stdout = run_interactive(&["abc", "q"]);
    assert!(stdout.contains("Error"));
}

#[test]
fn test_interactive_unmatched_paren() {
    let stdout = run_interactive(&["(1 + 2", "q"]);
    assert!(stdout.contains("Error"));
}

// ==================== Bitwise Mode Interactive Tests ====================

#[test]
fn test_interactive_bitwise_and() {
    let stdout = run_interactive(&["mode bitwise", "12 & 10", "q"]);
    assert!(stdout.contains("8"));
}

#[test]
fn test_interactive_bitwise_or() {
    let stdout = run_interactive(&["mode bitwise", "12 | 10", "q"]);
    assert!(stdout.contains("14"));
}

#[test]
fn test_interactive_bitwise_xor() {
    let stdout = run_interactive(&["mode bitwise", "12 ^ 10", "q"]);
    assert!(stdout.contains("6"));
}

#[test]
fn test_interactive_bitwise_not() {
    let stdout = run_interactive(&["mode bitwise", "~0", "q"]);
    assert!(stdout.contains("-1"));
}

#[test]
fn test_interactive_bitwise_left_shift() {
    let stdout = run_interactive(&["mode bitwise", "8 << 2", "q"]);
    assert!(stdout.contains("32"));
}

#[test]
fn test_interactive_bitwise_right_shift() {
    let stdout = run_interactive(&["mode bitwise", "8 >> 2", "q"]);
    assert!(stdout.contains("2"));
}

#[test]
fn test_interactive_bitwise_hex_input() {
    let stdout = run_interactive(&["mode bitwise", "0xFF & 0x0F", "q"]);
    assert!(stdout.contains("15"));
}

#[test]
fn test_interactive_bitwise_binary_input() {
    let stdout = run_interactive(&["mode bitwise", "0b1010 & 0b1100", "q"]);
    assert!(stdout.contains("8"));
}

#[test]
fn test_interactive_bitwise_negative_unary() {
    let stdout = run_interactive(&["mode bitwise", "-1 & 255", "q"]);
    assert!(stdout.contains("255"));
}

#[test]
fn test_interactive_bitwise_subtraction_error() {
    let stdout = run_interactive(&["mode bitwise", "10 - 3", "q"]);
    assert!(stdout.contains("Error"));
}
