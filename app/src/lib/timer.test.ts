import { describe, expect, it } from "vitest";
import { SolveTimer } from "./timer";

describe("SolveTimer", () => {
  it("accumulates running time only", () => {
    let now = 1000;
    const t = new SolveTimer(500, () => now);
    expect(t.running).toBe(false);
    expect(t.elapsed()).toBe(500);
    t.start();
    t.start();
    now += 2000;
    expect(t.elapsed()).toBe(2500);
    t.pause();
    t.pause();
    now += 5000;
    expect(t.elapsed()).toBe(2500);
    t.reset(100);
    expect(t.elapsed()).toBe(100);
    t.reset(-5);
    expect(t.elapsed()).toBe(0);
    expect(new SolveTimer(-3).elapsed()).toBe(0);
    expect(new SolveTimer().elapsed()).toBe(0);
  });
});
