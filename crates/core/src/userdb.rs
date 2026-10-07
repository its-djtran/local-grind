//! Per-user, writable state: submissions, drafts, solve timers and settings.

use rusqlite::{params, Connection, OptionalExtension, Row};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

use crate::catalog::Benchmark;
use crate::error::{Error, Result};
use crate::runner::{CaseResult, RunReport, Verdict};
use crate::settings::Settings;
use crate::timeutil::now_ms;
use crate::types::Difficulty;

const MIGRATIONS: &[&str] = &[
    "
CREATE TABLE submissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    problem_id INTEGER NOT NULL,
    language TEXT NOT NULL,
    code TEXT NOT NULL,
    verdict TEXT NOT NULL,
    passed INTEGER NOT NULL,
    total INTEGER NOT NULL,
    runtime_ms REAL NOT NULL,
    memory_kb INTEGER NOT NULL,
    error TEXT,
    failure TEXT,
    created_at INTEGER NOT NULL
);
CREATE INDEX submissions_problem ON submissions (problem_id, created_at);
CREATE TABLE drafts (
    problem_id INTEGER NOT NULL,
    language TEXT NOT NULL,
    code TEXT NOT NULL,
    updated_at INTEGER NOT NULL,
    PRIMARY KEY (problem_id, language)
);
CREATE TABLE solves (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    problem_id INTEGER,
    slug TEXT NOT NULL,
    title TEXT NOT NULL,
    difficulty TEXT NOT NULL,
    started_at INTEGER NOT NULL,
    completed_at INTEGER,
    elapsed_ms INTEGER NOT NULL DEFAULT 0,
    errors INTEGER NOT NULL DEFAULT 0,
    submission_id INTEGER
);
CREATE UNIQUE INDEX solves_active ON solves (slug) WHERE completed_at IS NULL;
CREATE INDEX solves_completed ON solves (completed_at);
CREATE TABLE settings (id INTEGER PRIMARY KEY CHECK (id = 1), json TEXT NOT NULL);
",
    "
CREATE TABLE local_benchmarks (
    problem_id INTEGER NOT NULL,
    language TEXT NOT NULL,
    runtime_ms REAL NOT NULL,
    memory_kb INTEGER NOT NULL,
    runs INTEGER NOT NULL,
    measured_at INTEGER NOT NULL,
    PRIMARY KEY (problem_id, language)
);
",
    "
CREATE TABLE notes (
    problem_id INTEGER PRIMARY KEY,
    body TEXT NOT NULL,
    updated_at INTEGER NOT NULL
);
",
    "
CREATE TABLE user_lists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    created_at INTEGER NOT NULL
);
CREATE TABLE user_list_items (
    list_id INTEGER NOT NULL REFERENCES user_lists (id) ON DELETE CASCADE,
    problem_id INTEGER NOT NULL,
    added_at INTEGER NOT NULL,
    PRIMARY KEY (list_id, problem_id)
);
",
];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Submission {
    pub id: i64,
    pub problem_id: i64,
    pub language: String,
    pub code: String,
    pub verdict: Verdict,
    pub passed: usize,
    pub total: usize,
    pub runtime_ms: f64,
    pub memory_kb: u64,
    pub error: Option<String>,
    pub failure: Option<CaseResult>,
    pub created_at: i64,
}

/// A list of problems the user curates (e.g. "Favorites", "Review before onsite").
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserList {
    pub id: i64,
    pub name: String,
    /// Problem ids in the order they were added.
    pub problem_ids: Vec<i64>,
}

/// The parts of a submission the statistics use.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Activity {
    pub problem_id: i64,
    pub verdict: Verdict,
    pub created_at: i64,
}

impl From<&Submission> for Activity {
    fn from(s: &Submission) -> Activity {
        Activity {
            problem_id: s.problem_id,
            verdict: s.verdict,
            created_at: s.created_at,
        }
    }
}

/// One timed attempt at a problem, from first keystroke until an Accepted submission, with the
/// number of failed submissions along the way.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Solve {
    pub id: i64,
    pub problem_id: Option<i64>,
    pub slug: String,
    pub title: String,
    pub difficulty: Difficulty,
    pub started_at: i64,
    pub completed_at: Option<i64>,
    pub elapsed_ms: i64,
    pub errors: i64,
    pub submission_id: Option<i64>,
}

/// Identifies a problem for solve bookkeeping.
#[derive(Debug, Clone, PartialEq)]
pub struct ProblemRef {
    pub id: Option<i64>,
    pub slug: String,
    pub title: String,
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Draft {
    pub problem_id: i64,
    pub language: String,
    pub code: String,
    pub updated_at: i64,
}

pub struct UserDb {
    conn: Connection,
}

fn conversion_err(
    idx: usize,
    e: impl std::error::Error + Send + Sync + 'static,
) -> rusqlite::Error {
    rusqlite::Error::FromSqlConversionFailure(idx, rusqlite::types::Type::Text, Box::new(e))
}

fn submission_from_row(r: &Row) -> rusqlite::Result<Submission> {
    let verdict: String = r.get(4)?;
    let failure: Option<String> = r.get(10)?;
    Ok(Submission {
        id: r.get(0)?,
        problem_id: r.get(1)?,
        language: r.get(2)?,
        code: r.get(3)?,
        verdict: Verdict::parse(&verdict)
            .ok_or_else(|| conversion_err(4, Error::Invalid(format!("verdict {verdict}"))))?,
        passed: r.get::<_, i64>(5)? as usize,
        total: r.get::<_, i64>(6)? as usize,
        runtime_ms: r.get(7)?,
        memory_kb: r.get::<_, i64>(8)? as u64,
        error: r.get(9)?,
        failure: failure
            .map(|f| serde_json::from_str(&f))
            .transpose()
            .map_err(|e| conversion_err(10, e))?,
        created_at: r.get(11)?,
    })
}

fn solve_from_row(r: &Row) -> rusqlite::Result<Solve> {
    let difficulty: String = r.get(4)?;
    Ok(Solve {
        id: r.get(0)?,
        problem_id: r.get(1)?,
        slug: r.get(2)?,
        title: r.get(3)?,
        difficulty: difficulty
            .parse()
            .map_err(|e: Error| conversion_err(4, e))?,
        started_at: r.get(5)?,
        completed_at: r.get(6)?,
        elapsed_ms: r.get(7)?,
        errors: r.get(8)?,
        submission_id: r.get(9)?,
    })
}

const SUBMISSION_COLS: &str = "id, problem_id, language, code, verdict, passed, total, runtime_ms,
    memory_kb, error, failure, created_at";
const SOLVE_COLS: &str =
    "id, problem_id, slug, title, difficulty, started_at, completed_at, elapsed_ms, errors, submission_id";

impl UserDb {
    pub fn open(path: &Path) -> Result<UserDb> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
        UserDb::init(conn)
    }

    pub fn in_memory() -> Result<UserDb> {
        UserDb::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> Result<UserDb> {
        let version: i64 = conn.query_row("PRAGMA user_version", [], |r| r.get(0))?;
        if version as usize > MIGRATIONS.len() {
            return Err(Error::Invalid(format!(
                "user database version {version} is newer than this app supports"
            )));
        }
        for (i, sql) in MIGRATIONS.iter().enumerate().skip(version as usize) {
            conn.execute_batch(&format!(
                "BEGIN; {sql}; PRAGMA user_version = {}; COMMIT;",
                i + 1
            ))?;
        }
        Ok(UserDb { conn })
    }

    // ---- settings ----

    pub fn settings(&self) -> Result<Settings> {
        let json: Option<String> = self
            .conn
            .query_row("SELECT json FROM settings WHERE id = 1", [], |r| r.get(0))
            .optional()?;
        Ok(match json {
            Some(j) => serde_json::from_str::<Settings>(&j)
                .unwrap_or_default()
                .sanitized(),
            None => Settings::default(),
        })
    }

    pub fn save_settings(&self, s: &Settings) -> Result<Settings> {
        let s = s.clone().sanitized();
        self.conn.execute(
            "INSERT INTO settings (id, json) VALUES (1, ?1)
             ON CONFLICT (id) DO UPDATE SET json = excluded.json",
            [serde_json::to_string(&s)?],
        )?;
        Ok(s)
    }

    // ---- local benchmarks ----

    pub fn save_local_benchmark(
        &self,
        problem_id: i64,
        language: &str,
        b: &Benchmark,
    ) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO local_benchmarks
                 (problem_id, language, runtime_ms, memory_kb, runs, measured_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                problem_id,
                language,
                b.runtime_ms,
                b.memory_kb as i64,
                b.runs as i64,
                now_ms()
            ],
        )?;
        Ok(())
    }

    pub fn local_benchmarks(&self, problem_id: i64) -> Result<BTreeMap<String, Benchmark>> {
        let mut stmt = self.conn.prepare(
            "SELECT language, runtime_ms, memory_kb, runs FROM local_benchmarks WHERE problem_id = ?1",
        )?;
        let rows = stmt.query_map([problem_id], |r| {
            Ok((
                r.get::<_, String>(0)?,
                Benchmark {
                    runtime_ms: r.get(1)?,
                    memory_kb: r.get::<_, i64>(2)? as u64,
                    runs: r.get::<_, i64>(3)? as u32,
                    local: true,
                },
            ))
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// `(benchmark count, newest measurement time)` of the local recalibration, if any.
    pub fn local_benchmark_summary(&self) -> Result<(usize, Option<i64>)> {
        Ok(self.conn.query_row(
            "SELECT COUNT(*), MAX(measured_at) FROM local_benchmarks",
            [],
            |r| Ok((r.get::<_, i64>(0)? as usize, r.get(1)?)),
        )?)
    }

    pub fn clear_local_benchmarks(&self) -> Result<()> {
        self.conn.execute("DELETE FROM local_benchmarks", [])?;
        Ok(())
    }

    // ---- user lists ----

    pub fn user_lists(&self) -> Result<Vec<UserList>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name FROM user_lists ORDER BY created_at, id")?;
        let mut lists: Vec<UserList> = stmt
            .query_map([], |r| {
                Ok(UserList {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    problem_ids: Vec::new(),
                })
            })?
            .collect::<rusqlite::Result<_>>()?;
        let mut items = self.conn.prepare(
            "SELECT problem_id FROM user_list_items WHERE list_id = ?1 ORDER BY added_at, rowid",
        )?;
        for list in &mut lists {
            list.problem_ids = items
                .query_map([list.id], |r| r.get(0))?
                .collect::<rusqlite::Result<_>>()?;
        }
        Ok(lists)
    }

    /// Creates a list, or returns the existing one with the same (trimmed) name.
    pub fn create_user_list(&self, name: &str) -> Result<i64> {
        let name = name.trim();
        if name.is_empty() || name.chars().count() > 60 {
            return Err(Error::Invalid(
                "list names must be 1 to 60 characters".into(),
            ));
        }
        self.conn.execute(
            "INSERT INTO user_lists (name, created_at) VALUES (?1, ?2) ON CONFLICT (name) DO NOTHING",
            params![name, now_ms()],
        )?;
        Ok(self
            .conn
            .query_row("SELECT id FROM user_lists WHERE name = ?1", [name], |r| {
                r.get(0)
            })?)
    }

    pub fn rename_user_list(&self, id: i64, name: &str) -> Result<()> {
        let name = name.trim();
        if name.is_empty() || name.chars().count() > 60 {
            return Err(Error::Invalid(
                "list names must be 1 to 60 characters".into(),
            ));
        }
        let changed = self
            .conn
            .execute(
                "UPDATE user_lists SET name = ?2 WHERE id = ?1",
                params![id, name],
            )
            .map_err(|e| match e {
                rusqlite::Error::SqliteFailure(f, _)
                    if f.code == rusqlite::ErrorCode::ConstraintViolation =>
                {
                    Error::Invalid(format!("a list named `{name}` already exists"))
                }
                other => other.into(),
            })?;
        if changed == 0 {
            return Err(Error::NotFound(format!("list {id}")));
        }
        Ok(())
    }

    pub fn delete_user_list(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM user_list_items WHERE list_id = ?1", [id])?;
        self.conn
            .execute("DELETE FROM user_lists WHERE id = ?1", [id])?;
        Ok(())
    }

    /// Adds or removes a problem; adding twice keeps the original position.
    pub fn set_in_user_list(&self, list_id: i64, problem_id: i64, member: bool) -> Result<()> {
        let exists: bool = self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM user_lists WHERE id = ?1)",
            [list_id],
            |r| r.get(0),
        )?;
        if !exists {
            return Err(Error::NotFound(format!("list {list_id}")));
        }
        if member {
            self.conn.execute(
                "INSERT OR IGNORE INTO user_list_items (list_id, problem_id, added_at) VALUES (?1, ?2, ?3)",
                params![list_id, problem_id, now_ms()],
            )?;
        } else {
            self.conn.execute(
                "DELETE FROM user_list_items WHERE list_id = ?1 AND problem_id = ?2",
                params![list_id, problem_id],
            )?;
        }
        Ok(())
    }

    // ---- notes ----

    pub fn note(&self, problem_id: i64) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row(
                "SELECT body FROM notes WHERE problem_id = ?1",
                [problem_id],
                |r| r.get(0),
            )
            .optional()?)
    }

    /// Saves a problem's note; an empty (whitespace-only) note deletes it.
    pub fn save_note(&self, problem_id: i64, body: &str) -> Result<()> {
        if body.trim().is_empty() {
            self.conn
                .execute("DELETE FROM notes WHERE problem_id = ?1", [problem_id])?;
        } else {
            self.conn.execute(
                "INSERT INTO notes (problem_id, body, updated_at) VALUES (?1, ?2, ?3)
                 ON CONFLICT (problem_id) DO UPDATE SET body = excluded.body, updated_at = excluded.updated_at",
                params![problem_id, body, now_ms()],
            )?;
        }
        Ok(())
    }

    /// All notes as `(problem_id, body)`.
    pub fn notes(&self) -> Result<Vec<(i64, String)>> {
        let mut stmt = self
            .conn
            .prepare("SELECT problem_id, body FROM notes ORDER BY problem_id")?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    // ---- drafts ----

    pub fn draft(&self, problem_id: i64, language: &str) -> Result<Option<Draft>> {
        Ok(self
            .conn
            .query_row(
                "SELECT problem_id, language, code, updated_at FROM drafts
                 WHERE problem_id = ?1 AND language = ?2",
                params![problem_id, language],
                |r| {
                    Ok(Draft {
                        problem_id: r.get(0)?,
                        language: r.get(1)?,
                        code: r.get(2)?,
                        updated_at: r.get(3)?,
                    })
                },
            )
            .optional()?)
    }

    pub fn save_draft(&self, problem_id: i64, language: &str, code: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO drafts (problem_id, language, code, updated_at) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT (problem_id, language) DO UPDATE SET code = excluded.code,
                 updated_at = excluded.updated_at",
            params![problem_id, language, code, now_ms()],
        )?;
        Ok(())
    }

    pub fn delete_draft(&self, problem_id: i64, language: &str) -> Result<()> {
        self.conn.execute(
            "DELETE FROM drafts WHERE problem_id = ?1 AND language = ?2",
            params![problem_id, language],
        )?;
        Ok(())
    }

    pub fn drafts(&self) -> Result<Vec<Draft>> {
        let mut stmt = self
            .conn
            .prepare("SELECT problem_id, language, code, updated_at FROM drafts ORDER BY problem_id, language")?;
        let rows = stmt.query_map([], |r| {
            Ok(Draft {
                problem_id: r.get(0)?,
                language: r.get(1)?,
                code: r.get(2)?,
                updated_at: r.get(3)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    // ---- submissions ----

    pub fn insert_submission(
        &self,
        problem_id: i64,
        language: &str,
        code: &str,
        report: &RunReport,
        created_at: i64,
    ) -> Result<Submission> {
        let failure = report
            .failure
            .as_ref()
            .map(serde_json::to_string)
            .transpose()?;
        self.conn.execute(
            "INSERT INTO submissions (problem_id, language, code, verdict, passed, total, runtime_ms,
                memory_kb, error, failure, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                problem_id,
                language,
                code,
                format!("{:?}", report.verdict),
                report.passed as i64,
                report.total as i64,
                report.runtime_ms,
                report.memory_kb as i64,
                report.error,
                failure,
                created_at,
            ],
        )?;
        self.submission(self.conn.last_insert_rowid())
    }

    /// Inserts a fully specified submission (used by import); the id is reassigned.
    pub fn import_submission(&self, s: &Submission) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO submissions (problem_id, language, code, verdict, passed, total, runtime_ms,
                memory_kb, error, failure, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                s.problem_id,
                s.language,
                s.code,
                format!("{:?}", s.verdict),
                s.passed as i64,
                s.total as i64,
                s.runtime_ms,
                s.memory_kb as i64,
                s.error,
                s.failure.as_ref().map(serde_json::to_string).transpose()?,
                s.created_at,
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn submission(&self, id: i64) -> Result<Submission> {
        self.conn
            .query_row(
                &format!("SELECT {SUBMISSION_COLS} FROM submissions WHERE id = ?1"),
                [id],
                submission_from_row,
            )
            .optional()?
            .ok_or_else(|| Error::NotFound(format!("submission {id}")))
    }

    /// Submissions for one problem, newest first.
    pub fn submissions_for(&self, problem_id: i64) -> Result<Vec<Submission>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT {SUBMISSION_COLS} FROM submissions WHERE problem_id = ?1
             ORDER BY created_at DESC, id DESC"
        ))?;
        let rows = stmt.query_map([problem_id], submission_from_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// All submissions, oldest first.
    pub fn all_submissions(&self) -> Result<Vec<Submission>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT {SUBMISSION_COLS} FROM submissions ORDER BY created_at, id"
        ))?;
        let rows = stmt.query_map([], submission_from_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Lightweight view of every submission (no code or failure details), oldest first. This is
    /// all the statistics need, and it stays cheap with tens of thousands of submissions.
    pub fn activity(&self) -> Result<Vec<Activity>> {
        let mut stmt = self.conn.prepare(
            "SELECT problem_id, verdict, created_at FROM submissions ORDER BY created_at, id",
        )?;
        let rows = stmt.query_map([], |r| {
            let verdict: String = r.get(1)?;
            Ok(Activity {
                problem_id: r.get(0)?,
                verdict: Verdict::parse(&verdict).ok_or_else(|| {
                    conversion_err(1, Error::Invalid(format!("verdict {verdict}")))
                })?,
                created_at: r.get(2)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// `(problem_id, ever_accepted)` for every attempted problem.
    pub fn problem_statuses(&self) -> Result<Vec<(i64, bool)>> {
        let mut stmt = self.conn.prepare(
            "SELECT problem_id, MAX(verdict = 'Accepted') FROM submissions GROUP BY problem_id",
        )?;
        let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get::<_, i64>(1)? == 1)))?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    // ---- solves ----

    pub fn active_solve(&self, slug: &str) -> Result<Option<Solve>> {
        Ok(self
            .conn
            .query_row(
                &format!(
                    "SELECT {SOLVE_COLS} FROM solves WHERE slug = ?1 AND completed_at IS NULL"
                ),
                [slug],
                solve_from_row,
            )
            .optional()?)
    }

    pub fn solve(&self, id: i64) -> Result<Solve> {
        self.conn
            .query_row(
                &format!("SELECT {SOLVE_COLS} FROM solves WHERE id = ?1"),
                [id],
                solve_from_row,
            )
            .optional()?
            .ok_or_else(|| Error::NotFound(format!("solve {id}")))
    }

    /// Returns the active solve for the problem, starting one if needed.
    pub fn start_solve(&self, p: &ProblemRef, now: i64) -> Result<Solve> {
        if let Some(s) = self.active_solve(&p.slug)? {
            return Ok(s);
        }
        self.conn.execute(
            "INSERT INTO solves (problem_id, slug, title, difficulty, started_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![p.id, p.slug, p.title, p.difficulty.as_str(), now],
        )?;
        self.solve(self.conn.last_insert_rowid())
    }

    pub fn set_solve_elapsed(&self, id: i64, elapsed_ms: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE solves SET elapsed_ms = ?2 WHERE id = ?1 AND completed_at IS NULL",
            params![id, elapsed_ms.max(0)],
        )?;
        Ok(())
    }

    /// Abandons an active solve without recording it.
    pub fn discard_solve(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "DELETE FROM solves WHERE id = ?1 AND completed_at IS NULL",
            [id],
        )?;
        Ok(())
    }

    /// Applies a judged submission to the active solve: errors accumulate until an Accepted
    /// verdict completes it.
    pub fn record_attempt(
        &self,
        p: &ProblemRef,
        submission: &Submission,
        elapsed_ms: i64,
    ) -> Result<Solve> {
        let solve = self.start_solve(p, submission.created_at)?;
        if submission.verdict == Verdict::Accepted {
            self.conn.execute(
                "UPDATE solves SET completed_at = ?2, elapsed_ms = ?3, submission_id = ?4 WHERE id = ?1",
                params![solve.id, submission.created_at, elapsed_ms.max(0), submission.id],
            )?;
        } else {
            self.conn.execute(
                "UPDATE solves SET errors = errors + 1, elapsed_ms = ?2 WHERE id = ?1",
                params![solve.id, elapsed_ms.max(0)],
            )?;
        }
        self.solve(solve.id)
    }

    /// Inserts a completed solve (used by import); returns false for an exact duplicate.
    pub fn import_solve(&self, s: &Solve) -> Result<bool> {
        let completed = s.completed_at.unwrap_or(s.started_at);
        let exists: bool = self.conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM solves WHERE slug = ?1 AND completed_at = ?2 AND elapsed_ms = ?3)",
            params![s.slug, completed, s.elapsed_ms],
            |r| r.get(0),
        )?;
        if exists {
            return Ok(false);
        }
        self.conn.execute(
            "INSERT INTO solves (problem_id, slug, title, difficulty, started_at, completed_at,
                elapsed_ms, errors, submission_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, NULL)",
            params![
                s.problem_id,
                s.slug,
                s.title,
                s.difficulty.as_str(),
                s.started_at,
                completed,
                s.elapsed_ms,
                s.errors
            ],
        )?;
        Ok(true)
    }

    /// Completed solves, oldest first.
    pub fn completed_solves(&self) -> Result<Vec<Solve>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT {SOLVE_COLS} FROM solves WHERE completed_at IS NOT NULL ORDER BY completed_at, id"
        ))?;
        let rows = stmt.query_map([], solve_from_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Deletes all user history (submissions, solves, drafts, notes), keeping settings.
    pub fn clear_history(&self) -> Result<()> {
        self.conn.execute_batch(
            "DELETE FROM submissions; DELETE FROM solves; DELETE FROM drafts; DELETE FROM notes;",
        )?;
        Ok(())
    }

    pub fn transaction<T>(&self, f: impl FnOnce(&UserDb) -> Result<T>) -> Result<T> {
        self.conn.execute_batch("BEGIN")?;
        match f(self) {
            Ok(v) => {
                self.conn.execute_batch("COMMIT")?;
                Ok(v)
            }
            Err(e) => {
                self.conn.execute_batch("ROLLBACK")?;
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn report(verdict: Verdict) -> RunReport {
        RunReport {
            verdict,
            passed: 1,
            total: 2,
            runtime_ms: 12.5,
            memory_kb: 9000,
            cases: vec![],
            failure: (verdict != Verdict::Accepted).then(|| CaseResult {
                index: 1,
                passed: false,
                args: json!([1]),
                expected: json!(1),
                output: Some(json!(2)),
                stdout: String::new(),
                runtime_ns: 5,
            }),
            error: None,
            stderr: String::new(),
        }
    }

    fn pref() -> ProblemRef {
        ProblemRef {
            id: Some(1),
            slug: "two-sum".into(),
            title: "Two Sum".into(),
            difficulty: Difficulty::Easy,
        }
    }

    #[test]
    fn settings_roundtrip() {
        let db = UserDb::in_memory().unwrap();
        assert_eq!(db.settings().unwrap(), Settings::default());
        let s = Settings {
            font_size: 100,
            ..Default::default()
        };
        let saved = db.save_settings(&s).unwrap();
        assert_eq!(saved.font_size, 32);
        assert_eq!(db.settings().unwrap().font_size, 32);
        db.conn
            .execute("UPDATE settings SET json = 'garbage'", [])
            .unwrap();
        assert_eq!(db.settings().unwrap(), Settings::default());
    }

    #[test]
    fn local_benchmarks_crud() {
        let db = UserDb::in_memory().unwrap();
        assert_eq!(db.local_benchmark_summary().unwrap(), (0, None));
        let b = Benchmark {
            runtime_ms: 1.5,
            memory_kb: 100,
            runs: 3,
            local: false,
        };
        db.save_local_benchmark(1, "python", &b).unwrap();
        db.save_local_benchmark(1, "cpp", &b).unwrap();
        let got = db.local_benchmarks(1).unwrap();
        assert_eq!(got.len(), 2);
        assert!(got["python"].local);
        let (count, at) = db.local_benchmark_summary().unwrap();
        assert_eq!(count, 2);
        assert!(at.is_some());
        db.clear_local_benchmarks().unwrap();
        assert!(db.local_benchmarks(1).unwrap().is_empty());
    }

    #[test]
    fn user_lists_crud() {
        let db = UserDb::in_memory().unwrap();
        let fav = db.create_user_list("  Favorites ").unwrap();
        assert_eq!(
            db.create_user_list("Favorites").unwrap(),
            fav,
            "same name returns the list"
        );
        let onsite = db.create_user_list("Onsite").unwrap();
        db.set_in_user_list(fav, 20, true).unwrap();
        db.set_in_user_list(fav, 1, true).unwrap();
        db.set_in_user_list(fav, 20, true).unwrap();
        let lists = db.user_lists().unwrap();
        assert_eq!(lists[0].name, "Favorites");
        assert_eq!(lists[0].problem_ids, vec![20, 1]);
        db.set_in_user_list(fav, 20, false).unwrap();
        assert_eq!(db.user_lists().unwrap()[0].problem_ids, vec![1]);
        assert!(db
            .rename_user_list(onsite, "Favorites")
            .unwrap_err()
            .to_string()
            .contains("already exists"));
        db.rename_user_list(onsite, "Final round").unwrap();
        assert!(db.rename_user_list(999, "x").is_err());
        assert!(db.rename_user_list(onsite, " ").is_err());
        assert!(db.create_user_list("").is_err());
        assert!(db.create_user_list(&"x".repeat(61)).is_err());
        assert!(db.set_in_user_list(999, 1, true).is_err());
        db.delete_user_list(fav).unwrap();
        let lists = db.user_lists().unwrap();
        assert_eq!(lists.len(), 1);
        assert_eq!(lists[0].name, "Final round");
    }

    #[test]
    fn notes_crud() {
        let db = UserDb::in_memory().unwrap();
        assert_eq!(db.note(1).unwrap(), None);
        db.save_note(1, "use a hash map").unwrap();
        db.save_note(1, "use a hash map!").unwrap();
        db.save_note(2, "two pointers").unwrap();
        assert_eq!(db.note(1).unwrap().as_deref(), Some("use a hash map!"));
        assert_eq!(db.notes().unwrap().len(), 2);
        db.save_note(2, "   ").unwrap();
        assert_eq!(db.note(2).unwrap(), None);
        db.clear_history().unwrap();
        assert!(db.notes().unwrap().is_empty());
    }

    #[test]
    fn drafts_crud() {
        let db = UserDb::in_memory().unwrap();
        assert!(db.draft(1, "python").unwrap().is_none());
        db.save_draft(1, "python", "a").unwrap();
        db.save_draft(1, "python", "b").unwrap();
        assert_eq!(db.draft(1, "python").unwrap().unwrap().code, "b");
        assert_eq!(db.drafts().unwrap().len(), 1);
        db.delete_draft(1, "python").unwrap();
        assert!(db.draft(1, "python").unwrap().is_none());
    }

    #[test]
    fn submissions_and_statuses() {
        let db = UserDb::in_memory().unwrap();
        let wa = db
            .insert_submission(1, "python", "x", &report(Verdict::WrongAnswer), 100)
            .unwrap();
        assert_eq!(wa.failure.unwrap().output, Some(json!(2)));
        db.insert_submission(1, "python", "y", &report(Verdict::Accepted), 200)
            .unwrap();
        db.insert_submission(2, "python", "z", &report(Verdict::RuntimeError), 300)
            .unwrap();
        let subs = db.submissions_for(1).unwrap();
        assert_eq!(subs.len(), 2);
        assert_eq!(subs[0].verdict, Verdict::Accepted);
        assert_eq!(db.all_submissions().unwrap()[0].code, "x");
        let activity = db.activity().unwrap();
        assert_eq!(activity.len(), 3);
        assert_eq!(
            activity[0],
            Activity::from(&db.all_submissions().unwrap()[0])
        );
        let mut statuses = db.problem_statuses().unwrap();
        statuses.sort();
        assert_eq!(statuses, vec![(1, true), (2, false)]);
        assert!(db.submission(99).is_err());
        let copy = db.import_submission(&subs[0]).unwrap();
        assert_eq!(db.submission(copy).unwrap().code, "y");
    }

    #[test]
    fn solve_lifecycle() {
        let db = UserDb::in_memory().unwrap();
        let s = db.start_solve(&pref(), 1000).unwrap();
        assert_eq!(db.start_solve(&pref(), 2000).unwrap().id, s.id);
        db.set_solve_elapsed(s.id, 5000).unwrap();
        assert_eq!(
            db.active_solve("two-sum").unwrap().unwrap().elapsed_ms,
            5000
        );

        let wa = db
            .insert_submission(1, "python", "x", &report(Verdict::WrongAnswer), 3000)
            .unwrap();
        let s1 = db.record_attempt(&pref(), &wa, 6000).unwrap();
        assert_eq!((s1.errors, s1.completed_at), (1, None));

        let ac = db
            .insert_submission(1, "python", "y", &report(Verdict::Accepted), 4000)
            .unwrap();
        let s2 = db.record_attempt(&pref(), &ac, 7000).unwrap();
        assert_eq!(s2.completed_at, Some(4000));
        assert_eq!(s2.submission_id, Some(ac.id));
        assert_eq!(s2.elapsed_ms, 7000);
        assert!(db.active_solve("two-sum").unwrap().is_none());
        assert_eq!(db.completed_solves().unwrap().len(), 1);

        // Elapsed updates no longer apply once completed.
        db.set_solve_elapsed(s2.id, 1).unwrap();
        assert_eq!(db.solve(s2.id).unwrap().elapsed_ms, 7000);

        let fresh = db.start_solve(&pref(), 9000).unwrap();
        assert_ne!(fresh.id, s2.id);
        db.discard_solve(fresh.id).unwrap();
        assert!(db.active_solve("two-sum").unwrap().is_none());
        assert!(db.solve(fresh.id).is_err());
    }

    #[test]
    fn import_solves_dedupes() {
        let db = UserDb::in_memory().unwrap();
        let s = Solve {
            id: 0,
            problem_id: None,
            slug: "foo".into(),
            title: "Foo".into(),
            difficulty: Difficulty::Hard,
            started_at: 10,
            completed_at: Some(20),
            elapsed_ms: 10,
            errors: 2,
            submission_id: None,
        };
        assert!(db.import_solve(&s).unwrap());
        assert!(!db.import_solve(&s).unwrap());
        assert_eq!(db.completed_solves().unwrap()[0].errors, 2);
        db.clear_history().unwrap();
        assert!(db.completed_solves().unwrap().is_empty());
    }

    #[test]
    fn transactions_roll_back() {
        let db = UserDb::in_memory().unwrap();
        let r: Result<()> = db.transaction(|db| {
            db.save_draft(1, "python", "x")?;
            Err(Error::Invalid("nope".into()))
        });
        assert!(r.is_err());
        assert!(db.drafts().unwrap().is_empty());
        db.transaction(|db| db.save_draft(1, "python", "x"))
            .unwrap();
        assert_eq!(db.drafts().unwrap().len(), 1);
    }

    #[test]
    fn file_db_and_version_guard() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nested/user.db");
        {
            let db = UserDb::open(&path).unwrap();
            db.save_draft(1, "python", "x").unwrap();
        }
        assert_eq!(UserDb::open(&path).unwrap().drafts().unwrap().len(), 1);
        let conn = Connection::open(&path).unwrap();
        conn.execute_batch("PRAGMA user_version = 99").unwrap();
        drop(conn);
        assert!(UserDb::open(&path).is_err());
    }

    #[test]
    fn corrupt_rows_error() {
        let db = UserDb::in_memory().unwrap();
        db.insert_submission(1, "python", "x", &report(Verdict::Accepted), 1)
            .unwrap();
        db.conn
            .execute("UPDATE submissions SET verdict = 'Bogus'", [])
            .unwrap();
        assert!(db.all_submissions().is_err());
        assert!(db.activity().is_err());
        db.conn
            .execute(
                "UPDATE submissions SET verdict = 'Accepted', failure = '{'",
                [],
            )
            .unwrap();
        assert!(db.all_submissions().is_err());
        db.start_solve(&pref(), 1).unwrap();
        db.conn
            .execute("UPDATE solves SET difficulty = 'Wat'", [])
            .unwrap();
        assert!(db.active_solve("two-sum").is_err());
    }
}
