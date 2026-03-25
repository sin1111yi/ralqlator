#!/usr/bin/env bash
# Ralqlator Version Information Generator
# This script generates version information from git

set -e

# Get version from git tag
GIT_TAG=$(git describe --tags 2>/dev/null || echo "unknown")

# Get short commit hash
GIT_COMMIT=$(git rev-parse --short HEAD)

# Get commit date
GIT_DATE=$(git log -1 --format=%cd --date=short)

# Get build timestamp
BUILD_TIME=$(date -u +"%Y-%m-%d %H:%M:%S UTC")

# Get Rust version
RUST_VERSION=$(rustc --version 2>/dev/null || echo "unknown")

# Generate version module
cat > src/version_info.rs << EOF
// Auto-generated version information
// DO NOT EDIT MANUALLY

/// Ralqlator version string
pub const VERSION: &str = "${GIT_TAG}";

/// Git commit hash
pub const GIT_COMMIT: &str = "${GIT_COMMIT}";

/// Git commit date
pub const GIT_DATE: &str = "${GIT_DATE}";

/// Build timestamp
pub const BUILD_TIME: &str = "${BUILD_TIME}";

/// Rust version used for building
pub const RUST_VERSION: &str = "${RUST_VERSION}";

/// Full version string
pub fn get_full_version() -> String {
    format!("{} (commit {}, built {})", VERSION, GIT_COMMIT, BUILD_TIME)
}

/// Version information for display
pub fn get_version_info() -> String {
    format!(
        "Ralqlator {}\\n\
         Git Commit: {} ({})\\n\
         Build Time: {}\\n\
         Rust Version: {}",
        VERSION, GIT_COMMIT, GIT_DATE, BUILD_TIME, RUST_VERSION
    )
}
EOF

echo "Version information generated successfully!"
echo "  Version: ${GIT_TAG}"
echo "  Commit: ${GIT_COMMIT} (${GIT_DATE})"
echo "  Build: ${BUILD_TIME}"
