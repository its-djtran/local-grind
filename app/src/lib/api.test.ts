import { describe, expect, it } from "vitest";
import { emit } from "@tauri-apps/api/event";
import { api, errorMessage } from "./api";
import { installBackend, tick } from "../test/backend";

describe("api", () => {
  it("normalizes errors", () => {
    expect(errorMessage("x")).toBe("x");
    expect(errorMessage(new Error("y"))).toBe("y");
    expect(errorMessage({ a: 1 })).toBe('{"a":1}');
  });

  it("wraps every command", async () => {
    installBackend();
    expect((await api.settings()).theme).toBe("dark");
    expect(await api.solves()).toEqual([]);
    expect(await api.referenceSolution(1, "python")).toContain("Solution");
    expect(await api.referenceSolution(2, "python")).toBeNull();
    const seen: number[] = [];
    const un = await api.onSubmitProgress((p) => seen.push(p.done));
    await emit("submit-progress", { problem_id: 1, done: 5, total: 9 });
    await tick();
    un();
    expect(seen).toEqual([5]);
  });
});
