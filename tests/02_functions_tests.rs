// Ralqlator - Functions Tests
// Combines: functions_tests

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

// ==================== Logarithm Tests ====================

#[test]
fn test_lg() {
    let (stdout, _, success) = run_cli(&["lg(100)"]);
    assert!(success);
    assert!(stdout.contains("2"));
}

#[test]
fn test_lg_custom_base() {
    let (stdout, _, success) = run_cli(&["lg(8, 2)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_ln() {
    let (stdout, _, success) = run_cli(&["ln(1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_log2() {
    let (stdout, _, success) = run_cli(&["log2(256)"]);
    assert!(success);
    assert!(stdout.contains("8"));
}

// ==================== Root Tests ====================

#[test]
fn test_sqrt() {
    let (stdout, _, success) = run_cli(&["sqrt(16)"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

#[test]
fn test_cbrt() {
    let (stdout, _, success) = run_cli(&["cbrt(27)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

// ==================== Power Tests ====================

#[test]
fn test_pow() {
    let (stdout, _, success) = run_cli(&["pow(2, 10)"]);
    assert!(success);
    assert!(stdout.contains("1024"));
}

// ==================== Trigonometric Tests ====================

#[test]
fn test_sin() {
    let (stdout, _, success) = run_cli(&["sin(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_cos() {
    let (stdout, _, success) = run_cli(&["cos(0)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_tan() {
    let (stdout, _, success) = run_cli(&["tan(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

// ==================== Inverse Trigonometric Tests ====================

#[test]
fn test_asin() {
    let (stdout, _, success) = run_cli(&["asin(1)"]);
    assert!(success);
    assert!(stdout.contains("1.57"));
}

#[test]
fn test_acos() {
    let (stdout, _, success) = run_cli(&["acos(1)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_atan() {
    let (stdout, _, success) = run_cli(&["atan(1)"]);
    assert!(success);
    assert!(stdout.contains("0.785"));
}

// ==================== Hyperbolic Tests ====================

#[test]
fn test_sinh() {
    let (stdout, _, success) = run_cli(&["sinh(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_cosh() {
    let (stdout, _, success) = run_cli(&["cosh(0)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_tanh() {
    let (stdout, _, success) = run_cli(&["tanh(0)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

// ==================== Utility Function Tests ====================

#[test]
fn test_abs() {
    let (stdout, _, success) = run_cli(&["abs(-5)"]);
    assert!(success);
    assert!(stdout.contains("5"));
}

#[test]
fn test_floor() {
    let (stdout, _, success) = run_cli(&["floor(3.7)"]);
    assert!(success);
    assert!(stdout.contains("3"));
}

#[test]
fn test_ceil() {
    let (stdout, _, success) = run_cli(&["ceil(3.2)"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

#[test]
fn test_round() {
    let (stdout, _, success) = run_cli(&["round(3.5)"]);
    assert!(success);
    assert!(stdout.contains("4"));
}

#[test]
fn test_mod() {
    let (stdout, _, success) = run_cli(&["mod(10, 3)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

// ==================== Multi-argument Function Tests ====================

#[test]
fn test_sum() {
    let (stdout, _, success) = run_cli(&["sum(1,2,3,4,5)"]);
    assert!(success);
    assert!(stdout.contains("15"));
}

#[test]
fn test_prod() {
    let (stdout, _, success) = run_cli(&["prod(1,2,3,4,5)"]);
    assert!(success);
    assert!(stdout.contains("120"));
}

// ==================== BigInt Function Tests ====================

#[test]
fn test_bfactorial() {
    let (stdout, _, success) = run_cli(&["bfactorial(50)"]);
    assert!(success);
    assert!(stdout.len() > 50);
}

#[test]
fn test_bpow() {
    let (stdout, _, success) = run_cli(&["bpow(2, 100)"]);
    assert!(success);
    assert!(stdout.len() >= 30);
}

#[test]
fn test_comb() {
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
fn test_gcd() {
    let (stdout, _, success) = run_cli(&["gcd(48, 18)"]);
    assert!(success);
    assert!(stdout.contains("6"));
}

#[test]
fn test_lcm() {
    let (stdout, _, success) = run_cli(&["lcm(12, 18)"]);
    assert!(success);
    assert!(stdout.contains("36"));
}

#[test]
fn test_isprime() {
    let (stdout, _, success) = run_cli(&["isprime(17)"]);
    assert!(success);
    assert!(stdout.contains("1"));
}

#[test]
fn test_isprime_not_prime() {
    let (stdout, _, success) = run_cli(&["isprime(15)"]);
    assert!(success);
    assert!(stdout.contains("0"));
}

#[test]
fn test_nextprime() {
    let (stdout, _, success) = run_cli(&["nextprime(100)"]);
    assert!(success);
    assert!(stdout.contains("101"));
}

// ==================== Error Tests ====================

#[test]
fn test_sqrt_negative_error() {
    let (_, stderr, success) = run_cli(&["sqrt(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_lg_negative_error() {
    let (_, stderr, success) = run_cli(&["lg(-1)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}

#[test]
fn test_asin_out_of_range_error() {
    let (_, stderr, success) = run_cli(&["asin(2)"]);
    assert!(!success);
    assert!(stderr.contains("Error"));
}
