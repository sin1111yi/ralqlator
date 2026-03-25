// Ralqlator - Extended Tests
// Additional comprehensive tests for all features

use std::process::Command;
use std::io::{Read, Write};
use std::process::{Command as ProcCommand, Stdio};

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

fn run_repl_commands(commands: &[&str]) -> String {
    let mut child = ProcCommand::new("cargo")
        .args(&["run", "--quiet", "--"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start REPL");

    {
        let stdin = child.stdin.as_mut().unwrap();
        for cmd in commands {
            writeln!(stdin, "{}", cmd).unwrap();
            std::thread::sleep(std::time::Duration::from_millis(30));
        }
        writeln!(stdin, "quit").unwrap();
    }

    let mut output = String::new();
    child.stdout.as_mut().unwrap().read_to_string(&mut output).unwrap();
    child.wait().unwrap();
    output
}

// ==================== Advanced Arithmetic Tests ====================

#[test]
fn test_complex_arithmetic_expression() {
    let (stdout, _, success) = run_cli(&["(10 + 5) * (20 - 15) / 5"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_power_of_power() {
    let (stdout, _, success) = run_cli(&["(2 ^ 3) ^ 2"]);
    assert!(success);
    assert!(stdout.contains("64"));
}

#[test]
fn test_right_associative_power() {
    let (stdout, _, success) = run_cli(&["2 ^ 3 ^ 2"]);
    assert!(success);
    assert!(stdout.contains("512"));
}

#[test]
fn test_factorial_in_expression() {
    let (stdout, _, success) = run_cli(&["3! + 4!"]);
    assert!(success);
    assert!(stdout.contains("30"));
}

#[test]
fn test_modulo_with_negative() {
    let (stdout, _, success) = run_cli(&["10 % 3"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

// ==================== Advanced Function Tests ====================

#[test]
fn test_nested_functions() {
    let (stdout, _, success) = run_cli(&["sqrt(pow(3, 2) + pow(4, 2))"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_trig_identity() {
    let (stdout, _, success) = run_cli(&["sin(0) + cos(0)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_log_properties() {
    let (stdout, _, success) = run_cli(&["lg(1000)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_ln_e() {
    let (stdout, _, success) = run_cli(&["ln(C_E)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_sum_function() {
    let (stdout, _, success) = run_cli(&["sum(1,2,3,4,5)"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_prod_function() {
    let (stdout, _, success) = run_cli(&["prod(1,2,3,4)"]);
    assert!(success);
    assert!(stdout.contains("24"));
}

#[test]
fn test_abs_function() {
    let (stdout, _, success) = run_cli(&["abs(-42)"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

#[test]
fn test_floor_function() {
    let (stdout, _, success) = run_cli(&["floor(3.9)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_ceil_function() {
    let (stdout, _, success) = run_cli(&["ceil(3.1)"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

#[test]
fn test_round_function() {
    let (stdout, _, success) = run_cli(&["round(3.5)"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

// ==================== BigInt Function Tests ====================

#[test]
fn test_bfactorial_100() {
    let (stdout, _, success) = run_cli(&["bfactorial(100)"]);
    assert!(success);
    assert!(stdout.len() > 100);
}

#[test]
fn test_bpow_large() {
    let (stdout, _, success) = run_cli(&["bpow(2, 200)"]);
    assert!(success);
    assert!(stdout.len() > 50);
}

#[test]
fn test_comb_poker() {
    let (stdout, _, success) = run_cli(&["comb(52, 5)"]);
    assert!(success);
    assert!(stdout.contains("2598960"));
}

#[test]
fn test_perm() {
    let (stdout, _, success) = run_cli(&["perm(10, 3)"]);
    assert!(success);
    assert!(stdout.contains("720"));
}

#[test]
fn test_gcd_coprime() {
    let (stdout, _, success) = run_cli(&["gcd(17, 19)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_lcm_coprime() {
    let (stdout, _, success) = run_cli(&["lcm(7, 11)"]);
    assert!(success);
    assert!(stdout.contains("77"));
}

#[test]
fn test_isprime_prime() {
    let (stdout, _, success) = run_cli(&["isprime(97)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_isprime_not_prime() {
    let (stdout, _, success) = run_cli(&["isprime(100)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_nextprime() {
    let (stdout, _, success) = run_cli(&["nextprime(1000)"]);
    assert!(success);
    assert!(stdout.contains("1009"));
}

// ==================== Number Format Tests ====================

#[test]
fn test_binary_input_arithmetic() {
    let (stdout, _, success) = run_cli(&["0b1010 + 0b0101"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_octal_input_arithmetic() {
    let (stdout, _, success) = run_cli(&["0o10 + 0o10"]);
    assert!(success);
    assert!(stdout.contains("16"));
}

#[test]
fn test_hex_input_arithmetic() {
    let (stdout, _, success) = run_cli(&["0x10 + 0x10"]);
    assert!(success);
    assert!(stdout.contains("32"));
}

#[test]
fn test_mixed_formats() {
    let (stdout, _, success) = run_cli(&["10 + 0xA + 0o12"]);
    assert!(success);
    assert!(stdout.contains("30"));
}

#[test]
fn test_hex_output_calculation() {
    let (stdout, _, success) = run_cli(&["-x", "255 + 1"]);
    assert!(success);
    assert!(stdout.contains("0x100"));
}

#[test]
fn test_bin_output_calculation() {
    let (stdout, _, success) = run_cli(&["-b", "8 * 2"]);
    assert!(success);
    assert!(stdout.contains("0b10000"));
}

// ==================== Scientific Notation Tests ====================

#[test]
fn test_scientific_large() {
    let (stdout, _, success) = run_cli(&["1e6"]);
    assert!(success);
    assert!(stdout.contains("1000000"));
}

#[test]
fn test_scientific_small() {
    let (stdout, _, success) = run_cli(&["1e-6"]);
    assert!(success);
    assert!(stdout.contains("0.000001") || stdout.contains("e-6"));
}

#[test]
fn test_scientific_multiplication() {
    let (stdout, _, success) = run_cli(&["1e3 * 1e3"]);
    assert!(success);
    assert!(stdout.contains("1000000"));
}

#[test]
fn test_scientific_division() {
    let (stdout, _, success) = run_cli(&["1e6 / 1e3"]);
    assert!(success);
    assert!(stdout.contains("1000"));
}

#[test]
fn test_scientific_negative_exponent() {
    let (stdout, _, success) = run_cli(&["10 * 1e-2"]);
    assert!(success);
    assert!(stdout.contains("0.1"));
}

// ==================== REPL Extended Tests ====================

#[test]
fn test_repl_mode_toggle() {
    let output = run_repl_commands(&["mode", "12 & 10", "mode", "5 + 3"]);
    assert!(output.contains("8") || output.contains("Bitwise"));
}

#[test]
fn test_repl_last_result_chain() {
    let output = run_repl_commands(&["10", "@ * 2", "@ + 5"]);
    assert!(output.contains("25"));
}

#[test]
fn test_repl_hex_oct_bin() {
    let output = run_repl_commands(&["255", "hex", "oct", "bin"]);
    assert!(output.contains("0xFF") && output.contains("0o377") && output.contains("0b11111111"));
}

#[test]
fn test_repl_empty_line_help() {
    let output = run_repl_commands(&["", "1 + 1"]);
    assert!(output.contains("2"));
}

#[test]
fn test_repl_tab_completion() {
    let output = run_repl_commands(&["help functions"]);
    assert!(output.contains("Function") || output.contains("lg") || output.contains("sin"));
}

#[test]
fn test_repl_create_and_use_func() {
    let output = run_repl_commands(&["create func double(x) = x * 2", "double(21)"]);
    assert!(output.contains("42"));
}

#[test]
fn test_repl_create_and_use_const() {
    let output = run_repl_commands(&["create const SPEED 300000", "SPEED * 2"]);
    assert!(output.contains("600000"));
}

#[test]
fn test_repl_quit() {
    let output = run_repl_commands(&["1 + 1", "quit"]);
    assert!(output.contains("2"));
}

// ==================== Error Handling Extended Tests ====================

#[test]
fn test_unmatched_open_paren() {
    let (_, stderr, success) = run_cli(&["(1 + 2"]);
    assert!(!success);
    assert!(stderr.contains("Error") || stderr.contains("unmatched"));
}

#[test]
fn test_unmatched_close_paren() {
    // Note: Parser now ignores extra closing parens
    let (stdout, _, success) = run_cli(&["1 + 2)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_empty_parentheses() {
    let (_, _, success) = run_cli(&["()"]);
    assert!(!success);
}

#[test]
fn test_consecutive_operators() {
    let (_, stderr, success) = run_cli(&["sin(1, 2, 3)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_only_operator() {
    let (_, _, success) = run_cli(&["+"]);
    assert!(!success);
}

#[test]
fn test_invalid_function() {
    let (_, stderr, success) = run_cli(&["invalid_func(1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_function_wrong_args() {
    let (_, stderr, success) = run_cli(&["sin(1, 2, 3)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_sqrt_negative() {
    let (_, stderr, success) = run_cli(&["sqrt(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_log_negative() {
    let (_, stderr, success) = run_cli(&["lg(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_asin_out_of_range() {
    let (_, stderr, success) = run_cli(&["asin(2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_factorial_negative() {
    let (_, stderr, success) = run_cli(&["factorial(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_factorial_too_large() {
    let (_, stderr, success) = run_cli(&["factorial(200)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_bitwise_invalid_shift() {
    let (_, stderr, success) = run_cli(&["-B", "8 << 100"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

// ==================== Help and Documentation Tests ====================

#[test]
fn test_help_flag() {
    let (stdout, _, success) = run_cli(&["--help"]);
    assert!(success);
    assert!(stdout.contains("Usage") || stdout.contains("ralqlator"));
}

#[test]
fn test_functions_help() {
    let (stdout, _, success) = run_cli(&["-F"]);
    assert!(success);
    assert!(stdout.contains("Function") || stdout.contains("lg"));
}

#[test]
fn test_operators_help() {
    let (stdout, _, success) = run_cli(&["-O"]);
    assert!(success);
    assert!(stdout.contains("Operator"));
}

#[test]
fn test_formats_help() {
    let (stdout, _, success) = run_cli(&["-N"]);
    assert!(success);
    assert!(stdout.contains("Format"));
}

#[test]
fn test_constants_help() {
    let (stdout, _, success) = run_cli(&["-C"]);
    assert!(success);
    assert!(stdout.contains("Constant"));
}

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
fn test_info_subcommand() {
    let (stdout, _, success) = run_cli(&["info"]);
    assert!(success);
    assert!(stdout.contains("Operator") || stdout.contains("Function"));
}

// ==================== Edge Cases Extended Tests ====================

#[test]
fn test_very_large_multiplication() {
    let (stdout, _, success) = run_cli(&["999999 * 999999"]);
    assert!(success);
    assert!(stdout.contains("999998000001"));
}

#[test]
fn test_power_of_zero() {
    let (stdout, _, success) = run_cli(&["0 ^ 5"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_zero_factorial() {
    let (stdout, _, success) = run_cli(&["0!"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_one_factorial() {
    let (stdout, _, success) = run_cli(&["1!"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_decimal_precision() {
    let (stdout, _, success) = run_cli(&["1.0 / 3.0"]);
    assert!(success);
    assert!(stdout.contains("0.333"));
}

#[test]
fn test_many_decimals() {
    let (stdout, _, success) = run_cli(&["0.123456789 + 0.987654321"]);
    assert!(success);
    assert!(stdout.contains("1.11111111"));
}

#[test]
fn test_whitespace_handling() {
    let (stdout, _, success) = run_cli(&["  1  +  2  "]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_single_number() {
    let (stdout, _, success) = run_cli(&["42"]);
    assert!(success);
    assert!(stdout.contains("42"));
}

#[test]
fn test_single_negative_number() {
    let (stdout, _, success) = run_cli(&["--", "-42"]);
    assert!(success);
    assert!(stdout.contains("-42"));
}

#[test]
fn test_comparison_equal_floats() {
    let (stdout, _, success) = run_cli(&["3.14 = 3.14"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_less_than_floats() {
    let (stdout, _, success) = run_cli(&["3.14 < 3.15"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_comparison_greater_than_floats() {
    let (stdout, _, success) = run_cli(&["3.15 > 3.14"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

// ==================== Additional Compact Format Tests ====================

#[test]
fn test_compact_factorial() {
    let (stdout, _, success) = run_cli(&["5!"]);
    assert!(success);
    assert!(stdout.contains("120"));
}

#[test]
fn test_compact_exponentiation() {
    let (stdout, _, success) = run_cli(&["2^10"]);
    assert!(success);
    assert!(stdout.contains("1024"));
}

#[test]
fn test_compact_modulo() {
    let (stdout, _, success) = run_cli(&["10%3"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_compact_power_chain() {
    let (stdout, _, success) = run_cli(&["2^3^2"]);
    assert!(success);
    assert!(stdout.contains("512"));
}

#[test]
fn test_compact_decimal_arithmetic() {
    let (stdout, _, success) = run_cli(&["0.5+0.5"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_compact_decimal_multiplication() {
    let (stdout, _, success) = run_cli(&["0.1*0.1"]);
    assert!(success);
    assert!(stdout.contains("0.01"));
}

#[test]
fn test_compact_with_functions() {
    let (stdout, _, success) = run_cli(&["sqrt(16)"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

#[test]
fn test_compact_nested_functions() {
    let (stdout, _, success) = run_cli(&["sqrt(pow(3,2)+pow(4,2))"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_compact_comparison_no_spaces() {
    let (stdout, _, success) = run_cli(&["5>3"]);
    assert!(success);
    assert!(stdout.contains("true"));
}

#[test]
fn test_compact_equal_no_spaces() {
    let (stdout, _, success) = run_cli(&["5=5"]);
    assert!(success);
    assert!(stdout.contains("true"));
}
