// Auto-generated version information
// DO NOT EDIT MANUALLY

/// Ralqlator version string
pub const VERSION: &str = "v0.2.0";

/// Git commit hash
pub const GIT_COMMIT: &str = "26f10fd";

/// Git commit date
pub const GIT_DATE: &str = "2026-03-25";

/// Build timestamp
pub const BUILD_TIME: &str = "2026-03-25 15:47:18 UTC";

/// Rust version used for building
pub const RUST_VERSION: &str = "rustc 1.94.0 (4a4ef493e 2026-03-02)";

/// Full version string
pub fn get_full_version() -> String {
    format!("{} (commit {}, built {})", VERSION, GIT_COMMIT, BUILD_TIME)
}

/// Version information for display
pub fn get_version_info() -> String {
    format!(
        "Ralqlator {}\n         Git Commit: {} ({})\n         Build Time: {}\n         Rust Version: {}",
        VERSION, GIT_COMMIT, GIT_DATE, BUILD_TIME, RUST_VERSION
    )
}
