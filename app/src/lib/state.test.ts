import { describe, expect, it } from "vitest";
import { installBackend, tick } from "../test/backend";
import { app, applyAppearance, followSystemTheme, resolveTheme } from "./state.svelte";
import { defaultSettings } from "../mock/backend";
import type { Settings } from "./types";

describe("app state", () => {
  it("resolves themes", () => {
    expect(resolveTheme("system", true)).toBe("dark");
    expect(resolveTheme("system", false)).toBe("light");
    expect(resolveTheme("light", true)).toBe("light");
  });

  it("applies appearance variables", () => {
    const el = document.createElement("div");
    applyAppearance({ ...defaultSettings(), theme: "light", font_size: 18, font_family: "Fira Code" }, el);
    expect(el.dataset.theme).toBe("light");
    expect(el.style.getPropertyValue("--font-size")).toBe("18px");
    expect(el.style.getPropertyValue("--font-family")).toContain("Fira Code");
    applyAppearance({ ...defaultSettings(), theme: "system" });
    expect(document.documentElement.dataset.theme).toBe("dark");
  });

  it("follows OS light/dark changes only while the theme is system", () => {
    let dark = true;
    let listener: (() => void) | null = null;
    const query = {
      get matches() {
        return dark;
      },
      addEventListener: (_: string, l: () => void) => (listener = l),
      removeEventListener: () => (listener = null),
    } as unknown as MediaQueryList;
    const original = window.matchMedia;
    window.matchMedia = () => query;
    try {
      let settings: Settings = { ...defaultSettings(), theme: "system" };
      const stop = followSystemTheme(() => settings, query);
      dark = false;
      listener!();
      expect(document.documentElement.dataset.theme).toBe("light");
      settings = { ...settings, theme: "dark" };
      document.documentElement.dataset.theme = "dark";
      dark = true;
      listener!();
      dark = false;
      listener!();
      expect(document.documentElement.dataset.theme).toBe("dark");
      stop();
      expect(listener).toBeNull();
    } finally {
      window.matchMedia = original;
    }
  });

  it("loads, saves settings and toggles theme", async () => {
    installBackend();
    await app.load();
    expect(app.boot?.problems.length).toBeGreaterThan(1);
    expect(app.error).toBeNull();
    await app.toggleTheme();
    expect(app.settings?.theme).toBe("light");
    await app.toggleTheme();
    expect(app.settings?.theme).toBe("dark");
    await app.saveSettings({ ...app.settings!, font_size: 99 });
    expect(app.settings?.font_size).toBe(32);
  });

  it("reports load and save failures", async () => {
    installBackend((cmd) => {
      if (cmd === "bootstrap") throw "catalog missing";
    });
    await app.load();
    expect(app.error).toBe("catalog missing");
    installBackend((cmd) => {
      if (cmd === "save_settings") throw "disk full";
    });
    await app.load();
    await app.saveSettings(defaultSettings());
    expect(app.toast).toBe("disk full");
  });

  it("tracks problem status with sticky solves", async () => {
    installBackend();
    await app.load();
    app.setStatus(1, "attempted");
    expect(app.boot!.problems.find((p) => p.id === 1)!.status).toBe("attempted");
    app.setStatus(1, "solved");
    app.setStatus(1, "attempted");
    expect(app.boot!.problems.find((p) => p.id === 1)!.status).toBe("solved");
    app.setStatus(12345, "solved");
  });

  it("shows and clears toasts", async () => {
    app.notify("hello", 5);
    expect(app.toast).toBe("hello");
    await tick(20);
    expect(app.toast).toBeNull();
  });

  it("ignores theme toggles before load", async () => {
    app.settings = null;
    await app.toggleTheme();
    expect(app.settings).toBeNull();
  });
});
