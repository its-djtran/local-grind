//! Submits every problem's bundled reference solutions (every language with an installed
//! toolchain) through the real Service against the built catalog. Guards against harness or
//! judge changes breaking existing content.
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
    let available: Vec<Language> = svc
        .check_toolchains()
        .into_iter()
        .filter(|t| t.ok)
        .map(|t| t.language)
        .collect();
    let mut failures = Vec::new();
    let mut judged = 0;
    for p in &problems {
        let id = p.summary.id;
        assert!(
            svc.reference_solution(id, Language::Python)
                .unwrap()
                .is_some(),
            "{} has no Python reference",
            p.summary.slug
        );
        for &lang in &available {
            let Some(code) = svc.reference_solution(id, lang).unwrap() else {
                continue;
            };
            judged += 1;
            let outcome = svc.submit(id, lang, &code, 0, &mut |_, _| {}).unwrap();
            if outcome.submission.verdict != Verdict::Accepted {
                failures.push(format!(
                    "{} ({lang}): {:?} {}",
                    p.summary.slug,
                    outcome.submission.verdict,
                    outcome.submission.error.unwrap_or_default()
                ));
            }
        }
    }
    println!("judged {judged} reference solutions");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
