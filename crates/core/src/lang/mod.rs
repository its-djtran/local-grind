//! Language drivers. A driver turns a [`Signature`] into starter code, and a user's code into a
//! runnable [`Program`] whose harness streams one result line per test case.
//!
//! Harness protocol (shared by every language):
//! * the program is started as `<run command> <input-file>`; the input file holds one JSON array
//!   of arguments per line (for design problems: `[ops, args]`);
//! * after each test the harness flushes user stdout and writes
//!   `\x1eLG {"i":<index>,"out":<json>,"ns":<nanoseconds>}` on its own line;
//! * an exception produces `\x1eLG {"i":<index>,"error":"<message>"}` and the harness exits;
//! * a source error detected by an interpreted language produces `\x1eLG {"compile_error":"..."}`.
//!
//! Anything else on stdout is user output and is attributed to the test that follows it.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::types::Signature;

pub mod python;

pub const RESULT_SENTINEL: &str = "\x1eLG ";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Python,
}

impl Language {
    pub const ALL: &'static [Language] = &[Language::Python];

    pub fn id(self) -> &'static str {
        match self {
            Language::Python => "python",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Language::Python => "Python3",
        }
    }

    pub fn driver(self) -> &'static dyn Driver {
        match self {
            Language::Python => &python::PythonDriver,
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.id())
    }
}

impl FromStr for Language {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Language::ALL
            .iter()
            .copied()
            .find(|l| l.id() == s)
            .ok_or_else(|| Error::Invalid(format!("unknown language `{s}`")))
    }
}

/// Source files plus the commands needed to compile (optional) and run them. Commands run with
/// the program directory as their working directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub files: Vec<(String, String)>,
    pub compile: Option<Vec<String>>,
    pub run: Vec<String>,
}

impl Program {
    pub fn write_to(&self, dir: &Path) -> Result<()> {
        for (name, contents) in &self.files {
            std::fs::write(dir.join(name), contents)?;
        }
        Ok(())
    }
}

/// Resolved executables for each language's toolchain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Toolchains {
    pub python: PathBuf,
}

impl Default for Toolchains {
    fn default() -> Self {
        Toolchains {
            python: PathBuf::from("python3"),
        }
    }
}

/// Directories searched for toolchains in addition to `$PATH`. Apps launched from Finder get a
/// minimal `PATH`, so Homebrew and other common prefixes are probed explicitly.
pub const EXTRA_SEARCH_DIRS: &[&str] = &[
    "/opt/homebrew/bin",
    "/usr/local/bin",
    "/usr/bin",
    "/opt/local/bin",
];

/// Finds `name` on `path_var` (a `PATH`-style list) or in [`EXTRA_SEARCH_DIRS`].
pub fn find_executable(name: &str, path_var: Option<&str>) -> Option<PathBuf> {
    let dirs = path_var
        .map(|p| std::env::split_paths(p).collect::<Vec<_>>())
        .unwrap_or_default();
    dirs.into_iter()
        .chain(EXTRA_SEARCH_DIRS.iter().map(PathBuf::from))
        .map(|d| d.join(name))
        .find(|p| p.is_file())
}

impl Toolchains {
    /// Detects toolchains from the environment, falling back to bare command names.
    pub fn detect() -> Toolchains {
        let path = std::env::var("PATH").ok();
        Toolchains {
            python: find_executable("python3", path.as_deref())
                .unwrap_or_else(|| PathBuf::from("python3")),
        }
    }
}

/// Result of probing a language's toolchain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolchainStatus {
    pub language: Language,
    pub executable: String,
    pub version: Option<String>,
    pub ok: bool,
    /// Human-readable problem and fix when `ok` is false.
    pub message: Option<String>,
}

/// Oldest Python the harness supports.
pub const MIN_PYTHON: (u32, u32) = (3, 8);

/// Parses `Python 3.12.1` (or `3.12`) into `(3, 12)`.
pub fn parse_python_version(s: &str) -> Option<(u32, u32)> {
    let v = s.trim().trim_start_matches("Python").trim();
    let mut parts = v.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts
        .next()?
        .trim_end_matches(|c: char| !c.is_ascii_digit())
        .parse()
        .ok()?;
    Some((major, minor))
}

/// Runs `<exe> --version` and judges whether the toolchain is usable.
pub fn probe(language: Language, tools: &Toolchains) -> ToolchainStatus {
    let exe = match language {
        Language::Python => &tools.python,
    };
    let executable = exe.to_string_lossy().into_owned();
    let output = std::process::Command::new(exe)
        .arg("--version")
        .stdin(std::process::Stdio::null())
        .output();
    let fail = |message: String, version: Option<String>| ToolchainStatus {
        language,
        executable: executable.clone(),
        version,
        ok: false,
        message: Some(message),
    };
    let out = match output {
        Ok(out) if out.status.success() => out,
        Ok(out) => {
            return fail(
                format!("`{executable} --version` failed: {}", String::from_utf8_lossy(&out.stderr).trim()),
                None,
            )
        }
        Err(e) => {
            return fail(
                format!(
                    "{} was not found ({e}). Install Python 3 (for example `brew install python`) or set its path in Settings.",
                    executable
                ),
                None,
            )
        }
    };
    // Python 2 printed its version to stderr.
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let version = text.trim().to_string();
    match parse_python_version(&version) {
        Some(v) if v >= MIN_PYTHON => ToolchainStatus {
            language,
            executable,
            version: Some(version),
            ok: true,
            message: None,
        },
        Some(_) => fail(
            format!(
                "{version} is too old; Python {}.{} or newer is required.",
                MIN_PYTHON.0, MIN_PYTHON.1
            ),
            Some(version.clone()),
        ),
        None => fail(
            format!("could not understand version output `{version}`"),
            Some(version.clone()),
        ),
    }
}

pub trait Driver: Sync {
    fn language(&self) -> Language;
    fn starter_code(&self, sig: &Signature) -> String;
    fn program(&self, sig: &Signature, user_code: &str, tools: &Toolchains) -> Program;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_ids_roundtrip() {
        for l in Language::ALL {
            assert_eq!(l.id().parse::<Language>().unwrap(), *l);
            assert_eq!(l.to_string(), l.id());
            assert!(!l.display_name().is_empty());
            assert_eq!(l.driver().language(), *l);
        }
        assert!("cobol".parse::<Language>().is_err());
    }

    #[test]
    fn finds_executables() {
        let dir = tempfile::tempdir().unwrap();
        let exe = dir.path().join("lg-fake-tool");
        std::fs::write(&exe, "").unwrap();
        let path = dir.path().to_string_lossy().to_string();
        assert_eq!(find_executable("lg-fake-tool", Some(&path)), Some(exe));
        assert_eq!(find_executable("lg-missing-tool", Some(&path)), None);
        assert_eq!(find_executable("lg-missing-tool", None), None);
    }

    #[test]
    fn detects_python() {
        let t = Toolchains::detect();
        assert!(t.python.to_string_lossy().contains("python3"));
        assert_eq!(Toolchains::default().python, PathBuf::from("python3"));
    }

    #[test]
    fn probes_toolchains() {
        let ok = probe(Language::Python, &Toolchains::detect());
        assert!(ok.ok, "{ok:?}");
        assert!(ok.version.unwrap().starts_with("Python 3"));

        let missing = probe(
            Language::Python,
            &Toolchains {
                python: "/nonexistent/python3".into(),
            },
        );
        assert!(!missing.ok);
        assert!(missing.message.unwrap().contains("was not found"));

        let dir = tempfile::tempdir().unwrap();
        let fake = |name: &str, script: &str| {
            let p = dir.path().join(name);
            std::fs::write(&p, format!("#!/bin/sh\n{script}\n")).unwrap();
            std::fs::set_permissions(&p, std::os::unix::fs::PermissionsExt::from_mode(0o755))
                .unwrap();
            probe(Language::Python, &Toolchains { python: p })
        };
        let old = fake("old", "echo 'Python 2.7.18' >&2");
        assert!(!old.ok && old.message.unwrap().contains("too old"));
        let weird = fake("weird", "echo hello");
        assert!(weird.message.unwrap().contains("could not understand"));
        let broken = fake("broken", "echo nope >&2; exit 3");
        assert!(broken.message.unwrap().contains("failed: nope"));
    }

    #[test]
    fn parses_python_versions() {
        assert_eq!(parse_python_version("Python 3.12.1"), Some((3, 12)));
        assert_eq!(parse_python_version("3.9"), Some((3, 9)));
        assert_eq!(parse_python_version("Python 3.13.0rc1"), Some((3, 13)));
        assert_eq!(parse_python_version("Python 3.14b"), Some((3, 14)));
        assert_eq!(parse_python_version("Python"), None);
        assert_eq!(parse_python_version("x.y"), None);
    }

    #[test]
    fn program_writes_files() {
        let dir = tempfile::tempdir().unwrap();
        let p = Program {
            files: vec![("a.txt".into(), "hello".into())],
            compile: None,
            run: vec![],
        };
        p.write_to(dir.path()).unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.path().join("a.txt")).unwrap(),
            "hello"
        );
        assert!(p.write_to(&dir.path().join("missing")).is_err());
    }
}
