import { describe, expect, it } from "vitest";
import { fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";

const table = async () => within(await screen.findByRole("table", { name: "Problems" }));
import App from "../App.svelte";
import { installBackend, tick } from "../test/backend";
import type { Stats } from "../lib/types";
import { router } from "../lib/router.svelte";

async function boot(hash = "#/problems") {
  return boot_(hash);
}

async function boot_(hash: string) {
  location.hash = hash;
  router.sync();
  render(App);
  await screen.findByLabelText("Local Grind home");
}

describe("App", () => {
  it("shows the problem list and filters it", async () => {
    installBackend();
    await boot();
    await (await table()).findByText("1. Two Sum");
    const search = screen.getByLabelText("Search problems");
    await fireEvent.input(search, { target: { value: "trap" } });
    await waitFor(async () => expect((await table()).queryByText("1. Two Sum")).not.toBeInTheDocument());
    expect((await table()).getByText("42. Trapping Rain Water")).toBeInTheDocument();
    await fireEvent.click(screen.getByText("Reset"));
    await (await table()).findByText("1. Two Sum");

    await fireEvent.click(screen.getByText("Blind 75"));
    expect(await screen.findByText("Linked List", { selector: "h3" })).toBeInTheDocument();
    await fireEvent.change(screen.getByLabelText("Difficulty"), { target: { value: "Hard" } });
    expect(await screen.findByText("No problems match these filters.")).toBeInTheDocument();
    await fireEvent.click(screen.getByText("All problems"));
    await fireEvent.change(screen.getByLabelText("Company"), { target: { value: "Google" } });
    await fireEvent.change(screen.getByLabelText("Sort"), { target: { value: "frequency" } });
    await fireEvent.click(screen.getByText("Pick one"));
    await waitFor(() => expect(router.route.name).toBe("problem"));
  });

  it("suggests what to practice next", async () => {
    installBackend();
    const { api } = await import("../lib/api");
    await api.submit(1, "python", "return 1", 90_000);
    await boot();
    const practice = await screen.findByRole("region", { name: "Practice next" });
    expect(within(practice).getByText("Next in Blind 75 · Array", { exact: false })).toBeInTheDocument();
    expect(await within(practice).findByText(/Due for review/)).toBeInTheDocument();
    expect(within(practice).getByText("clean solve, just due for a refresh")).toBeInTheDocument();
  });

  it("still suggests list progress when stats fail", async () => {
    installBackend((cmd) => {
      if (cmd === "get_stats") throw "nope";
    });
    await boot();
    const practice = await screen.findByRole("region", { name: "Practice next" });
    expect(within(practice).queryByText(/Due for review/)).not.toBeInTheDocument();
  });

  it("manages personal lists from the list page and the workspace", async () => {
    const backend = installBackend();
    await boot();
    await (await table()).findByText("1. Two Sum");
    await fireEvent.click(screen.getByText("+ New list"));
    await fireEvent.click(screen.getByText("Cancel"));
    await fireEvent.click(screen.getByText("+ New list"));
    await fireEvent.input(screen.getByLabelText("List name"), { target: { value: "Onsite" } });
    await fireEvent.click(screen.getByText("Create"));
    expect(await screen.findByText("Onsite")).toBeInTheDocument();

    router.go({ name: "problem", id: 42 });
    await screen.findByRole("heading", { name: "42. Trapping Rain Water" });
    await fireEvent.click(screen.getByText("☆ Save"));
    const menu = screen.getByRole("dialog", { name: "Save to lists" });
    await fireEvent.click(within(menu).getByLabelText("Onsite"));
    await waitFor(() => expect(backend.state.userLists[0].problem_ids).toEqual([42]));
    expect(screen.getByText("★ Saved")).toBeInTheDocument();
    await fireEvent.input(within(menu).getByLabelText("New list name"), { target: { value: "Hard ones" } });
    await fireEvent.click(within(menu).getByText("Add"));
    await waitFor(() => expect(backend.state.userLists[1]).toMatchObject({ name: "Hard ones", problem_ids: [42] }));
    await fireEvent.click(within(menu).getByLabelText("Onsite"));
    await waitFor(() => expect(backend.state.userLists[0].problem_ids).toEqual([]));

    router.go({ name: "problems" });
    await table(); // wait for the list page: the workspace's Save menu also says "Hard ones"
    await fireEvent.click(screen.getByText("Hard ones", { selector: "strong" }));
    await waitFor(async () => expect((await table()).queryByText("1. Two Sum")).not.toBeInTheDocument());
    expect((await table()).getByText("42. Trapping Rain Water")).toBeInTheDocument();
    await fireEvent.click(screen.getByLabelText("Delete list Hard ones"));
    await waitFor(() => expect(backend.state.userLists.map((l) => l.name)).toEqual(["Onsite"]));
    await (await table()).findByText("1. Two Sum");
  });

  it("reports list errors", async () => {
    installBackend((cmd) => {
      if (cmd === "create_user_list" || cmd === "set_in_user_list") throw "invalid data: nope";
    });
    await boot();
    await fireEvent.click(await screen.findByText("+ New list"));
    await fireEvent.input(screen.getByLabelText("List name"), { target: { value: "X" } });
    await fireEvent.click(screen.getByText("Create"));
    expect(await screen.findByRole("status")).toHaveTextContent("nope");
    router.go({ name: "problem", id: 1 });
    await screen.findByRole("heading", { name: "1. Two Sum" });
    await fireEvent.click(screen.getByText("☆ Save"));
    expect(screen.getByText(/No lists yet/)).toBeInTheDocument();
    await fireEvent.input(screen.getByLabelText("New list name"), { target: { value: "Y" } });
    await fireEvent.click(screen.getByText("Add"));
  });

  it("restores filters from the session and tolerates corrupt state", async () => {
    installBackend();
    sessionStorage.setItem("lg-filters", JSON.stringify({ query: "valid" }));
    await boot();
    await (await table()).findByText("20. Valid Parentheses");
    expect((await table()).queryByText("1. Two Sum")).not.toBeInTheDocument();
  });

  it("falls back to empty filters when session state is corrupt", async () => {
    installBackend();
    sessionStorage.setItem("lg-filters", "{nope");
    await boot();
    await (await table()).findByText("1. Two Sum");
  });

  it("shows a retry screen when the backend fails", async () => {
    let fail = true;
    installBackend((cmd) => {
      if (cmd === "bootstrap" && fail) throw "problems.db not found";
    });
    await boot();
    await screen.findByText("problems.db not found");
    fail = false;
    await fireEvent.click(screen.getByText("Retry"));
    await (await table()).findByText("1. Two Sum");
  });

  it("works through a problem: run, submit, history", async () => {
    installBackend();
    await boot("#/problems/1");
    await screen.findByRole("heading", { name: "1. Two Sum" });
    await fireEvent.click(screen.getByText("Topics"));
    expect(screen.getAllByText("Hash Table").length).toBeGreaterThan(0);
    await fireEvent.click(screen.getByText("Companies"));
    expect(screen.getByText("Google")).toBeInTheDocument();
    await fireEvent.click(screen.getByText("Hints"));
    expect(screen.getByText(/brute-force approach/)).toBeInTheDocument();

    await fireEvent.click(screen.getByRole("button", { name: "Run" }));
    await screen.findByText("Wrong Answer", { selector: "h3" });
    await fireEvent.click(screen.getByText("Case 2", { selector: "button" }));

    await fireEvent.click(screen.getByRole("button", { name: "Submit" }));
    await screen.findByText("Wrong Answer", { selector: "h2" });
    expect(screen.getByText("Failing testcase #4")).toBeInTheDocument();
    await fireEvent.click(screen.getByText("Use testcase"));
    expect(await screen.findByText("Case 4")).toBeInTheDocument();
    await fireEvent.click(screen.getByText("Case 4"));
    expect(screen.getByLabelText("nums =")).toHaveValue("[3,3]");
    await fireEvent.click(screen.getByText("← All submissions"));
    const row = await screen.findByText("(3/57)", { exact: false });
    await fireEvent.click(row);
    await screen.findByText("Failing testcase #4");

    await fireEvent.click(screen.getByRole("tab", { name: "Notes" }));
    await fireEvent.input(screen.getByLabelText("Notes"), { target: { value: "complement lookup" } });
    expect(screen.getByLabelText("has notes")).toBeInTheDocument();
    await fireEvent.click(screen.getByRole("tab", { name: "Editorial" }));
    expect(await screen.findByText("Approach 2: One-Pass Hash Map (Optimal)")).toBeInTheDocument();
    expect(screen.getByRole("tab", { name: "Python3", selected: true })).toBeInTheDocument();
    await fireEvent.click(screen.getByRole("tab", { name: "C++" }));
    expect(screen.getByRole("tab", { name: "C++", selected: true })).toBeInTheDocument();
    await fireEvent.click(screen.getByRole("tab", { name: "Result" }));
    await fireEvent.click(screen.getByRole("tab", { name: "Description" }));

    await fireEvent.click(screen.getByLabelText("Next problem"));
    await waitFor(() => expect(router.route).toEqual({ name: "problem", id: 3 }));
  });

  it("controls the timer and resets code", async () => {
    const backend = installBackend();
    await boot("#/problems/1");
    await screen.findByRole("heading", { name: "1. Two Sum" });
    await fireEvent.click(screen.getByLabelText("Start timer"));
    await waitFor(() => expect(backend.state.solves).toHaveLength(1));
    await fireEvent.click(screen.getByLabelText("Pause timer"));
    await fireEvent.click(screen.getByLabelText("Reset timer"));
    await waitFor(() => expect(backend.state.solves).toHaveLength(0));
    const languageSelect = screen.getByLabelText("Language");
    expect(languageSelect).toHaveTextContent("Rust (not found)");
    await fireEvent.change(languageSelect, { target: { value: "javascript" } });
    await waitFor(() => expect(backend.state.drafts.size).toBe(0));
    await fireEvent.click(screen.getByLabelText("Reset code"));
    await fireEvent.click(screen.getByText("Test Result"));
    expect(screen.getByText("Run your code to see results here.")).toBeInTheDocument();
  });

  it("shows an error for unknown problems", async () => {
    installBackend();
    await boot("#/problems/4242");
    await screen.findByText(/not found/);
  });

  it("renders statistics after an accepted solve", async () => {
    const backend = installBackend();
    await boot("#/problems/1");
    await screen.findByRole("heading", { name: "1. Two Sum" });
    backend.state.drafts.set("1:python", "return 1");
    // Submit through the API directly to seed history, then view stats.
    const { api } = await import("../lib/api");
    await api.submit(1, "python", "return 1", 90_000);
    router.go({ name: "stats" });
    await screen.findByText("Statistics");
    expect(await screen.findByText("Review queue")).toBeInTheDocument();
    expect(screen.getByText("clean solve, just due for a refresh")).toBeInTheDocument();
    await fireEvent.change(screen.getByLabelText("Sort review queue"), { target: { value: "time" } });
    const table = screen.getByText("By difficulty").closest("section")!;
    expect(within(table).getByText("Easy")).toBeInTheDocument();
    await fireEvent.change(screen.getByLabelText("Recent window"), { target: { value: "5" } });
  });

  it("keeps the newest statistics when an older request finishes last", async () => {
    let release!: () => void;
    const gate = new Promise<void>((r) => (release = r));
    let calls = 0;
    const backend = installBackend((cmd, a) => {
      if (cmd !== "get_stats" || ++calls !== 2) return undefined;
      // The first change's request is slow and carries a marker value.
      return gate.then(async () => {
        const stale = (await backend.handle(cmd, a)) as Stats;
        return { ...stale, totals: { ...stale.totals, longest_streak: 4242 } };
      });
    });
    await boot("#/stats");
    await screen.findByText("Statistics");
    const recent = screen.getByLabelText("Recent window");
    await fireEvent.change(recent, { target: { value: "5" } });
    await fireEvent.change(recent, { target: { value: "6" } });
    await waitFor(() => expect(calls).toBe(3));
    release();
    await new Promise((r) => setTimeout(r, 50));
    await tick();
    expect(screen.getByText("Longest streak")).toBeInTheDocument();
    expect(screen.queryByText(/4242/)).not.toBeInTheDocument();
  });

  it("shows stats errors", async () => {
    installBackend((cmd) => {
      if (cmd === "get_stats") throw "stats exploded";
    });
    await boot("#/stats");
    await screen.findByText("stats exploded");
  });

  it("edits settings and manages data", async () => {
    const backend = installBackend();
    await boot("#/settings");
    await screen.findByRole("heading", { name: "Settings" });
    await fireEvent.change(screen.getByLabelText("Theme"), { target: { value: "light" } });
    await waitFor(() => expect(backend.state.settings.theme).toBe("light"));
    await fireEvent.change(screen.getByLabelText("Key bindings"), { target: { value: "vim" } });
    await waitFor(() => expect(backend.state.settings.keymap).toBe("vim"));
    await fireEvent.change(screen.getByLabelText("Python3 toolchain path"), { target: { value: "/opt/py" } });
    await waitFor(() => expect(backend.state.settings.eval.toolchain_paths.python).toBe("/opt/py"));
    await fireEvent.click(screen.getByLabelText("Low priority judging"));
    await waitFor(() => expect(backend.state.settings.eval.low_priority).toBe(true));

    await fireEvent.click(screen.getByText("Export history…"));
    await screen.findByText("Exported to /tmp/localgrind-history.json");
    await fireEvent.click(screen.getByText("Import…"));
    await screen.findByText(/Imported 2 submissions/);
    backend.state.dialogPath = "/tmp/bad.json";
    await fireEvent.click(screen.getByText("Import…"));
    await screen.findByText(/Import failed/);
    backend.state.dialogPath = null;
    await fireEvent.click(screen.getByText("Import…"));
    await fireEvent.click(screen.getByText("Export history…"));

    backend.state.confirm = false;
    await fireEvent.click(screen.getByText("Clear history…"));
    backend.state.confirm = true;
    await fireEvent.click(screen.getByText("Clear history…"));
    await screen.findByText("History cleared.");
    expect(screen.getByText(/Problem catalog built/)).toBeInTheDocument();
  });

  it("recalibrates benchmarks on this machine", async () => {
    const backend = installBackend();
    await boot("#/settings");
    expect(await screen.findByText("Using the bundled benchmarks.")).toBeInTheDocument();
    await fireEvent.click(screen.getByText("Recalibrate on this machine"));
    expect(await screen.findByText("Measured 4 reference solutions.")).toBeInTheDocument();
    expect(await screen.findByText(/Using 4 benchmarks measured on this machine/)).toBeInTheDocument();
    expect(backend.state.localBenchmarks).toBe(4);
    await fireEvent.click(screen.getByText("Use bundled benchmarks"));
    expect(await screen.findByText("Using the bundled benchmarks again.")).toBeInTheDocument();
  });

  it("can stop a recalibration and reports failures", async () => {
    let release!: () => void;
    const gate = new Promise<void>((r) => (release = r));
    const backend = installBackend((cmd) => {
      if (cmd === "recalibrate_benchmarks") return gate.then(() => backend.handle(cmd, { runs: 3 }));
    });
    await boot("#/settings");
    await fireEvent.click(await screen.findByText("Recalibrate on this machine"));
    await fireEvent.click(await screen.findByText("Stop"));
    release();
    expect(await screen.findByText("Stopped after 1 reference solutions.")).toBeInTheDocument();
  });

  it("shows recalibration errors", async () => {
    installBackend((cmd) => {
      if (cmd === "recalibrate_benchmarks") throw "no toolchains";
      if (cmd === "calibration_status") throw "db error";
    });
    await boot("#/settings");
    await fireEvent.click(await screen.findByText("Recalibrate on this machine"));
    expect(await screen.findByText("Recalibration failed: no toolchains")).toBeInTheDocument();
  });

  it("reports export failures", async () => {
    installBackend((cmd) => {
      if (cmd === "export_history") throw "permission denied";
    });
    await boot("#/settings");
    await screen.findByRole("heading", { name: "Settings" });
    await fireEvent.click(screen.getByText("Export history…"));
    await screen.findByText("permission denied");
  });

  it("warns when no toolchain is available and shows probe results", async () => {
    const backend = installBackend((cmd) => {
      if (cmd === "check_toolchains") {
        return [{ language: "python", executable: "/nope/python3", version: null, ok: false, message: "python3 was not found" }];
      }
    });
    const boot = backend.handle;
    backend.handle = async (cmd, args) => {
      const r = (await boot(cmd, args)) as Record<string, unknown>;
      if (cmd === "bootstrap") r.languages = [{ id: "python", name: "Python3", available: false, executable: "python3" }];
      return r;
    };
    await boot_("#/settings");
    expect(await screen.findByRole("alert")).toHaveTextContent("No language toolchain was found");
    expect(await screen.findByText("python3 was not found")).toBeInTheDocument();
  });

  it("reports toolchain probe failures", async () => {
    installBackend((cmd) => {
      if (cmd === "check_toolchains") throw "probe crashed";
    });
    await boot_("#/settings");
    expect(await screen.findByText("probe crashed")).toBeInTheDocument();
    expect(screen.queryByRole("alert")).not.toBeInTheDocument();
  });

  it("supports keyboard navigation and a shortcuts overlay", async () => {
    installBackend();
    await boot();
    await (await table()).findByText("1. Two Sum");
    await fireEvent.keyDown(window, { key: "?" });
    const dialog = await screen.findByRole("dialog", { name: "Keyboard shortcuts" });
    expect(within(dialog).getByText("Search problems")).toBeInTheDocument();
    expect(document.activeElement).toBe(dialog);
    // Page shortcuts don't fire behind the modal overlay.
    await fireEvent.keyDown(window, { key: "/" });
    expect(document.activeElement).toBe(dialog);
    await fireEvent.keyDown(dialog, { key: "Escape" });
    expect(screen.queryByRole("dialog")).not.toBeInTheDocument();
    await fireEvent.click(screen.getByLabelText("Keyboard shortcuts"));
    await fireEvent.click(within(screen.getByRole("dialog")).getByLabelText("Close"));

    await fireEvent.keyDown(window, { key: "/" });
    expect(document.activeElement).toBe(screen.getByLabelText("Search problems"));
    await fireEvent.keyDown(document.activeElement!, { key: "j" });
    // Plain keys are ignored while typing in an input.
    expect(document.activeElement).toBe(screen.getByLabelText("Search problems"));
    (document.activeElement as HTMLElement).blur();
    await fireEvent.keyDown(window, { key: "j" });
    expect(document.activeElement?.textContent).toContain("1. Two Sum");
    await fireEvent.keyDown(window, { key: "j" });
    await fireEvent.keyDown(window, { key: "k" });
    expect(document.activeElement?.textContent).toContain("1. Two Sum");
    (document.activeElement as HTMLElement).blur();
    await fireEvent.keyDown(window, { key: "k" });
    expect(document.activeElement?.textContent).toContain("206.");

    await fireEvent.keyDown(window, { key: "2", metaKey: true });
    await waitFor(() => expect(router.route).toEqual({ name: "stats" }));
    // Outside the problem list the arrow keys keep scrolling.
    expect(await fireEvent.keyDown(window, { key: "ArrowDown" })).toBe(true);
    await fireEvent.keyDown(window, { key: ",", ctrlKey: true });
    await waitFor(() => expect(router.route).toEqual({ name: "settings" }));
    await fireEvent.keyDown(window, { key: "1", metaKey: true });
    await waitFor(() => expect(router.route).toEqual({ name: "problems" }));
    await fireEvent.keyDown(window, { key: "r" });
    await waitFor(() => expect(router.route.name).toBe("problem"));
  });

  it("runs, submits and navigates problems from the keyboard", async () => {
    const backend = installBackend();
    await boot("#/problems/1");
    await screen.findByRole("heading", { name: "1. Two Sum" });
    await fireEvent.keyDown(window, { key: "'", metaKey: true });
    await screen.findByText("Wrong Answer", { selector: "h3" });
    await fireEvent.keyDown(window, { key: "Enter", metaKey: true });
    await waitFor(() => expect(backend.state.submissions).toHaveLength(1));
    await fireEvent.keyDown(window, { key: "]" });
    await waitFor(() => expect(router.route).toEqual({ name: "problem", id: 3 }));
    await fireEvent.keyDown(window, { key: "{", metaKey: true, shiftKey: true });
    await waitFor(() => expect(router.route).toEqual({ name: "problem", id: 1 }));
    // Handled-by-editor events are not processed twice.
    const handled = new KeyboardEvent("keydown", { key: "]", bubbles: true, cancelable: true });
    handled.preventDefault();
    window.dispatchEvent(handled);
    expect(router.route).toEqual({ name: "problem", id: 1 });
  });

  it("toggles the theme from the top bar", async () => {
    const backend = installBackend();
    await boot();
    await (await table()).findByText("1. Two Sum");
    await fireEvent.click(screen.getByLabelText("Toggle theme"));
    await waitFor(() => expect(backend.state.settings.theme).toBe("light"));
  });

  it("keeps the settings page in sync with the top-bar theme toggle", async () => {
    const backend = installBackend();
    await boot("#/settings");
    const theme = (await screen.findByLabelText("Theme")) as HTMLSelectElement;
    await waitFor(() => expect(theme.value).toBe("dark"));
    await fireEvent.click(screen.getByLabelText("Toggle theme"));
    await waitFor(() => expect(theme.value).toBe("light"));
    // A later edit on the page must not write the old theme back.
    await fireEvent.change(screen.getByLabelText("Key bindings"), { target: { value: "vim" } });
    await waitFor(() => expect(backend.state.settings.keymap).toBe("vim"));
    expect(backend.state.settings.theme).toBe("light");
  });

  it("reports failures to clear history or revert benchmarks", async () => {
    installBackend((cmd) => {
      if (cmd === "clear_history") throw "db locked";
      if (cmd === "clear_local_benchmarks") throw "db busy";
    });
    await boot("#/settings");
    await fireEvent.click(await screen.findByText("Recalibrate on this machine"));
    await fireEvent.click(await screen.findByText("Use bundled benchmarks"));
    expect(await screen.findByText("db busy")).toBeInTheDocument();
    await fireEvent.click(screen.getByText("Clear history…"));
    expect(await screen.findByText("db locked")).toBeInTheDocument();
  });
});
