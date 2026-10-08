//! Local Grind core: problem model, judge, language drivers, persistence and metrics.
//! The Tauri shell is a thin layer over [`service::Service`].

pub mod catalog;
pub mod error;
pub mod judge;
pub mod lang;
pub mod metrics;
pub mod portable;
pub mod runner;
pub mod service;
pub mod settings;
pub mod timeutil;
pub mod types;
pub mod userdb;

pub use error::{Error, Result};

use serde::{Deserialize, Serialize};

/// Which build this is: the SemVer version (the workspace `Cargo.toml` is the single source of
/// truth) and the git commit it was built from, when known.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildInfo {
    pub version: String,
    pub commit: Option<String>,
}

impl BuildInfo {
    pub fn current() -> BuildInfo {
        BuildInfo {
            version: env!("CARGO_PKG_VERSION").into(),
            commit: option_env!("LOCALGRIND_COMMIT").map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_info_is_semver_with_the_commit() {
        let b = BuildInfo::current();
        let parts: Vec<&str> = b.version.split('.').collect();
        assert_eq!(parts.len(), 3, "{}", b.version);
        assert!(
            parts.iter().all(|p| p.parse::<u64>().is_ok()),
            "{}",
            b.version
        );
        // Built from a git checkout here, so the commit is recorded.
        let commit = b.commit.expect("commit recorded by build.rs");
        assert!(commit.len() >= 7 && commit.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
