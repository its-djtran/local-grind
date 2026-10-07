import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import Console from "./Console.svelte";
import type { RunOutcome } from "../lib/types";

const outcome = (over: Partial<RunOutcome> = {}): RunOutcome => ({
  verdict: "Accepted",
  passed: 2,
  total: 2,
  runtime_ms: 3,
  memory_kb: 1,
  cases: [
    { index: 0, passed: true, args: [[1], 2], expected: 3, output: 3, stdout: "hi\n", runtime_ns: 1 },
    { index: 1, passed: false, args: [[4], 5], expected: 9, output: 8, stdout: "", runtime_ns: 1 },
  ],
  failure: null,
  error: null,
  stderr: "",
  expected_available: true,
  ...over,
});

function setup(over: Record<string, unknown> = {}) {
  const props = {
    names: ["nums", "target"],
    cases: [["[1]", "2"], ["[3]", "4"]],
    result: null,
    running: false,
    tab: "testcase" as const,
    error: null,
    onTab: vi.fn(),
    onCasesChange: vi.fn(),
    onResetCases: vi.fn(),
    ...over,
  };
  render(Console, props);
  return props;
}

describe("Console", () => {
  it("edits, adds and removes cases", async () => {
    const p = setup();
    await fireEvent.input(screen.getAllByRole("textbox")[0], { target: { value: "[7]" } });
    expect(p.onCasesChange).toHaveBeenLastCalledWith([["[7]", "2"], ["[3]", "4"]]);
    await fireEvent.click(screen.getByLabelText("Add testcase"));
    expect(p.onCasesChange).toHaveBeenLastCalledWith([["[1]", "2"], ["[3]", "4"], ["[1]", "2"]]);
    await fireEvent.click(screen.getByText("Case 2"));
    await fireEvent.click(screen.getByLabelText("Remove case 2"));
    expect(p.onCasesChange).toHaveBeenLastCalledWith([["[1]", "2"]]);
    await fireEvent.click(screen.getByText("Reset"));
    expect(p.onResetCases).toHaveBeenCalled();
    await fireEvent.click(screen.getByRole("tab", { name: "Test Result" }));
    expect(p.onTab).toHaveBeenCalledWith("result");
  });

  it("does not remove the last case", async () => {
    setup({ cases: [["[1]", "2"]] });
    expect(screen.queryByLabelText("Remove case 1")).not.toBeInTheDocument();
  });

  it("adds a blank case when none exist", async () => {
    const p = setup({ cases: [] });
    await fireEvent.click(screen.getByLabelText("Add testcase"));
    expect(p.onCasesChange).toHaveBeenLastCalledWith([["", ""]]);
  });

  it("shows run states", () => {
    setup({ tab: "result", running: true });
    expect(screen.getByText("Running…")).toBeInTheDocument();
  });

  it("shows run errors", () => {
    setup({ tab: "result", error: "Case 1: nums is empty" });
    expect(screen.getByText("Case 1: nums is empty")).toBeInTheDocument();
  });

  it("shows results per case", async () => {
    setup({ tab: "result", result: outcome({ stderr: "warn" }) });
    expect(screen.getByText("Accepted")).toBeInTheDocument();
    expect(screen.getByText("Runtime: 3 ms")).toBeInTheDocument();
    expect(screen.getByText("hi")).toBeInTheDocument();
    expect(screen.getByText("warn")).toBeInTheDocument();
    await fireEvent.click(screen.getByText("Case 2"));
    expect(screen.getByText("8")).toBeInTheDocument();
    expect(screen.getByText("9")).toBeInTheDocument();
  });

  it("says Finished when outputs could not be judged", () => {
    setup({ tab: "result", result: outcome({ expected_available: false }) });
    expect(screen.getByText("Finished")).toBeInTheDocument();
    expect(screen.queryByText("Accepted")).not.toBeInTheDocument();
  });

  it("hides expectations when no reference is available and shows errors", () => {
    setup({ tab: "result", result: outcome({ verdict: "RuntimeError", expected_available: false, error: "boom", cases: [] }) });
    expect(screen.getByText("Runtime Error")).toBeInTheDocument();
    expect(screen.getByText("boom")).toBeInTheDocument();
    expect(screen.queryByText("Expected")).not.toBeInTheDocument();
  });
});
