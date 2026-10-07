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
use std::sync::Mutex;

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
    let mut jobs = Vec::new();
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
            if let Some(code) = svc.reference_solution(id, lang).unwrap() {
                jobs.push((p.summary.slug.clone(), id, lang, code));
            }
        }
    }
    let judged = jobs.len();
    let next = Mutex::new(jobs.into_iter());
    let failures = Mutex::new(Vec::new());
    // Half the cores: judging time counts against limits, so leave headroom against contention.
    let workers = std::thread::available_parallelism().map_or(2, |n| (n.get() / 2).max(1));
    std::thread::scope(|s| {
        for _ in 0..workers {
            s.spawn(|| loop {
                let Some((slug, id, lang, code)) = next.lock().unwrap().next() else {
                    break;
                };
                let outcome = svc.submit(id, lang, &code, 0, &mut |_, _| {}).unwrap();
                if outcome.submission.verdict != Verdict::Accepted {
                    failures.lock().unwrap().push(format!(
                        "{slug} ({lang}): {:?} {}",
                        outcome.submission.verdict,
                        outcome.submission.error.unwrap_or_default()
                    ));
                }
            });
        }
    });
    let failures = failures.into_inner().unwrap();
    println!("judged {judged} reference solutions");
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
