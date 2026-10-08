//! User preferences, persisted as one JSON document in the user database.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

use crate::lang::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Keymap {
    Standard,
    Vim,
}

/// Knobs that let users trade judging strictness for local compute.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct EvalSettings {
    /// Multiplies every problem's time limit (slow machines may want > 1).
    pub time_limit_multiplier: f64,
    pub memory_limit_mb: u64,
    /// Maximum concurrent judge processes. With 1, "Run" executes your code and the reference
    /// solution one after the other instead of side by side.
    pub max_parallel_jobs: usize,
    /// Run judged programs at a lower scheduling priority so the rest of the machine stays
    /// responsive (timings may vary more under load).
    pub low_priority: bool,
    /// Explicit interpreter/compiler paths by language id; missing or empty means auto-detect.
    pub toolchain_paths: BTreeMap<String, String>,
}

impl Default for EvalSettings {
    fn default() -> Self {
        EvalSettings {
            time_limit_multiplier: 1.0,
            memory_limit_mb: 1024,
            max_parallel_jobs: std::thread::available_parallelism()
                .map(|n| (n.get() / 2).max(1))
                .unwrap_or(1),
            low_priority: false,
            toolchain_paths: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub theme: Theme,
    /// Interface font size in px.
    pub font_size: u32,
    /// Editor font size in px.
    pub editor_font_size: u32,
    pub font_family: String,
    pub language: Language,
    pub keymap: Keymap,
    pub tab_size: u32,
    /// Start the solve timer on the first keystroke in a problem.
    pub auto_start_timer: bool,
    pub show_timer: bool,
    pub eval: EvalSettings,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            theme: Theme::Dark,
            font_size: 14,
            editor_font_size: 14,
            font_family: "Inconsolata".into(),
            language: Language::Python,
            keymap: Keymap::Standard,
            tab_size: 4,
            auto_start_timer: true,
            show_timer: true,
            eval: EvalSettings::default(),
        }
    }
}

impl Settings {
    /// Reads stored or imported settings field by field: a value this version cannot parse (an
    /// option from a newer or older app, a hand edit) falls back to its default instead of
    /// discarding every other setting. The result is sanitized.
    pub fn from_value_lenient(stored: &Value) -> Settings {
        // Fast path: missing fields already take their defaults (`#[serde(default)]`).
        if let Ok(s) = Settings::deserialize(stored) {
            return s.sanitized();
        }
        let mut merged = serde_json::to_value(Settings::default()).expect("settings serialize");
        if let Value::Object(fields) = stored {
            for (key, value) in fields {
                match (key.as_str(), value) {
                    ("eval", Value::Object(eval)) => {
                        for (k, v) in eval {
                            try_set(&mut merged, &["eval", k], v);
                        }
                    }
                    _ => try_set(&mut merged, &[key], value),
                }
            }
        }
        serde_json::from_value::<Settings>(merged)
            .unwrap_or_default()
            .sanitized()
    }

    /// Clamps values into supported ranges so a hand-edited import cannot break the UI.
    pub fn sanitized(mut self) -> Settings {
        self.font_size = self.font_size.clamp(10, 32);
        self.editor_font_size = self.editor_font_size.clamp(10, 32);
        self.tab_size = self.tab_size.clamp(1, 8);
        if self.font_family.trim().is_empty() {
            self.font_family = Settings::default().font_family;
        }
        let e = &mut self.eval;
        if !e.time_limit_multiplier.is_finite() {
            e.time_limit_multiplier = 1.0;
        }
        e.time_limit_multiplier = e.time_limit_multiplier.clamp(0.25, 20.0);
        e.memory_limit_mb = e.memory_limit_mb.clamp(64, 64 * 1024);
        e.max_parallel_jobs = e.max_parallel_jobs.clamp(1, 64);
        self
    }
}

/// Sets `path` in `merged` to `value` if the result still parses as [`Settings`].
fn try_set(merged: &mut Value, path: &[&str], value: &Value) {
    let mut candidate = merged.clone();
    *path.iter().fold(&mut candidate, |v, k| &mut v[*k]) = value.clone();
    if serde_json::from_value::<Settings>(candidate.clone()).is_ok() {
        *merged = candidate;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_and_partial_json() {
        let s: Settings =
            serde_json::from_str(r#"{"theme":"light","eval":{"memory_limit_mb":512}}"#).unwrap();
        assert_eq!(s.theme, Theme::Light);
        assert_eq!(s.eval.memory_limit_mb, 512);
        assert_eq!(s.eval.time_limit_multiplier, 1.0);
        assert_eq!(s.font_family, "Inconsolata");
        assert!(s.eval.max_parallel_jobs >= 1);
        let back: Settings = serde_json::from_str(&serde_json::to_string(&s).unwrap()).unwrap();
        assert_eq!(back, s);
    }

    #[test]
    fn lenient_parsing_keeps_every_readable_field() {
        let stored = serde_json::json!({
            "theme": "neon",
            "font_size": 20,
            "language": "java",
            "keymap": "vim",
            "eval": {
                "memory_limit_mb": 512,
                "max_parallel_jobs": "many",
                "toolchain_paths": {"python": "/opt/py/bin/python3"}
            },
            "from_a_newer_app": true
        });
        let s = Settings::from_value_lenient(&stored);
        let d = Settings::default();
        assert_eq!(
            (s.theme, s.language),
            (d.theme, d.language),
            "unreadable values default"
        );
        assert_eq!((s.font_size, s.keymap), (20, Keymap::Vim));
        assert_eq!(s.eval.memory_limit_mb, 512);
        assert_eq!(s.eval.max_parallel_jobs, d.eval.max_parallel_jobs);
        assert_eq!(s.eval.toolchain_paths["python"], "/opt/py/bin/python3");
        // Values are sanitized; non-objects give the defaults.
        let s = Settings::from_value_lenient(&serde_json::json!({"font_size": 99}));
        assert_eq!(s.font_size, 32);
        assert_eq!(Settings::from_value_lenient(&serde_json::json!("x")), d);
    }

    #[test]
    fn sanitize_clamps() {
        let mut s = Settings {
            font_size: 2,
            editor_font_size: 99,
            tab_size: 0,
            font_family: " ".into(),
            keymap: Keymap::Vim,
            theme: Theme::System,
            ..Default::default()
        };
        s.eval.time_limit_multiplier = f64::NAN;
        s.eval.memory_limit_mb = 1;
        s.eval.max_parallel_jobs = 0;
        let s = s.sanitized();
        assert_eq!((s.font_size, s.editor_font_size, s.tab_size), (10, 32, 1));
        assert_eq!(s.font_family, "Inconsolata");
        assert_eq!(s.eval.time_limit_multiplier, 1.0);
        assert_eq!(s.eval.memory_limit_mb, 64);
        assert_eq!(s.eval.max_parallel_jobs, 1);
    }
}
