// Mirrors the serde payloads produced by localgrind-core (crates/core/src/service.rs).

export type Difficulty = "Easy" | "Medium" | "Hard";
export type Status = "solved" | "attempted" | "todo";
export type LanguageId = "python" | "javascript" | "cpp" | "rust";
export type Verdict =
  | "Accepted"
  | "WrongAnswer"
  | "TimeLimitExceeded"
  | "MemoryLimitExceeded"
  | "RuntimeError"
  | "CompileError";

export interface ProblemSummary {
  id: number;
  slug: string;
  title: string;
  difficulty: Difficulty;
  topics: string[];
}

export interface ProblemListing extends ProblemSummary {
  status: Status;
}

export interface Param {
  name: string;
  type: string;
  ref_of?: string;
  hidden?: boolean;
}

export interface Method {
  name: string;
  params: Param[];
  returns: string;
}

export type Signature =
  | { kind: "function"; name: string; params: Param[]; returns: string; output_param?: string }
  | { kind: "design"; class: string; constructor: Param[]; methods: Method[] }
  | { kind: "codec"; class: string; encode: Method; decode: Method };

export interface Example {
  input: string;
  output: string;
  explanation?: string | null;
}

export interface Benchmark {
  runtime_ms: number;
  memory_kb: number;
  runs: number;
  /** Measured on this machine by a local recalibration. */
  local?: boolean;
}

export interface CalibrationSummary {
  measured: number;
  failed: number;
  cancelled: boolean;
}

export interface CalibrationStatus {
  local_benchmarks: number;
  measured_at: number | null;
}

export interface CompanyTag {
  company: string;
  frequency: number;
  recency: string;
}

export interface Solve {
  id: number;
  problem_id: number | null;
  slug: string;
  title: string;
  difficulty: Difficulty;
  started_at: number;
  completed_at: number | null;
  elapsed_ms: number;
  errors: number;
  submission_id: number | null;
}

export interface ReferenceSolution {
  language: LanguageId;
  html: string;
}

export interface ProblemView extends ProblemSummary {
  signature: Signature;
  time_limit_ms: number;
  description_html: string;
  editorial_html: string;
  hints: string[];
  examples: Example[];
  test_count: number;
  companies: CompanyTag[];
  benchmarks: Record<string, Benchmark>;
  solutions: ReferenceSolution[];
  lists: string[];
  language: LanguageId;
  languages: LanguageId[];
  code: string;
  starter: string;
  active_solve: Solve | null;
  status: Status;
}

export interface CaseResult {
  index: number;
  passed: boolean;
  args: unknown[];
  expected: unknown;
  output: unknown;
  stdout: string;
  runtime_ns: number;
}

export interface RunReport {
  verdict: Verdict;
  passed: number;
  total: number;
  runtime_ms: number;
  memory_kb: number;
  cases: CaseResult[];
  failure: CaseResult | null;
  error: string | null;
  stderr: string;
}

export interface RunOutcome extends RunReport {
  expected_available: boolean;
}

export interface Submission {
  id: number;
  problem_id: number;
  language: string;
  code: string;
  verdict: Verdict;
  passed: number;
  total: number;
  runtime_ms: number;
  memory_kb: number;
  error: string | null;
  failure: CaseResult | null;
  created_at: number;
}

export interface SubmitOutcome {
  submission: Submission;
  solve: Solve;
  benchmark: Benchmark | null;
}

export interface EvalSettings {
  time_limit_multiplier: number;
  memory_limit_mb: number;
  max_parallel_jobs: number;
  low_priority: boolean;
  toolchain_paths: Record<string, string>;
}

export interface Settings {
  theme: "dark" | "light" | "system";
  font_size: number;
  editor_font_size: number;
  font_family: string;
  language: LanguageId;
  keymap: "standard" | "vim";
  tab_size: number;
  auto_start_timer: boolean;
  show_timer: boolean;
  eval: EvalSettings;
}

export interface ListItem {
  problem_id: number;
  group: string;
}

export interface ProblemList {
  id: string;
  name: string;
  description: string;
  items: ListItem[];
}

export interface CompanyEntry {
  name: string;
  problem_ids: number[];
}

export interface LanguageInfo {
  id: LanguageId;
  name: string;
  available: boolean;
  executable: string;
}

export interface ToolchainStatus {
  language: LanguageId;
  executable: string;
  version: string | null;
  ok: boolean;
  message: string | null;
}

export interface Bootstrap {
  problems: ProblemListing[];
  lists: ProblemList[];
  companies: CompanyEntry[];
  topics: string[];
  settings: Settings;
  languages: LanguageInfo[];
  catalog_built_at: string | null;
}

export interface Aggregate {
  solves: number;
  avg_ms: number;
  min_ms: number;
  max_ms: number;
  errors: number;
  error_rate: number;
}

export interface DifficultyStats {
  difficulty: Difficulty;
  solved: number;
  available: number;
  overall: Aggregate;
  recent: Aggregate;
}

export interface TopicStats {
  topic: string;
  solved: number;
  available: number;
  solves: number;
  avg_ms: number;
  error_rate: number;
}

export interface ListProgress {
  id: string;
  name: string;
  solved: number;
  total: number;
}

export interface DayCount {
  date: string;
  submissions: number;
  accepted: number;
}

export interface Reason {
  text: string;
  severity: "bad" | "warn" | "";
}

export interface ReviewItem {
  slug: string;
  problem_id: number | null;
  title: string;
  difficulty: Difficulty;
  solves: number;
  errors: number;
  pass_rate: number;
  avg_ms: number;
  days_since: number;
  time_ratio: number;
  score: number;
  reasons: Reason[];
}

export interface Totals {
  solved: number;
  attempted: number;
  available: number;
  submissions: number;
  accepted_submissions: number;
  solves: number;
  current_streak: number;
  longest_streak: number;
}

export interface WeekPoint {
  week_start: string;
  solves: number;
  avg_ms: number | null;
  error_rate: number | null;
}

export interface Stats {
  totals: Totals;
  recent_n: number;
  overall: Aggregate;
  recent: Aggregate;
  by_difficulty: DifficultyStats[];
  topics: TopicStats[];
  lists: ListProgress[];
  heatmap: DayCount[];
  weekly: WeekPoint[];
  review: ReviewItem[];
}

export interface ImportSummary {
  submissions_added: number;
  submissions_skipped: number;
  solves_added: number;
  solves_skipped: number;
  drafts_added: number;
  settings_applied: boolean;
  source: string;
}

export interface Progress {
  problem_id: number;
  done: number;
  total: number;
}
