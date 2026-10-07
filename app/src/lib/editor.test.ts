import { describe, expect, it } from "vitest";
import { loadKeymap, loadLanguage } from "./editor";

const nonEmpty = (ext: unknown) => (Array.isArray(ext) ? ext.length > 0 : Boolean(ext));

describe("lazy editor extensions", () => {
  it("loads every language pack and the vim keymap on demand", async () => {
    for (const lang of ["python", "javascript", "cpp", "rust"] as const) {
      expect(nonEmpty(await loadLanguage(lang))).toBe(true);
    }
    expect(await loadKeymap("standard")).toEqual([]);
    expect(nonEmpty(await loadKeymap("vim"))).toBe(true);
  });
});
