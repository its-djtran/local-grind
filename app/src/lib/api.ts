// Typed wrappers over the Tauri commands defined in app/src-tauri/src/lib.rs.
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  Bootstrap,
  ImportSummary,
  LanguageId,
  ProblemView,
  Progress,
  RunOutcome,
  Settings,
  Solve,
  Stats,
  Submission,
  SubmitOutcome,
} from "./types";

export const api = {
  bootstrap: () => invoke<Bootstrap>("bootstrap"),
  problem: (id: number, language?: LanguageId) => invoke<ProblemView>("get_problem", { id, language }),
  saveDraft: (id: number, language: LanguageId, code: string) =>
    invoke<void>("save_draft", { id, language, code }),
  resetCode: (id: number, language: LanguageId) => invoke<string>("reset_code", { id, language }),
  run: (id: number, language: LanguageId, code: string, input: string) =>
    invoke<RunOutcome>("run_code", { id, language, code, input }),
  submit: (id: number, language: LanguageId, code: string, elapsedMs: number) =>
    invoke<SubmitOutcome>("submit_code", { id, language, code, elapsedMs }),
  submissions: (id: number) => invoke<Submission[]>("list_submissions", { id }),
  startSolve: (id: number) => invoke<Solve>("start_solve", { id }),
  updateSolve: (solveId: number, elapsedMs: number) => invoke<void>("update_solve", { solveId, elapsedMs }),
  discardSolve: (solveId: number) => invoke<void>("discard_solve", { solveId }),
  settings: () => invoke<Settings>("get_settings"),
  saveSettings: (settings: Settings) => invoke<Settings>("save_settings", { settings }),
  stats: (recentN: number, reviewMinDays: number, utcOffsetMinutes: number) =>
    invoke<Stats>("get_stats", { recentN, reviewMinDays, utcOffsetMinutes }),
  solves: () => invoke<Solve[]>("list_solves"),
  exportHistory: (path: string) => invoke<void>("export_history", { path }),
  importHistory: (path: string, applySettings: boolean) =>
    invoke<ImportSummary>("import_history", { path, applySettings }),
  clearHistory: () => invoke<void>("clear_history"),
  referenceSolution: (id: number, language: LanguageId) =>
    invoke<string | null>("reference_solution", { id, language }),
  onSubmitProgress: (cb: (p: Progress) => void): Promise<UnlistenFn> =>
    listen<Progress>("submit-progress", (e) => cb(e.payload)),
};

export type Api = typeof api;

/** Normalizes errors thrown by `invoke` (strings from Rust) into messages. */
export function errorMessage(e: unknown): string {
  if (typeof e === "string") return e;
  if (e instanceof Error) return e.message;
  return JSON.stringify(e);
}
