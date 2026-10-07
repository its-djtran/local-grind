import { describe, expect, it } from "vitest";
import {
  compareToReference,
  formatClock,
  formatDate,
  formatDateTime,
  formatDuration,
  formatMemory,
  formatRelative,
  formatRuntime,
  formatValue,
  percent,
  verdictLabel,
} from "./format";

describe("format", () => {
  it("formats runtimes", () => {
    expect(formatRuntime(0)).toBe("0 ms");
    expect(formatRuntime(0.4)).toBe("<1 ms");
    expect(formatRuntime(12.4)).toBe("12 ms");
    expect(formatRuntime(1234)).toBe("1.23 s");
    expect(formatRuntime(-1)).toBe("—");
    expect(formatRuntime(NaN)).toBe("—");
  });

  it("formats memory", () => {
    expect(formatMemory(512)).toBe("512 KB");
    expect(formatMemory(16_800)).toBe("16.4 MB");
    expect(formatMemory(3 * 1024 * 1024)).toBe("3.00 GB");
    expect(formatMemory(-5)).toBe("—");
  });

  it("formats clocks and durations", () => {
    expect(formatClock(0)).toBe("00:00");
    expect(formatClock(65_000)).toBe("01:05");
    expect(formatClock(3_725_000)).toBe("1:02:05");
    expect(formatClock(-10)).toBe("00:00");
    expect(formatDuration(45_000)).toBe("45s");
    expect(formatDuration(725_000)).toBe("12m 5s");
    expect(formatDuration(3_780_000)).toBe("1h 3m");
    expect(formatDuration(-1)).toBe("0s");
  });

  it("formats relative times", () => {
    const now = Date.UTC(2026, 9, 7, 12);
    expect(formatRelative(now - 10_000, now)).toBe("just now");
    expect(formatRelative(now - 5 * 60_000, now)).toBe("5 min ago");
    expect(formatRelative(now - 3 * 3_600_000, now)).toBe("3 hr ago");
    expect(formatRelative(now - 86_400_000, now)).toBe("1 day ago");
    expect(formatRelative(now - 4 * 86_400_000, now)).toBe("4 days ago");
    expect(formatRelative(now - 60 * 86_400_000, now)).toBe(formatDate(now - 60 * 86_400_000));
    expect(formatRelative(Date.now() - 1000)).toBe("just now");
  });

  it("formats dates", () => {
    const ts = new Date(2026, 0, 5, 9, 7).getTime();
    expect(formatDate(ts)).toBe("2026-01-05");
    expect(formatDateTime(ts)).toBe("2026-01-05 09:07");
  });

  it("labels verdicts and values", () => {
    expect(verdictLabel("WrongAnswer")).toBe("Wrong Answer");
    expect(verdictLabel("Bogus" as never)).toBe("Bogus");
    expect(formatValue([1, 2])).toBe("[1,2]");
    expect(formatValue("a")).toBe('"a"');
    expect(formatValue(undefined)).toBe("");
    expect(percent(12.345, 1)).toBe("12.3%");
    expect(percent(NaN)).toBe("—");
  });

  it("compares with references", () => {
    expect(compareToReference(10, 10)).toBe("on par with or better than the reference");
    expect(compareToReference(18, 10)).toBe("1.8x the reference");
    expect(compareToReference(1, 0)).toBe("");
    expect(compareToReference(NaN, 3)).toBe("");
  });
});
