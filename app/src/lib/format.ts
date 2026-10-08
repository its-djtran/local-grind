import type { BuildInfo, Verdict } from "./types";

export function formatRuntime(ms: number): string {
  if (!Number.isFinite(ms) || ms < 0) return "—";
  if (ms < 1) return ms === 0 ? "0 ms" : "<1 ms";
  if (ms < 1000) return `${Math.round(ms)} ms`;
  return `${(ms / 1000).toFixed(2)} s`;
}

export function formatMemory(kb: number): string {
  if (!Number.isFinite(kb) || kb < 0) return "—";
  if (kb < 1024) return `${Math.round(kb)} KB`;
  if (kb < 1024 * 1024) return `${(kb / 1024).toFixed(1)} MB`;
  return `${(kb / 1024 / 1024).toFixed(2)} GB`;
}

/** Solve-timer style clock: `MM:SS`, or `H:MM:SS` from one hour. */
export function formatClock(ms: number): string {
  const total = Math.max(0, Math.floor(ms / 1000));
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  const mm = String(m).padStart(2, "0");
  const ss = String(s).padStart(2, "0");
  return h > 0 ? `${h}:${mm}:${ss}` : `${mm}:${ss}`;
}

/** Compact human duration: `45s`, `12m 5s`, `1h 3m`. */
export function formatDuration(ms: number): string {
  const total = Math.max(0, Math.round(ms / 1000));
  if (total < 60) return `${total}s`;
  const h = Math.floor(total / 3600);
  const m = Math.floor((total % 3600) / 60);
  const s = total % 60;
  return h > 0 ? `${h}h ${m}m` : `${m}m ${s}s`;
}

export function formatRelative(ts: number, now: number = Date.now()): string {
  const diff = Math.max(0, now - ts);
  const min = 60_000;
  const hour = 60 * min;
  const day = 24 * hour;
  if (diff < min) return "just now";
  if (diff < hour) return `${Math.floor(diff / min)} min ago`;
  if (diff < day) return `${Math.floor(diff / hour)} hr ago`;
  if (diff < 30 * day) {
    const d = Math.floor(diff / day);
    return `${d} day${d === 1 ? "" : "s"} ago`;
  }
  return formatDate(ts);
}

export function formatDate(ts: number): string {
  const d = new Date(ts);
  const mm = String(d.getMonth() + 1).padStart(2, "0");
  const dd = String(d.getDate()).padStart(2, "0");
  return `${d.getFullYear()}-${mm}-${dd}`;
}

export function formatDateTime(ts: number): string {
  const d = new Date(ts);
  const hh = String(d.getHours()).padStart(2, "0");
  const mi = String(d.getMinutes()).padStart(2, "0");
  return `${formatDate(ts)} ${hh}:${mi}`;
}

const VERDICT_LABELS: Record<Verdict, string> = {
  Accepted: "Accepted",
  WrongAnswer: "Wrong Answer",
  TimeLimitExceeded: "Time Limit Exceeded",
  MemoryLimitExceeded: "Memory Limit Exceeded",
  RuntimeError: "Runtime Error",
  CompileError: "Compile Error",
};

export function verdictLabel(v: Verdict): string {
  return VERDICT_LABELS[v] ?? v;
}

/** Compact JSON as LeetCode displays values: `[1,2,3]`, `"abc"`, `null`. */
export function formatValue(v: unknown): string {
  if (v === undefined) return "";
  return JSON.stringify(v);
}

export function percent(n: number, digits = 0): string {
  return Number.isFinite(n) ? `${n.toFixed(digits)}%` : "—";
}

/** Relative comparison against a benchmark: "1.8x the reference" / "faster than the reference". */
export function compareToReference(value: number, reference: number): string {
  if (!(reference > 0) || !Number.isFinite(value)) return "";
  const ratio = value / reference;
  if (ratio <= 1.05) return "on par with or better than the reference";
  return `${ratio.toFixed(1)}x the reference`;
}

/** "Local Grind 0.2.0 (4381dab0c0de) · problem catalog built …" for the About line in Settings. */
export function formatBuild(build: BuildInfo, catalogBuiltAt: string | null): string {
  const app = `Local Grind ${build.version}${build.commit ? ` (${build.commit})` : ""}`;
  return catalogBuiltAt ? `${app} · problem catalog built ${catalogBuiltAt}` : app;
}
