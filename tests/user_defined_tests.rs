// User-Defined Functions, Sequences, and Constants Tests

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

// ==================== User-Defined Function Tests ====================

#[test]
fn test_user_func_simple() {
    let stdout = run_interactive(&["create func f(x) = x + 1", "f(5)", "q"]);
    assert!(stdout.contains("6"));
}

#[test]
fn test_user_func_two_args() {
    let stdout = run_interactive(&["create func add(a,b) = a + b", "add(3, 7)", "q"]);
    assert!(stdout.contains("10"));
}

#[test]
fn test_user_func_shorthand_fn() {
    let stdout = run_interactive(&["create fn double(x) = x * 2", "double(21)", "q"]);
    assert!(stdout.contains("42"));
}

#[test]
fn test_user_func_shorthand_f() {
    let stdout = run_interactive(&["create f triple(x) = x * 3", "triple(11)", "q"]);
    assert!(stdout.contains("33"));
}

#[test]
fn test_user_func_keyword() {
    let stdout = run_interactive(&["create function square(x) = x * x", "square(12)", "q"]);
    assert!(stdout.contains("144"));
}

#[test]
fn test_user_func_nested() {
    let stdout = run_interactive(&[
        "create func f(x) = x + 1",
        "create func g(x) = (f(x)) * 2",
        "g(5)",
        "q"
    ]);
    assert!(stdout.contains("12"));
}

#[test]
fn test_user_func_multiple_calls() {
    let stdout = run_interactive(&[
        "create func double(x) = x * 2",
        "double(3) + double(5)",
        "q"
    ]);
    assert!(stdout.contains("16"));
}

#[test]
fn test_user_func_with_expression_args() {
    let stdout = run_interactive(&[
        "create func f(x) = x + 1",
        "f(2 * 3)",
        "q"
    ]);
    assert!(stdout.contains("7"));
}

#[test]
fn test_user_func_error_wrong_args() {
    let stdout = run_interactive(&[
        "create func f(x, y) = x + y",
        "f(5)",
        "q"
    ]);
    assert!(stdout.contains("Error"));
}

#[test]
fn test_user_func_error_reserved_name() {
    let stdout = run_interactive(&["create func C_PI(x) = x", "q"]);
    assert!(stdout.contains("Error"));
    assert!(stdout.contains("reserved"));
}

// ==================== User-Defined Sequence Tests ====================

#[test]
fn test_user_seq_simple() {
    let stdout = run_interactive(&["create seq a(n) = n * 2", "a(10)", "q"]);
    assert!(stdout.contains("20"));
}

#[test]
fn test_user_seq_shorthand_s() {
    let stdout = run_interactive(&["create s b(n) = n + 1", "b(99)", "q"]);
    assert!(stdout.contains("100"));
}

#[test]
fn test_user_seq_keyword() {
    let stdout = run_interactive(&["create sequence c(n) = n ^ 2", "c(12)", "q"]);
    assert!(stdout.contains("144"));
}

#[test]
fn test_user_seq_with_suma() {
    let stdout = run_interactive(&[
        "create seq triangle(n) = n * (n + 1) / 2",
        "suma(triangle, 1, 5)",
        "q"
    ]);
    assert!(stdout.contains("35"));
}

#[test]
fn test_user_seq_suma_square() {
    let stdout = run_interactive(&[
        "create seq square(n) = n ^ 2",
        "suma(square, 1, 5)",
        "q"
    ]);
    assert!(stdout.contains("55"));
}

#[test]
fn test_user_seq_suma_single_term() {
    let stdout = run_interactive(&[
        "create seq const5(n) = 5",
        "suma(const5, 1, 3)",
        "q"
    ]);
    assert!(stdout.contains("15"));
}

#[test]
fn test_user_seq_error_reserved_name() {
    let stdout = run_interactive(&["create seq C_E(n) = n", "q"]);
    assert!(stdout.contains("Error"));
}

// ==================== User-Defined Constant Tests ====================

#[test]
fn test_user_const_simple() {
    let stdout = run_interactive(&["create const MY_CONST 3.14", "MY_CONST * 2", "q"]);
    assert!(stdout.contains("6.28"));
}

#[test]
fn test_user_const_shorthand_c() {
    let stdout = run_interactive(&["create c G 9.81", "G * 10", "q"]);
    assert!(stdout.contains("98.1"));
}

#[test]
fn test_user_const_keyword_const() {
    let stdout = run_interactive(&["create const SPEED 299792458", "SPEED", "q"]);
    assert!(stdout.contains("299792458"));
}

#[test]
fn test_user_const_keyword_constant() {
    let stdout = run_interactive(&["create constant PLANCK 6.626e-34", "PLANCK", "q"]);
    assert!(stdout.contains("6626") || stdout.contains("6.626"));
}

#[test]
fn test_user_const_error_reserved_name() {
    let stdout = run_interactive(&["create const C_PI 3", "q"]);
    assert!(stdout.contains("Error"));
}

#[test]
fn test_user_const_error_invalid_value() {
    let stdout = run_interactive(&["create const INVALID abc", "q"]);
    assert!(stdout.contains("Error"));
}
