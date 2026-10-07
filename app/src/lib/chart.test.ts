import { describe, expect, it } from "vitest";
import { areaPath, labelIndices, linePath, nearestIndex, niceScale, segments, shortDate, ticks } from "./chart";

describe("chart helpers", () => {
  it("picks nice scales", () => {
    expect(niceScale(0)).toEqual({ max: 1, step: 0.25 });
    expect(niceScale(NaN)).toEqual({ max: 1, step: 0.25 });
    expect(niceScale(37)).toEqual({ max: 40, step: 10 });
    expect(niceScale(7)).toEqual({ max: 8, step: 2 });
    expect(niceScale(950_000)).toEqual({ max: 1_000_000, step: 500_000 });
    expect(niceScale(3)).toEqual({ max: 3, step: 1 });
    expect(ticks(40, 10)).toEqual([0, 10, 20, 30, 40]);
    expect(ticks(0.6, 0.2)).toEqual([0, 0.2, 0.4, 0.6]);
  });

  it("splits runs at gaps and builds paths", () => {
    const runs = segments([
      { label: "a", value: 1 },
      { label: "b", value: null },
      { label: "c", value: 2 },
      { label: "d", value: 3 },
      { label: "e", value: Infinity },
    ]);
    expect(runs).toEqual([[{ index: 0, value: 1 }], [{ index: 2, value: 2 }, { index: 3, value: 3 }]]);
    const x = (i: number) => i * 10;
    const y = (v: number) => 100 - v;
    expect(linePath(runs[1], x, y)).toBe("M20.0,98.0L30.0,97.0");
    expect(areaPath(runs[1], x, y, 100)).toBe("M20.0,98.0L30.0,97.0L30.0,100L20.0,100Z");
    expect(areaPath([], x, y, 100)).toBe("");
    expect(segments([])).toEqual([]);
  });

  it("finds the nearest point", () => {
    expect(nearestIndex(0, 10, 100, 5)).toBe(0);
    expect(nearestIndex(60, 10, 100, 5)).toBe(2);
    expect(nearestIndex(500, 10, 100, 5)).toBe(4);
    expect(nearestIndex(50, 10, 100, 1)).toBe(0);
  });

  it("formats dates and spaces labels", () => {
    expect(shortDate("2026-03-02")).toBe("Mar 2");
    expect(shortDate("bad")).toBe("bad");
    expect(labelIndices(0)).toEqual([]);
    expect(labelIndices(4)).toEqual([0, 1, 2, 3]);
    expect(labelIndices(26)).toEqual([0, 5, 10, 15, 20, 25]);
  });
});
