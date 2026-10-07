//! Executes a [`Program`] against a test suite, judging results as they stream in.
//!
//! The child runs in its own process group with a wall-clock watchdog. Solution time is the sum
//! of the per-test timings reported by the harness (interpreter start-up and input parsing are
//! excluded); memory is the peak RSS reported by `wait4`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader, Read};
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use crate::error::{Error, Result};
use crate::judge;
use crate::lang::{Program, RESULT_SENTINEL};
use crate::types::Comparator;

const MAX_CAPTURE: usize = 64 * 1024;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TestCase {
    pub args: Value,
    pub expected: Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    Accepted,
    WrongAnswer,
    TimeLimitExceeded,
    MemoryLimitExceeded,
    RuntimeError,
    CompileError,
}

impl Verdict {
    pub fn label(self) -> &'static str {
        match self {
            Verdict::Accepted => "Accepted",
            Verdict::WrongAnswer => "Wrong Answer",
            Verdict::TimeLimitExceeded => "Time Limit Exceeded",
            Verdict::MemoryLimitExceeded => "Memory Limit Exceeded",
            Verdict::RuntimeError => "Runtime Error",
            Verdict::CompileError => "Compile Error",
        }
    }

    pub fn parse(s: &str) -> Option<Verdict> {
        [
            Verdict::Accepted,
            Verdict::WrongAnswer,
            Verdict::TimeLimitExceeded,
            Verdict::MemoryLimitExceeded,
            Verdict::RuntimeError,
            Verdict::CompileError,
        ]
        .into_iter()
        .find(|v| format!("{v:?}") == s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Limits {
    /// Budget for the summed solution time across all tests.
    pub time_limit_ms: u64,
    /// Hard wall-clock cap for the whole process, including start-up.
    pub wall_limit_ms: u64,
    /// Peak RSS cap; exceeding it yields Memory Limit Exceeded.
    pub memory_limit_kb: u64,
    pub compile_timeout_ms: u64,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            time_limit_ms: 5_000,
            wall_limit_ms: 15_000,
            memory_limit_kb: 1024 * 1024,
            compile_timeout_ms: 30_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RunOptions {
    pub limits: Limits,
    /// Kill the program at the first wrong answer (submissions do; "Run" does not).
    pub stop_on_failure: bool,
    /// Record every case result, not just the first failure.
    pub keep_all_cases: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaseResult {
    pub index: usize,
    pub passed: bool,
    pub args: Value,
    pub expected: Value,
    pub output: Option<Value>,
    pub stdout: String,
    pub runtime_ns: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunReport {
    pub verdict: Verdict,
    pub passed: usize,
    pub total: usize,
    pub runtime_ms: f64,
    pub memory_kb: u64,
    pub cases: Vec<CaseResult>,
    /// First failing case (wrong answer, runtime error or the case that hit the time limit).
    pub failure: Option<CaseResult>,
    /// Compile error output or runtime error message.
    pub error: Option<String>,
    pub stderr: String,
}

#[derive(Deserialize)]
struct HarnessLine {
    i: Option<usize>,
    out: Option<Value>,
    ns: Option<u64>,
    error: Option<String>,
    compile_error: Option<String>,
}

/// Splits a stdout line into user output and an optional harness message.
fn split_line(line: &str) -> (&str, Option<&str>) {
    match line.find(RESULT_SENTINEL) {
        Some(pos) => (&line[..pos], Some(&line[pos + RESULT_SENTINEL.len()..])),
        None => (line, None),
    }
}

fn push_capped(buf: &mut String, s: &str) {
    if buf.len() < MAX_CAPTURE {
        let room = MAX_CAPTURE - buf.len();
        let mut end = s.len().min(room);
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        buf.push_str(&s[..end]);
    }
}

struct Exit {
    signal: Option<i32>,
    code: Option<i32>,
    max_rss_kb: u64,
}

/// Blocks until `pid` exits, then reaps it. `before_reap` runs while the zombie still holds the
/// pid, so the watchdog can be disarmed without racing pid reuse.
fn wait_child(pid: libc::pid_t, before_reap: impl FnOnce()) -> Result<Exit> {
    // SAFETY: plain libc calls on a child pid we own.
    unsafe {
        let mut info: libc::siginfo_t = std::mem::zeroed();
        loop {
            if libc::waitid(
                libc::P_PID,
                pid as libc::id_t,
                &mut info,
                libc::WEXITED | libc::WNOWAIT,
            ) == 0
            {
                break;
            }
            let err = std::io::Error::last_os_error();
            if err.kind() != std::io::ErrorKind::Interrupted {
                return Err(err.into());
            }
        }
        before_reap();
        let mut status = 0;
        let mut usage: libc::rusage = std::mem::zeroed();
        if libc::wait4(pid, &mut status, 0, &mut usage) < 0 {
            return Err(std::io::Error::last_os_error().into());
        }
        // macOS reports bytes, Linux kilobytes.
        let max_rss_kb = if cfg!(target_os = "macos") {
            usage.ru_maxrss as u64 / 1024
        } else {
            usage.ru_maxrss as u64
        };
        Ok(Exit {
            signal: libc::WIFSIGNALED(status).then(|| libc::WTERMSIG(status)),
            code: libc::WIFEXITED(status).then(|| libc::WEXITSTATUS(status)),
            max_rss_kb,
        })
    }
}

fn kill_group(pid: libc::pid_t) {
    // SAFETY: signalling our own child's process group.
    unsafe {
        libc::kill(-pid, libc::SIGKILL);
    }
}

/// Runs `cmd` to completion with a timeout, returning (success, combined output).
fn run_to_completion(cmd: &[String], dir: &Path, timeout: Duration) -> Result<(bool, String)> {
    let mut child = Command::new(&cmd[0])
        .args(&cmd[1..])
        .current_dir(dir)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0)
        .spawn()
        .map_err(|e| Error::Toolchain(format!("{}: {e}", cmd[0])))?;
    let pid = child.id() as libc::pid_t;
    let mut stdout = child.stdout.take().expect("piped");
    let mut stderr = child.stderr.take().expect("piped");
    let err_thread = thread::spawn(move || {
        let mut s = String::new();
        let _ = stderr.read_to_string(&mut s);
        s
    });
    let watchdog = Watchdog::arm(pid, timeout);
    let mut out = String::new();
    let _ = stdout.read_to_string(&mut out);
    let exit = wait_child(pid, || {
        watchdog.disarm();
        kill_group(pid);
    })?;
    out.push_str(&err_thread.join().unwrap_or_default());
    if watchdog.fired() {
        out.push_str("\nCompilation timed out.");
    }
    Ok((exit.code == Some(0) && !watchdog.fired(), out))
}

struct Watchdog {
    state: Arc<Mutex<bool>>, // true once disarmed
    fired: Arc<AtomicBool>,
    cancel: mpsc::Sender<()>,
}

impl Watchdog {
    fn arm(pid: libc::pid_t, timeout: Duration) -> Watchdog {
        let state = Arc::new(Mutex::new(false));
        let fired = Arc::new(AtomicBool::new(false));
        let (tx, rx) = mpsc::channel::<()>();
        let (s, f) = (state.clone(), fired.clone());
        thread::spawn(move || {
            if rx.recv_timeout(timeout) == Err(mpsc::RecvTimeoutError::Timeout) {
                let disarmed = s.lock().unwrap();
                if !*disarmed {
                    f.store(true, Ordering::SeqCst);
                    kill_group(pid);
                }
            }
        });
        Watchdog {
            state,
            fired,
            cancel: tx,
        }
    }

    fn disarm(&self) {
        *self.state.lock().unwrap() = true;
        let _ = self.cancel.send(());
    }

    fn fired(&self) -> bool {
        self.fired.load(Ordering::SeqCst)
    }
}

/// Compiles (if needed) and runs `program` over `cases`. `progress(done, total)` is invoked
/// after each judged case.
pub fn execute(
    program: &Program,
    cases: &[TestCase],
    cmp: &Comparator,
    opts: &RunOptions,
    progress: &mut dyn FnMut(usize, usize),
) -> Result<RunReport> {
    let dir = tempfile::Builder::new().prefix("localgrind-").tempdir()?;
    program.write_to(dir.path())?;
    let mut report = RunReport {
        verdict: Verdict::Accepted,
        passed: 0,
        total: cases.len(),
        runtime_ms: 0.0,
        memory_kb: 0,
        cases: Vec::new(),
        failure: None,
        error: None,
        stderr: String::new(),
    };

    if let Some(compile) = &program.compile {
        let timeout = Duration::from_millis(opts.limits.compile_timeout_ms);
        let (ok, output) = run_to_completion(compile, dir.path(), timeout)?;
        if !ok {
            report.verdict = Verdict::CompileError;
            report.error = Some(output);
            return Ok(report);
        }
    }

    let input_path = dir.path().join("input.jsonl");
    let mut input = String::new();
    for case in cases {
        input.push_str(&serde_json::to_string(&case.args)?);
        input.push('\n');
    }
    std::fs::write(&input_path, input)?;

    let mut child = Command::new(&program.run[0])
        .args(&program.run[1..])
        .arg(&input_path)
        .current_dir(dir.path())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0)
        .spawn()
        .map_err(|e| Error::Toolchain(format!("{}: {e}", program.run[0])))?;
    let pid = child.id() as libc::pid_t;
    let stdout = child.stdout.take().expect("piped");
    let mut stderr = child.stderr.take().expect("piped");
    let err_thread = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr
            .by_ref()
            .take(MAX_CAPTURE as u64)
            .read_to_end(&mut buf);
        let _ = std::io::copy(&mut stderr, &mut std::io::sink());
        String::from_utf8_lossy(&buf).into_owned()
    });
    let started = Instant::now();
    let watchdog = Watchdog::arm(pid, Duration::from_millis(opts.limits.wall_limit_ms));

    let mut total_ns: u64 = 0;
    let mut pending_stdout = String::new();
    let mut killed_by_us = false;
    let mut time_exceeded = false;
    let mut runtime_error = false;
    let mut harness_compile_error = None;
    let mut reader = BufReader::new(stdout);
    let mut raw = Vec::new();
    loop {
        raw.clear();
        match reader.read_until(b'\n', &mut raw) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
        let line = String::from_utf8_lossy(&raw);
        let line = line.strip_suffix('\n').unwrap_or(&line);
        let (user, message) = split_line(line);
        let Some(message) = message else {
            push_capped(&mut pending_stdout, user);
            push_capped(&mut pending_stdout, "\n");
            continue;
        };
        push_capped(&mut pending_stdout, user);
        let Ok(msg) = serde_json::from_str::<HarnessLine>(message) else {
            continue;
        };
        if let Some(ce) = msg.compile_error {
            harness_compile_error = Some(ce);
            continue;
        }
        let Some(i) = msg.i.filter(|i| *i < cases.len()) else {
            continue;
        };
        let case = &cases[i];
        let stdout_text = std::mem::take(&mut pending_stdout);
        if let Some(err) = msg.error {
            runtime_error = true;
            report.error = Some(err);
            report.failure = Some(CaseResult {
                index: i,
                passed: false,
                args: case.args.clone(),
                expected: case.expected.clone(),
                output: None,
                stdout: stdout_text,
                runtime_ns: 0,
            });
            continue;
        }
        let output = msg.out.unwrap_or(Value::Null);
        let ns = msg.ns.unwrap_or(0);
        total_ns += ns;
        let passed = judge::matches(cmp, &case.args, &case.expected, &output);
        let result = CaseResult {
            index: i,
            passed,
            args: case.args.clone(),
            expected: case.expected.clone(),
            output: Some(output),
            stdout: stdout_text,
            runtime_ns: ns,
        };
        if passed {
            report.passed += 1;
        } else if report.failure.is_none() {
            report.failure = Some(result.clone());
        }
        if opts.keep_all_cases {
            report.cases.push(result);
        }
        progress(report.passed, cases.len());
        if total_ns > opts.limits.time_limit_ms * 1_000_000 {
            time_exceeded = true;
        }
        if time_exceeded || (!passed && opts.stop_on_failure) {
            killed_by_us = true;
            kill_group(pid);
            break;
        }
    }
    drop(reader);
    let exit = wait_child(pid, || {
        watchdog.disarm();
        kill_group(pid);
    })?;
    let wall = started.elapsed();
    report.stderr = err_thread.join().unwrap_or_default();
    report.runtime_ms = total_ns as f64 / 1e6;
    report.memory_kb = exit.max_rss_kb;

    report.verdict = if let Some(ce) = harness_compile_error {
        report.error = Some(ce);
        Verdict::CompileError
    } else if report.failure.as_ref().is_some_and(|f| f.output.is_some()) {
        Verdict::WrongAnswer
    } else if runtime_error {
        Verdict::RuntimeError
    } else if time_exceeded || watchdog.fired() {
        report.error = Some(format!(
            "Time limit exceeded after {:.0} ms of solution time ({:.1} s wall clock).",
            report.runtime_ms,
            wall.as_secs_f64()
        ));
        Verdict::TimeLimitExceeded
    } else if exit.max_rss_kb > opts.limits.memory_limit_kb {
        Verdict::MemoryLimitExceeded
    } else if report.passed < cases.len() || (exit.code != Some(0) && !killed_by_us) {
        let reason = match (exit.signal, exit.code) {
            (Some(sig), _) => format!("Process terminated by signal {sig}."),
            (None, Some(code)) if code != 0 => format!("Process exited with code {code}."),
            _ => "Process exited before all tests completed.".into(),
        };
        report.error = Some(
            format!("{reason}\n{}", report.stderr)
                .trim_end()
                .to_string(),
        );
        Verdict::RuntimeError
    } else {
        Verdict::Accepted
    };
    if report.verdict == Verdict::Accepted && !pending_stdout.is_empty() {
        report.stderr.push_str(&pending_stdout);
    }
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn shell(script: &str) -> Program {
        Program {
            files: vec![("run.sh".into(), script.into())],
            compile: None,
            run: vec!["/bin/sh".into(), "run.sh".into()],
        }
    }

    fn cases(n: usize) -> Vec<TestCase> {
        (0..n)
            .map(|i| TestCase {
                args: json!([i]),
                expected: json!(i),
            })
            .collect()
    }

    fn run(p: &Program, cs: &[TestCase], opts: RunOptions) -> RunReport {
        execute(p, cs, &Comparator::Exact, &opts, &mut |_, _| {}).unwrap()
    }

    const ECHO_ALL: &str = r#"i=0; while read line; do printf 'hello %s\n' "$i"; printf '\036LG {"i":%d,"ns":1000,"out":%d}\n' "$i" "$i"; i=$((i+1)); done < "$1""#;

    #[test]
    fn accepted_with_stdout_and_progress() {
        let mut seen = vec![];
        let opts = RunOptions {
            keep_all_cases: true,
            ..Default::default()
        };
        let r = execute(
            &shell(ECHO_ALL),
            &cases(3),
            &Comparator::Exact,
            &opts,
            &mut |d, t| seen.push((d, t)),
        )
        .unwrap();
        assert_eq!(r.verdict, Verdict::Accepted, "{r:?}");
        assert_eq!(r.passed, 3);
        assert_eq!(r.cases.len(), 3);
        assert_eq!(r.cases[1].stdout, "hello 1\n");
        assert!((r.runtime_ms - 0.003).abs() < 1e-9);
        assert!(r.memory_kb > 0);
        assert_eq!(seen, vec![(1, 3), (2, 3), (3, 3)]);
    }

    #[test]
    fn wrong_answer_stops_early() {
        let script = r#"printf '\036LG {"i":0,"ns":5,"out":0}\n'; printf '\036LG {"i":1,"ns":5,"out":7}\n'; sleep 5"#;
        let opts = RunOptions {
            stop_on_failure: true,
            ..Default::default()
        };
        let start = Instant::now();
        let r = run(&shell(script), &cases(3), opts);
        assert!(start.elapsed() < Duration::from_secs(3));
        assert_eq!(r.verdict, Verdict::WrongAnswer);
        assert_eq!(r.passed, 1);
        let f = r.failure.unwrap();
        assert_eq!(
            (f.index, f.output, f.expected),
            (1, Some(json!(7)), json!(1))
        );
    }

    #[test]
    fn wrong_answer_without_stopping_counts_all() {
        let script = r#"printf '\036LG {"i":0,"ns":5,"out":9}\n\036LG {"i":1,"ns":5,"out":1}\n'"#;
        let r = run(&shell(script), &cases(2), RunOptions::default());
        assert_eq!(r.verdict, Verdict::WrongAnswer);
        assert_eq!(r.passed, 1);
        assert_eq!(r.failure.unwrap().index, 0);
    }

    #[test]
    fn runtime_error_from_harness() {
        let script = r#"printf 'debug\n\036LG {"i":0,"error":"ZeroDivisionError"}\n'"#;
        let r = run(&shell(script), &cases(2), RunOptions::default());
        assert_eq!(r.verdict, Verdict::RuntimeError);
        assert_eq!(r.error.as_deref(), Some("ZeroDivisionError"));
        assert_eq!(r.failure.unwrap().stdout, "debug\n");
    }

    #[test]
    fn crash_and_early_exit_are_runtime_errors() {
        let r = run(
            &shell("echo oops >&2; exit 3"),
            &cases(1),
            RunOptions::default(),
        );
        assert_eq!(r.verdict, Verdict::RuntimeError);
        let msg = r.error.unwrap();
        assert!(
            msg.contains("exited with code 3") && msg.contains("oops"),
            "{msg}"
        );

        let r = run(&shell("kill -SEGV $$"), &cases(1), RunOptions::default());
        assert!(r.error.unwrap().contains("signal 11"));

        let r = run(&shell("exit 0"), &cases(1), RunOptions::default());
        assert!(r.error.unwrap().contains("before all tests"));
    }

    #[test]
    fn harness_compile_error() {
        let script = r#"printf '\036LG {"compile_error":"Line 1: SyntaxError"}\n'"#;
        let r = run(&shell(script), &cases(1), RunOptions::default());
        assert_eq!(r.verdict, Verdict::CompileError);
        assert_eq!(r.error.as_deref(), Some("Line 1: SyntaxError"));
    }

    #[test]
    fn wall_clock_timeout() {
        let opts = RunOptions {
            limits: Limits {
                wall_limit_ms: 300,
                ..Default::default()
            },
            ..Default::default()
        };
        let r = run(&shell("sleep 10"), &cases(1), opts);
        assert_eq!(r.verdict, Verdict::TimeLimitExceeded);
    }

    #[test]
    fn solution_time_budget() {
        let script = r#"printf '\036LG {"i":0,"ns":3000000,"out":0}\n'; sleep 10"#;
        let opts = RunOptions {
            limits: Limits {
                time_limit_ms: 2,
                ..Default::default()
            },
            ..Default::default()
        };
        let r = run(&shell(script), &cases(2), opts);
        assert_eq!(r.verdict, Verdict::TimeLimitExceeded);
        assert!(r.error.unwrap().contains("3 ms"));
    }

    #[test]
    fn memory_limit() {
        let opts = RunOptions {
            limits: Limits {
                memory_limit_kb: 1,
                ..Default::default()
            },
            ..Default::default()
        };
        let r = run(&shell(ECHO_ALL), &cases(1), opts);
        assert_eq!(r.verdict, Verdict::MemoryLimitExceeded);
    }

    #[test]
    fn ignores_garbage_harness_lines() {
        let script = r#"printf '\036LG nonsense\n\036LG {"i":99,"out":1}\n\036LG {"i":0,"ns":1,"out":0}\n'; printf 'tail'"#;
        let r = run(&shell(script), &cases(1), RunOptions::default());
        assert_eq!(r.verdict, Verdict::Accepted);
        assert_eq!(r.stderr, "tail\n");
    }

    #[test]
    fn compile_step() {
        let mut p = shell(ECHO_ALL);
        p.compile = Some(vec![
            "/bin/sh".into(),
            "-c".into(),
            "echo bad syntax; exit 1".into(),
        ]);
        let r = run(&p, &cases(1), RunOptions::default());
        assert_eq!(r.verdict, Verdict::CompileError);
        assert!(r.error.unwrap().contains("bad syntax"));

        p.compile = Some(vec!["/bin/sh".into(), "-c".into(), "true".into()]);
        assert_eq!(
            run(&p, &cases(1), RunOptions::default()).verdict,
            Verdict::Accepted
        );

        p.compile = Some(vec!["/bin/sleep".into(), "5".into()]);
        let opts = RunOptions {
            limits: Limits {
                compile_timeout_ms: 200,
                ..Default::default()
            },
            ..Default::default()
        };
        let r = run(&p, &cases(1), opts);
        assert!(r.error.unwrap().contains("timed out"));
    }

    #[test]
    fn missing_executables() {
        let mut p = shell("");
        p.run = vec!["/nonexistent/lg-bin".into()];
        let err = execute(
            &p,
            &cases(1),
            &Comparator::Exact,
            &RunOptions::default(),
            &mut |_, _| {},
        );
        assert!(matches!(err, Err(Error::Toolchain(_))));
        p.compile = Some(vec!["/nonexistent/lg-cc".into()]);
        let err = execute(
            &p,
            &cases(1),
            &Comparator::Exact,
            &RunOptions::default(),
            &mut |_, _| {},
        );
        assert!(matches!(err, Err(Error::Toolchain(_))));
    }

    #[test]
    fn verdict_labels() {
        for v in [
            Verdict::Accepted,
            Verdict::WrongAnswer,
            Verdict::TimeLimitExceeded,
            Verdict::MemoryLimitExceeded,
            Verdict::RuntimeError,
            Verdict::CompileError,
        ] {
            assert!(!v.label().is_empty());
            assert_eq!(Verdict::parse(&format!("{v:?}")), Some(v));
        }
        assert_eq!(Verdict::parse("Nope"), None);
    }

    #[test]
    fn capture_is_capped_on_char_boundary() {
        let mut s = "a".repeat(MAX_CAPTURE - 1);
        push_capped(&mut s, "é");
        assert_eq!(s.len(), MAX_CAPTURE - 1);
        push_capped(&mut s, "b");
        assert_eq!(s.len(), MAX_CAPTURE);
        push_capped(&mut s, "c");
        assert_eq!(s.len(), MAX_CAPTURE);
    }
}
