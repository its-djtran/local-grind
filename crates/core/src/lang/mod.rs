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
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::error::{Error, Result};
use crate::types::Signature;

pub mod cpp;
pub mod javascript;
pub mod python;
pub mod rust;

pub const RESULT_SENTINEL: &str = "\x1eLG ";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    Python,
    JavaScript,
    Cpp,
    Rust,
}

impl Language {
    pub const ALL: &'static [Language] = &[
        Language::Python,
        Language::JavaScript,
        Language::Cpp,
        Language::Rust,
    ];

    pub fn id(self) -> &'static str {
        match self {
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::Cpp => "cpp",
            Language::Rust => "rust",
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            Language::Python => "Python3",
            Language::JavaScript => "JavaScript",
            Language::Cpp => "C++",
            Language::Rust => "Rust",
        }
    }

    pub fn driver(self) -> &'static dyn Driver {
        match self {
            Language::Python => &python::PythonDriver,
            Language::JavaScript => &javascript::JavaScriptDriver,
            Language::Cpp => &cpp::CppDriver,
            Language::Rust => &rust::RustDriver,
        }
    }

    /// Executable looked up on the search path when no explicit path is configured.
    pub fn executable_name(self) -> &'static str {
        match self {
            Language::Python => "python3",
            Language::JavaScript => "node",
            Language::Cpp => "clang++",
            Language::Rust => "rustc",
        }
    }

    /// Oldest toolchain version the harness supports.
    pub fn min_version(self) -> (u32, u32) {
        match self {
            Language::Python => (3, 8),
            Language::JavaScript => (16, 0),
            Language::Cpp => (10, 0),
            Language::Rust => (1, 70),
        }
    }

    /// Source file extension used for solutions in this language.
    pub fn extension(self) -> &'static str {
        match self {
            Language::Python => "py",
            Language::JavaScript => "js",
            Language::Cpp => "cpp",
            Language::Rust => "rs",
        }
    }

    pub fn from_extension(ext: &str) -> Option<Language> {
        match ext {
            "cc" | "cxx" => Some(Language::Cpp),
            _ => Language::ALL.iter().copied().find(|l| l.extension() == ext),
        }
    }

    pub fn install_hint(self) -> &'static str {
        match self {
            Language::Python => "Install Python 3 (for example `brew install python`)",
            Language::JavaScript => "Install Node.js (for example `brew install node`)",
            Language::Cpp => "Install the Xcode Command Line Tools (`xcode-select --install`)",
            Language::Rust => "Install Rust with rustup (https://rustup.rs)",
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
    /// Files produced by `compile` that can be cached and reused for identical sources.
    pub artifacts: Vec<String>,
}

impl Program {
    pub fn write_to(&self, dir: &Path) -> Result<()> {
        for (name, contents) in &self.files {
            std::fs::write(dir.join(name), contents)?;
        }
        Ok(())
    }

    /// Stable key over everything that determines the compiled artifacts (FNV-1a, 64-bit).
    pub fn cache_key(&self) -> String {
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        let mut eat = |bytes: &[u8]| {
            for b in bytes {
                h ^= u64::from(*b);
                h = h.wrapping_mul(0x0100_0000_01b3);
            }
            h ^= 0xff;
            h = h.wrapping_mul(0x0100_0000_01b3);
        };
        for (name, contents) in &self.files {
            eat(name.as_bytes());
            eat(contents.as_bytes());
        }
        for part in self.compile.iter().flatten() {
            eat(part.as_bytes());
        }
        format!("{h:016x}")
    }
}

/// Resolved executables for each language's toolchain.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Toolchains {
    paths: BTreeMap<Language, PathBuf>,
}

/// Directories searched for toolchains in addition to `$PATH`. Apps launched from Finder get a
/// minimal `PATH`, so Homebrew, rustup and other common prefixes are probed explicitly.
pub const EXTRA_SEARCH_DIRS: &[&str] = &[
    "/opt/homebrew/bin",
    "/usr/local/bin",
    "/usr/bin",
    "/opt/local/bin",
];

/// Finds `name` on `path_var` (a `PATH`-style list), in `~/.cargo/bin`, or in
/// [`EXTRA_SEARCH_DIRS`].
pub fn find_executable(name: &str, path_var: Option<&str>) -> Option<PathBuf> {
    let dirs = path_var
        .map(|p| std::env::split_paths(p).collect::<Vec<_>>())
        .unwrap_or_default();
    let cargo = std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".cargo/bin"));
    dirs.into_iter()
        .chain(cargo)
        .chain(EXTRA_SEARCH_DIRS.iter().map(PathBuf::from))
        .map(|d| d.join(name))
        .find(|p| p.is_file())
}

impl Toolchains {
    /// Detects every toolchain from the environment.
    pub fn detect() -> Toolchains {
        let path = std::env::var("PATH").ok();
        let mut t = Toolchains::default();
        for &l in Language::ALL {
            if let Some(p) = find_executable(l.executable_name(), path.as_deref()) {
                t.set(l, p);
            }
        }
        t
    }

    /// The executable for `language`: configured, detected, or the bare command name.
    pub fn path(&self, language: Language) -> PathBuf {
        self.paths
            .get(&language)
            .cloned()
            .unwrap_or_else(|| PathBuf::from(language.executable_name()))
    }

    pub fn set(&mut self, language: Language, path: PathBuf) {
        self.paths.insert(language, path);
    }

    pub fn with(mut self, language: Language, path: impl Into<PathBuf>) -> Toolchains {
        self.set(language, path.into());
        self
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

/// Extracts the first `major.minor` pair from version output such as `Python 3.12.1`,
/// `v20.1.0`, `rustc 1.79.0 (...)` or `Apple clang version 15.0.0`.
pub fn parse_version(s: &str) -> Option<(u32, u32)> {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            if i + 1 < bytes.len() && bytes[i] == b'.' && bytes[i + 1].is_ascii_digit() {
                let major = s[start..i].parse().ok()?;
                let mstart = i + 1;
                let mut j = mstart;
                while j < bytes.len() && bytes[j].is_ascii_digit() {
                    j += 1;
                }
                return Some((major, s[mstart..j].parse().ok()?));
            }
        }
        i += 1;
    }
    None
}

/// Runs `<exe> --version` and judges whether the toolchain is usable.
pub fn probe(language: Language, tools: &Toolchains) -> ToolchainStatus {
    let exe = tools.path(language);
    let executable = exe.to_string_lossy().into_owned();
    let output = std::process::Command::new(&exe)
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
                format!(
                    "`{executable} --version` failed: {}",
                    String::from_utf8_lossy(&out.stderr).trim()
                ),
                None,
            )
        }
        Err(e) => {
            return fail(
                format!(
                    "{executable} was not found ({e}). {} or set its path in Settings.",
                    language.install_hint()
                ),
                None,
            )
        }
    };
    // Some tools (Python 2) print their version to stderr.
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    let version = text.lines().next().unwrap_or_default().trim().to_string();
    let (min_major, min_minor) = language.min_version();
    match parse_version(&version) {
        Some(v) if v >= (min_major, min_minor) => ToolchainStatus {
            language,
            executable,
            version: Some(version),
            ok: true,
            message: None,
        },
        Some(_) => fail(
            format!(
                "{version} is too old; {} {min_major}.{min_minor} or newer is required.",
                language.display_name()
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
    /// Whether this language can express the problem (LeetCode omits some languages for some
    /// problems, e.g. Rust for graph-node problems).
    fn supports(&self, _sig: &Signature) -> bool {
        true
    }
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
            let any: Signature = serde_json::from_value(serde_json::json!({"kind": "function",
                "name": "f", "params": [], "returns": "int"}))
            .unwrap();
            assert!(l.driver().supports(&any));
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
    fn detects_toolchains() {
        let t = Toolchains::detect();
        assert!(t
            .path(Language::Python)
            .to_string_lossy()
            .contains("python3"));
        assert_eq!(
            Toolchains::default().path(Language::Python),
            PathBuf::from("python3")
        );
        let custom = Toolchains::default().with(Language::Python, "/x/py");
        assert_eq!(custom.path(Language::Python), PathBuf::from("/x/py"));
        for l in Language::ALL {
            assert!(!l.executable_name().is_empty());
            assert!(!l.install_hint().is_empty());
            assert!(l.min_version() > (0, 0));
            assert_eq!(Language::from_extension(l.extension()), Some(*l));
        }
    }

    #[test]
    fn extensions() {
        assert_eq!(Language::from_extension("cc"), Some(Language::Cpp));
        assert_eq!(Language::from_extension("java"), None);
    }

    #[test]
    fn cache_keys() {
        let p = Program {
            files: vec![("a".into(), "1".into())],
            compile: Some(vec!["cc".into()]),
            run: vec![],
            artifacts: vec![],
        };
        let mut q = p.clone();
        assert_eq!(p.cache_key(), q.cache_key());
        q.files[0].1 = "2".into();
        assert_ne!(p.cache_key(), q.cache_key());
        let mut r = p.clone();
        r.compile = Some(vec!["c".into(), "c".into()]);
        assert_ne!(p.cache_key(), r.cache_key());
        assert_eq!(p.cache_key().len(), 16);
    }

    #[test]
    fn probes_toolchains() {
        let ok = probe(Language::Python, &Toolchains::detect());
        assert!(ok.ok, "{ok:?}");
        assert!(ok.version.unwrap().starts_with("Python 3"));

        let missing = probe(
            Language::Python,
            &Toolchains::default().with(Language::Python, "/nonexistent/python3"),
        );
        assert!(!missing.ok);
        assert!(missing.message.unwrap().contains("was not found"));

        let dir = tempfile::tempdir().unwrap();
        let fake = |name: &str, script: &str| {
            let p = dir.path().join(name);
            std::fs::write(&p, format!("#!/bin/sh\n{script}\n")).unwrap();
            std::fs::set_permissions(&p, std::os::unix::fs::PermissionsExt::from_mode(0o755))
                .unwrap();
            probe(
                Language::Python,
                &Toolchains::default().with(Language::Python, p),
            )
        };
        let old = fake("old", "echo 'Python 2.7.18' >&2");
        assert!(!old.ok && old.message.unwrap().contains("too old"));
        let weird = fake("weird", "echo hello");
        assert!(weird.message.unwrap().contains("could not understand"));
        let broken = fake("broken", "echo nope >&2; exit 3");
        assert!(broken.message.unwrap().contains("failed: nope"));
    }

    #[test]
    fn parses_versions() {
        assert_eq!(parse_version("Python 3.12.1"), Some((3, 12)));
        assert_eq!(parse_version("3.9"), Some((3, 9)));
        assert_eq!(parse_version("Python 3.13.0rc1"), Some((3, 13)));
        assert_eq!(parse_version("v26.10.0"), Some((26, 10)));
        assert_eq!(
            parse_version("rustc 1.97.1 (8bab26f4f 2026-07-14)"),
            Some((1, 97))
        );
        assert_eq!(
            parse_version("Apple clang version 21.0.0 (clang-2100.3.34.2)"),
            Some((21, 0))
        );
        assert_eq!(parse_version("Python"), None);
        assert_eq!(parse_version("x.y 3."), None);
    }

    #[test]
    fn program_writes_files() {
        let dir = tempfile::tempdir().unwrap();
        let p = Program {
            files: vec![("a.txt".into(), "hello".into())],
            compile: None,
            run: vec![],
            artifacts: vec![],
        };
        p.write_to(dir.path()).unwrap();
        assert_eq!(
            std::fs::read_to_string(dir.path().join("a.txt")).unwrap(),
            "hello"
        );
        assert!(p.write_to(&dir.path().join("missing")).is_err());
    }
}
