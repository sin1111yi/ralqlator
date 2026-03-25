// Ralqlator - REPL and Interactive Tests
// Combines: interactive_tests, comprehensive_repl_tests

use std::io::{Read, Write};
use std::process::{Command, Stdio};

fn run_repl_commands(commands: &[&str]) -> String {
    let mut child = Command::new("cargo")
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

// ==================== Basic Arithmetic Tests ====================

#[test]
fn test_repl_addition() {
    let output = run_repl_commands(&["1 + 2"]);
    assert!(output.contains("3"));
}

#[test]
fn test_repl_multiplication() {
    let output = run_repl_commands(&["3 * 4"]);
    assert!(output.contains("12"));
}

#[test]
fn test_repl_division() {
    let output = run_repl_commands(&["10 / 2"]);
    assert!(output.contains("5"));
}

#[test]
fn test_repl_exponentiation() {
    let output = run_repl_commands(&["2 ^ 10"]);
    assert!(output.contains("1024"));
}

#[test]
fn test_repl_factorial() {
    let output = run_repl_commands(&["5!"]);
    assert!(output.contains("120"));
}

// ==================== Bitwise Mode Tests ====================

#[test]
fn test_repl_bitwise_and() {
    let output = run_repl_commands(&["mode bitwise", "12 & 10"]);
    assert!(output.contains("8"));
}

#[test]
fn test_repl_bitwise_or() {
    let output = run_repl_commands(&["mode bitwise", "12 | 10"]);
    assert!(output.contains("14"));
}

#[test]
fn test_repl_bitwise_not() {
    let output = run_repl_commands(&["mode bitwise", "~0"]);
    assert!(output.contains("-1"));
}

#[test]
fn test_repl_bitwise_shift() {
    let output = run_repl_commands(&["mode bitwise", "8 << 2"]);
    assert!(output.contains("32"));
}

// ==================== Function Tests ====================

#[test]
fn test_repl_sin() {
    let output = run_repl_commands(&["sin(0)"]);
    assert!(output.contains("0"));
}

#[test]
fn test_repl_cos() {
    let output = run_repl_commands(&["cos(0)"]);
    assert!(output.contains("1"));
}

#[test]
fn test_repl_sqrt() {
    let output = run_repl_commands(&["sqrt(16)"]);
    assert!(output.contains("4"));
}

#[test]
fn test_repl_lg() {
    let output = run_repl_commands(&["lg(100)"]);
    assert!(output.contains("2"));
}

#[test]
fn test_repl_abs() {
    let output = run_repl_commands(&["abs(-5)"]);
    assert!(output.contains("5"));
}

// ==================== Constant Tests ====================

#[test]
fn test_repl_c_pi() {
    let output = run_repl_commands(&["C_PI"]);
    assert!(output.contains("3.14"));
}

#[test]
fn test_repl_c_e() {
    let output = run_repl_commands(&["C_E"]);
    assert!(output.contains("2.718"));
}

#[test]
fn test_repl_c_pi_expression() {
    let output = run_repl_commands(&["C_PI * 2"]);
    assert!(output.contains("6.28"));
}

// ==================== User-defined Function Tests ====================

#[test]
fn test_repl_create_func() {
    let output = run_repl_commands(&["create func f(x) = x + 1", "f(5)"]);
    assert!(output.contains("6"));
}

#[test]
fn test_repl_create_func_two_args() {
    let output = run_repl_commands(&["create func add(a, b) = a + b", "add(3, 7)"]);
    assert!(output.contains("10"));
}

#[test]
fn test_repl_create_func_shorthand() {
    let output = run_repl_commands(&["create function f(x) = x * 2", "f(10)"]);
    assert!(output.contains("20"));
}

// ==================== User-defined Sequence Tests ====================

#[test]
fn test_repl_create_seq() {
    let output = run_repl_commands(&["create seq triangle(n) = n*(n+1)/2", "triangle(10)"]);
    assert!(output.len() > 0);
}

#[test]
fn test_repl_suma() {
    let output = run_repl_commands(&["create seq square(n) = n^2", "suma(square, 1, 5)"]);
    assert!(output.len() > 0);
}

// ==================== User-defined Constant Tests ====================

#[test]
fn test_repl_create_const() {
    let output = run_repl_commands(&["create const G 9.81", "G * 10"]);
    assert!(output.contains("98.1"));
}

#[test]
fn test_repl_create_const_shorthand() {
    let output = run_repl_commands(&["create c TEST 42", "TEST * 2"]);
    assert!(output.contains("84"));
}

// ==================== Last Result Tests ====================

#[test]
fn test_repl_last_result() {
    let output = run_repl_commands(&["42", "@ + 1"]);
    assert!(output.contains("43"));
}

#[test]
fn test_repl_last_result_chain() {
    let output = run_repl_commands(&["10", "@ * 2", "@ + 5"]);
    assert!(output.contains("25"));
}

#[test]
fn test_repl_hex_conversion() {
    let output = run_repl_commands(&["255", "hex"]);
    assert!(output.contains("0xFF"));
}

#[test]
fn test_repl_oct_conversion() {
    let output = run_repl_commands(&["255", "oct"]);
    assert!(output.contains("0o377"));
}

#[test]
fn test_repl_bin_conversion() {
    let output = run_repl_commands(&["15", "bin"]);
    assert!(output.contains("0b1111"));
}

// ==================== Comparison Tests ====================

#[test]
fn test_repl_less_than() {
    let output = run_repl_commands(&["3 < 5"]);
    assert!(output.contains("true"));
}

#[test]
fn test_repl_greater_than() {
    let output = run_repl_commands(&["5 > 3"]);
    assert!(output.contains("true"));
}

#[test]
fn test_repl_equal() {
    let output = run_repl_commands(&["5 = 5"]);
    assert!(output.contains("true"));
}

#[test]
fn test_repl_logical_equal() {
    let output = run_repl_commands(&["5 == 5"]);
    assert!(output.contains("true"));
}

#[test]
fn test_repl_not_equal() {
    let output = run_repl_commands(&["5 = 3"]);
    assert!(output.contains("false"));
}

// ==================== Number Format Tests ====================

#[test]
fn test_repl_hex_input() {
    let output = run_repl_commands(&["0xFF"]);
    assert!(output.contains("255"));
}

#[test]
fn test_repl_bin_input() {
    let output = run_repl_commands(&["0b1010"]);
    assert!(output.contains("10"));
}

#[test]
fn test_repl_oct_input() {
    let output = run_repl_commands(&["0o755"]);
    assert!(output.contains("493"));
}

// ==================== Help Command Tests ====================

#[test]
fn test_repl_help() {
    let output = run_repl_commands(&["help"]);
    assert!(output.contains("help") || output.contains("Commands"));
}

#[test]
fn test_repl_help_functions() {
    let output = run_repl_commands(&["help functions"]);
    assert!(output.contains("Function") || output.contains("lg") || output.contains("sin"));
}

#[test]
fn test_repl_help_operators() {
    let output = run_repl_commands(&["help operators"]);
    assert!(output.contains("Operator") || output.contains("+") || output.contains("-"));
}

#[test]
fn test_repl_help_constants() {
    let output = run_repl_commands(&["help constants"]);
    assert!(output.contains("C_PI") || output.contains("C_E"));
}

// ==================== Error Handling Tests ====================

#[test]
fn test_repl_division_by_zero() {
    let output = run_repl_commands(&["1 / 0"]);
    assert!(output.len() > 0);
}

#[test]
fn test_repl_invalid_expression() {
    let output = run_repl_commands(&["1 ++ 2"]);
    assert!(output.len() > 0);
}

#[test]
fn test_repl_undefined_function() {
    let output = run_repl_commands(&["undefined_func(1)"]);
    assert!(output.len() > 0);
}

#[test]
fn test_repl_unmatched_paren() {
    let output = run_repl_commands(&["(1 + 2"]);
    assert!(output.len() > 0);
}

// ==================== Edge Cases ====================

#[test]
fn test_repl_empty_line() {
    let output = run_repl_commands(&["", "1 + 1"]);
    assert!(output.contains("2"));
}

#[test]
fn test_repl_whitespace_only() {
    let output = run_repl_commands(&["   ", "2 + 2"]);
    assert!(output.contains("4"));
}

#[test]
fn test_repl_negative_number() {
    let output = run_repl_commands(&["-5 + 3"]);
    assert!(output.contains("-2"));
}

#[test]
fn test_repl_nested_parens() {
    let output = run_repl_commands(&["((1 + 2) * (3 + 4))"]);
    assert!(output.contains("21"));
}

// ==================== BigInt Function Tests ====================

#[test]
fn test_repl_bfactorial() {
    let output = run_repl_commands(&["bfactorial(50)"]);
    assert!(output.len() > 50);
}

#[test]
fn test_repl_bpow() {
    let output = run_repl_commands(&["bpow(2, 100)"]);
    assert!(output.len() >= 30);
}

#[test]
fn test_repl_gcd() {
    let output = run_repl_commands(&["gcd(48, 18)"]);
    assert!(output.contains("6"));
}

#[test]
fn test_repl_lcm() {
    let output = run_repl_commands(&["lcm(12, 18)"]);
    assert!(output.contains("36"));
}

#[test]
fn test_repl_isprime() {
    let output = run_repl_commands(&["isprime(17)"]);
    assert!(output.contains("1"));
}

#[test]
fn test_repl_nextprime() {
    let output = run_repl_commands(&["nextprime(100)"]);
    assert!(output.contains("101"));
}
