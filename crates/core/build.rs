//! Records the git commit the crate is built from as `LOCALGRIND_COMMIT` (absent outside a git
//! checkout), and rebuilds when HEAD moves so the recorded commit stays current.

use std::path::Path;
use std::process::Command;

fn git(args: &[&str]) -> Option<String> {
    let out = Command::new("git").args(args).output().ok()?;
    let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
    (out.status.success() && !text.is_empty()).then_some(text)
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if let Some(commit) = git(&["rev-parse", "--short=12", "HEAD"]) {
        println!("cargo:rustc-env=LOCALGRIND_COMMIT={commit}");
    }
    let dirs = [
        git(&["rev-parse", "--absolute-git-dir"]),
        git(&["rev-parse", "--path-format=absolute", "--git-common-dir"]),
    ];
    for dir in dirs.into_iter().flatten() {
        for file in ["HEAD", "refs/heads", "refs/tags", "packed-refs"] {
            let path = Path::new(&dir).join(file);
            // A missing path would make Cargo rerun this script on every build.
            if path.exists() {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    }
}
