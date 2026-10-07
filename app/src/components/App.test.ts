import { describe, expect, it } from "vitest";
import { fireEvent, render, screen, waitFor, within } from "@testing-library/svelte";
import App from "../App.svelte";
import { installBackend } from "../test/backend";
import { router } from "../lib/router.svelte";

async function boot(hash = "#/problems") {
  location.hash = hash;
  router.sync();
  render(App);
  await screen.findByLabelText("Local Grind home");
}

describe("App", () => {
  it("shows the problem list and filters it", async () => {
    installBackend();
    await boot();
    await screen.findByText("1. Two Sum");
    const search = screen.getByLabelText("Search problems");
    await fireEvent.input(search, { target: { value: "trap" } });
    await waitFor(() => expect(screen.queryByText("1. Two Sum")).not.toBeInTheDocument());
    expect(screen.getByText("42. Trapping Rain Water")).toBeInTheDocument();
    await fireEvent.click(screen.getByText("Reset"));
    await screen.findByText("1. Two Sum");

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

  it("restores filters from the session and tolerates corrupt state", async () => {
    installBackend();
    sessionStorage.setItem("lg-filters", JSON.stringify({ query: "valid" }));
    await boot();
    await screen.findByText("20. Valid Parentheses");
    expect(screen.queryByText("1. Two Sum")).not.toBeInTheDocument();
  });

  it("falls back to empty filters when session state is corrupt", async () => {
    installBackend();
    sessionStorage.setItem("lg-filters", "{nope");
    await boot();
    await screen.findByText("1. Two Sum");
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
    await screen.findByText("1. Two Sum");
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
    await fireEvent.click(screen.getByText("← All submissions"));
    const row = await screen.findByText("(3/57)", { exact: false });
    await fireEvent.click(row);
    await screen.findByText("Failing testcase #4");

    await fireEvent.click(screen.getByRole("tab", { name: "Editorial" }));
    expect(await screen.findByText("Approach 2: One-Pass Hash Map (Optimal)")).toBeInTheDocument();
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

  it("reports export failures", async () => {
    installBackend((cmd) => {
      if (cmd === "export_history") throw "permission denied";
    });
    await boot("#/settings");
    await screen.findByRole("heading", { name: "Settings" });
    await fireEvent.click(screen.getByText("Export history…"));
    await screen.findByText("permission denied");
  });

  it("toggles the theme from the top bar", async () => {
    const backend = installBackend();
    await boot();
    await screen.findByText("1. Two Sum");
    await fireEvent.click(screen.getByLabelText("Toggle theme"));
    await waitFor(() => expect(backend.state.settings.theme).toBe("light"));
  });
});
