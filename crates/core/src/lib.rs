//! Local Grind core: problem model, judge, language drivers, persistence and metrics.
//! The Tauri shell is a thin layer over [`service::Service`].

pub mod catalog;
pub mod error;
pub mod judge;
pub mod lang;
pub mod metrics;
pub mod portable;
pub mod runner;
pub mod service;
pub mod settings;
pub mod timeutil;
pub mod types;
pub mod userdb;

pub use error::{Error, Result};
