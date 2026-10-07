// Self-test run inside the real desktop app when launched by scripts/smoke.sh. It drives the
// actual UI through the core flows against the real backend and reports to the shell.
import { invoke } from "@tauri-apps/api/core";
import { api, errorMessage } from "./api";
import { router } from "./router.svelte";

export interface SmokeDeps {
  root: ParentNode;
  sleep: (ms: number) => Promise<void>;
  timeoutMs: number;
}

const defaultDeps = (): SmokeDeps => ({
  root: document,
  sleep: (ms) => new Promise((r) => setTimeout(r, ms)),
  timeoutMs: 30_000,
});

/** Polls until `find` returns a truthy value. */
export async function waitFor<T>(find: () => T | null | undefined | false, deps: SmokeDeps, what: string): Promise<T> {
  const deadline = Date.now() + deps.timeoutMs;
  for (;;) {
    const v = find();
    if (v) return v;
    if (Date.now() > deadline) throw new Error(`timed out waiting for ${what}`);
    await deps.sleep(50);
  }
}

function text(root: ParentNode, selector: string, needle: string): Element | null {
  return [...root.querySelectorAll(selector)].find((e) => e.textContent?.includes(needle)) ?? null;
}

export async function runSmoke(deps: SmokeDeps = defaultDeps()): Promise<{ ok: boolean; report: string }> {
  const lines: string[] = [];
  const step = async (name: string, f: () => Promise<string | void>) => {
    const detail = await f();
    lines.push(`ok   ${name}${detail ? ` — ${detail}` : ""}`);
  };
  try {
    const boot = await api.bootstrap();
    await step("bootstrap", async () => {
      if (boot.problems.length < 3) throw new Error(`only ${boot.problems.length} problems`);
      return `${boot.problems.length} problems, ${boot.lists.length} lists, ${boot.companies.length} companies`;
    });
    await step("problem list renders", async () => {
      router.go({ name: "problems" });
      await waitFor(() => text(deps.root, ".row", boot.problems[0].title), deps, "problem rows");
    });
    const first = boot.problems[0];
    await step("workspace renders", async () => {
      router.go({ name: "problem", id: first.id });
      await waitFor(() => text(deps.root, "h1", first.title), deps, "problem heading");
      await waitFor(() => deps.root.querySelector(".cm-content"), deps, "editor");
    });
    await step("run examples from the console", async () => {
      const run = await waitFor(() => text(deps.root, "button", "Run"), deps, "Run button");
      (run as HTMLButtonElement).click();
      await waitFor(() => deps.root.querySelector(".verdict"), deps, "run verdict");
      return deps.root.querySelector(".verdict")?.textContent?.trim().split("\n")[0];
    });
    const reference = await api.referenceSolution(first.id, "python");
    if (!reference) throw new Error("no reference solution bundled");
    await step("run reference solution", async () => {
      const view = await api.problem(first.id);
      const input = view.examples.map((e) => e.input).join("\n");
      const r = await api.run(first.id, "python", reference, input);
      if (r.verdict !== "Accepted") throw new Error(`verdict ${r.verdict}: ${r.error ?? ""}`);
      return `${r.cases.length} cases`;
    });
    await step("submit reference solution", async () => {
      const s = await api.submit(first.id, "python", reference, 60_000);
      if (s.submission.verdict !== "Accepted") throw new Error(`verdict ${s.submission.verdict}`);
      return `${s.submission.passed}/${s.submission.total} tests, ${s.submission.runtime_ms.toFixed(1)} ms, ${s.submission.memory_kb} KB`;
    });
    if (first.slug === "two-sum" && boot.languages.some((l) => l.id === "javascript" && l.available)) {
      await step("submit JavaScript solution", async () => {
        const code = "var twoSum = function(nums, target) {\n  const seen = new Map();\n  for (let i = 0; i < nums.length; i++) {\n    if (seen.has(target - nums[i])) return [seen.get(target - nums[i]), i];\n    seen.set(nums[i], i);\n  }\n};\n";
        const s = await api.submit(first.id, "javascript", code, 1000);
        if (s.submission.verdict !== "Accepted") throw new Error(`verdict ${s.submission.verdict}: ${s.submission.error ?? ""}`);
        return `${s.submission.passed}/${s.submission.total} tests, ${s.submission.runtime_ms.toFixed(1)} ms`;
      });
    }
    await step("stats page renders", async () => {
      router.go({ name: "stats" });
      await waitFor(() => text(deps.root, "h2", "By difficulty"), deps, "stats");
      const stats = await api.stats(10, 0, 0);
      if (stats.totals.solved < 1) throw new Error("solve not counted");
    });
    await step("settings page renders", async () => {
      router.go({ name: "settings" });
      await waitFor(() => text(deps.root, "h2", "Evaluation"), deps, "settings");
    });
    return { ok: true, report: lines.join("\n") };
  } catch (e) {
    lines.push(`FAIL ${errorMessage(e)}`);
    return { ok: false, report: lines.join("\n") };
  }
}

/** Entry point from main.ts: no-op unless the shell says this is a smoke run. */
export async function maybeRunSmoke(): Promise<void> {
  const enabled = await invoke<boolean>("smoke_mode").catch(() => false);
  if (!enabled) return;
  const { ok, report } = await runSmoke();
  await invoke("smoke_finish", { ok, report });
}
