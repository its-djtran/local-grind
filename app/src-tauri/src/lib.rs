//! Tauri shell: resolves data locations, owns the [`Service`] and exposes it as commands.
//! All behaviour lives in `localgrind-core`; commands here only adapt types and threads.

use localgrind_core::lang::Language;
use localgrind_core::metrics::{ReviewItem, Stats, Zone};
use localgrind_core::portable::ImportSummary;
use localgrind_core::service::{
    Bootstrap, CalibrationStatus, CalibrationSummary, ProblemView, RunOutcome, Service,
    SubmitOutcome,
};
use localgrind_core::settings::Settings;
use localgrind_core::userdb::{Solve, Submission, UserList};
use localgrind_core::{Error, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, State};

type Svc<'a> = State<'a, Arc<Service>>;

#[derive(Clone, Serialize)]
struct Progress {
    problem_id: i64,
    done: usize,
    total: usize,
}

/// Picks the catalog: an explicit override, the bundled resource, or the repository build.
pub fn resolve_catalog(
    env_override: Option<PathBuf>,
    resource_dir: Option<PathBuf>,
    manifest_dir: &Path,
) -> Option<PathBuf> {
    env_override
        .into_iter()
        .chain(resource_dir.map(|d| d.join("problems.db")))
        .chain([manifest_dir.join("../../assets/problems.db")])
        .find(|p| p.is_file())
}

/// The relocatable CPython shipped in the app bundle (see scripts/fetch_python.sh), if present.
pub fn bundled_python(resource_dir: Option<PathBuf>) -> Option<PathBuf> {
    resource_dir
        .map(|d| d.join("python/bin/python3"))
        .filter(|p| p.is_file())
}

/// User database location: `LOCALGRIND_DATA_DIR` (used by the smoke test) or the app data dir.
pub fn resolve_user_db(env_override: Option<PathBuf>, app_data_dir: PathBuf) -> PathBuf {
    env_override.unwrap_or(app_data_dir).join("user.db")
}

async fn blocking<T: Send + 'static>(f: impl FnOnce() -> Result<T> + Send + 'static) -> Result<T> {
    tauri::async_runtime::spawn_blocking(f)
        .await
        .map_err(|e| Error::Invalid(format!("background task failed: {e}")))?
}

#[tauri::command]
fn bootstrap(svc: Svc) -> Result<Bootstrap> {
    svc.bootstrap()
}

#[tauri::command]
fn get_problem(svc: Svc, id: i64, language: Option<Language>) -> Result<ProblemView> {
    svc.problem(id, language)
}

#[tauri::command]
fn save_draft(svc: Svc, id: i64, language: Language, code: String) -> Result<()> {
    svc.save_draft(id, language, &code)
}

#[tauri::command]
fn create_user_list(svc: Svc, name: String) -> Result<Vec<UserList>> {
    svc.create_user_list(&name)
}

#[tauri::command]
fn rename_user_list(svc: Svc, id: i64, name: String) -> Result<Vec<UserList>> {
    svc.rename_user_list(id, &name)
}

#[tauri::command]
fn delete_user_list(svc: Svc, id: i64) -> Result<Vec<UserList>> {
    svc.delete_user_list(id)
}

#[tauri::command]
fn set_in_user_list(
    svc: Svc,
    list_id: i64,
    problem_id: i64,
    member: bool,
) -> Result<Vec<UserList>> {
    svc.set_in_user_list(list_id, problem_id, member)
}

#[tauri::command]
fn save_note(svc: Svc, id: i64, body: String) -> Result<()> {
    svc.save_note(id, &body)
}

#[tauri::command]
fn reset_code(svc: Svc, id: i64, language: Language) -> Result<String> {
    svc.reset_code(id, language)
}

#[tauri::command]
async fn run_code(
    svc: Svc<'_>,
    id: i64,
    language: Language,
    code: String,
    input: String,
) -> Result<RunOutcome> {
    let svc = svc.inner().clone();
    blocking(move || svc.run(id, language, &code, &input)).await
}

#[tauri::command]
async fn submit_code(
    app: AppHandle,
    svc: Svc<'_>,
    id: i64,
    language: Language,
    code: String,
    elapsed_ms: i64,
) -> Result<SubmitOutcome> {
    let svc = svc.inner().clone();
    blocking(move || {
        svc.submit(id, language, &code, elapsed_ms, &mut |done, total| {
            let _ = app.emit(
                "submit-progress",
                Progress {
                    problem_id: id,
                    done,
                    total,
                },
            );
        })
    })
    .await
}

#[tauri::command]
fn list_submissions(svc: Svc, id: i64) -> Result<Vec<Submission>> {
    svc.submissions(id)
}

#[tauri::command]
fn start_solve(svc: Svc, id: i64) -> Result<Solve> {
    svc.start_solve(id)
}

#[tauri::command]
fn update_solve(svc: Svc, solve_id: i64, elapsed_ms: i64) -> Result<()> {
    svc.update_solve(solve_id, elapsed_ms)
}

#[tauri::command]
fn discard_solve(svc: Svc, solve_id: i64) -> Result<()> {
    svc.discard_solve(solve_id)
}

#[tauri::command]
fn save_settings(svc: Svc, settings: Settings) -> Result<Settings> {
    svc.save_settings(&settings)
}

#[tauri::command]
async fn get_stats(svc: Svc<'_>, recent_n: usize, review_min_days: f64) -> Result<Stats> {
    let svc = svc.inner().clone();
    blocking(move || svc.stats(recent_n, review_min_days, Zone::Local)).await
}

#[tauri::command]
async fn get_review_queue(
    svc: Svc<'_>,
    review_min_days: f64,
    limit: usize,
) -> Result<Vec<ReviewItem>> {
    let svc = svc.inner().clone();
    blocking(move || svc.review_queue(review_min_days, limit)).await
}

#[tauri::command]
async fn export_history(svc: Svc<'_>, path: PathBuf) -> Result<()> {
    let svc = svc.inner().clone();
    blocking(move || svc.export_to(&path)).await
}

#[tauri::command]
async fn import_history(
    svc: Svc<'_>,
    path: PathBuf,
    apply_settings: bool,
) -> Result<ImportSummary> {
    let svc = svc.inner().clone();
    blocking(move || svc.import_from(&path, apply_settings)).await
}

#[tauri::command]
fn clear_history(svc: Svc) -> Result<()> {
    svc.clear_history()
}

#[tauri::command]
async fn check_toolchains(svc: Svc<'_>) -> Result<Vec<localgrind_core::lang::ToolchainStatus>> {
    let svc = svc.inner().clone();
    blocking(move || Ok(svc.check_toolchains())).await
}

#[tauri::command]
async fn recalibrate_benchmarks(
    app: AppHandle,
    svc: Svc<'_>,
    runs: u32,
) -> Result<CalibrationSummary> {
    let svc = svc.inner().clone();
    blocking(move || {
        svc.recalibrate(runs, &|done, total| {
            let _ = app.emit(
                "calibration-progress",
                Progress {
                    problem_id: 0,
                    done,
                    total,
                },
            );
        })
    })
    .await
}

#[tauri::command]
fn cancel_recalibration(svc: Svc) {
    svc.cancel_recalibration()
}

#[tauri::command]
fn calibration_status(svc: Svc) -> Result<CalibrationStatus> {
    svc.calibration_status()
}

#[tauri::command]
fn clear_local_benchmarks(svc: Svc) -> Result<()> {
    svc.clear_local_benchmarks()
}

#[tauri::command]
fn reference_solution(svc: Svc, id: i64, language: Language) -> Result<Option<String>> {
    svc.reference_solution(id, language)
}

/// True when launched by `scripts/smoke.sh`; the UI then runs its self-test and reports back.
#[tauri::command]
fn smoke_mode() -> bool {
    std::env::var_os("LOCALGRIND_SMOKE").is_some()
}

#[tauri::command]
fn smoke_finish(app: AppHandle, ok: bool, report: String) {
    println!("{report}");
    println!("{}", if ok { "SMOKE PASS" } else { "SMOKE FAIL" });
    app.exit(if ok { 0 } else { 1 });
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let catalog = resolve_catalog(
                std::env::var_os("LOCALGRIND_CATALOG").map(PathBuf::from),
                app.path().resource_dir().ok(),
                Path::new(env!("CARGO_MANIFEST_DIR")),
            )
            .ok_or("problems.db not found; run `cargo run -p lg-content -- build`")?;
            let user = resolve_user_db(
                std::env::var_os("LOCALGRIND_DATA_DIR").map(PathBuf::from),
                app.path().app_data_dir()?,
            );
            let mut service = Service::open(&catalog, &user)?
                .with_cache_dir(app.path().app_cache_dir()?.join("compiled"));
            if let Some(python) = bundled_python(app.path().resource_dir().ok()) {
                service = service.with_bundled_toolchain(Language::Python, python)?;
            }
            app.manage(Arc::new(service));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            bootstrap,
            get_problem,
            save_draft,
            save_note,
            create_user_list,
            rename_user_list,
            delete_user_list,
            set_in_user_list,
            reset_code,
            run_code,
            submit_code,
            list_submissions,
            start_solve,
            update_solve,
            discard_solve,
            save_settings,
            get_stats,
            get_review_queue,
            export_history,
            import_history,
            clear_history,
            reference_solution,
            check_toolchains,
            recalibrate_benchmarks,
            cancel_recalibration,
            calibration_status,
            clear_local_benchmarks,
            smoke_mode,
            smoke_finish,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Local Grind");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundled_python_lookup() {
        assert_eq!(bundled_python(None), None);
        let dir = std::env::temp_dir().join(format!("lg-bundled-{}", std::process::id()));
        assert_eq!(bundled_python(Some(dir.clone())), None);
        std::fs::create_dir_all(dir.join("python/bin")).unwrap();
        std::fs::write(dir.join("python/bin/python3"), "").unwrap();
        assert_eq!(
            bundled_python(Some(dir.clone())),
            Some(dir.join("python/bin/python3"))
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn user_db_location() {
        let default = PathBuf::from("/data");
        assert_eq!(
            resolve_user_db(None, default.clone()),
            PathBuf::from("/data/user.db")
        );
        assert_eq!(
            resolve_user_db(Some(PathBuf::from("/tmp/x")), default),
            PathBuf::from("/tmp/x/user.db")
        );
    }

    #[test]
    fn catalog_resolution_order() {
        let dir = std::env::temp_dir().join(format!("lg-resolve-{}", std::process::id()));
        let res = dir.join("res");
        std::fs::create_dir_all(&res).unwrap();
        let manifest = dir.join("a/b");
        std::fs::create_dir_all(&manifest).unwrap();
        assert_eq!(resolve_catalog(None, Some(res.clone()), &manifest), None);

        std::fs::create_dir_all(dir.join("assets")).unwrap();
        std::fs::write(dir.join("assets/problems.db"), "").unwrap();
        assert_eq!(
            resolve_catalog(None, Some(res.clone()), &manifest),
            Some(manifest.join("../../assets/problems.db"))
        );
        std::fs::write(res.join("problems.db"), "").unwrap();
        assert_eq!(
            resolve_catalog(None, Some(res.clone()), &manifest),
            Some(res.join("problems.db"))
        );
        let custom = dir.join("custom.db");
        std::fs::write(&custom, "").unwrap();
        assert_eq!(
            resolve_catalog(Some(custom.clone()), Some(res), &manifest),
            Some(custom)
        );
        std::fs::remove_dir_all(dir).unwrap();
    }
}
