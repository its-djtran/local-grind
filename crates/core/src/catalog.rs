//! The static, read-only problem catalog (`problems.db`), produced by `lg-content` at build
//! time. Everything here is pre-rendered so the app never parses markdown or generates tests.

use rusqlite::{params, Connection, OpenFlags, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

use crate::error::{Error, Result};
use crate::lang::Language;
use crate::runner::TestCase;
use crate::types::{Comparator, Difficulty, Signature};

pub const SCHEMA_VERSION: i64 = 1;

/// Orders company tags by how recently they were reported, then by frequency.
const RECENCY_RANK: &str = "CASE recency WHEN '30d' THEN 0 WHEN '3m' THEN 1 WHEN '6m' THEN 2 \
     WHEN '6m+' THEN 3 ELSE 4 END";

const SCHEMA: &str = "
CREATE TABLE meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE problems (
    id INTEGER PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    title TEXT NOT NULL,
    difficulty TEXT NOT NULL,
    topics TEXT NOT NULL,
    signature TEXT NOT NULL,
    comparator TEXT NOT NULL,
    time_limit_ms INTEGER NOT NULL,
    description_html TEXT NOT NULL,
    editorial_html TEXT NOT NULL,
    hints TEXT NOT NULL,
    examples TEXT NOT NULL,
    test_count INTEGER NOT NULL
);
CREATE TABLE tests (problem_id INTEGER PRIMARY KEY, data BLOB NOT NULL);
CREATE TABLE solutions (
    problem_id INTEGER NOT NULL, language TEXT NOT NULL, code TEXT NOT NULL,
    PRIMARY KEY (problem_id, language)
);
CREATE TABLE benchmarks (
    problem_id INTEGER NOT NULL, language TEXT NOT NULL,
    runtime_ms REAL NOT NULL, memory_kb INTEGER NOT NULL, runs INTEGER NOT NULL,
    PRIMARY KEY (problem_id, language)
);
CREATE TABLE lists (
    id TEXT PRIMARY KEY, name TEXT NOT NULL, description TEXT NOT NULL, position INTEGER NOT NULL
);
CREATE TABLE list_items (
    list_id TEXT NOT NULL, problem_id INTEGER NOT NULL, grp TEXT NOT NULL, position INTEGER NOT NULL,
    PRIMARY KEY (list_id, problem_id)
);
CREATE TABLE problem_companies (
    problem_id INTEGER NOT NULL, company TEXT NOT NULL, frequency REAL NOT NULL, recency TEXT NOT NULL,
    PRIMARY KEY (problem_id, company)
);
CREATE INDEX problem_companies_company ON problem_companies (company);
";

/// A worked example shown in the statement and preloaded into the testcase console.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Example {
    /// LeetCode-style testcase text: one JSON value per parameter, one per line.
    pub input: String,
    pub output: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProblemSummary {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub difficulty: Difficulty,
    pub topics: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Benchmark {
    pub runtime_ms: f64,
    pub memory_kb: u64,
    pub runs: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompanyTag {
    pub company: String,
    pub frequency: f64,
    /// Most recent window the problem was reported in: `30d`, `3m`, `6m`, `6m+` or `all`.
    pub recency: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Problem {
    #[serde(flatten)]
    pub summary: ProblemSummary,
    pub signature: Signature,
    pub comparator: Comparator,
    pub time_limit_ms: u64,
    pub description_html: String,
    pub editorial_html: String,
    pub hints: Vec<String>,
    pub examples: Vec<Example>,
    pub test_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListItem {
    pub problem_id: i64,
    pub group: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProblemList {
    pub id: String,
    pub name: String,
    pub description: String,
    pub items: Vec<ListItem>,
}

pub struct Catalog {
    conn: Connection,
}

fn json_col<T: serde::de::DeserializeOwned>(s: String) -> rusqlite::Result<T> {
    serde_json::from_str(&s).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })
}

fn summary_from_row(row: &rusqlite::Row) -> rusqlite::Result<ProblemSummary> {
    let difficulty: String = row.get(3)?;
    Ok(ProblemSummary {
        id: row.get(0)?,
        slug: row.get(1)?,
        title: row.get(2)?,
        difficulty: difficulty.parse().map_err(|e: Error| {
            rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, e.into())
        })?,
        topics: json_col(row.get(4)?)?,
    })
}

impl Catalog {
    pub fn open(path: &Path) -> Result<Catalog> {
        let conn = Connection::open_with_flags(
            path,
            OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        Catalog::from_connection(conn)
    }

    pub fn from_connection(conn: Connection) -> Result<Catalog> {
        let version: Option<String> = conn
            .query_row(
                "SELECT value FROM meta WHERE key = 'schema_version'",
                [],
                |r| r.get(0),
            )
            .optional()?;
        if version.as_deref() != Some(&SCHEMA_VERSION.to_string()) {
            return Err(Error::Invalid(format!(
                "catalog schema version {version:?} is not supported (expected {SCHEMA_VERSION})"
            )));
        }
        Ok(Catalog { conn })
    }

    pub fn meta(&self, key: &str) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row("SELECT value FROM meta WHERE key = ?1", [key], |r| r.get(0))
            .optional()?)
    }

    pub fn summaries(&self) -> Result<Vec<ProblemSummary>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, slug, title, difficulty, topics FROM problems ORDER BY id")?;
        let rows = stmt.query_map([], summary_from_row)?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    pub fn problem(&self, id: i64) -> Result<Problem> {
        self.conn
            .query_row(
                "SELECT id, slug, title, difficulty, topics, signature, comparator, time_limit_ms,
                        description_html, editorial_html, hints, examples, test_count
                 FROM problems WHERE id = ?1",
                [id],
                |row| {
                    Ok(Problem {
                        summary: summary_from_row(row)?,
                        signature: json_col(row.get(5)?)?,
                        comparator: json_col(row.get(6)?)?,
                        time_limit_ms: row.get::<_, i64>(7)? as u64,
                        description_html: row.get(8)?,
                        editorial_html: row.get(9)?,
                        hints: json_col(row.get(10)?)?,
                        examples: json_col(row.get(11)?)?,
                        test_count: row.get::<_, i64>(12)? as usize,
                    })
                },
            )
            .optional()?
            .ok_or_else(|| Error::NotFound(format!("problem {id}")))
    }

    pub fn tests(&self, id: i64) -> Result<Vec<TestCase>> {
        let blob: Vec<u8> = self
            .conn
            .query_row("SELECT data FROM tests WHERE problem_id = ?1", [id], |r| {
                r.get(0)
            })
            .optional()?
            .ok_or_else(|| Error::NotFound(format!("tests for problem {id}")))?;
        let raw = miniz_oxide::inflate::decompress_to_vec(&blob)
            .map_err(|e| Error::Invalid(format!("corrupt test data for problem {id}: {e:?}")))?;
        let data = String::from_utf8(raw)
            .map_err(|_| Error::Invalid(format!("test data for problem {id} is not UTF-8")))?;
        data.lines()
            .filter(|l| !l.is_empty())
            .map(|l| Ok(serde_json::from_str(l)?))
            .collect()
    }

    pub fn solution(&self, id: i64, lang: Language) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row(
                "SELECT code FROM solutions WHERE problem_id = ?1 AND language = ?2",
                params![id, lang.id()],
                |r| r.get(0),
            )
            .optional()?)
    }

    pub fn benchmarks(&self, id: i64) -> Result<BTreeMap<String, Benchmark>> {
        let mut stmt = self.conn.prepare(
            "SELECT language, runtime_ms, memory_kb, runs FROM benchmarks WHERE problem_id = ?1",
        )?;
        let rows = stmt.query_map([id], |r| {
            Ok((
                r.get::<_, String>(0)?,
                Benchmark {
                    runtime_ms: r.get(1)?,
                    memory_kb: r.get::<_, i64>(2)? as u64,
                    runs: r.get::<_, i64>(3)? as u32,
                },
            ))
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    pub fn lists(&self) -> Result<Vec<ProblemList>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, description FROM lists ORDER BY position")?;
        let mut lists: Vec<ProblemList> = stmt
            .query_map([], |r| {
                Ok(ProblemList {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    description: r.get(2)?,
                    items: Vec::new(),
                })
            })?
            .collect::<rusqlite::Result<_>>()?;
        let mut items = self.conn.prepare(
            "SELECT problem_id, grp FROM list_items WHERE list_id = ?1 ORDER BY position",
        )?;
        for list in &mut lists {
            list.items = items
                .query_map([&list.id], |r| {
                    Ok(ListItem {
                        problem_id: r.get(0)?,
                        group: r.get(1)?,
                    })
                })?
                .collect::<rusqlite::Result<_>>()?;
        }
        Ok(lists)
    }

    /// Company tags for one problem, most frequent first.
    pub fn companies_for(&self, id: i64) -> Result<Vec<CompanyTag>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT company, frequency, recency FROM problem_companies
             WHERE problem_id = ?1 ORDER BY {RECENCY_RANK}, frequency DESC, company"
        ))?;
        let rows = stmt.query_map([id], |r| {
            Ok(CompanyTag {
                company: r.get(0)?,
                frequency: r.get(1)?,
                recency: r.get(2)?,
            })
        })?;
        Ok(rows.collect::<rusqlite::Result<_>>()?)
    }

    /// Every company with the ids of its tagged problems (most recent and frequent first), for
    /// filtering and sorting the problem list.
    pub fn company_index(&self) -> Result<BTreeMap<String, Vec<i64>>> {
        let mut stmt = self.conn.prepare(&format!(
            "SELECT company, problem_id FROM problem_companies
             ORDER BY company, {RECENCY_RANK}, frequency DESC, problem_id"
        ))?;
        let mut index: BTreeMap<String, Vec<i64>> = BTreeMap::new();
        let rows = stmt.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))?;
        for row in rows {
            let (company, id) = row?;
            index.entry(company).or_default().push(id);
        }
        Ok(index)
    }
}

/// Builds a catalog database. Used by the content builder and by tests.
pub struct CatalogWriter {
    conn: Connection,
}

impl CatalogWriter {
    pub fn create(path: &Path) -> Result<CatalogWriter> {
        if path.exists() {
            std::fs::remove_file(path)?;
        }
        CatalogWriter::init(Connection::open(path)?)
    }

    pub fn in_memory() -> Result<CatalogWriter> {
        CatalogWriter::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> Result<CatalogWriter> {
        conn.execute_batch("PRAGMA journal_mode = OFF; PRAGMA synchronous = OFF;")?;
        conn.execute_batch(SCHEMA)?;
        conn.execute_batch("BEGIN")?;
        let w = CatalogWriter { conn };
        w.set_meta("schema_version", &SCHEMA_VERSION.to_string())?;
        Ok(w)
    }

    pub fn set_meta(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO meta (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    pub fn add_problem(&self, p: &Problem, tests: &[TestCase]) -> Result<()> {
        let s = &p.summary;
        self.conn.execute(
            "INSERT INTO problems (id, slug, title, difficulty, topics, signature, comparator,
                time_limit_ms, description_html, editorial_html, hints, examples, test_count)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                s.id,
                s.slug,
                s.title,
                s.difficulty.as_str(),
                serde_json::to_string(&s.topics)?,
                serde_json::to_string(&p.signature)?,
                serde_json::to_string(&p.comparator)?,
                p.time_limit_ms as i64,
                p.description_html,
                p.editorial_html,
                serde_json::to_string(&p.hints)?,
                serde_json::to_string(&p.examples)?,
                tests.len() as i64,
            ],
        )?;
        let mut data = String::new();
        for t in tests {
            data.push_str(&serde_json::to_string(t)?);
            data.push('\n');
        }
        let blob = miniz_oxide::deflate::compress_to_vec(data.as_bytes(), 6);
        self.conn.execute(
            "INSERT INTO tests (problem_id, data) VALUES (?1, ?2)",
            params![s.id, blob],
        )?;
        Ok(())
    }

    pub fn add_solution(&self, id: i64, lang: Language, code: &str) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO solutions (problem_id, language, code) VALUES (?1, ?2, ?3)",
            params![id, lang.id(), code],
        )?;
        Ok(())
    }

    pub fn add_benchmark(&self, id: i64, lang: &str, b: &Benchmark) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO benchmarks (problem_id, language, runtime_ms, memory_kb, runs)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, lang, b.runtime_ms, b.memory_kb as i64, b.runs as i64],
        )?;
        Ok(())
    }

    pub fn add_list(&self, list: &ProblemList, position: usize) -> Result<()> {
        self.conn.execute(
            "INSERT INTO lists (id, name, description, position) VALUES (?1, ?2, ?3, ?4)",
            params![list.id, list.name, list.description, position as i64],
        )?;
        for (i, item) in list.items.iter().enumerate() {
            self.conn.execute(
                "INSERT INTO list_items (list_id, problem_id, grp, position) VALUES (?1, ?2, ?3, ?4)",
                params![list.id, item.problem_id, item.group, i as i64],
            )?;
        }
        Ok(())
    }

    pub fn add_company(&self, id: i64, tag: &CompanyTag) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO problem_companies (problem_id, company, frequency, recency)
             VALUES (?1, ?2, ?3, ?4)",
            params![id, tag.company, tag.frequency, tag.recency],
        )?;
        Ok(())
    }

    /// Commits and returns a read-only view over the same connection.
    pub fn finish(self) -> Result<Catalog> {
        self.conn.execute_batch("COMMIT")?;
        Catalog::from_connection(self.conn)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use serde_json::json;

    pub fn sample_problem(id: i64, slug: &str, difficulty: Difficulty) -> Problem {
        Problem {
            summary: ProblemSummary {
                id,
                slug: slug.into(),
                title: format!("Title {slug}"),
                difficulty,
                topics: vec!["Array".into()],
            },
            signature: serde_json::from_value(json!({"kind": "function", "name": "twoSum",
                "params": [{"name": "nums", "type": "int[]"}, {"name": "target", "type": "int"}],
                "returns": "int[]"}))
            .unwrap(),
            comparator: Comparator::Unordered,
            time_limit_ms: 2000,
            description_html: "<p>desc</p>".into(),
            editorial_html: "<p>ed</p>".into(),
            hints: vec!["hint".into()],
            examples: vec![Example {
                input: "[2,7,11,15]\n9".into(),
                output: "[0,1]".into(),
                explanation: None,
            }],
            test_count: 0,
        }
    }

    pub fn sample_catalog() -> Catalog {
        let w = CatalogWriter::in_memory().unwrap();
        w.set_meta("built_at", "2026-10-07").unwrap();
        let tests = vec![
            TestCase {
                args: json!([[2, 7, 11, 15], 9]),
                expected: json!([0, 1]),
            },
            TestCase {
                args: json!([[3, 2, 4], 6]),
                expected: json!([1, 2]),
            },
        ];
        w.add_problem(&sample_problem(1, "two-sum", Difficulty::Easy), &tests)
            .unwrap();
        w.add_problem(
            &sample_problem(2, "add-two", Difficulty::Medium),
            &tests[..1],
        )
        .unwrap();
        w.add_problem(
            &sample_problem(3, "hard-one", Difficulty::Hard),
            &tests[..1],
        )
        .unwrap();
        w.add_solution(
            1,
            Language::Python,
            "class Solution:\n    def twoSum(self, nums, target):\n        seen = {}\n        for i, n in enumerate(nums):\n            if target - n in seen:\n                return [seen[target - n], i]\n            seen[n] = i\n",
        )
        .unwrap();
        w.add_benchmark(
            1,
            "python",
            &Benchmark {
                runtime_ms: 1.5,
                memory_kb: 9000,
                runs: 10,
            },
        )
        .unwrap();
        w.add_list(
            &ProblemList {
                id: "blind75".into(),
                name: "Blind 75".into(),
                description: "classic".into(),
                items: vec![
                    ListItem {
                        problem_id: 1,
                        group: "Array".into(),
                    },
                    ListItem {
                        problem_id: 2,
                        group: "Linked List".into(),
                    },
                ],
            },
            0,
        )
        .unwrap();
        for (company, f) in [("Google", 80.0), ("Amazon", 95.5)] {
            w.add_company(
                1,
                &CompanyTag {
                    company: company.into(),
                    frequency: f,
                    recency: "30d".into(),
                },
            )
            .unwrap();
        }
        w.add_company(
            2,
            &CompanyTag {
                company: "Google".into(),
                frequency: 10.0,
                recency: "all".into(),
            },
        )
        .unwrap();
        w.finish().unwrap()
    }

    #[test]
    fn reads_back_everything() {
        let c = sample_catalog();
        assert_eq!(c.meta("built_at").unwrap().as_deref(), Some("2026-10-07"));
        assert_eq!(c.meta("missing").unwrap(), None);
        let s = c.summaries().unwrap();
        assert_eq!(s.len(), 3);
        assert_eq!(s[0].slug, "two-sum");
        let p = c.problem(1).unwrap();
        assert_eq!(p.test_count, 2);
        assert_eq!(p.examples[0].output, "[0,1]");
        assert_eq!(p.comparator, Comparator::Unordered);
        assert!(matches!(c.problem(99), Err(Error::NotFound(_))));
        assert_eq!(c.tests(1).unwrap().len(), 2);
        assert!(c.tests(99).is_err());
        assert!(c.solution(1, Language::Python).unwrap().is_some());
        assert!(c.solution(2, Language::Python).unwrap().is_none());
        assert_eq!(c.benchmarks(1).unwrap()["python"].runs, 10);
        let lists = c.lists().unwrap();
        assert_eq!(lists[0].items.len(), 2);
        assert_eq!(lists[0].items[1].group, "Linked List");
        let companies = c.companies_for(1).unwrap();
        assert_eq!(companies[0].company, "Amazon");
        let index = c.company_index().unwrap();
        assert_eq!(index["Google"], vec![1, 2]);
    }

    #[test]
    fn file_roundtrip_and_version_check() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("p.db");
        std::fs::write(&path, "stale").unwrap();
        let w = CatalogWriter::create(&path).unwrap();
        w.add_problem(&sample_problem(1, "a", Difficulty::Easy), &[])
            .unwrap();
        drop(w.finish().unwrap());
        let c = Catalog::open(&path).unwrap();
        assert_eq!(c.summaries().unwrap().len(), 1);

        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(SCHEMA).unwrap();
        assert!(Catalog::from_connection(conn).is_err());
    }

    #[test]
    fn corrupt_rows_surface_errors() {
        let w = CatalogWriter::in_memory().unwrap();
        w.add_problem(&sample_problem(1, "a", Difficulty::Easy), &[])
            .unwrap();
        w.conn
            .execute("UPDATE problems SET difficulty = 'Insane'", [])
            .unwrap();
        let blob = miniz_oxide::deflate::compress_to_vec(b"not json", 6);
        w.conn
            .execute("INSERT INTO tests VALUES (5, ?1)", [blob])
            .unwrap();
        w.conn
            .execute("INSERT INTO tests VALUES (6, x'0102')", [])
            .unwrap();
        let invalid_utf8 = miniz_oxide::deflate::compress_to_vec(&[0xff, 0xfe], 6);
        w.conn
            .execute("INSERT INTO tests VALUES (7, ?1)", [invalid_utf8])
            .unwrap();
        let c = w.finish().unwrap();
        assert!(c.summaries().is_err());
        assert!(c.tests(5).is_err());
        assert!(c.tests(6).unwrap_err().to_string().contains("corrupt"));
        assert!(c.tests(7).unwrap_err().to_string().contains("UTF-8"));
    }
}
