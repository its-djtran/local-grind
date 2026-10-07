import { describe, expect, it } from "vitest";
import { buildCalendar, level, monthLabels } from "./calendar";

describe("calendar", () => {
  it("quantizes levels", () => {
    expect(level(0, 10)).toBe(0);
    expect(level(1, 0)).toBe(0);
    expect(level(1, 10)).toBe(1);
    expect(level(5, 10)).toBe(2);
    expect(level(10, 10)).toBe(4);
    expect(level(30, 10)).toBe(4);
  });

  it("lays out weeks ending today", () => {
    const today = new Date(2026, 9, 7); // Wednesday
    const cols = buildCalendar([{ date: "2026-10-07", submissions: 4, accepted: 1 }, { date: "2026-10-05", submissions: 2, accepted: 2 }], today, 3);
    expect(cols).toHaveLength(3);
    expect(cols[0][0].date).toBe("2026-09-20");
    expect(cols[2]).toHaveLength(4);
    expect(cols[2][3]).toMatchObject({ date: "2026-10-07", submissions: 4, level: 4 });
    expect(cols[2][1]).toMatchObject({ date: "2026-10-05", level: 2, accepted: 2 });
    expect(buildCalendar([], today).length).toBe(53);
  });

  it("labels months", () => {
    const cols = buildCalendar([], new Date(2026, 9, 7), 10);
    const labels = monthLabels(cols).map((m) => m.label);
    expect(labels).toEqual(["Aug", "Sep", "Oct"]);
    expect(monthLabels([[]])).toEqual([]);
  });
});
