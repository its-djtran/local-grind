import { describe, expect, it } from "vitest";
import { hasData, severity, sortReview, trend } from "./review";
import type { ReviewItem } from "./types";

const item = (slug: string, score: number, pass_rate: number, avg_ms: number, days_since: number): ReviewItem => ({
  slug,
  problem_id: 1,
  title: slug,
  difficulty: "Easy",
  solves: 1,
  errors: 0,
  pass_rate,
  avg_ms,
  days_since,
  time_ratio: 1,
  score,
  reasons: [],
});

describe("review", () => {
  const items = [item("a", 10, 0.5, 100, 3), item("b", 50, 1, 300, 1), item("c", 30, 0.2, 200, 9), item("d", 30, 0.9, 50, 2)];
  it("sorts by each mode", () => {
    const order = (m: Parameters<typeof sortReview>[1]) => sortReview(items, m).map((i) => i.slug);
    expect(order("score")).toEqual(["b", "c", "d", "a"]);
    expect(order("passRate")).toEqual(["c", "a", "d", "b"]);
    expect(order("time")).toEqual(["b", "c", "a", "d"]);
    expect(order("stale")).toEqual(["c", "a", "d", "b"]);
  });

  it("buckets severity", () => {
    expect([severity(60), severity(30), severity(5)]).toEqual(["high", "med", "low"]);
  });

  it("describes trends", () => {
    expect(trend(90, 100, true, ["faster", "slower"])).toEqual({ text: "10% faster", tone: "ok" });
    expect(trend(120, 100, true, ["faster", "slower"])).toEqual({ text: "20% slower", tone: "bad" });
    expect(trend(120, 100, false, ["up", "down"])).toEqual({ text: "20% up", tone: "ok" });
    expect(trend(100.5, 100, true, ["a", "b"]).text).toBe("steady");
    expect(trend(5, 0, true, ["a", "b"]).tone).toBe("muted");
    expect(hasData({ solves: 0, avg_ms: 0, min_ms: 0, max_ms: 0, errors: 0, error_rate: 0 })).toBe(false);
  });
});
