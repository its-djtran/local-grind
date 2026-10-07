import { api, errorMessage } from "./api";
import type { Bootstrap, Settings, Status, UserList } from "./types";

/** Resolves the effective theme, following the OS when set to "system". */
export function resolveTheme(theme: Settings["theme"], prefersDark: boolean): "dark" | "light" {
  if (theme === "system") return prefersDark ? "dark" : "light";
  return theme;
}

export function applyAppearance(s: Settings, root: HTMLElement = document.documentElement): void {
  const prefersDark =
    typeof window !== "undefined" && typeof window.matchMedia === "function"
      ? window.matchMedia("(prefers-color-scheme: dark)").matches
      : true;
  root.dataset.theme = resolveTheme(s.theme, prefersDark);
  root.style.setProperty("--font-size", `${s.font_size}px`);
  root.style.setProperty("--editor-font-size", `${s.editor_font_size}px`);
  root.style.setProperty("--font-family", `"${s.font_family}", "Inconsolata", ui-monospace, Menlo, monospace`);
}

class AppState {
  boot = $state<Bootstrap | null>(null);
  settings = $state<Settings | null>(null);
  error = $state<string | null>(null);
  toast = $state<string | null>(null);
  private toastTimer: ReturnType<typeof setTimeout> | undefined;

  async load(): Promise<void> {
    try {
      const boot = await api.bootstrap();
      this.boot = boot;
      this.settings = boot.settings;
      applyAppearance(boot.settings);
      this.error = null;
    } catch (e) {
      this.error = errorMessage(e);
    }
  }

  async saveSettings(next: Settings): Promise<void> {
    const paths = JSON.stringify(this.settings?.eval.toolchain_paths ?? {});
    try {
      const saved = await api.saveSettings(next);
      this.settings = saved;
      applyAppearance(saved);
      // Only toolchain paths change which languages can run.
      if (this.boot && JSON.stringify(saved.eval.toolchain_paths) !== paths) {
        this.boot.languages = (await api.bootstrap()).languages;
      }
    } catch (e) {
      this.notify(errorMessage(e));
    }
  }

  async toggleTheme(): Promise<void> {
    if (!this.settings) return;
    const current = document.documentElement.dataset.theme === "light" ? "light" : "dark";
    await this.saveSettings({ ...this.settings, theme: current === "dark" ? "light" : "dark" });
  }

  setStatus(id: number, status: Status): void {
    const p = this.boot?.problems.find((x) => x.id === id);
    if (!p) return;
    // Solved is sticky: a later failed attempt does not unsolve a problem.
    if (p.status === "solved" && status !== "solved") return;
    p.status = status;
  }

  setUserLists(lists: UserList[]): void {
    if (this.boot) this.boot.user_lists = lists;
  }

  notify(message: string, ms = 4000): void {
    this.toast = message;
    clearTimeout(this.toastTimer);
    this.toastTimer = setTimeout(() => (this.toast = null), ms);
  }
}

export const app = new AppState();
