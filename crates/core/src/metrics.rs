//! Review-tool metrics, modelled on Yet-Another-LeetCode-Timer: per-difficulty time and error
//! statistics, recent-vs-overall comparison, activity heatmap, topic breakdown, streaks and a
//! spaced-review queue. Everything is computed in one pass over the user's history so the
//! statistics view never blocks on heavy work.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::catalog::{ProblemList, ProblemSummary};
use crate::runner::Verdict;
use crate::timeutil::{civil_from_days, days_from_civil};
use crate::types::Difficulty;
use crate::userdb::{Solve, Submission};

const MS_PER_DAY: i64 = 86_400_000;
/// Beyond this many days since the last solve, staleness stops adding to the review score.
pub const REVIEW_STALE_CAP_DAYS: f64 = 180.0;
/// How much slower than your own difficulty average counts as maximally slow.
pub const REVIEW_SLOW_CEILING: f64 = 1.5;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Aggregate {
    pub solves: usize,
    pub avg_ms: f64,
    pub min_ms: i64,
    pub max_ms: i64,
    pub errors: i64,
    /// Failed submissions / all submissions that went into these solves, as a percentage.
    pub error_rate: f64,
}

impl Aggregate {
    pub fn of<'a>(solves: impl IntoIterator<Item = &'a Solve>) -> Aggregate {
        let mut a = Aggregate {
            min_ms: i64::MAX,
            ..Default::default()
        };
        let mut total = 0i64;
        for s in solves {
            a.solves += 1;
            a.errors += s.errors;
            total += s.elapsed_ms;
            a.min_ms = a.min_ms.min(s.elapsed_ms);
            a.max_ms = a.max_ms.max(s.elapsed_ms);
        }
        if a.solves == 0 {
            return Aggregate::default();
        }
        a.avg_ms = total as f64 / a.solves as f64;
        a.error_rate = 100.0 * a.errors as f64 / (a.errors + a.solves as i64) as f64;
        a
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifficultyStats {
    pub difficulty: Difficulty,
    /// Distinct catalog problems with an Accepted submission.
    pub solved: usize,
    pub available: usize,
    pub overall: Aggregate,
    pub recent: Aggregate,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TopicStats {
    pub topic: String,
    pub solved: usize,
    pub available: usize,
    pub solves: usize,
    pub avg_ms: f64,
    pub error_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListProgress {
    pub id: String,
    pub name: String,
    pub solved: usize,
    pub total: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DayCount {
    pub date: String,
    pub submissions: usize,
    pub accepted: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reason {
    pub text: String,
    /// `bad`, `warn` or empty.
    pub severity: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReviewItem {
    pub slug: String,
    pub problem_id: Option<i64>,
    pub title: String,
    pub difficulty: Difficulty,
    pub solves: usize,
    pub errors: i64,
    pub pass_rate: f64,
    pub avg_ms: f64,
    pub days_since: f64,
    pub time_ratio: f64,
    pub score: u32,
    pub reasons: Vec<Reason>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Totals {
    pub solved: usize,
    pub attempted: usize,
    pub available: usize,
    pub submissions: usize,
    pub accepted_submissions: usize,
    pub solves: usize,
    pub current_streak: usize,
    pub longest_streak: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Stats {
    pub totals: Totals,
    pub recent_n: usize,
    pub overall: Aggregate,
    pub recent: Aggregate,
    pub by_difficulty: Vec<DifficultyStats>,
    pub topics: Vec<TopicStats>,
    pub lists: Vec<ListProgress>,
    pub heatmap: Vec<DayCount>,
    pub review: Vec<ReviewItem>,
}

pub struct StatsInput<'a> {
    pub problems: &'a [ProblemSummary],
    pub lists: &'a [ProblemList],
    pub submissions: &'a [Submission],
    /// Completed solves, any order.
    pub solves: &'a [Solve],
    pub now_ms: i64,
    pub recent_n: usize,
    /// Problems solved more recently than this are left out of the review queue.
    pub review_min_days: f64,
    /// Local timezone offset applied when bucketing activity into days.
    pub utc_offset_minutes: i32,
}

fn day_key(ms: i64, offset_min: i32) -> i64 {
    (ms + offset_min as i64 * 60_000).div_euclid(MS_PER_DAY)
}

fn day_string(day: i64) -> String {
    let (y, m, d) = civil_from_days(day);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Day number for a `YYYY-MM-DD` string, the inverse of the heatmap's date keys.
pub fn parse_day(s: &str) -> Option<i64> {
    let mut it = s.splitn(3, '-');
    let y = it.next()?.parse().ok()?;
    let m = it.next()?.parse().ok()?;
    let d = it.next()?.parse().ok()?;
    Some(days_from_civil(y, m, d))
}

fn streaks(days: &HashSet<i64>, today: i64) -> (usize, usize) {
    let mut sorted: Vec<i64> = days.iter().copied().collect();
    sorted.sort_unstable();
    let (mut longest, mut run) = (0, 0);
    for (i, d) in sorted.iter().enumerate() {
        run = if i > 0 && sorted[i - 1] == d - 1 {
            run + 1
        } else {
            1
        };
        longest = longest.max(run);
    }
    // The current streak survives until the end of today even if today has no activity yet.
    let mut current = 0;
    let mut day = if days.contains(&today) {
        today
    } else {
        today - 1
    };
    while days.contains(&day) {
        current += 1;
        day -= 1;
    }
    (current, longest)
}

fn clamp01(v: f64) -> f64 {
    if v.is_finite() {
        v.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

fn format_minutes(ratio: f64, difficulty: Difficulty) -> String {
    format!("{ratio:.1}x your {} average", difficulty.as_str())
}

fn review_queue(solves: &[Solve], now: i64, min_days: f64) -> Vec<ReviewItem> {
    struct Rec<'a> {
        latest: &'a Solve,
        solves: usize,
        errors: i64,
        total_ms: i64,
    }
    let mut by_slug: HashMap<&str, Rec> = HashMap::new();
    for s in solves {
        let rec = by_slug.entry(&s.slug).or_insert(Rec {
            latest: s,
            solves: 0,
            errors: 0,
            total_ms: 0,
        });
        rec.solves += 1;
        rec.errors += s.errors;
        rec.total_ms += s.elapsed_ms;
        if s.completed_at > rec.latest.completed_at {
            rec.latest = s;
        }
    }
    // Baseline per difficulty: mean of per-problem averages, so repeats don't dominate.
    let mut sums: HashMap<Difficulty, (f64, usize)> = HashMap::new();
    for rec in by_slug.values() {
        let e = sums.entry(rec.latest.difficulty).or_default();
        e.0 += rec.total_ms as f64 / rec.solves as f64;
        e.1 += 1;
    }
    let stale_span = (REVIEW_STALE_CAP_DAYS - min_days).max(1.0);
    let mut items: Vec<ReviewItem> = by_slug
        .into_values()
        .filter_map(|rec| {
            let last = rec.latest.completed_at?;
            let days = (now - last) as f64 / MS_PER_DAY as f64;
            if days < min_days {
                return None;
            }
            let avg_ms = rec.total_ms as f64 / rec.solves as f64;
            let (sum, n) = sums[&rec.latest.difficulty];
            let baseline = sum / n as f64;
            let time_ratio = if baseline > 0.0 {
                avg_ms / baseline
            } else {
                1.0
            };
            let attempts = rec.solves as i64 + rec.errors;
            let pass_rate = rec.solves as f64 / attempts as f64;
            let fail_w = 1.0 - pass_rate;
            let slow_w = clamp01((time_ratio - 1.0) / (REVIEW_SLOW_CEILING - 1.0));
            let stale_w = clamp01((days - min_days) / stale_span);
            let score = (100.0 * (0.45 * fail_w + 0.35 * slow_w + 0.20 * stale_w)).round() as u32;

            let mut reasons = Vec::new();
            if rec.errors > 0 {
                let mut text = format!(
                    "{} wrong submission{}",
                    rec.errors,
                    if rec.errors == 1 { "" } else { "s" }
                );
                if rec.solves > 1 {
                    text.push_str(&format!(" over {} solves", rec.solves));
                }
                let severity = if pass_rate <= 0.5 { "bad" } else { "warn" };
                reasons.push(Reason {
                    text,
                    severity: severity.into(),
                });
            }
            if time_ratio >= 1.15 {
                let severity = if time_ratio >= REVIEW_SLOW_CEILING {
                    "bad"
                } else {
                    "warn"
                };
                reasons.push(Reason {
                    text: format_minutes(time_ratio, rec.latest.difficulty),
                    severity: severity.into(),
                });
            }
            if days >= 90.0 {
                reasons.push(Reason {
                    text: format!("cold for {} months", (days / 30.0).round() as i64),
                    severity: String::new(),
                });
            }
            if reasons.is_empty() {
                reasons.push(Reason {
                    text: "clean solve, just due for a refresh".into(),
                    severity: String::new(),
                });
            }
            Some(ReviewItem {
                slug: rec.latest.slug.clone(),
                problem_id: rec.latest.problem_id,
                title: rec.latest.title.clone(),
                difficulty: rec.latest.difficulty,
                solves: rec.solves,
                errors: rec.errors,
                pass_rate,
                avg_ms,
                days_since: days,
                time_ratio,
                score,
                reasons,
            })
        })
        .collect();
    items.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.pass_rate.total_cmp(&b.pass_rate))
            .then(a.slug.cmp(&b.slug))
    });
    items
}

pub fn compute(input: &StatsInput) -> Stats {
    let offset = input.utc_offset_minutes;
    let today = day_key(input.now_ms, offset);

    let mut solved_ids: HashSet<i64> = HashSet::new();
    let mut attempted_ids: HashSet<i64> = HashSet::new();
    let mut per_day: BTreeMap<i64, DayCount> = BTreeMap::new();
    let mut accepted_days: HashSet<i64> = HashSet::new();
    let mut accepted_submissions = 0;
    for s in input.submissions {
        attempted_ids.insert(s.problem_id);
        let day = day_key(s.created_at, offset);
        let entry = per_day.entry(day).or_insert_with(|| DayCount {
            date: day_string(day),
            submissions: 0,
            accepted: 0,
        });
        entry.submissions += 1;
        if s.verdict == Verdict::Accepted {
            accepted_submissions += 1;
            solved_ids.insert(s.problem_id);
            entry.accepted += 1;
            accepted_days.insert(day);
        }
    }
    // Solves count as activity too (imported solves may have no matching submission).
    for s in input.solves {
        if let Some(c) = s.completed_at {
            accepted_days.insert(day_key(c, offset));
            if let Some(id) = s.problem_id {
                solved_ids.insert(id);
                attempted_ids.insert(id);
            }
        }
    }
    // Only catalog problems count towards solved/attempted totals.
    let known: HashSet<i64> = input.problems.iter().map(|p| p.id).collect();
    solved_ids.retain(|id| known.contains(id));
    attempted_ids.retain(|id| known.contains(id));
    let year_ago = today - 365;
    let heatmap: Vec<DayCount> = per_day.range(year_ago..).map(|(_, v)| v.clone()).collect();
    let (current_streak, longest_streak) = streaks(&accepted_days, today);

    let mut ordered: Vec<&Solve> = input
        .solves
        .iter()
        .filter(|s| s.completed_at.is_some())
        .collect();
    ordered.sort_by_key(|s| (s.completed_at, s.id));
    let recent: Vec<&Solve> = ordered.iter().rev().take(input.recent_n).copied().collect();

    let difficulty_of: HashMap<i64, &ProblemSummary> =
        input.problems.iter().map(|p| (p.id, p)).collect();
    let by_difficulty = [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard]
        .into_iter()
        .map(|d| DifficultyStats {
            difficulty: d,
            solved: solved_ids
                .iter()
                .filter(|id| difficulty_of.get(id).is_some_and(|p| p.difficulty == d))
                .count(),
            available: input.problems.iter().filter(|p| p.difficulty == d).count(),
            overall: Aggregate::of(ordered.iter().copied().filter(|s| s.difficulty == d)),
            recent: Aggregate::of(recent.iter().copied().filter(|s| s.difficulty == d)),
        })
        .collect();

    let mut topic_map: BTreeMap<&str, (usize, usize, Vec<&Solve>)> = BTreeMap::new();
    for p in input.problems {
        for t in &p.topics {
            let e = topic_map.entry(t).or_default();
            e.1 += 1;
            if solved_ids.contains(&p.id) {
                e.0 += 1;
            }
        }
    }
    for s in &ordered {
        if let Some(p) = s.problem_id.and_then(|id| difficulty_of.get(&id)) {
            for t in &p.topics {
                topic_map.entry(t).or_default().2.push(s);
            }
        }
    }
    let topics = topic_map
        .into_iter()
        .map(|(topic, (solved, available, solves))| {
            let agg = Aggregate::of(solves.iter().copied());
            TopicStats {
                topic: topic.to_string(),
                solved,
                available,
                solves: agg.solves,
                avg_ms: agg.avg_ms,
                error_rate: agg.error_rate,
            }
        })
        .collect();

    let lists = input
        .lists
        .iter()
        .map(|l| ListProgress {
            id: l.id.clone(),
            name: l.name.clone(),
            solved: l
                .items
                .iter()
                .filter(|i| solved_ids.contains(&i.problem_id))
                .count(),
            total: l.items.len(),
        })
        .collect();

    Stats {
        totals: Totals {
            solved: solved_ids.len(),
            attempted: attempted_ids.len(),
            available: input.problems.len(),
            submissions: input.submissions.len(),
            accepted_submissions,
            solves: ordered.len(),
            current_streak,
            longest_streak,
        },
        recent_n: input.recent_n,
        overall: Aggregate::of(ordered.iter().copied()),
        recent: Aggregate::of(recent.iter().copied()),
        by_difficulty,
        topics,
        lists,
        heatmap,
        review: review_queue(input.solves, input.now_ms, input.review_min_days),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::ListItem;

    fn solve(id: i64, slug: &str, d: Difficulty, day: i64, elapsed_min: i64, errors: i64) -> Solve {
        Solve {
            id,
            problem_id: Some(id_for(slug)),
            slug: slug.into(),
            title: slug.into(),
            difficulty: d,
            started_at: day * MS_PER_DAY,
            completed_at: Some(day * MS_PER_DAY + 1000),
            elapsed_ms: elapsed_min * 60_000,
            errors,
            submission_id: None,
        }
    }

    fn id_for(slug: &str) -> i64 {
        match slug {
            "a" => 1,
            "b" => 2,
            "c" => 3,
            _ => 99,
        }
    }

    fn sub(id: i64, problem_id: i64, day: i64, verdict: Verdict) -> Submission {
        Submission {
            id,
            problem_id,
            language: "python".into(),
            code: String::new(),
            verdict,
            passed: 0,
            total: 0,
            runtime_ms: 0.0,
            memory_kb: 0,
            error: None,
            failure: None,
            created_at: day * MS_PER_DAY + 5000,
        }
    }

    fn problems() -> Vec<ProblemSummary> {
        [
            (1, "a", Difficulty::Easy, "Array"),
            (2, "b", Difficulty::Medium, "Graph"),
            (3, "c", Difficulty::Hard, "Array"),
        ]
        .into_iter()
        .map(|(id, slug, d, t)| ProblemSummary {
            id,
            slug: slug.into(),
            title: slug.into(),
            difficulty: d,
            topics: vec![t.into()],
        })
        .collect()
    }

    #[test]
    fn aggregates() {
        assert_eq!(Aggregate::of([]), Aggregate::default());
        let s = [
            solve(1, "a", Difficulty::Easy, 0, 10, 1),
            solve(2, "a", Difficulty::Easy, 1, 20, 0),
        ];
        let a = Aggregate::of(&s);
        assert_eq!(
            (a.solves, a.min_ms, a.max_ms, a.errors),
            (2, 600_000, 1_200_000, 1)
        );
        assert_eq!(a.avg_ms, 900_000.0);
        assert!((a.error_rate - 100.0 / 3.0).abs() < 1e-9);
    }

    #[test]
    fn streak_math() {
        let days: HashSet<i64> = [1, 2, 3, 7, 8].into_iter().collect();
        assert_eq!(streaks(&days, 8), (2, 3));
        assert_eq!(streaks(&days, 9), (2, 3));
        assert_eq!(streaks(&days, 10), (0, 3));
        assert_eq!(streaks(&HashSet::new(), 10), (0, 0));
    }

    #[test]
    fn full_stats() {
        let probs = problems();
        let lists = vec![ProblemList {
            id: "l".into(),
            name: "L".into(),
            description: String::new(),
            items: vec![
                ListItem {
                    problem_id: 1,
                    group: "g".into(),
                },
                ListItem {
                    problem_id: 3,
                    group: "g".into(),
                },
            ],
        }];
        let subs = vec![
            sub(1, 1, 100, Verdict::WrongAnswer),
            sub(2, 1, 100, Verdict::Accepted),
            sub(3, 2, 101, Verdict::Accepted),
            sub(4, 3, 50, Verdict::RuntimeError),
        ];
        let solves = vec![
            solve(1, "a", Difficulty::Easy, 100, 10, 1),
            solve(2, "b", Difficulty::Medium, 101, 30, 0),
            solve(3, "zz", Difficulty::Hard, 2, 50, 3),
        ];
        let stats = compute(&StatsInput {
            problems: &probs,
            lists: &lists,
            submissions: &subs,
            solves: &solves,
            now_ms: 101 * MS_PER_DAY + 10_000,
            recent_n: 2,
            review_min_days: 0.0,
            utc_offset_minutes: 0,
        });
        let t = &stats.totals;
        assert_eq!((t.solved, t.attempted, t.available), (2, 3, 3));
        assert_eq!((t.submissions, t.accepted_submissions, t.solves), (4, 2, 3));
        assert_eq!((t.current_streak, t.longest_streak), (2, 2));
        assert_eq!(stats.recent.solves, 2);
        assert_eq!(stats.overall.solves, 3);
        assert_eq!(stats.by_difficulty[0].solved, 1);
        assert_eq!(stats.by_difficulty[2].overall.solves, 1);
        assert_eq!(stats.by_difficulty[2].recent.solves, 0);
        let array = stats.topics.iter().find(|t| t.topic == "Array").unwrap();
        assert_eq!((array.solved, array.available, array.solves), (1, 2, 1));
        assert_eq!(stats.lists[0].solved, 1);
        assert_eq!(stats.heatmap.len(), 3);
        assert_eq!(stats.heatmap[1].date, day_string(100));
        assert_eq!(stats.heatmap[1].submissions, 2);
        assert_eq!(stats.review.len(), 3);
        assert_eq!(stats.review[0].slug, "zz");
        assert!(stats.review[0]
            .reasons
            .iter()
            .any(|r| r.text.contains("cold for 3 months")));
        assert!(stats.review[0]
            .reasons
            .iter()
            .any(|r| r.text.contains("3 wrong submissions")));
    }

    #[test]
    fn review_reasons_and_filtering() {
        let solves = vec![
            solve(1, "a", Difficulty::Easy, 0, 10, 0),
            solve(2, "a", Difficulty::Easy, 5, 10, 1),
            solve(3, "b", Difficulty::Easy, 5, 40, 0),
            solve(4, "c", Difficulty::Easy, 5, 13, 0),
            solve(5, "d", Difficulty::Easy, 9, 10, 0),
        ];
        let q = review_queue(&solves, 10 * MS_PER_DAY, 3.0);
        assert_eq!(q.len(), 3, "d is too recent");
        let a = q.iter().find(|i| i.slug == "a").unwrap();
        assert_eq!(a.reasons[0].text, "1 wrong submission over 2 solves");
        assert_eq!(a.reasons[0].severity, "warn");
        let b = q.iter().find(|i| i.slug == "b").unwrap();
        assert_eq!(b.reasons[0].severity, "bad");
        assert!(b.reasons[0].text.contains("x your Easy average"));
        let c = q.iter().find(|i| i.slug == "c").unwrap();
        assert_eq!(c.reasons[0].text, "clean solve, just due for a refresh");
        assert!(q.windows(2).all(|w| w[0].score >= w[1].score));
    }

    #[test]
    fn day_helpers() {
        assert_eq!(parse_day(&day_string(20_000)), Some(20_000));
        assert_eq!(parse_day("nope"), None);
        assert_eq!(day_key(0, -60), -1);
        assert_eq!(clamp01(f64::NAN), 0.0);
        assert_eq!(clamp01(3.0), 1.0);
    }
}
