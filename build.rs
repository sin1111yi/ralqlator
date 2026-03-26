// Ralqlator Build Script
// Generates version information from git during build

use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

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

    // Get build timestamp
    let build_time = chrono_lite_timestamp();

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

/// Generate build timestamp without external dependencies
fn chrono_lite_timestamp() -> String {
    // Use SOURCE_DATE_EPOCH for reproducible builds if set
    if let Ok(epoch) = std::env::var("SOURCE_DATE_EPOCH") {
        if let Ok(secs) = epoch.parse::<u64>() {
            return format_epoch(secs);
        }
    }
    
    // Use current system time
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    
    format_epoch(secs)
}

/// Format epoch seconds as YYYY-MM-DD HH:MM:SS UTC
fn format_epoch(secs: u64) -> String {
    // Days since epoch and time of day
    let days = secs / 86400;
    let remaining = secs % 86400;
    let hour = (remaining / 3600) as u32;
    let min = ((remaining % 3600) / 60) as u32;
    let sec = (remaining % 60) as u32;
    
    // Calculate year, month, day from days since 1970-01-01
    let (year, month, day) = days_to_ymd(days);
    
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC", year, month, day, hour, min, sec)
}

/// Convert days since 1970-01-01 to year, month, day
/// Uses a simplified algorithm that accounts for leap years
fn days_to_ymd(days: u64) -> (u32, u32, u32) {
    let mut remaining_days = days as i64;
    let mut year = 1970;
    
    // Find the year
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }
    
    // Find the month
    let days_in_months = [
        31, // January
        if is_leap_year(year) { 29 } else { 28 }, // February
        31, // March
        30, // April
        31, // May
        30, // June
        31, // July
        31, // August
        30, // September
        31, // October
        30, // November
        31, // December
    ];
    
    let mut month = 1;
    for days_in_month in days_in_months.iter() {
        if remaining_days < *days_in_month {
            break;
        }
        remaining_days -= *days_in_month;
        month += 1;
    }
    
    let day = (remaining_days + 1) as u32;
    (year, month, day)
}

/// Check if a year is a leap year
fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
