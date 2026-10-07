import { describe, expect, it, vi } from "vitest";
import { api } from "./api";
import { installBackend, tick } from "../test/backend";
import { defaultSettings } from "../mock/backend";
import { DRAFT_SAVE_DELAY_MS, TIMER_PERSIST_MS, Workspace } from "./workspace.svelte";
import type { Status } from "./types";

function make(id = 1, overrides: Partial<{ auto: boolean; now: () => number }> = {}) {
  const statuses: [number, Status][] = [];
  const notes: string[] = [];
  const settings = { ...defaultSettings(), auto_start_timer: overrides.auto ?? true };
  const ws = new Workspace(id, api, {
    settings: () => settings,
    onStatus: (pid, s) => statuses.push([pid, s]),
    notify: (m) => notes.push(m),
    now: overrides.now,
  });
  return { ws, statuses, notes };
}

describe("Workspace", () => {
  it("loads the problem and example cases", async () => {
    installBackend();
    const { ws } = make();
    await ws.load();
    expect(ws.view?.title).toBe("Two Sum");
    expect(ws.names).toEqual(["nums", "target"]);
    expect(ws.cases).toEqual([["[2,7,11,15]", "9"], ["[3,2,4]", "6"], ["[3,3]", "6"]]);
    expect(ws.code).toContain("class Solution");
  });

  it("reports load errors", async () => {
    installBackend();
    const { ws } = make(999);
    await ws.load();
    expect(ws.loadError).toContain("not found");
    expect(ws.names).toEqual([]);
  });

  it("debounces draft saves and auto-starts the timer", async () => {
    vi.useFakeTimers();
    try {
      const backend = installBackend();
      const { ws } = make();
      await ws.load();
      ws.edit(ws.code);
      expect(ws.timerRunning).toBe(false);
      ws.edit("print(1)");
      ws.edit("print(2)");
      expect(ws.timerRunning).toBe(true);
      await vi.advanceTimersByTimeAsync(DRAFT_SAVE_DELAY_MS + 10);
      expect(backend.state.drafts.get("1:python")).toBe("print(2)");
      expect(backend.state.solves).toHaveLength(1);
    } finally {
      vi.useRealTimers();
    }
  });

  it("does not auto-start when disabled and notifies on draft failures", async () => {
    installBackend((cmd) => {
      if (cmd === "save_draft") throw "readonly";
    });
    const { ws, notes } = make(1, { auto: false });
    await ws.load();
    ws.edit("x");
    expect(ws.timerRunning).toBe(false);
    await ws.flushDraft();
    expect(notes[0]).toContain("readonly");
    await ws.flushDraft();
  });

  it("resumes a persisted solve and persists elapsed time", async () => {
    let now = 0;
    const backend = installBackend();
    const solve = await api.startSolve(1);
    await api.updateSolve(solve.id, 60_000);
    const { ws } = make(1, { now: () => now });
    await ws.load();
    expect(ws.elapsed).toBe(60_000);
    expect(ws.timerRunning).toBe(false);
    await ws.toggleTimer();
    now += TIMER_PERSIST_MS;
    ws.tick();
    await tick();
    expect(backend.state.solves[0].elapsed_ms).toBe(60_000 + TIMER_PERSIST_MS);
    expect(ws.elapsed).toBe(60_000 + TIMER_PERSIST_MS);
    now += 1000;
    await ws.toggleTimer();
    expect(ws.timerRunning).toBe(false);
    expect(backend.state.solves[0].elapsed_ms).toBe(61_000 + TIMER_PERSIST_MS);
    await ws.persistTimer(false);
    await ws.resetTimer();
    expect(ws.elapsed).toBe(0);
    expect(backend.state.solves).toHaveLength(0);
    await ws.resetTimer();
  });

  it("handles timer start failures", async () => {
    installBackend((cmd) => {
      if (cmd === "start_solve") throw "db locked";
    });
    const { ws, notes } = make();
    await ws.load();
    await ws.startTimer();
    expect(notes[0]).toContain("db locked");
    await ws.startTimer();
  });

  it("runs testcases", async () => {
    installBackend();
    const { ws } = make();
    await ws.load();
    ws.code = "return x";
    await ws.run();
    expect(ws.consoleTab).toBe("result");
    expect(ws.runResult?.verdict).toBe("Accepted");
    expect(ws.runResult?.cases).toHaveLength(3);

    ws.setCases([["[1,", "2"]]);
    await ws.run();
    expect(ws.runError).toContain("not valid JSON");
    expect(ws.runResult).toBeNull();
    ws.resetCases();
    expect(ws.cases).toHaveLength(3);
  });

  it("surfaces run failures", async () => {
    installBackend((cmd) => {
      if (cmd === "run_code") throw "toolchain unavailable: python3";
    });
    const { ws } = make();
    await ws.load();
    await ws.run();
    expect(ws.runError).toContain("python3");
    expect(ws.running).toBe(false);
  });

  it("restores edited cases from the session", async () => {
    installBackend();
    sessionStorage.setItem("lg-cases-1", JSON.stringify([["[9]", "9"]]));
    const { ws } = make();
    await ws.load();
    expect(ws.cases).toEqual([["[9]", "9"]]);
    sessionStorage.setItem("lg-cases-1", "{broken");
    const other = make().ws;
    await other.load();
    expect(other.cases).toHaveLength(3);
  });

  it("submits, tracking errors until accepted", async () => {
    installBackend();
    const { ws, statuses } = make();
    await ws.load();
    ws.code = "pass";
    await ws.startTimer();
    await ws.submit();
    expect(ws.lastSubmit?.submission.verdict).toBe("WrongAnswer");
    expect(ws.leftTab).toBe("result");
    expect(ws.timerRunning).toBe(true);
    expect(statuses).toEqual([[1, "attempted"]]);

    ws.code = "return 1";
    await ws.submit();
    expect(ws.lastSubmit?.submission.verdict).toBe("Accepted");
    expect(ws.lastSubmit?.solve.errors).toBe(1);
    expect(ws.solveCompleted).toBe(true);
    expect(ws.timerRunning).toBe(false);
    expect(ws.submissions).toHaveLength(2);
    expect(statuses[1]).toEqual([1, "solved"]);

    // Editing after a completed solve starts a fresh attempt.
    ws.edit("again");
    await tick();
    expect(ws.solveCompleted).toBe(false);
    expect(ws.timerRunning).toBe(true);
  });

  it("reports submit failures and ignores concurrent actions", async () => {
    installBackend((cmd) => {
      if (cmd === "submit_code") throw "judge crashed";
    });
    const { ws, notes } = make();
    await ws.submit();
    await ws.run();
    await ws.load();
    const first = ws.submit();
    await ws.submit();
    await ws.run();
    await first;
    expect(notes).toEqual(["judge crashed"]);
    expect(ws.submitting).toBe(false);
  });

  it("tracks progress for its own problem only", async () => {
    installBackend();
    const { ws } = make();
    await ws.load();
    ws.onProgress(1, 1, 2);
    expect(ws.progress).toBeNull();
    ws.submitting = true;
    ws.onProgress(2, 1, 2);
    expect(ws.progress).toBeNull();
    ws.onProgress(1, 1, 2);
    expect(ws.progress).toEqual({ done: 1, total: 2 });
  });

  it("switches language and resets code", async () => {
    const backend = installBackend();
    const { ws } = make();
    await ws.load();
    await ws.changeLanguage("python");
    ws.edit("mine");
    await ws.resetCode();
    expect(ws.code).toContain("class Solution");
    expect(backend.state.drafts.size).toBe(0);
    ws.language = "other" as never;
    await ws.changeLanguage("python");
    expect(ws.language).toBe("python");
  });

  it("selects submissions and disposes cleanly", async () => {
    const backend = installBackend();
    const { ws } = make();
    await ws.load();
    ws.selectSubmission(null);
    expect(ws.leftTab).toBe("description");
    ws.code = "return 1";
    await ws.submit();
    ws.leftTab = "submissions";
    ws.selectSubmission(ws.submissions[0]);
    expect(ws.leftTab).toBe("result");
    ws.edit("unsaved");
    await tick();
    await ws.dispose();
    expect(backend.state.drafts.get("1:python")).toBe("unsaved");
    expect(ws.timerRunning).toBe(false);
    await ws.dispose();
  });

  it("ignores run, submit and case reset before load", async () => {
    installBackend();
    const { ws } = make();
    ws.resetCases();
    await ws.run();
    await ws.submit();
    expect(ws.runResult).toBeNull();
  });
});
