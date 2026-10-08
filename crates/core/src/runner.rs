//! Executes a [`Program`] against a test suite, judging results as they stream in.
//!
//! The child runs in its own process group with a wall-clock watchdog. Solution time is the sum
//! of the per-test timings reported by the harness (interpreter start-up and input parsing are
//! excluded); memory is the peak RSS reported by `wait4`.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader, Read};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
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

    /// Stable name used in the user database (the variant name, as serde writes it).
    pub fn as_str(self) -> &'static str {
        match self {
            Verdict::Accepted => "Accepted",
            Verdict::WrongAnswer => "WrongAnswer",
            Verdict::TimeLimitExceeded => "TimeLimitExceeded",
            Verdict::MemoryLimitExceeded => "MemoryLimitExceeded",
            Verdict::RuntimeError => "RuntimeError",
            Verdict::CompileError => "CompileError",
        }
    }

    /// Inverse of [`Verdict::as_str`].
    pub fn parse(s: &str) -> Option<Verdict> {
        Some(match s {
            "Accepted" => Verdict::Accepted,
            "WrongAnswer" => Verdict::WrongAnswer,
            "TimeLimitExceeded" => Verdict::TimeLimitExceeded,
            "MemoryLimitExceeded" => Verdict::MemoryLimitExceeded,
            "RuntimeError" => Verdict::RuntimeError,
            "CompileError" => Verdict::CompileError,
            _ => return None,
        })
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
    /// Run the program at a lower scheduling priority so judging yields to other work.
    pub low_priority: bool,
    /// Where compiled artifacts are cached, keyed by [`Program::cache_key`].
    pub cache_dir: Option<PathBuf>,
    /// Record outputs without judging them (Run computes expectations separately). The verdict
    /// then only reflects how the process ended: Accepted means every case completed.
    pub collect_only: bool,
}

/// Longest single stdout line the runner will buffer (a harness result for a huge output).
pub const MAX_LINE_BYTES: usize = 64 << 20;
/// Total stdout a judged program may write before it is stopped.
pub const MAX_OUTPUT_BYTES: u64 = 256 << 20;

/// Compiled programs kept in the cache; older entries are pruned.
pub const CACHE_ENTRIES: usize = 64;

fn cache_lookup(program: &Program, cache_dir: Option<&Path>) -> Option<PathBuf> {
    if program.artifacts.is_empty() {
        return None;
    }
    cache_dir.map(|d| d.join(program.cache_key()))
}

fn cache_store(program: &Program, build_dir: &Path, entry: &Path) {
    let Some(root) = entry.parent() else { return };
    if std::fs::create_dir_all(root).is_err() {
        return;
    }
    // A unique staging directory per store, so concurrent compiles of the same program (Run and
    // Submit, or user code identical to a reference) never interfere; the rename is atomic.
    let Ok(staging) = tempfile::Builder::new()
        .prefix(".staging-")
        .tempdir_in(root)
    else {
        return;
    };
    let copied = program
        .artifacts
        .iter()
        .all(|a| std::fs::copy(build_dir.join(a), staging.path().join(a)).is_ok());
    if copied && std::fs::rename(staging.path(), entry).is_ok() {
        // The directory moved; nothing left for the TempDir guard to delete.
        let _ = staging.keep();
    }
    prune_cache(root, CACHE_ENTRIES);
}

/// Marks a cache entry as used so eviction is least-recently-used. Eviction orders entries by
/// their directory's modification time, which rewriting a file inside does not change, so the
/// time is set directly.
fn touch_entry(entry: &Path) {
    let _ = std::fs::File::open(entry).and_then(|d| d.set_modified(std::time::SystemTime::now()));
}

/// Keeps the `keep` most recently used cache entries and removes staging directories left
/// behind by crashes (older than an hour).
fn prune_cache(root: &Path, keep: usize) {
    let Ok(read) = std::fs::read_dir(root) else {
        return;
    };
    let mut entries: Vec<(std::time::SystemTime, PathBuf)> = Vec::new();
    for e in read.filter_map(|e| e.ok()) {
        let Some(modified) = e.metadata().ok().and_then(|m| m.modified().ok()) else {
            continue;
        };
        if e.file_name().to_string_lossy().starts_with(".staging-") {
            if modified
                .elapsed()
                .is_ok_and(|age| age > Duration::from_secs(3600))
            {
                let _ = std::fs::remove_dir_all(e.path());
            }
        } else if !e.file_name().to_string_lossy().starts_with('.') {
            entries.push((modified, e.path()));
        }
    }
    entries.sort();
    let excess = entries.len().saturating_sub(keep);
    for (_, path) in entries.into_iter().take(excess) {
        let _ = std::fs::remove_dir_all(path);
    }
}

/// Upper bound for the raised stack limit of judged processes (1 GiB).
pub const MAX_STACK_BYTES: libc::rlim_t = 1 << 30;

/// Nice value applied to low-priority judge processes.
pub const LOW_PRIORITY_NICE: libc::c_int = 10;

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

/// Human-readable meaning of a fatal signal, as shown in Runtime Error messages.
pub fn signal_name(sig: i32) -> &'static str {
    match sig {
        libc::SIGSEGV => {
            "segmentation fault: invalid memory access, e.g. a null pointer or out-of-bounds index"
        }
        libc::SIGBUS => "bus error: invalid memory access",
        libc::SIGFPE => "floating point exception, e.g. integer division by zero",
        libc::SIGABRT => "aborted, e.g. a failed assertion",
        libc::SIGILL | libc::SIGTRAP => {
            "trap: the program hit undefined behaviour, e.g. a null pointer dereference"
        }
        libc::SIGKILL => "killed",
        _ => "terminated",
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
    let watchdog = Watchdog::arm(pid, timeout, None, None);
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

/// Resident memory and CPU time (user + system) of a live process.
#[derive(Debug, Clone, Copy)]
struct ProcStats {
    rss_kb: u64,
    cpu: Duration,
}

#[cfg(target_os = "macos")]
fn proc_stats(pid: libc::pid_t) -> Option<ProcStats> {
    use std::sync::OnceLock;
    // CPU times are reported in Mach absolute-time units.
    #[repr(C)]
    struct Timebase {
        numer: u32,
        denom: u32,
    }
    extern "C" {
        // libSystem; declared here because the libc crate's binding is deprecated.
        fn mach_timebase_info(info: *mut Timebase) -> libc::c_int;
    }
    static TIMEBASE: OnceLock<(u64, u64)> = OnceLock::new();
    let &(numer, denom) = TIMEBASE.get_or_init(|| {
        let mut tb = Timebase { numer: 0, denom: 0 };
        // SAFETY: mach_timebase_info only writes the struct it is given.
        let ok = unsafe { mach_timebase_info(&mut tb) } == 0 && tb.denom != 0;
        if ok {
            (tb.numer as u64, tb.denom as u64)
        } else {
            (1, 1)
        }
    });
    // SAFETY: proc_pidinfo writes at most `size` bytes into a zeroed proc_taskinfo.
    let info = unsafe {
        let mut info: libc::proc_taskinfo = std::mem::zeroed();
        let size = std::mem::size_of::<libc::proc_taskinfo>() as libc::c_int;
        let n = libc::proc_pidinfo(
            pid,
            libc::PROC_PIDTASKINFO,
            0,
            &mut info as *mut _ as *mut libc::c_void,
            size,
        );
        (n == size).then_some(info)?
    };
    let ticks = (info.pti_total_user + info.pti_total_system) as u128;
    Some(ProcStats {
        rss_kb: info.pti_resident_size / 1024,
        cpu: Duration::from_nanos((ticks * numer as u128 / denom as u128) as u64),
    })
}

#[cfg(not(target_os = "macos"))]
fn proc_stats(pid: libc::pid_t) -> Option<ProcStats> {
    // SAFETY: sysconf has no preconditions.
    let (page, tick) = unsafe {
        (
            libc::sysconf(libc::_SC_PAGESIZE) as u64,
            libc::sysconf(libc::_SC_CLK_TCK) as u64,
        )
    };
    let statm = std::fs::read_to_string(format!("/proc/{pid}/statm")).ok()?;
    let pages: u64 = statm.split_whitespace().nth(1)?.parse().ok()?;
    // Fields after the parenthesised command name start at field 3 (state); utime and stime
    // are fields 14 and 15.
    let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let mut rest = stat.rsplit_once(')')?.1.split_whitespace().skip(11);
    let utime: u64 = rest.next()?.parse().ok()?;
    let stime: u64 = rest.next()?.parse().ok()?;
    Some(ProcStats {
        rss_kb: pages * page / 1024,
        cpu: Duration::from_millis((utime + stime) * 1000 / tick.max(1)),
    })
}

const MEMORY_POLL: Duration = Duration::from_millis(20);

/// Solution-time budget shared between the reader and the watchdog. A case that burns more
/// CPU time than the remaining budget (plus a grace period for harness overhead) is stopped
/// without waiting for the wall-clock limit. CPU time rather than wall time is what counts
/// here, so a correct program that is merely descheduled on a busy machine is not killed; a
/// blocked program still ends at the wall-clock limit.
struct Budget {
    limit: Duration,
    grace: Duration,
    /// (results received so far, solution time used so far)
    state: Mutex<(u64, Duration)>,
}

impl Budget {
    fn record(&self, used: Duration) {
        let mut state = self.state.lock().unwrap();
        *state = (state.0 + 1, used);
    }

    fn snapshot(&self) -> (u64, Duration) {
        *self.state.lock().unwrap()
    }
}

/// Kills the child's process group when it outlives `timeout`, exceeds its solution-time
/// budget, or its resident memory exceeds `memory_limit_kb`. Disarming happens under a lock
/// while the child's pid is still held by its zombie, so the watchdog can never signal a
/// recycled pid.
struct Watchdog {
    disarmed: Arc<Mutex<bool>>,
    timed_out: Arc<AtomicBool>,
    out_of_memory: Arc<AtomicBool>,
    cancel: mpsc::Sender<()>,
}

impl Watchdog {
    fn arm(
        pid: libc::pid_t,
        timeout: Duration,
        memory_limit_kb: Option<u64>,
        budget: Option<Arc<Budget>>,
    ) -> Watchdog {
        let disarmed = Arc::new(Mutex::new(false));
        let timed_out = Arc::new(AtomicBool::new(false));
        let out_of_memory = Arc::new(AtomicBool::new(false));
        let (tx, rx) = mpsc::channel::<()>();
        let (d, t, m) = (disarmed.clone(), timed_out.clone(), out_of_memory.clone());
        let deadline = Instant::now() + timeout;
        let polling = memory_limit_kb.is_some() || budget.is_some();
        // (results seen, process CPU time when the latest of them was noticed)
        let mut mark = (0u64, Duration::ZERO);
        thread::spawn(move || loop {
            let remaining = deadline.saturating_duration_since(Instant::now());
            let wait = if polling {
                remaining.min(MEMORY_POLL)
            } else {
                remaining
            };
            if rx.recv_timeout(wait) != Err(mpsc::RecvTimeoutError::Timeout) {
                return;
            }
            let stats = if polling { proc_stats(pid) } else { None };
            let over_budget = match (&budget, stats) {
                (Some(b), Some(s)) => {
                    let (results, used) = b.snapshot();
                    if results != mark.0 {
                        mark = (results, s.cpu);
                    }
                    used + s.cpu.saturating_sub(mark.1) > b.limit + b.grace
                }
                _ => false,
            };
            let over_time = Instant::now() >= deadline || over_budget;
            let over_memory = matches!(
                (memory_limit_kb, stats),
                (Some(limit), Some(s)) if s.rss_kb > limit
            );
            if over_time || over_memory {
                let guard = d.lock().unwrap();
                if !*guard {
                    if over_memory {
                        m.store(true, Ordering::SeqCst);
                    } else {
                        t.store(true, Ordering::SeqCst);
                    }
                    kill_group(pid);
                }
                return;
            }
        });
        Watchdog {
            disarmed,
            timed_out,
            out_of_memory,
            cancel: tx,
        }
    }

    fn disarm(&self) {
        *self.disarmed.lock().unwrap() = true;
        let _ = self.cancel.send(());
    }

    fn fired(&self) -> bool {
        self.timed_out.load(Ordering::SeqCst)
    }

    fn out_of_memory(&self) -> bool {
        self.out_of_memory.load(Ordering::SeqCst)
    }
}

/// Reads one line (without the newline) into `buf`, keeping at most `cap` bytes; the rest of an
/// over-long line is consumed and discarded. Stops early once more than `allowance` bytes have
/// been consumed. Returns `(bytes consumed, truncated, terminated)`, where `terminated` means the
/// line ended with a newline; 0 bytes consumed means end of stream.
fn read_line_capped(
    reader: &mut impl BufRead,
    buf: &mut Vec<u8>,
    cap: usize,
    allowance: u64,
) -> std::io::Result<(usize, bool, bool)> {
    buf.clear();
    let (mut consumed, mut truncated) = (0, false);
    loop {
        let available = match reader.fill_buf() {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        if available.is_empty() {
            return Ok((consumed, truncated, false));
        }
        let (chunk, done) = match available.iter().position(|&b| b == b'\n') {
            Some(pos) => (&available[..pos], Some(pos + 1)),
            None => (available, None),
        };
        let room = cap.saturating_sub(buf.len());
        if chunk.len() > room {
            truncated = true;
        }
        buf.extend_from_slice(&chunk[..chunk.len().min(room)]);
        let used = done.unwrap_or(available.len());
        reader.consume(used);
        consumed += used;
        if done.is_some() || consumed as u64 > allowance {
            return Ok((consumed, truncated, done.is_some()));
        }
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
        let entry = cache_lookup(program, opts.cache_dir.as_deref());
        let cached = entry.as_ref().is_some_and(|e| {
            let hit = program
                .artifacts
                .iter()
                .all(|a| std::fs::copy(e.join(a), dir.path().join(a)).is_ok());
            if hit {
                touch_entry(e);
            }
            hit
        });
        if !cached {
            let timeout = Duration::from_millis(opts.limits.compile_timeout_ms);
            let (ok, output) = run_to_completion(compile, dir.path(), timeout)?;
            if !ok {
                report.verdict = Verdict::CompileError;
                report.error = Some(output);
                return Ok(report);
            }
            if let Some(entry) = &entry {
                cache_store(program, dir.path(), entry);
            }
        }
    }

    let input_path = dir.path().join("input.jsonl");
    let mut input = String::new();
    for case in cases {
        input.push_str(&serde_json::to_string(&case.args)?);
        input.push('\n');
    }
    std::fs::write(&input_path, input)?;

    let mut command = Command::new(&program.run[0]);
    command
        .args(&program.run[1..])
        .arg(&input_path)
        .current_dir(dir.path())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0);
    let low_priority = opts.low_priority;
    // SAFETY: the hook only makes async-signal-safe syscalls (getrlimit/setrlimit/setpriority)
    // on the child itself, none of which need privileges: the stack soft limit is raised to the
    // hard limit (so single-threaded runtimes such as JavaScriptCore get a deep main stack) and
    // priority can always be lowered.
    unsafe {
        command.pre_exec(move || {
            let mut limit: libc::rlimit = std::mem::zeroed();
            if libc::getrlimit(libc::RLIMIT_STACK, &mut limit) == 0
                && limit.rlim_cur < limit.rlim_max
            {
                limit.rlim_cur = limit.rlim_max.min(MAX_STACK_BYTES);
                libc::setrlimit(libc::RLIMIT_STACK, &limit);
            }
            if low_priority {
                libc::setpriority(libc::PRIO_PROCESS, 0, LOW_PRIORITY_NICE);
            }
            Ok(())
        });
    }
    let mut child = command
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
    let limit = Duration::from_millis(opts.limits.time_limit_ms);
    let budget = Arc::new(Budget {
        limit,
        grace: limit + Duration::from_secs(1),
        state: Mutex::new((0, Duration::ZERO)),
    });
    let watchdog = Watchdog::arm(
        pid,
        Duration::from_millis(opts.limits.wall_limit_ms),
        Some(opts.limits.memory_limit_kb),
        Some(budget.clone()),
    );

    // How the run ended abnormally, if it did: (case index, verdict, message).
    let mut abnormal: Option<(usize, Verdict, String)> = None;
    let mut first_wrong: Option<usize> = None;
    let mut next = 0usize;
    let mut total_ns: u64 = 0;
    let mut output_bytes: u64 = 0;
    let mut pending_stdout = String::new();
    let mut killed_by_us = false;
    let mut harness_compile_error = None;
    let mut reader = BufReader::new(stdout);
    let mut raw = Vec::new();
    let failed_case = |i: usize, stdout: String| CaseResult {
        index: i,
        passed: false,
        args: cases[i].args.clone(),
        expected: cases[i].expected.clone(),
        output: None,
        stdout,
        runtime_ns: 0,
    };
    loop {
        let allowance = MAX_OUTPUT_BYTES - output_bytes;
        let (n, truncated, terminated) =
            match read_line_capped(&mut reader, &mut raw, MAX_LINE_BYTES, allowance) {
                Ok((0, ..)) | Err(_) => break,
                Ok(r) => r,
            };
        output_bytes += n as u64;
        if output_bytes > MAX_OUTPUT_BYTES {
            abnormal = Some((
                next,
                Verdict::RuntimeError,
                "Output limit exceeded: the program printed more than 256 MB.".into(),
            ));
            break;
        }
        let line = String::from_utf8_lossy(&raw);
        let (user, message) = split_line(&line);
        push_capped(&mut pending_stdout, user);
        let Some(message) = message else {
            push_capped(&mut pending_stdout, "\n");
            continue;
        };
        if !terminated {
            // The process ended mid-result (killed or crashed): the exit status explains why.
            break;
        }
        let msg = match serde_json::from_str::<HarnessLine>(message) {
            Ok(m) if !truncated => m,
            _ => {
                let why = if truncated {
                    "too large"
                } else {
                    "not valid JSON (for example NaN or Infinity)"
                };
                abnormal = Some((next, Verdict::RuntimeError, format!("The program produced a result the judge could not read: the output is {why}.")));
                break;
            }
        };
        if let Some(ce) = msg.compile_error {
            harness_compile_error = Some(ce);
            continue;
        }
        // Results must arrive in order; anything else (duplicates, forged lines) is ignored.
        if msg.i != Some(next) || next >= cases.len() {
            continue;
        }
        let i = next;
        let stdout_text = std::mem::take(&mut pending_stdout);
        if let Some(err) = msg.error {
            if opts.keep_all_cases {
                report.cases.push(failed_case(i, stdout_text.clone()));
            }
            report.failure = Some(failed_case(i, stdout_text));
            abnormal = Some((i, Verdict::RuntimeError, err));
            break;
        }
        let output = msg.out.unwrap_or(Value::Null);
        let ns = msg.ns.unwrap_or(0);
        total_ns += ns;
        budget.record(Duration::from_nanos(total_ns));
        next += 1;
        if total_ns > opts.limits.time_limit_ms * 1_000_000 {
            // This case pushed the solution over its time budget: it does not count as passed.
            if opts.keep_all_cases {
                report.cases.push(failed_case(i, stdout_text.clone()));
            }
            report
                .failure
                .get_or_insert_with(|| failed_case(i, stdout_text));
            abnormal = Some((i, Verdict::TimeLimitExceeded, String::new()));
            killed_by_us = true;
            kill_group(pid);
            break;
        }
        let passed =
            opts.collect_only || judge::matches(cmp, &cases[i].args, &cases[i].expected, &output);
        let first_failure = !passed && first_wrong.is_none();
        // Building a result copies the case's arguments and expected value (up to megabytes),
        // so it is only done when the result is kept.
        if opts.keep_all_cases || first_failure {
            let result = CaseResult {
                index: i,
                passed,
                args: cases[i].args.clone(),
                expected: cases[i].expected.clone(),
                output: Some(output),
                stdout: stdout_text,
                runtime_ns: ns,
            };
            if first_failure {
                report.failure = Some(result.clone());
            }
            if opts.keep_all_cases {
                report.cases.push(result);
            }
        }
        if passed {
            report.passed += 1;
        } else if first_failure {
            first_wrong = Some(i);
        }
        progress(report.passed, cases.len());
        if !passed && opts.stop_on_failure {
            killed_by_us = true;
            kill_group(pid);
            break;
        }
    }
    drop(reader);
    if abnormal.is_some() {
        killed_by_us = true;
        kill_group(pid);
    }
    let exit = wait_child(pid, || {
        watchdog.disarm();
        kill_group(pid);
    })?;
    let wall = started.elapsed();
    report.stderr = err_thread.join().unwrap_or_default();
    report.runtime_ms = total_ns as f64 / 1e6;
    report.memory_kb = exit.max_rss_kb;

    if abnormal.is_none() {
        let unfinished = next < cases.len();
        abnormal = if watchdog.fired() {
            Some((next, Verdict::TimeLimitExceeded, String::new()))
        } else if watchdog.out_of_memory() || exit.max_rss_kb > opts.limits.memory_limit_kb {
            Some((
                next,
                Verdict::MemoryLimitExceeded,
                format!(
                    "Memory limit of {} MB exceeded.",
                    opts.limits.memory_limit_kb / 1024
                ),
            ))
        } else if (exit.code != Some(0) && !killed_by_us) || (unfinished && first_wrong.is_none()) {
            let reason = match (exit.signal, exit.code) {
                (Some(sig), _) => {
                    format!("Process terminated by signal {sig} ({}).", signal_name(sig))
                }
                (None, Some(code)) if code != 0 => format!("Process exited with code {code}."),
                _ => "Process exited before all tests completed.".into(),
            };
            Some((
                next,
                Verdict::RuntimeError,
                format!("{reason}\n{}", report.stderr)
                    .trim_end()
                    .to_string(),
            ))
        } else {
            None
        };
    }
    if let Some((_, Verdict::TimeLimitExceeded, msg)) = &mut abnormal {
        *msg = format!(
            "Time limit exceeded after {:.0} ms of solution time ({:.1} s wall clock).",
            report.runtime_ms,
            wall.as_secs_f64()
        );
    }

    // The earliest failure decides: a wrong answer only wins if it came before the process
    // crashed, ran out of time or memory.
    report.verdict = if let Some(ce) = harness_compile_error {
        report.error = Some(ce);
        Verdict::CompileError
    } else if first_wrong.is_some_and(|w| abnormal.as_ref().is_none_or(|(a, _, _)| w < *a)) {
        Verdict::WrongAnswer
    } else if let Some((index, verdict, msg)) = abnormal {
        // Record the case that was running when the process failed (errors and budget
        // overruns were recorded as they happened).
        let recorded = report.failure.as_ref().is_some_and(|f| f.index == index);
        if index < cases.len() && !recorded {
            let case = failed_case(index, std::mem::take(&mut pending_stdout));
            if opts.keep_all_cases {
                report.cases.push(case.clone());
            }
            report.failure = Some(case);
        }
        report.error = (!msg.is_empty()).then_some(msg);
        verdict
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
            artifacts: vec![],
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
        assert!(r.error.unwrap().contains("signal 11 (segmentation fault"));
        for sig in [
            libc::SIGBUS,
            libc::SIGFPE,
            libc::SIGABRT,
            libc::SIGILL,
            libc::SIGTRAP,
            libc::SIGKILL,
            libc::SIGTERM,
        ] {
            assert!(!signal_name(sig).is_empty());
        }

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
    fn memory_hog_is_killed_live() {
        let program = Program {
            files: vec![],
            compile: None,
            artifacts: vec![],
            run: vec![
                crate::lang::Toolchains::detect()
                    .path(crate::lang::Language::Python)
                    .to_string_lossy()
                    .into_owned(),
                "-c".into(),
                "import time\nblocks = []\nwhile True:\n    blocks.append(bytearray(8 * 1024 * 1024))\n    time.sleep(0.005)\n".into(),
            ],
        };
        let opts = RunOptions {
            limits: Limits {
                memory_limit_kb: 64 * 1024,
                wall_limit_ms: 20_000,
                ..Default::default()
            },
            ..Default::default()
        };
        let start = Instant::now();
        let r = run(&program, &cases(1), opts);
        assert_eq!(r.verdict, Verdict::MemoryLimitExceeded, "{r:?}");
        assert!(r.error.unwrap().contains("64 MB"));
        assert!(start.elapsed() < Duration::from_secs(10));
    }

    #[test]
    fn low_priority_lowers_niceness() {
        let script = r#"sleep 0.3; n=$(ps -o nice= -p $$ | tr -d ' '); printf '\036LG {"i":0,"ns":1,"out":%s}\n' "$n""#;
        let cs = [TestCase {
            args: json!([0]),
            expected: json!(LOW_PRIORITY_NICE),
        }];
        let opts = RunOptions {
            low_priority: true,
            ..Default::default()
        };
        assert_eq!(run(&shell(script), &cs, opts).verdict, Verdict::Accepted);
        let r = run(&shell(script), &cs, RunOptions::default());
        assert_eq!(r.verdict, Verdict::WrongAnswer);
    }

    #[test]
    fn raises_the_stack_limit() {
        let script = r#"n=$(ulimit -s); printf '\036LG {"i":0,"ns":1,"out":"%s"}\n' "$n""#;
        let r = run(
            &shell(script),
            &[TestCase {
                args: json!([0]),
                expected: json!(""),
            }],
            RunOptions::default(),
        );
        let soft = r.failure.and_then(|f| f.output).unwrap();
        let hard = std::process::Command::new("/bin/sh")
            .args(["-c", "ulimit -Hs"])
            .output()
            .unwrap();
        let hard = String::from_utf8_lossy(&hard.stdout).trim().to_string();
        if hard != "unlimited" {
            assert_eq!(
                soft,
                json!(hard),
                "soft stack limit raised to the hard limit"
            );
        }
    }

    #[test]
    fn reads_process_memory_and_cpu_time() {
        let pid = std::process::id() as libc::pid_t;
        let before = proc_stats(pid).unwrap();
        assert!(before.rss_kb > 100);
        let spin = Instant::now();
        let mut x = 0u64;
        while spin.elapsed() < Duration::from_millis(100) {
            x = std::hint::black_box(x.wrapping_add(1));
        }
        let gained = proc_stats(pid).unwrap().cpu - before.cpu;
        // Busy-waiting 100 ms adds roughly that much CPU time (other test threads may add more).
        assert!(gained >= Duration::from_millis(50), "{gained:?}");
        assert!(gained < Duration::from_secs(60), "{gained:?}");
        assert!(proc_stats(-5).is_none());
    }

    #[test]
    fn unreadable_results_are_runtime_errors() {
        let script = r#"printf '\036LG {"i":0,"ns":1,"out":0}\n\036LG {"i":1,"ns":1,"out":NaN}\n'"#;
        let opts = RunOptions {
            keep_all_cases: true,
            ..Default::default()
        };
        let r = run(&shell(script), &cases(2), opts);
        assert_eq!(r.verdict, Verdict::RuntimeError);
        assert!(r.error.unwrap().contains("could not read"));
        assert_eq!(r.failure.unwrap().index, 1);
        assert_eq!(r.cases.len(), 2, "the failing case is recorded");
    }

    #[test]
    fn out_of_order_and_duplicate_results_are_ignored() {
        // A duplicated index can't stand in for a missing one, and stray lines are skipped.
        let script = r#"printf '\036LG {"i":0,"ns":1,"out":0}\n\036LG {"i":0,"ns":1,"out":0}\n\036LG {"i":99,"out":1}\n'; printf 'tail'"#;
        let r = run(&shell(script), &cases(2), RunOptions::default());
        assert_eq!(r.verdict, Verdict::RuntimeError);
        assert_eq!(r.passed, 1);
        assert!(r.error.unwrap().contains("before all tests"));
        let ok = r#"printf '\036LG {"i":0,"ns":1,"out":0}\n\036LG {"i":0,"ns":1,"out":7}\n'; printf 'tail'"#;
        let r = run(&shell(ok), &cases(1), RunOptions::default());
        assert_eq!(r.verdict, Verdict::Accepted);
        assert_eq!(r.stderr, "tail\n");
    }

    #[test]
    fn huge_unterminated_output_is_bounded() {
        let start = Instant::now();
        let script = "head -c 300000000 /dev/zero | tr '\\000' x";
        let r = run(&shell(script), &cases(1), RunOptions::default());
        assert_eq!(r.verdict, Verdict::RuntimeError, "{:?}", r.error);
        assert!(r.error.unwrap().contains("Output limit exceeded"));
        assert!(start.elapsed() < Duration::from_secs(15));
        let mut buf = Vec::new();
        let mut reader = std::io::Cursor::new(b"abcdef\nxyz".to_vec());
        assert_eq!(
            read_line_capped(&mut reader, &mut buf, 3, 1 << 20).unwrap(),
            (7, true, true)
        );
        assert_eq!(buf, b"abc");
        assert_eq!(
            read_line_capped(&mut reader, &mut buf, 3, 1 << 20).unwrap(),
            (3, false, false)
        );
        assert_eq!(
            read_line_capped(&mut reader, &mut buf, 3, 1 << 20).unwrap(),
            (0, false, false)
        );
    }

    #[test]
    fn a_result_cut_off_by_the_process_ending_is_not_misread() {
        // Crashing mid-write reports the crash, not an unreadable result.
        let crash = r#"printf '\036LG {"i":0,"ns":1,"out":[1,2'; kill -SEGV $$"#;
        let r = run(&shell(crash), &cases(1), RunOptions::default());
        assert_eq!(r.verdict, Verdict::RuntimeError);
        let msg = r.error.unwrap();
        assert!(msg.contains("signal 11"), "{msg}");
        // Being killed by the time budget mid-write is a time limit.
        let spin = r#"printf '\036LG {"i":0,"ns":1,"out":[1,2'; while :; do :; done"#;
        let opts = RunOptions {
            limits: Limits {
                time_limit_ms: 50,
                wall_limit_ms: 30_000,
                ..Default::default()
            },
            ..Default::default()
        };
        let r = run(&shell(spin), &cases(1), opts);
        assert_eq!(r.verdict, Verdict::TimeLimitExceeded, "{:?}", r.error);
    }

    #[test]
    fn waiting_without_cpu_does_not_use_the_budget() {
        // A tight budget and a case that takes a while to arrive without using CPU, as when the
        // judge is descheduled on a loaded machine: the wall-clock limit still applies, the
        // budget does not.
        let script = r#"sleep 1.5; printf '\036LG {"i":0,"ns":1000,"out":0}\n'"#;
        let opts = RunOptions {
            limits: Limits {
                time_limit_ms: 5,
                wall_limit_ms: 30_000,
                ..Default::default()
            },
            ..Default::default()
        };
        let r = run(&shell(script), &cases(1), opts);
        assert_eq!(r.verdict, Verdict::Accepted, "{r:?}");
    }

    #[test]
    fn hung_case_is_stopped_within_the_budget() {
        // An infinite loop burns CPU, so the budget stops it long before the wall-clock limit.
        let script = r#"printf '\036LG {"i":0,"ns":1000,"out":0}\n'; while :; do :; done"#;
        let opts = RunOptions {
            limits: Limits {
                time_limit_ms: 200,
                wall_limit_ms: 30_000,
                ..Default::default()
            },
            keep_all_cases: true,
            ..Default::default()
        };
        let start = Instant::now();
        let r = run(&shell(script), &cases(2), opts);
        assert!(
            start.elapsed() < Duration::from_secs(5),
            "{:?}",
            start.elapsed()
        );
        assert_eq!(r.verdict, Verdict::TimeLimitExceeded);
        assert_eq!(r.passed, 1);
        let failure = r.failure.unwrap();
        assert_eq!(
            (failure.index, failure.output),
            (1, None),
            "the hung case is recorded"
        );
        assert_eq!(r.cases.len(), 2);
    }

    #[test]
    fn earliest_failure_decides_the_verdict() {
        // Wrong answer on case 0, crash later: the wrong answer came first.
        let wa_then_crash = r#"printf '\036LG {"i":0,"ns":1,"out":9}\n'; kill -SEGV $$"#;
        let r = run(&shell(wa_then_crash), &cases(2), RunOptions::default());
        assert_eq!(r.verdict, Verdict::WrongAnswer);
        // Crash on case 0 before anything wrong: runtime error.
        let crash = r#"kill -SEGV $$"#;
        let r = run(&shell(crash), &cases(2), RunOptions::default());
        assert_eq!(r.verdict, Verdict::RuntimeError);
        assert_eq!(r.failure.unwrap().index, 0);
    }

    #[test]
    fn collect_only_reports_process_outcomes() {
        let opts = RunOptions {
            collect_only: true,
            keep_all_cases: true,
            ..Default::default()
        };
        // Outputs are recorded, never judged.
        let r = run(
            &shell(ECHO_ALL),
            &[TestCase {
                args: json!([0]),
                expected: json!(null),
            }],
            opts.clone(),
        );
        assert_eq!((r.verdict, r.passed), (Verdict::Accepted, 1));
        // A crash after the first case is still a crash, with the second case recorded.
        let script = r#"printf 'dbg\n\036LG {"i":0,"ns":1,"out":5}\nboom\n'; exit 3"#;
        let r = run(&shell(script), &cases(2), opts);
        assert_eq!(r.verdict, Verdict::RuntimeError);
        assert_eq!(r.cases.len(), 2);
        assert_eq!(r.cases[1].stdout, "boom\n");
        assert_eq!(r.cases[0].stdout, "dbg\n");
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
    fn compile_cache_reuses_artifacts() {
        let cache = tempfile::tempdir().unwrap();
        let count = cache.path().join("compiles.txt");
        let compile_script = format!("echo x >> {}; cp run.sh prog.sh", count.display());
        let mut p = shell(ECHO_ALL);
        p.compile = Some(vec!["/bin/sh".into(), "-c".into(), compile_script]);
        p.run = vec!["/bin/sh".into(), "prog.sh".into()];
        p.artifacts = vec!["prog.sh".into()];
        let opts = RunOptions {
            cache_dir: Some(cache.path().join("entries")),
            ..Default::default()
        };
        for _ in 0..3 {
            assert_eq!(run(&p, &cases(1), opts.clone()).verdict, Verdict::Accepted);
        }
        assert_eq!(std::fs::read_to_string(&count).unwrap().lines().count(), 1);
        // Different sources compile again.
        p.files.push(("other.txt".into(), "x".into()));
        assert_eq!(run(&p, &cases(1), opts.clone()).verdict, Verdict::Accepted);
        assert_eq!(std::fs::read_to_string(&count).unwrap().lines().count(), 2);
        // A compile that produces no artifact is not cached and does not break the run.
        let mut broken = p.clone();
        broken.artifacts = vec!["missing.bin".into()];
        broken.run = vec!["/bin/sh".into(), "run.sh".into()];
        assert_eq!(
            run(&broken, &cases(1), opts.clone()).verdict,
            Verdict::Accepted
        );
        // Without artifacts nothing is looked up.
        assert!(cache_lookup(&shell(""), Some(cache.path())).is_none());
    }

    #[test]
    fn cache_pruning_keeps_newest() {
        let dir = tempfile::tempdir().unwrap();
        for i in 0..5 {
            std::fs::create_dir(dir.path().join(format!("e{i}"))).unwrap();
            std::thread::sleep(Duration::from_millis(15));
        }
        std::fs::create_dir(dir.path().join(".staging-x")).unwrap();
        // Using an old entry makes it the most recent one, also when it was used before.
        touch_entry(&dir.path().join("e0"));
        std::thread::sleep(Duration::from_millis(15));
        touch_entry(&dir.path().join("e1"));
        touch_entry(&dir.path().join("e4"));
        std::thread::sleep(Duration::from_millis(15));
        touch_entry(&dir.path().join("e0"));
        prune_cache(dir.path(), 2);
        let mut left: Vec<String> = std::fs::read_dir(dir.path())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        left.sort();
        assert_eq!(
            left,
            vec![".staging-x", "e0", "e4"],
            "fresh staging dirs are kept"
        );
        prune_cache(&dir.path().join("missing"), 1);
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
            assert_eq!(Verdict::parse(v.as_str()), Some(v));
            // Matches serde's representation, which exports and the UI use.
            assert_eq!(serde_json::to_value(v).unwrap(), v.as_str());
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
