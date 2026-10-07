// In-memory stand-in for the Rust backend. Used by component tests and by `npm run dev:mock`
// to preview the UI in a plain browser. It mimics command names and payload shapes only; the
// real judging logic lives in crates/core.
import { emit } from "@tauri-apps/api/event";
import catalog from "./catalog.json";
import type {
  Bootstrap,
  ProblemListing,
  ProblemView,
  RunOutcome,
  Settings,
  Solve,
  Stats,
  Submission,
  SubmitOutcome,
} from "../lib/types";

type Args = Record<string, unknown>;

export const defaultSettings = (): Settings => ({
  theme: "dark",
  font_size: 14,
  editor_font_size: 14,
  font_family: "Inconsolata",
  language: "python",
  keymap: "standard",
  tab_size: 4,
  auto_start_timer: true,
  show_timer: true,
  eval: { time_limit_multiplier: 1, memory_limit_mb: 1024, max_parallel_jobs: 4, low_priority: false, toolchain_paths: {} },
});

const EXTRA: [number, string, string, "Easy" | "Medium" | "Hard", string[]][] = [
  [20, "valid-parentheses", "Valid Parentheses", "Easy", ["String", "Stack"]],
  [121, "best-time-to-buy-and-sell-stock", "Best Time to Buy and Sell Stock", "Easy", ["Array", "Dynamic Programming"]],
  [206, "reverse-linked-list", "Reverse Linked List", "Easy", ["Linked List", "Recursion"]],
  [3, "longest-substring-without-repeating-characters", "Longest Substring Without Repeating Characters", "Medium", ["Hash Table", "String", "Sliding Window"]],
  [15, "3sum", "3Sum", "Medium", ["Array", "Two Pointers", "Sorting"]],
  [42, "trapping-rain-water", "Trapping Rain Water", "Hard", ["Array", "Two Pointers", "Stack"]],
];

const STARTER = (name: string) => `class Solution:\n    def ${name}(self):\n        `;

export interface Backend {
  handle: (cmd: string, args: Args) => Promise<unknown>;
  state: {
    settings: Settings;
    submissions: Submission[];
    solves: Solve[];
    drafts: Map<string, string>;
    dialogPath: string | null;
    confirm: boolean;
    files: Map<string, string>;
  };
}

export function createBackend(): Backend {
  const views = new Map<number, Omit<ProblemView, "code" | "starter" | "active_solve" | "status" | "language">>();
  for (const p of catalog as unknown as ProblemView[]) {
    views.set(p.id, {
      ...p,
      companies: [
        { company: "Google", frequency: 92.4, recency: "30d" },
        { company: "Amazon", frequency: 88.1, recency: "3m" },
      ],
      benchmarks: { python: { runtime_ms: 4.2, memory_kb: 17_400, runs: 10 } },
      lists: ["Blind 75", "Grind 75"],
    });
  }
  for (const [id, slug, title, difficulty, topics] of EXTRA) {
    views.set(id, {
      id,
      slug,
      title,
      difficulty,
      topics,
      signature: { kind: "function", name: "solve", params: [{ name: "s", type: "string" }], returns: "bool" },
      time_limit_ms: 2000,
      description_html: `<p>Placeholder statement for <strong>${title}</strong>.</p>`,
      editorial_html: "<p>Editorial coming soon.</p>",
      hints: [],
      examples: [{ input: '"()"', output: "true" }],
      test_count: 40,
      companies: [],
      benchmarks: {},
      lists: [],
    });
  }

  const state: Backend["state"] = {
    settings: defaultSettings(),
    submissions: [],
    solves: [],
    drafts: new Map(),
    dialogPath: "/tmp/localgrind-history.json",
    confirm: true,
    files: new Map(),
  };
  let nextId = 1;

  const starter = (id: number) => {
    const v = views.get(id)!;
    return v.signature.kind === "function" ? STARTER(v.signature.name) : `class ${v.signature.class}:\n    pass\n`;
  };
  const statusOf = (id: number): ProblemListing["status"] => {
    const subs = state.submissions.filter((s) => s.problem_id === id);
    if (subs.some((s) => s.verdict === "Accepted")) return "solved";
    return subs.length ? "attempted" : "todo";
  };
  const activeSolve = (id: number) => state.solves.find((s) => s.problem_id === id && s.completed_at === null) ?? null;
  const view = (id: number): ProblemView => {
    const v = views.get(id);
    if (!v) throw `not found: problem ${id}`;
    return {
      ...v,
      language: "python",
      starter: starter(id),
      code: state.drafts.get(`${id}:python`) ?? starter(id),
      active_solve: activeSolve(id),
      status: statusOf(id),
    };
  };
  const listings = (): ProblemListing[] =>
    [...views.values()]
      .sort((a, b) => a.id - b.id)
      .map(({ id, slug, title, difficulty, topics }) => ({ id, slug, title, difficulty, topics, status: statusOf(id) }));

  const judge = (code: string) => {
    if (code.includes("syntax error")) return "CompileError" as const;
    if (code.includes("raise")) return "RuntimeError" as const;
    if (code.includes("while True")) return "TimeLimitExceeded" as const;
    return code.includes("return") ? ("Accepted" as const) : ("WrongAnswer" as const);
  };

  const stats = (): Stats => {
    const done = state.solves.filter((s) => s.completed_at !== null);
    const agg = (xs: Solve[]) => {
      if (!xs.length) return { solves: 0, avg_ms: 0, min_ms: 0, max_ms: 0, errors: 0, error_rate: 0 };
      const errors = xs.reduce((a, s) => a + s.errors, 0);
      return {
        solves: xs.length,
        avg_ms: xs.reduce((a, s) => a + s.elapsed_ms, 0) / xs.length,
        min_ms: Math.min(...xs.map((s) => s.elapsed_ms)),
        max_ms: Math.max(...xs.map((s) => s.elapsed_ms)),
        errors,
        error_rate: (100 * errors) / (errors + xs.length),
      };
    };
    const all = listings();
    const solvedIds = new Set(all.filter((p) => p.status === "solved").map((p) => p.id));
    const today = new Date();
    const key = (d: Date) =>
      `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
    const heat = new Map<string, { submissions: number; accepted: number }>();
    for (const s of state.submissions) {
      const k = key(new Date(s.created_at));
      const e = heat.get(k) ?? { submissions: 0, accepted: 0 };
      e.submissions++;
      if (s.verdict === "Accepted") e.accepted++;
      heat.set(k, e);
    }
    void today;
    return {
      totals: {
        solved: solvedIds.size,
        attempted: new Set(state.submissions.map((s) => s.problem_id)).size,
        available: all.length,
        submissions: state.submissions.length,
        accepted_submissions: state.submissions.filter((s) => s.verdict === "Accepted").length,
        solves: done.length,
        current_streak: done.length ? 1 : 0,
        longest_streak: done.length ? 1 : 0,
      },
      recent_n: 10,
      overall: agg(done),
      recent: agg(done.slice(-10)),
      by_difficulty: (["Easy", "Medium", "Hard"] as const).map((d) => ({
        difficulty: d,
        solved: all.filter((p) => p.difficulty === d && solvedIds.has(p.id)).length,
        available: all.filter((p) => p.difficulty === d).length,
        overall: agg(done.filter((s) => s.difficulty === d)),
        recent: agg(done.slice(-10).filter((s) => s.difficulty === d)),
      })),
      topics: [...new Set(all.flatMap((p) => p.topics))].sort().map((t) => ({
        topic: t,
        solved: all.filter((p) => p.topics.includes(t) && solvedIds.has(p.id)).length,
        available: all.filter((p) => p.topics.includes(t)).length,
        solves: 0,
        avg_ms: 0,
        error_rate: 0,
      })),
      lists: [{ id: "blind75", name: "Blind 75", solved: solvedIds.has(1) ? 1 : 0, total: 4 }],
      heatmap: [...heat.entries()].map(([date, v]) => ({ date, ...v })),
      review: done.map((s) => ({
        slug: s.slug,
        problem_id: s.problem_id,
        title: s.title,
        difficulty: s.difficulty,
        solves: 1,
        errors: s.errors,
        pass_rate: 1 / (1 + s.errors),
        avg_ms: s.elapsed_ms,
        days_since: 12,
        time_ratio: 1,
        score: 40,
        reasons: [{ text: "clean solve, just due for a refresh", severity: "" as const }],
      })),
    };
  };

  async function handle(cmd: string, a: Args): Promise<unknown> {
    const id = a.id as number;
    switch (cmd) {
      case "bootstrap":
        return {
          problems: listings(),
          lists: [
            {
              id: "blind75",
              name: "Blind 75",
              description: "The classic 75",
              items: [
                { problem_id: 1, group: "Array" },
                { problem_id: 121, group: "Array" },
                { problem_id: 3, group: "String" },
                { problem_id: 206, group: "Linked List" },
              ],
            },
            { id: "grind75", name: "Grind 75", description: "By week", items: [{ problem_id: 1, group: "Week 1" }, { problem_id: 20, group: "Week 1" }] },
          ],
          companies: [{ name: "Google", problem_ids: [1, 42] }, { name: "Amazon", problem_ids: [1] }],
          topics: [...new Set(listings().flatMap((p) => p.topics))].sort(),
          settings: state.settings,
          languages: [{ id: "python", name: "Python3", available: true, executable: "/opt/homebrew/bin/python3" }],
          catalog_built_at: "2026-10-07T12:00:00.000Z",
        } satisfies Bootstrap;
      case "get_problem":
        return view(id);
      case "save_draft":
        state.drafts.set(`${id}:${a.language}`, a.code as string);
        return null;
      case "reset_code":
        state.drafts.delete(`${id}:${a.language}`);
        return starter(id);
      case "run_code": {
        const verdict = judge(a.code as string);
        const lines = (a.input as string).split("\n").filter(Boolean);
        const arity = Math.max(1, view(id).signature.kind === "function" ? (view(id).signature as { params: unknown[] }).params.length : 2);
        const cases = [];
        for (let i = 0; i < lines.length; i += arity) {
          const args = lines.slice(i, i + arity).map((l) => JSON.parse(l));
          const passed = verdict === "Accepted";
          cases.push({ index: i / arity, passed, args, expected: [0, 1], output: passed ? [0, 1] : [1, 1], stdout: i === 0 ? "debug\n" : "", runtime_ns: 1000 });
        }
        const failed = verdict === "RuntimeError" || verdict === "CompileError";
        return {
          verdict: verdict === "TimeLimitExceeded" ? "TimeLimitExceeded" : verdict,
          passed: cases.filter((c) => c.passed).length,
          total: cases.length,
          runtime_ms: 0.4,
          memory_kb: 16_000,
          cases: failed ? [] : cases,
          failure: null,
          error: failed ? `${verdict}: boom on line 3` : null,
          stderr: "",
          expected_available: !(a.code as string).includes("noexpected"),
        } satisfies RunOutcome;
      }
      case "submit_code": {
        const verdict = judge(a.code as string);
        const total = view(id).test_count;
        for (const done of [Math.floor(total / 2), total]) await emit("submit-progress", { problem_id: id, done, total });
        const sub: Submission = {
          id: nextId++,
          problem_id: id,
          language: a.language as string,
          code: a.code as string,
          verdict,
          passed: verdict === "Accepted" ? total : 3,
          total,
          runtime_ms: 12.6,
          memory_kb: 17_900,
          error: verdict === "RuntimeError" ? "IndexError: list index out of range" : null,
          failure:
            verdict === "WrongAnswer"
              ? { index: 3, passed: false, args: [[3, 3], 6], expected: [0, 1], output: [1, 1], stdout: "dbg\n", runtime_ns: 5 }
              : null,
          created_at: Date.now(),
        };
        state.submissions.push(sub);
        let solve = activeSolve(id);
        if (!solve) {
          const v = views.get(id)!;
          solve = { id: nextId++, problem_id: id, slug: v.slug, title: v.title, difficulty: v.difficulty, started_at: Date.now(), completed_at: null, elapsed_ms: 0, errors: 0, submission_id: null };
          state.solves.push(solve);
        }
        solve.elapsed_ms = a.elapsedMs as number;
        if (verdict === "Accepted") {
          solve.completed_at = sub.created_at;
          solve.submission_id = sub.id;
        } else solve.errors++;
        return { submission: sub, solve: { ...solve }, benchmark: views.get(id)!.benchmarks.python ?? null } satisfies SubmitOutcome;
      }
      case "list_submissions":
        return state.submissions.filter((s) => s.problem_id === id).reverse();
      case "start_solve": {
        const existing = activeSolve(id);
        if (existing) return existing;
        const v = views.get(id)!;
        const solve: Solve = { id: nextId++, problem_id: id, slug: v.slug, title: v.title, difficulty: v.difficulty, started_at: Date.now(), completed_at: null, elapsed_ms: 0, errors: 0, submission_id: null };
        state.solves.push(solve);
        return solve;
      }
      case "update_solve": {
        const s = state.solves.find((x) => x.id === a.solveId);
        if (s && s.completed_at === null) s.elapsed_ms = a.elapsedMs as number;
        return null;
      }
      case "discard_solve":
        state.solves = state.solves.filter((x) => !(x.id === a.solveId && x.completed_at === null));
        return null;
      case "get_settings":
        return state.settings;
      case "save_settings":
        state.settings = { ...(a.settings as Settings), font_size: Math.min(32, Math.max(10, (a.settings as Settings).font_size)) };
        return state.settings;
      case "get_stats":
        return stats();
      case "list_solves":
        return state.solves.filter((s) => s.completed_at !== null);
      case "export_history":
        state.files.set(a.path as string, JSON.stringify({ format: "localgrind-export", submissions: state.submissions }));
        return null;
      case "import_history": {
        if ((a.path as string).includes("bad")) throw "invalid data: unrecognized import file";
        return { submissions_added: 2, submissions_skipped: 1, solves_added: 1, solves_skipped: 0, drafts_added: 0, settings_applied: a.applySettings as boolean, source: "localgrind-export" };
      }
      case "clear_history":
        state.submissions = [];
        state.solves = [];
        state.drafts.clear();
        return null;
      case "reference_solution":
        return id === 1 ? "class Solution: ..." : null;
      case "check_toolchains":
        return [{ language: "python", executable: "/opt/homebrew/bin/python3", version: "Python 3.14.6", ok: true, message: null }];
      case "smoke_mode":
        return false;
      case "smoke_finish":
        return null;
      case "plugin:dialog|save":
      case "plugin:dialog|open":
        return state.dialogPath;
      case "plugin:dialog|message":
        return state.confirm ? "Yes" : "No";
      default:
        throw `unknown command ${cmd}`;
    }
  }

  return { handle, state };
}
