import { describe, expect, it } from "vitest";
import { render } from "@testing-library/svelte";
import App from "../App.svelte";
import { installBackend } from "../test/backend";
import { maybeRunSmoke, runSmoke, waitFor } from "./smoke";

const deps = (timeoutMs = 3000) => ({ root: document, sleep: (ms: number) => new Promise<void>((r) => setTimeout(r, ms)), timeoutMs });

describe("smoke self-test", () => {
  it("passes against a healthy backend", async () => {
    installBackend((cmd) => (cmd === "reference_solution" ? "class Solution:\n    return 1" : undefined));
    render(App);
    const { ok, report } = await runSmoke(deps());
    expect(report).toContain("ok   submit reference solution");
    expect(report).toContain("ok   settings page renders");
    expect(ok).toBe(true);
  });

  it("reports the failing step", async () => {
    installBackend();
    render(App);
    const { ok, report } = await runSmoke(deps());
    expect(ok).toBe(false);
    expect(report).toMatch(/FAIL verdict WrongAnswer/);
  });

  it("fails without a reference solution or with too few problems", async () => {
    installBackend((cmd) => (cmd === "reference_solution" ? null : undefined));
    render(App);
    expect((await runSmoke(deps())).report).toContain("no reference solution");
  });

  it("times out waiting for elements", async () => {
    await expect(waitFor(() => null, deps(60), "nothing")).rejects.toThrow("timed out waiting for nothing");
    expect(await waitFor(() => 5, deps(), "five")).toBe(5);
  });

  it("only runs when the shell enables smoke mode", async () => {
    const calls: string[] = [];
    installBackend((cmd) => {
      calls.push(cmd);
      return undefined;
    });
    await maybeRunSmoke();
    expect(calls).toEqual(["smoke_mode"]);

    const finished: unknown[] = [];
    installBackend((cmd, args) => {
      if (cmd === "smoke_mode") return true;
      if (cmd === "bootstrap") throw "boom";
      if (cmd === "smoke_finish") {
        finished.push(args);
        return null;
      }
      return undefined;
    });
    await maybeRunSmoke();
    expect(finished).toEqual([{ ok: false, report: "FAIL boom" }]);
  });
});
