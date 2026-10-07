//! REQS: reviewing performance metrics must not hang. Statistics over a heavy history (20k
//! submissions, 10k solves) must stay well under interactive latency, database read included.

use localgrind_core::catalog::{ProblemList, ProblemSummary};
use localgrind_core::metrics::{compute, StatsInput};
use localgrind_core::runner::{RunReport, Verdict};
use localgrind_core::types::Difficulty;
use localgrind_core::userdb::{Solve, UserDb};
use std::time::{Duration, Instant};

const DAY: i64 = 86_400_000;

#[test]
fn stats_stay_fast_with_a_large_history() {
    let dir = tempfile::tempdir().unwrap();
    let db = UserDb::open(&dir.path().join("user.db")).unwrap();
    let problems: Vec<ProblemSummary> = (1..=111)
        .map(|id| ProblemSummary {
            id,
            slug: format!("p{id}"),
            title: format!("Problem {id}"),
            difficulty: [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard][id as usize % 3],
            topics: vec![format!("Topic {}", id % 12), "Array".into()],
        })
        .collect();
    let now = 20_000 * DAY;
    let code = "x = 1\n".repeat(170); // ~1 KB of source per submission
    db.transaction(|db| {
        for i in 0..20_000i64 {
            let verdict = if i % 3 == 0 {
                Verdict::WrongAnswer
            } else {
                Verdict::Accepted
            };
            let report = RunReport {
                verdict,
                passed: 1,
                total: 1,
                runtime_ms: 1.0,
                memory_kb: 1,
                cases: vec![],
                failure: None,
                error: None,
                stderr: String::new(),
            };
            db.insert_submission(1 + i % 111, "python", &code, &report, now - (i % 700) * DAY)?;
        }
        for i in 0..10_000i64 {
            let id = 1 + i % 111;
            db.import_solve(&Solve {
                id: 0,
                problem_id: Some(id),
                slug: format!("p{id}"),
                title: format!("Problem {id}"),
                difficulty: problems[(id - 1) as usize].difficulty,
                started_at: now - (i % 700) * DAY - 600_000,
                completed_at: Some(now - (i % 700) * DAY + i),
                elapsed_ms: 60_000 + (i % 50) * 30_000,
                errors: i % 4,
                submission_id: None,
            })?;
        }
        Ok(())
    })
    .unwrap();
    let lists: Vec<ProblemList> = vec![];

    let start = Instant::now();
    let activity = db.activity().unwrap();
    let solves = db.completed_solves().unwrap();
    let stats = compute(&StatsInput {
        problems: &problems,
        lists: &lists,
        submissions: &activity,
        solves: &solves,
        now_ms: now,
        recent_n: 20,
        review_min_days: 7.0,
        utc_offset_minutes: 0,
    });
    let elapsed = start.elapsed();
    println!(
        "stats over {} submissions / {} solves: {elapsed:?}",
        activity.len(),
        solves.len()
    );
    assert_eq!(stats.totals.submissions, 20_000);
    assert_eq!(stats.totals.solves, 10_000);
    assert!(!stats.review.is_empty() && stats.review.len() <= 111);
    // Generous enough for unoptimized debug builds on slow machines; release is far faster.
    assert!(
        elapsed < Duration::from_millis(1500),
        "stats took {elapsed:?}"
    );
}
