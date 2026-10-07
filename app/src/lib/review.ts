import type { Aggregate, ReviewItem } from "./types";

export type ReviewSort = "score" | "passRate" | "time" | "stale";

export function sortReview(items: ReviewItem[], mode: ReviewSort): ReviewItem[] {
  const out = [...items];
  const byScore = (a: ReviewItem, b: ReviewItem) => b.score - a.score;
  switch (mode) {
    case "passRate":
      return out.sort((a, b) => a.pass_rate - b.pass_rate || byScore(a, b));
    case "time":
      return out.sort((a, b) => b.avg_ms - a.avg_ms || byScore(a, b));
    case "stale":
      return out.sort((a, b) => b.days_since - a.days_since || byScore(a, b));
    default:
      return out.sort((a, b) => byScore(a, b) || a.pass_rate - b.pass_rate);
  }
}

export function severity(score: number): "high" | "med" | "low" {
  if (score >= 50) return "high";
  if (score >= 25) return "med";
  return "low";
}

/**
 * Change of a recent metric against the overall value, e.g. "12% faster". `lowerIsBetter`
 * decides whether a decrease is good.
 */
export function trend(
  recent: number,
  overall: number,
  lowerIsBetter: boolean,
  words: [better: string, worse: string],
): { text: string; tone: "ok" | "bad" | "muted" } {
  if (!(overall > 0) || !Number.isFinite(recent)) return { text: "—", tone: "muted" };
  const pct = ((recent - overall) / overall) * 100;
  if (Math.abs(pct) < 1) return { text: "steady", tone: "muted" };
  const better = lowerIsBetter ? pct < 0 : pct > 0;
  return { text: `${Math.abs(pct).toFixed(0)}% ${better ? words[0] : words[1]}`, tone: better ? "ok" : "bad" };
}

export function hasData(a: Aggregate): boolean {
  return a.solves > 0;
}
