import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/svelte";
import SubmissionDetail from "./SubmissionDetail.svelte";
import type { Signature, Submission } from "../lib/types";

const sig: Signature = { kind: "function", name: "f", params: [{ name: "nums", type: "int[]" }], returns: "int" };
const base: Submission = {
  id: 1,
  problem_id: 1,
  language: "python",
  code: "class Solution: pass",
  verdict: "Accepted",
  passed: 10,
  total: 10,
  runtime_ms: 20,
  memory_kb: 20_000,
  error: null,
  failure: null,
  created_at: Date.now(),
};

describe("SubmissionDetail", () => {
  it("compares an accepted run with the reference and shows solve time", async () => {
    const onBack = vi.fn();
    render(SubmissionDetail, {
      submission: base,
      signature: sig,
      benchmark: { runtime_ms: 10, memory_kb: 20_000, runs: 10, local: false },
      solve: { id: 1, problem_id: 1, slug: "a", title: "A", difficulty: "Easy", started_at: 0, completed_at: 5, elapsed_ms: 125_000, errors: 1, submission_id: 1 },
      onBack,
    });
    expect(screen.getByText("Accepted")).toBeInTheDocument();
    expect(screen.getByText(/2.0x the reference/)).toBeInTheDocument();
    expect(screen.queryByText(/on this machine/)).not.toBeInTheDocument();
    expect(screen.getByText(/on par with or better/)).toBeInTheDocument();
    expect(screen.getByText("02:05")).toBeInTheDocument();
    expect(screen.getByText("1 wrong submission before accepting")).toBeInTheDocument();
    await fireEvent.click(screen.getByText("← All submissions"));
    expect(onBack).toHaveBeenCalled();
  });

  it("shows errors and failing cases without a back button", () => {
    render(SubmissionDetail, {
      submission: {
        ...base,
        verdict: "RuntimeError",
        error: "IndexError",
        failure: { index: 0, passed: false, args: [[1]], expected: 3, output: null, stdout: "", runtime_ns: 0 },
      },
      signature: sig,
    });
    expect(screen.getByText("Runtime Error")).toBeInTheDocument();
    expect(screen.getByText("IndexError")).toBeInTheDocument();
    expect(screen.getByText("Failing testcase #1")).toBeInTheDocument();
    expect(screen.queryByText("Output")).not.toBeInTheDocument();
    expect(screen.queryByText("← All submissions")).not.toBeInTheDocument();
  });

  it("labels benchmarks measured locally", () => {
    render(SubmissionDetail, { submission: base, signature: sig, benchmark: { runtime_ms: 10, memory_kb: 1, runs: 3, local: true } });
    expect(screen.getByText(/avg of 3 runs on this machine/)).toBeInTheDocument();
  });

  it("pluralizes and handles accepted runs without a benchmark", () => {
    render(SubmissionDetail, {
      submission: base,
      signature: { kind: "design", class: "C", constructor: [], methods: [] },
      solve: { id: 1, problem_id: 1, slug: "a", title: "A", difficulty: "Easy", started_at: 0, completed_at: 5, elapsed_ms: 1000, errors: 2, submission_id: 1 },
    });
    expect(screen.getByText("2 wrong submissions before accepting")).toBeInTheDocument();
    expect(screen.queryByText(/reference/)).not.toBeInTheDocument();
  });
});
