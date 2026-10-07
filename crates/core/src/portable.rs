//! Portable user data: a single JSON document holding settings, submissions, solves and drafts.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::catalog::ProblemSummary;
use crate::error::{Error, Result};
use crate::runner::{CaseResult, Verdict};
use crate::settings::Settings;
use crate::timeutil::{now_ms, parse_iso, to_iso};
use crate::types::Difficulty;
use crate::userdb::{Solve, Submission, UserDb};

pub const FORMAT: &str = "localgrind-export";
pub const VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportedSubmission {
    pub problem_id: i64,
    pub slug: String,
    pub language: String,
    pub code: String,
    pub verdict: Verdict,
    pub passed: usize,
    pub total: usize,
    pub runtime_ms: f64,
    pub memory_kb: u64,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub failure: Option<CaseResult>,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportedSolve {
    #[serde(default)]
    pub problem_id: Option<i64>,
    pub slug: String,
    pub title: String,
    pub difficulty: Difficulty,
    pub started_at: String,
    pub completed_at: String,
    pub elapsed_ms: i64,
    pub errors: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportedDraft {
    pub problem_id: i64,
    pub language: String,
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExportedNote {
    pub problem_id: i64,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Export {
    pub format: String,
    pub version: u32,
    pub exported_at: String,
    pub app_version: String,
    pub settings: Settings,
    pub submissions: Vec<ExportedSubmission>,
    pub solves: Vec<ExportedSolve>,
    #[serde(default)]
    pub drafts: Vec<ExportedDraft>,
    #[serde(default)]
    pub notes: Vec<ExportedNote>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ImportSummary {
    pub submissions_added: usize,
    pub submissions_skipped: usize,
    pub solves_added: usize,
    pub solves_skipped: usize,
    pub drafts_added: usize,
    #[serde(default)]
    pub notes_added: usize,
    pub settings_applied: bool,
    pub source: String,
}

pub fn export(db: &UserDb, problems: &[ProblemSummary]) -> Result<Export> {
    let slugs: HashMap<i64, &str> = problems.iter().map(|p| (p.id, p.slug.as_str())).collect();
    let submissions = db
        .all_submissions()?
        .into_iter()
        .map(|s| ExportedSubmission {
            slug: slugs.get(&s.problem_id).unwrap_or(&"").to_string(),
            problem_id: s.problem_id,
            language: s.language,
            code: s.code,
            verdict: s.verdict,
            passed: s.passed,
            total: s.total,
            runtime_ms: s.runtime_ms,
            memory_kb: s.memory_kb,
            error: s.error,
            failure: s.failure,
            created_at: to_iso(s.created_at),
        })
        .collect();
    let solves = db
        .completed_solves()?
        .into_iter()
        .map(|s| ExportedSolve {
            problem_id: s.problem_id,
            slug: s.slug,
            title: s.title,
            difficulty: s.difficulty,
            started_at: to_iso(s.started_at),
            completed_at: to_iso(s.completed_at.unwrap_or(s.started_at)),
            elapsed_ms: s.elapsed_ms,
            errors: s.errors,
        })
        .collect();
    let drafts = db
        .drafts()?
        .into_iter()
        .map(|d| ExportedDraft {
            problem_id: d.problem_id,
            language: d.language,
            code: d.code,
        })
        .collect();
    Ok(Export {
        format: FORMAT.into(),
        version: VERSION,
        exported_at: to_iso(now_ms()),
        app_version: env!("CARGO_PKG_VERSION").into(),
        settings: db.settings()?,
        submissions,
        solves,
        drafts,
        notes: db
            .notes()?
            .into_iter()
            .map(|(problem_id, body)| ExportedNote { problem_id, body })
            .collect(),
    })
}

fn time_of(s: &str) -> Result<i64> {
    parse_iso(s).ok_or_else(|| Error::Invalid(format!("bad timestamp `{s}`")))
}

fn import_native(db: &UserDb, data: Export, apply_settings: bool) -> Result<ImportSummary> {
    if data.version > VERSION {
        return Err(Error::Invalid(format!(
            "export version {} is newer than this app supports ({VERSION})",
            data.version
        )));
    }
    let mut summary = ImportSummary {
        source: FORMAT.into(),
        ..Default::default()
    };
    let existing: std::collections::HashSet<(i64, i64, String)> = db
        .all_submissions()?
        .into_iter()
        .map(|s| (s.problem_id, s.created_at, s.language))
        .collect();
    let mut existing = existing;
    for s in data.submissions {
        let created_at = time_of(&s.created_at)?;
        // Also dedupes repeated entries within the file itself.
        if !existing.insert((s.problem_id, created_at, s.language.clone())) {
            summary.submissions_skipped += 1;
            continue;
        }
        db.import_submission(&Submission {
            id: 0,
            problem_id: s.problem_id,
            language: s.language,
            code: s.code,
            verdict: s.verdict,
            passed: s.passed,
            total: s.total,
            runtime_ms: s.runtime_ms,
            memory_kb: s.memory_kb,
            error: s.error,
            failure: s.failure,
            created_at,
        })?;
        summary.submissions_added += 1;
    }
    for s in data.solves {
        let solve = Solve {
            id: 0,
            problem_id: s.problem_id,
            slug: s.slug,
            title: s.title,
            difficulty: s.difficulty,
            started_at: time_of(&s.started_at)?,
            completed_at: Some(time_of(&s.completed_at)?),
            elapsed_ms: s.elapsed_ms,
            errors: s.errors,
            submission_id: None,
        };
        if db.import_solve(&solve)? {
            summary.solves_added += 1;
        } else {
            summary.solves_skipped += 1;
        }
    }
    for d in data.drafts {
        if db.draft(d.problem_id, &d.language)?.is_none() {
            db.save_draft(d.problem_id, &d.language, &d.code)?;
            summary.drafts_added += 1;
        }
    }
    for n in data.notes {
        if db.note(n.problem_id)?.is_none() {
            db.save_note(n.problem_id, &n.body)?;
            summary.notes_added += 1;
        }
    }
    if apply_settings {
        // Never take executable paths from a file: an import must not change what gets run.
        let mut settings = data.settings;
        settings.eval.toolchain_paths = db.settings()?.eval.toolchain_paths;
        db.save_settings(&settings)?;
        summary.settings_applied = true;
    }
    Ok(summary)
}

/// Imports a Local Grind export. Existing records are kept; exact duplicates are skipped. The
/// whole import is atomic.
pub fn import(db: &UserDb, json: &str, apply_settings: bool) -> Result<ImportSummary> {
    let value: Value = serde_json::from_str(json)?;
    if value.get("format").and_then(Value::as_str) != Some(FORMAT) {
        return Err(Error::Invalid("not a Local Grind export file".into()));
    }
    let data: Export = serde_json::from_value(value)?;
    db.transaction(|db| import_native(db, data, apply_settings))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::RunReport;
    use crate::userdb::ProblemRef;

    fn problems() -> Vec<ProblemSummary> {
        vec![ProblemSummary {
            id: 1,
            slug: "two-sum".into(),
            title: "Two Sum".into(),
            difficulty: Difficulty::Easy,
            topics: vec![],
        }]
    }

    fn seeded() -> UserDb {
        let db = UserDb::in_memory().unwrap();
        let report = RunReport {
            verdict: Verdict::Accepted,
            passed: 3,
            total: 3,
            runtime_ms: 1.0,
            memory_kb: 100,
            cases: vec![],
            failure: None,
            error: None,
            stderr: String::new(),
        };
        let sub = db
            .insert_submission(1, "python", "code", &report, 1_700_000_000_000)
            .unwrap();
        let p = ProblemRef {
            id: Some(1),
            slug: "two-sum".into(),
            title: "Two Sum".into(),
            difficulty: Difficulty::Easy,
        };
        db.start_solve(&p, 1_699_999_000_000).unwrap();
        db.record_attempt(&p, &sub, 1_000_000).unwrap();
        db.save_draft(1, "python", "draft").unwrap();
        db.save_note(1, "remember the complement").unwrap();
        let s = Settings {
            font_size: 20,
            ..Default::default()
        };
        db.save_settings(&s).unwrap();
        db
    }

    #[test]
    fn export_import_roundtrip() {
        let src = seeded();
        let data = export(&src, &problems()).unwrap();
        assert_eq!(data.submissions[0].slug, "two-sum");
        assert_eq!(data.solves.len(), 1);
        let json = serde_json::to_string(&data).unwrap();

        let dst = UserDb::in_memory().unwrap();
        let summary = import(&dst, &json, true).unwrap();
        assert_eq!(
            (
                summary.submissions_added,
                summary.solves_added,
                summary.drafts_added
            ),
            (1, 1, 1)
        );
        assert!(summary.settings_applied);
        assert_eq!(summary.notes_added, 1);
        assert_eq!(
            dst.note(1).unwrap().as_deref(),
            Some("remember the complement")
        );
        assert_eq!(dst.settings().unwrap().font_size, 20);
        let again = import(&dst, &json, false).unwrap();
        assert_eq!(
            (
                again.submissions_skipped,
                again.solves_skipped,
                again.drafts_added
            ),
            (1, 1, 0)
        );
        let re = export(&dst, &problems()).unwrap();
        assert_eq!(re.submissions, data.submissions);
        assert_eq!(re.solves, data.solves);
    }

    #[test]
    fn imports_cannot_change_toolchains_and_dedupe_within_a_file() {
        let src = seeded();
        let mut data = export(&src, &problems()).unwrap();
        data.settings
            .eval
            .toolchain_paths
            .insert("python".into(), "/tmp/evil".into());
        data.submissions.push(data.submissions[0].clone());
        let dst = UserDb::in_memory().unwrap();
        let summary = import(&dst, &serde_json::to_string(&data).unwrap(), true).unwrap();
        assert_eq!(
            (summary.submissions_added, summary.submissions_skipped),
            (1, 1)
        );
        assert!(dst.settings().unwrap().eval.toolchain_paths.is_empty());
        assert_eq!(
            dst.settings().unwrap().font_size,
            20,
            "other settings still apply"
        );
    }

    #[test]
    fn rejects_newer_versions_and_garbage() {
        let db = UserDb::in_memory().unwrap();
        let mut data = export(&db, &[]).unwrap();
        data.version = 99;
        let json = serde_json::to_string(&data).unwrap();
        assert!(import(&db, &json, false).is_err());
        assert!(import(&db, "{\"x\":1}", false).is_err());
        assert!(import(&db, "not json", false).is_err());
    }
}
