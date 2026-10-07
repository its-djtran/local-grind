//! User preferences, persisted as one JSON document in the user database.

use serde::{Deserialize, Serialize};

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
    /// Explicit interpreter/compiler paths; empty means auto-detect.
    pub python_path: String,
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
            python_path: String::new(),
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
