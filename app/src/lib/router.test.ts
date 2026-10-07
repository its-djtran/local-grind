import { describe, expect, it } from "vitest";
import { hrefFor, parseHash, router } from "./router.svelte";

describe("router", () => {
  it("parses hashes", () => {
    expect(parseHash("")).toEqual({ name: "problems" });
    expect(parseHash("#/problems/42")).toEqual({ name: "problem", id: 42 });
    expect(parseHash("#/problems/abc")).toEqual({ name: "problems" });
    expect(parseHash("#/stats")).toEqual({ name: "stats" });
    expect(parseHash("#settings")).toEqual({ name: "settings" });
    expect(parseHash("#/nope")).toEqual({ name: "problems" });
  });

  it("builds hrefs", () => {
    expect(hrefFor({ name: "problem", id: 7 })).toBe("#/problems/7");
    expect(hrefFor({ name: "problems" })).toBe("#/problems");
    expect(hrefFor({ name: "stats" })).toBe("#/stats");
  });

  it("navigates and syncs", () => {
    router.go({ name: "stats" });
    expect(location.hash).toBe("#/stats");
    expect(router.route).toEqual({ name: "stats" });
    location.hash = "#/problems/9";
    router.sync();
    expect(router.route).toEqual({ name: "problem", id: 9 });
  });
});
