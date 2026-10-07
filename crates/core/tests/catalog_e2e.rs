//! Submits every problem's bundled reference solution through the real Service against the
//! built catalog. Guards against harness or judge changes breaking existing content.
//! Run with `cargo test -p localgrind-core --test catalog_e2e -- --ignored` after
//! `lg-content build` (scripts/check.sh does this).

use localgrind_core::catalog::Catalog;
use localgrind_core::lang::Language;
use localgrind_core::runner::Verdict;
use localgrind_core::service::Service;
use localgrind_core::userdb::UserDb;
use std::path::Path;

#[test]
#[ignore = "needs assets/problems.db; run via scripts/check.sh"]
fn every_reference_solution_is_accepted() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/problems.db");
    let svc = Service::new(Catalog::open(&path).unwrap(), UserDb::in_memory().unwrap()).unwrap();
    let problems = svc.bootstrap().unwrap().problems;
    assert!(
        problems.len() >= 111,
        "catalog has {} problems",
        problems.len()
    );
    let mut failures = Vec::new();
    for p in &problems {
        let code = svc
            .reference_solution(p.summary.id, Language::Python)
            .unwrap()
            .expect("reference solution");
        let outcome = svc
            .submit(p.summary.id, Language::Python, &code, 0, &mut |_, _| {})
            .unwrap();
        if outcome.submission.verdict != Verdict::Accepted {
            failures.push(format!(
                "{}: {:?} {}",
                p.summary.slug,
                outcome.submission.verdict,
                outcome.submission.error.unwrap_or_default()
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
