// Ralqlator Build Script
// Generates version information from git during build

use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;
use chrono::{DateTime, Local};

fn main() {
    // Get git commit hash
    let git_commit = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get git tag
    let git_tag = Command::new("git")
        .args(["describe", "--tags", "--always"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get git commit date
    let git_date = Command::new("git")
        .args(["log", "-1", "--format=%cd", "--date=short"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get rust version
    let rust_version = Command::new("rustc")
        .args(["--version"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    // Get build timestamp using chrono
    let build_time = get_build_timestamp();

    // Set environment variables for use in the code
    println!("cargo:rustc-env=GIT_TAG={}", git_tag);
    println!("cargo:rustc-env=GIT_COMMIT={}", git_commit);
    println!("cargo:rustc-env=GIT_DATE={}", git_date);
    println!("cargo:rustc-env=RUST_VERSION={}", rust_version);
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);

    // Generate version_info.rs file
    let out_dir = env::var("OUT_DIR").unwrap_or_else(|_| ".".to_string());
    let dest_path = Path::new(&out_dir).join("version_info.rs");
    
    let version_content = format!(
        r#"// Auto-generated version information
// DO NOT EDIT MANUALLY
// Generated at: {}

/// Ralqlator version string
pub const VERSION: &str = "{}";

/// Git commit hash
pub const GIT_COMMIT: &str = "{}";

/// Git commit date
pub const GIT_DATE: &str = "{}";

/// Build timestamp
pub const BUILD_TIME: &str = "{}";

/// Rust version used for building
pub const RUST_VERSION: &str = "{}";

/// Full version string
pub fn get_full_version() -> String {{
    format!("{{}} (commit {{}}, built {{}})", VERSION, GIT_COMMIT, BUILD_TIME)
}}

/// Version information for display
pub fn get_version_info() -> String {{
    format!(
        "Ralqlator {{}}\\n\
         Git Commit: {{}} ({{}})\\n\
         Build Time: {{}}\\n\
         Rust Version: {{}}",
        VERSION, GIT_COMMIT, GIT_DATE, BUILD_TIME, RUST_VERSION
    )
}}
"#,
        build_time, git_tag, git_commit, git_date, build_time, rust_version
    );
    
    fs::write(&dest_path, version_content).expect("Failed to write version_info.rs");

    // Re-run if git HEAD changes
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/heads");
    println!("cargo:rerun-if-changed=.git/refs/tags");
}

/// Get build timestamp using chrono
/// Supports SOURCE_DATE_EPOCH for reproducible builds
fn get_build_timestamp() -> String {
    // Use SOURCE_DATE_EPOCH for reproducible builds if set
    if let Ok(epoch) = env::var("SOURCE_DATE_EPOCH") {
        if let Ok(secs) = epoch.parse::<i64>() {
            let datetime = DateTime::from_timestamp(secs, 0)
                .map(|dt| dt.with_timezone(&Local))
                .unwrap_or_else(|| Local::now());
            return datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string();
        }
    }
    
    // Use current local time
    let now = Local::now();
    now.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}
