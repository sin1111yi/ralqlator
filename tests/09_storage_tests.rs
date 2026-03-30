// Ralqlator - Storage and User Definition Tests
// Tests for --create, --destroy, --list commands and auto-save/load functionality

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;

// Global lock to ensure tests run serially (they share ~/.ralqlator)
static TEST_LOCK: Mutex<()> = Mutex::new(());

/// Get the storage file path (~/.ralqlator)
fn get_storage_path() -> PathBuf {
    let home = std::env::var("HOME").expect("HOME environment variable must be set");
    PathBuf::from(home).join(".ralqlator")
}

/// Clean up storage file
fn cleanup_storage() {
    let path = get_storage_path();
    if path.exists() {
        let _ = fs::remove_file(&path);
    }
    std::thread::sleep(std::time::Duration::from_millis(50));
}

/// Run CLI command and capture output
fn run_cli(args: &[&str]) -> (String, String, bool) {
    let output = Command::new("cargo")
        .args(["run", "--"])
        .args(args)
        .output()
        .expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.success())
}

/// Wait for storage file to be created
fn wait_for_storage(timeout_ms: u64) -> bool {
    for _ in 0..(timeout_ms / 10) {
        if get_storage_path().exists() {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    false
}

// ==================== Create Function Tests ====================

#[test]
fn test_create_functions() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    // Test simple function
    let (stdout, stderr, success) = run_cli(&["-c", "func f(x) = x + 1"]);
    assert!(success, "Failed to create function: {}\nstdout: {}", stderr, stdout);
    assert!(stdout.contains("Function 'f' created and saved"));
    assert!(wait_for_storage(500));
    
    let content = fs::read_to_string(get_storage_path()).unwrap();
    assert!(content.contains("[functions.f]"));
    assert!(content.contains("expr = \"x + 1\""));
    
    // Test usage
    let (out, err, ok) = run_cli(&["f(5)"]);
    assert!(ok, "Failed: {}\n{}", err, out);
    assert!(out.contains("6"), "Expected 6, got: {}", out);
    
    cleanup_storage();
    
    // Test multiple params
    run_cli(&["-c", "func add(a, b) = a + b"]);
    let (out, err, ok) = run_cli(&["add(3, 7)"]);
    assert!(ok, "Failed: {}\n{}", err, out);
    assert!(out.contains("10"));
    cleanup_storage();
    
    // Test complex expression
    run_cli(&["-c", "func quadratic(x) = x * x + 2 * x + 1"]);
    let (out, err, ok) = run_cli(&["quadratic(3)"]);
    assert!(ok, "Failed: {}\n{}", err, out);
    assert!(out.contains("16")); // 3*3 + 2*3 + 1 = 16
    cleanup_storage();
    
    // Test short syntax
    let (stdout, stderr, success) = run_cli(&["-c", "fn square(x) = x * x"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    let (out, err, ok) = run_cli(&["square(7)"]);
    assert!(ok, "Failed: {}\n{}", err, out);
    assert!(out.contains("49"));
    cleanup_storage();
}

#[test]
fn test_create_sequences() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    // Test sequence
    let (stdout, stderr, success) = run_cli(&["-c", "seq triangle(n) = n * (n + 1) / 2"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    assert!(stdout.contains("Sequence 'triangle' created and saved"));
    
    let (out, err, ok) = run_cli(&["triangle(10)"]);
    assert!(ok, "Failed: {}\n{}", err, out);
    assert!(out.contains("55")); // 10*11/2 = 55
    cleanup_storage();
    
    // Test short syntax
    let (stdout, stderr, success) = run_cli(&["-c", "s square(n) = n * n"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    cleanup_storage();
}

#[test]
fn test_create_constants() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    // Test constant
    let (stdout, stderr, success) = run_cli(&["-c", "const G 9.81"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    assert!(stdout.contains("Constant 'G'"));
    assert!(stdout.contains("9.81"));
    
    let content = fs::read_to_string(get_storage_path()).unwrap();
    assert!(content.contains("G = 9.81"));
    
    let (out, err, ok) = run_cli(&["G * 10"]);
    assert!(ok, "Failed: {}\n{}", err, out);
    assert!(out.contains("98.1"));
    cleanup_storage();
    
    // Test short syntax
    let (stdout, stderr, success) = run_cli(&["-c", "c MY_CONST 42"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    assert!(stdout.contains("Constant 'MY_CONST'"));
    cleanup_storage();
    
    // Test float value
    let (stdout, stderr, success) = run_cli(&["-c", "const PI 3.14159"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    assert!(stdout.contains("3.14159"));
    cleanup_storage();
    
    // Test large number
    let (stdout, stderr, success) = run_cli(&["-c", "const BIG 123456789.123456789"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    assert!(stdout.contains("BIG"));
    cleanup_storage();
    
    // Test negative
    let (stdout, stderr, success) = run_cli(&["-c", "const NEG -42.5"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    assert!(stdout.contains("NEG"));
    cleanup_storage();
}

#[test]
fn test_create_invalid() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    // Test invalid syntax
    let (_stdout, stderr, success) = run_cli(&["-c", "invalid syntax"]);
    assert!(!success, "Should fail with invalid syntax");
    assert!(stderr.contains("Usage:") || stderr.contains("Error"));
    
    // Test invalid number
    let (_stdout, _stderr, success) = run_cli(&["-c", "const BAD notanumber"]);
    assert!(!success, "Should fail with invalid number");
    
    cleanup_storage();
}

// ==================== List Command Tests ====================

#[test]
fn test_list_operations() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    // Test empty list
    let (stdout, _stderr, success) = run_cli(&["-L"]);
    assert!(success, "Failed to list");
    assert!(stdout.contains("No user definitions found"));
    
    // Test with function
    run_cli(&["-c", "func f(x) = x * 2"]);
    let (stdout, l_err, l_ok) = run_cli(&["-L"]);
    assert!(l_ok, "Failed: {}", l_err);
    assert!(stdout.contains("User definitions"));
    assert!(stdout.contains("f(x) = x * 2"));
    
    // Test with multiple definitions
    run_cli(&["-c", "seq triangle(n) = n * (n + 1) / 2"]);
    run_cli(&["-c", "const G 9.81"]);
    
    let (stdout, stderr, success) = run_cli(&["-L"]);
    assert!(success, "Failed: {}", stderr);
    assert!(stdout.contains("3 items"));
    assert!(stdout.contains("Functions:"));
    assert!(stdout.contains("Constants:"));
    
    cleanup_storage();
}

// ==================== Destroy Command Tests ====================

#[test]
fn test_destroy_operations() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    // Test destroy function
    run_cli(&["-c", "func f(x) = x * 2"]);
    let (stdout, d_err, d_ok) = run_cli(&["-d", "f"]);
    assert!(d_ok, "Failed: {}", d_err);
    assert!(stdout.contains("Definition 'f' deleted"));
    
    let (list_out, _, _) = run_cli(&["-L"]);
    assert!(!list_out.contains("f(x)"));
    cleanup_storage();
    
    // Test destroy constant
    run_cli(&["-c", "const G 9.81"]);
    let (stdout, stderr, success) = run_cli(&["-d", "G"]);
    assert!(success, "Failed: {}", stderr);
    assert!(stdout.contains("Definition 'G' deleted"));
    cleanup_storage();
    
    // Test destroy nonexistent
    let (_stdout, stderr, success) = run_cli(&["-d", "nonexistent"]);
    assert!(!success, "Should fail for nonexistent");
    assert!(stderr.contains("not found"));
    cleanup_storage();
    
    // Test destroy multiple sequential
    run_cli(&["-c", "func f(x) = x"]);
    run_cli(&["-c", "func g(x) = x * 2"]);
    run_cli(&["-c", "const C 100"]);
    
    run_cli(&["-d", "f"]);
    run_cli(&["-d", "g"]);
    run_cli(&["-d", "C"]);
    
    let (stdout, _, _) = run_cli(&["-L"]);
    assert!(stdout.contains("No user definitions found"));
    cleanup_storage();
    
    // Test case sensitivity
    run_cli(&["-c", "const CaseTest 100"]);
    let (_, stderr, success) = run_cli(&["-d", "casetest"]);
    assert!(!success, "Should fail with wrong case");
    assert!(stderr.contains("not found"));
    
    let (stdout, _, success2) = run_cli(&["-d", "CaseTest"]);
    assert!(success2, "Should succeed with correct case");
    assert!(stdout.contains("deleted"));
    cleanup_storage();
}

// ==================== Auto-save Tests ====================

#[test]
fn test_auto_save() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    // Test auto-save after create function
    run_cli(&["-c", "func auto_f(x) = x + 10"]);
    assert!(wait_for_storage(500));
    
    let content = fs::read_to_string(get_storage_path()).unwrap();
    assert!(content.contains("auto_f"));
    assert!(content.contains("x + 10"));
    cleanup_storage();
    
    // Test auto-save after create constant
    run_cli(&["-c", "const AUTO_CONST 123.456"]);
    let content = fs::read_to_string(get_storage_path()).unwrap();
    assert!(content.contains("AUTO_CONST"));
    assert!(content.contains("123.456"));
    cleanup_storage();
    
    // Test auto-save after destroy
    run_cli(&["-c", "func temp(x) = x"]);
    assert!(get_storage_path().exists());
    
    run_cli(&["-d", "temp"]);
    
    let content = fs::read_to_string(get_storage_path()).unwrap();
    assert!(!content.contains("temp"));
    cleanup_storage();
}

// ==================== Auto-load Tests ====================

#[test]
fn test_auto_load() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    // Test auto-load in CLI calculation
    run_cli(&["-c", "func loaded_f(x) = x * 100"]);
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let (stdout, stderr, success) = run_cli(&["loaded_f(5)"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    assert!(stdout.contains("500"));
    cleanup_storage();
    
    // Test auto-load constant
    run_cli(&["-c", "const LOADED_G 50"]);
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let (stdout, stderr, success) = run_cli(&["LOADED_G * 2"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    assert!(stdout.contains("100"));
    cleanup_storage();
    
    // Test auto-load multiple definitions
    run_cli(&["-c", "func a(x) = x + 1"]);
    run_cli(&["-c", "func b(x) = x * 2"]);
    run_cli(&["-c", "const C1 10"]);
    run_cli(&["-c", "const C2 20"]);
    std::thread::sleep(std::time::Duration::from_millis(100));
    
    let (out1, err1, ok1) = run_cli(&["a(5)"]);
    assert!(ok1, "Failed: {}\n{}", err1, out1);
    assert!(out1.contains("6"));
    
    let (out2, err2, ok2) = run_cli(&["b(5)"]);
    assert!(ok2, "Failed: {}\n{}", err2, out2);
    assert!(out2.contains("10"));
    
    let (out3, err3, ok3) = run_cli(&["C1 + C2"]);
    assert!(ok3, "Failed: {}\n{}", err3, out3);
    assert!(out3.contains("30"));
    cleanup_storage();
}

// ==================== Integration Tests ====================

#[test]
fn test_full_workflow() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    // Create multiple definitions
    let (out1, err1, ok1) = run_cli(&["-c", "func multiply(x, y) = x * y"]);
    assert!(ok1, "Failed: {}\n{}", err1, out1);
    
    let (out2, err2, ok2) = run_cli(&["-c", "seq arithmetic(n) = n * 2"]);
    assert!(ok2, "Failed: {}\n{}", err2, out2);
    
    let (out3, err3, ok3) = run_cli(&["-c", "const SPEED 300000"]);
    assert!(ok3, "Failed: {}\n{}", err3, out3);
    
    // List all
    let (list_out, list_err, list_ok) = run_cli(&["-L"]);
    assert!(list_ok, "Failed: {}", list_err);
    assert!(list_out.contains("3 items"));
    
    // Use in calculations
    let (calc1, calc1_err, calc1_ok) = run_cli(&["multiply(6, 7)"]);
    assert!(calc1_ok, "Failed: {}\n{}", calc1_err, calc1);
    assert!(calc1.contains("42"));
    
    let (calc2, calc2_err, calc2_ok) = run_cli(&["SPEED * 2"]);
    assert!(calc2_ok, "Failed: {}\n{}", calc2_err, calc2);
    assert!(calc2.contains("600000"));
    
    // Delete one
    let (del_out, del_err, del_ok) = run_cli(&["-d", "multiply"]);
    assert!(del_ok, "Failed: {}\n{}", del_err, del_out);
    assert!(del_out.contains("deleted"));
    
    // Verify deletion
    let (list2, _, _) = run_cli(&["-L"]);
    assert!(!list2.contains("multiply("));
    
    cleanup_storage();
}

#[test]
fn test_persistence_across_sessions() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    // Session 1: Create definitions
    run_cli(&["-c", "func persistent_func(x) = x + 999"]);
    run_cli(&["-c", "const PERSISTENT_CONST 777"]);
    
    // Verify storage file
    assert!(wait_for_storage(500));
    let content1 = fs::read_to_string(get_storage_path()).unwrap();
    assert!(content1.contains("persistent_func"));
    assert!(content1.contains("PERSISTENT_CONST"));
    
    // Session 2: Use definitions
    let (stdout, stderr, success) = run_cli(&["persistent_func(1)"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    assert!(stdout.contains("1000"));
    
    let (stdout2, stderr2, success2) = run_cli(&["PERSISTENT_CONST + 223"]);
    assert!(success2, "Failed: {}\n{}", stderr2, stdout2);
    assert!(stdout2.contains("1000"));
    
    cleanup_storage();
}

#[test]
fn test_storage_file_format() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    run_cli(&["-c", "func fmt_test(x) = x * 2"]);
    run_cli(&["-c", "const FMT_CONST 42.5"]);
    
    let content = fs::read_to_string(get_storage_path()).unwrap();
    
    assert!(content.contains("[functions.fmt_test]"));
    assert!(content.contains("params = "));
    assert!(content.contains("expr = "));
    assert!(content.contains("[constants]"));
    assert!(content.contains("FMT_CONST = 42.5"));
    cleanup_storage();
}

#[test]
fn test_create_and_use_same_session() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    run_cli(&["-c", "func immediate(x) = x * 7"]);
    let (stdout, stderr, success) = run_cli(&["immediate(8)"]);
    
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    assert!(stdout.contains("56"));
    cleanup_storage();
}

#[test]
fn test_destroy_then_recreate() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    // Create
    run_cli(&["-c", "func recycle(x) = x + 1"]);
    let (out1, err1, ok1) = run_cli(&["recycle(10)"]);
    assert!(ok1, "Failed: {}\n{}", err1, out1);
    assert!(out1.contains("11"));
    
    // Destroy
    run_cli(&["-d", "recycle"]);
    
    // Try to use (should fail)
    let (_, _stderr, success) = run_cli(&["recycle(10)"]);
    assert!(!success, "Should fail after destroy");
    
    // Recreate
    run_cli(&["-c", "func recycle(x) = x * 100"]);
    let (out2, err2, success2) = run_cli(&["recycle(10)"]);
    assert!(success2, "Failed: {}\n{}", err2, out2);
    assert!(out2.contains("1000"));
    
    cleanup_storage();
}

#[test]
fn test_list_after_destroy_all() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    run_cli(&["-c", "func f1(x) = x"]);
    run_cli(&["-c", "func f2(x) = x * 2"]);
    run_cli(&["-c", "const C1 1"]);
    
    run_cli(&["-d", "f1"]);
    run_cli(&["-d", "f2"]);
    run_cli(&["-d", "C1"]);
    
    let (stdout, _, success) = run_cli(&["-L"]);
    assert!(success);
    assert!(stdout.contains("No user definitions found"));
    cleanup_storage();
}

#[test]
fn test_multiple_create_same_type() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    run_cli(&["-c", "func func1(x) = x + 1"]);
    run_cli(&["-c", "func func2(x) = x + 2"]);
    run_cli(&["-c", "func func3(x) = x + 3"]);
    
    let (stdout, _, success) = run_cli(&["-L"]);
    assert!(success);
    assert!(stdout.contains("func1"));
    assert!(stdout.contains("func2"));
    assert!(stdout.contains("func3"));
    
    cleanup_storage();
}

// ==================== REPL Tests ====================

#[test]
fn test_repl_destroy() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    run_cli(&["-c", "func repl_test(x) = x * 3"]);
    
    let output = Command::new("bash")
        .arg("-c")
        .arg("printf 'destroy repl_test\\nquit\\n' | cargo run -- 2>&1")
        .output()
        .expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    
    assert!(stdout.contains("Deleted") || stdout.contains("deleted"),
            "Should show deleted: {}", stdout);
    cleanup_storage();
}

#[test]
fn test_repl_destroy_nonexistent() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    let output = Command::new("bash")
        .arg("-c")
        .arg("printf 'destroy nonexistent\\nquit\\n' | cargo run -- 2>&1")
        .output()
        .expect("Failed to execute command");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    
    assert!(stdout.contains("not found"), "Should show not found: {}", stdout);
    cleanup_storage();
}

// ==================== Edge Cases ====================

#[test]
fn test_create_with_spaces() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    
    let (stdout, stderr, success) = run_cli(&["-c", "func spaced_func( x , y ) = x + y"]);
    assert!(success, "Failed: {}\n{}", stderr, stdout);
    assert!(stdout.contains("created and saved"));
    cleanup_storage();
}

// ==================== Cleanup ====================

#[test]
fn test_final_cleanup() {
    let _guard = TEST_LOCK.lock().unwrap();
    cleanup_storage();
    std::thread::sleep(std::time::Duration::from_millis(100));
    assert!(!get_storage_path().exists(), "Storage file should be cleaned up");
}
