// State and behaviour of the problem workspace (editor, console, timer, submissions). The
// ProblemView component is a thin rendering layer over this class.
import { errorMessage, type Api } from "./api";
import { casesFromExamples, casesToInput, paramNames, validateCases } from "./testcases";
import { SolveTimer } from "./timer";
import type {
  LanguageId,
  ProblemView,
  RunOutcome,
  Settings,
  Status,
  Submission,
  SubmitOutcome,
} from "./types";

export type LeftTab = "description" | "editorial" | "submissions" | "result";
export type ConsoleTab = "testcase" | "result";

export const DRAFT_SAVE_DELAY_MS = 600;
export const TIMER_PERSIST_MS = 15_000;

export interface WorkspaceHooks {
  settings: () => Settings | null;
  onStatus: (id: number, status: Status) => void;
  notify: (message: string) => void;
  now?: () => number;
}

export class Workspace {
  view = $state<ProblemView | null>(null);
  loadError = $state<string | null>(null);
  code = $state("");
  language = $state<LanguageId>("python");
  cases = $state<string[][]>([]);
  consoleTab = $state<ConsoleTab>("testcase");
  leftTab = $state<LeftTab>("description");
  runResult = $state<RunOutcome | null>(null);
  runError = $state<string | null>(null);
  running = $state(false);
  submitting = $state(false);
  progress = $state<{ done: number; total: number } | null>(null);
  submissions = $state<Submission[]>([]);
  selected = $state<Submission | null>(null);
  lastSubmit = $state<SubmitOutcome | null>(null);
  elapsed = $state(0);
  timerRunning = $state(false);
  solveCompleted = $state(false);

  private timer: SolveTimer;
  private solveId: number | null = null;
  private draftTimer: ReturnType<typeof setTimeout> | undefined;
  private pendingDraft: string | null = null;
  private lastPersist = 0;

  constructor(
    readonly id: number,
    private api: Api,
    private hooks: WorkspaceHooks,
  ) {
    this.timer = new SolveTimer(0, hooks.now);
  }

  private now(): number {
    return (this.hooks.now ?? Date.now)();
  }

  get names(): string[] {
    return this.view ? paramNames(this.view.signature) : [];
  }

  async load(language?: LanguageId): Promise<void> {
    try {
      const view = await this.api.problem(this.id, language);
      this.view = view;
      this.code = view.code;
      this.language = view.language;
      this.cases = this.restoreCases() ?? casesFromExamples(view.examples, view.signature);
      this.loadError = null;
      if (view.active_solve) {
        this.solveId = view.active_solve.id;
        this.timer.reset(view.active_solve.elapsed_ms);
      }
      this.syncTimer();
      this.submissions = await this.api.submissions(this.id);
    } catch (e) {
      this.loadError = errorMessage(e);
    }
  }

  private casesKey(): string {
    return `lg-cases-${this.id}`;
  }

  private restoreCases(): string[][] | null {
    try {
      const raw = sessionStorage.getItem(this.casesKey());
      const parsed = raw ? JSON.parse(raw) : null;
      return Array.isArray(parsed) && parsed.length ? parsed : null;
    } catch {
      return null;
    }
  }

  setCases(cases: string[][]): void {
    this.cases = cases;
    sessionStorage.setItem(this.casesKey(), JSON.stringify(cases));
  }

  resetCases(): void {
    if (!this.view) return;
    sessionStorage.removeItem(this.casesKey());
    this.cases = casesFromExamples(this.view.examples, this.view.signature);
  }

  private syncTimer(): void {
    this.elapsed = this.timer.elapsed();
    this.timerRunning = this.timer.running;
  }

  /** Called by the editor on every change. */
  edit(code: string): void {
    if (code === this.code) return;
    this.code = code;
    this.pendingDraft = code;
    clearTimeout(this.draftTimer);
    this.draftTimer = setTimeout(() => void this.flushDraft(), DRAFT_SAVE_DELAY_MS);
    if (this.hooks.settings()?.auto_start_timer !== false && !this.timer.running) {
      void this.startTimer();
    }
  }

  async flushDraft(): Promise<void> {
    clearTimeout(this.draftTimer);
    if (this.pendingDraft === null) return;
    const code = this.pendingDraft;
    this.pendingDraft = null;
    try {
      await this.api.saveDraft(this.id, this.language, code);
    } catch (e) {
      this.hooks.notify(`Could not save draft: ${errorMessage(e)}`);
    }
  }

  async startTimer(): Promise<void> {
    if (this.timer.running) return;
    if (this.solveCompleted || this.solveId === null) {
      this.solveCompleted = false;
      this.timer.reset(0);
      // Start the clock immediately; the solve record is created in the background.
      this.timer.start();
      this.syncTimer();
      try {
        const solve = await this.api.startSolve(this.id);
        this.solveId = solve.id;
        if (solve.elapsed_ms > 0) this.timer.reset(solve.elapsed_ms + this.timer.elapsed());
        if (!this.timer.running) this.timer.start();
      } catch (e) {
        this.hooks.notify(`Could not start timer: ${errorMessage(e)}`);
      }
    } else {
      this.timer.start();
    }
    this.syncTimer();
  }

  async toggleTimer(): Promise<void> {
    if (this.timer.running) {
      this.timer.pause();
      this.syncTimer();
      await this.persistTimer(true);
    } else {
      await this.startTimer();
    }
  }

  async resetTimer(): Promise<void> {
    const id = this.solveId;
    this.timer.reset(0);
    this.solveId = null;
    this.solveCompleted = false;
    this.syncTimer();
    if (id !== null) await this.api.discardSolve(id).catch(() => {});
  }

  /** Advances the displayed clock; persists periodically. Called once a second. */
  tick(): void {
    this.syncTimer();
    if (this.timer.running && this.now() - this.lastPersist >= TIMER_PERSIST_MS) void this.persistTimer(false);
  }

  async persistTimer(force: boolean): Promise<void> {
    if (this.solveId === null || this.solveCompleted) return;
    if (!force && !this.timer.running) return;
    this.lastPersist = this.now();
    await this.api.updateSolve(this.solveId, Math.round(this.timer.elapsed())).catch(() => {});
  }

  async changeLanguage(language: LanguageId): Promise<void> {
    if (language === this.language) return;
    await this.flushDraft();
    const view = await this.api.problem(this.id, language);
    this.language = view.language;
    this.code = view.code;
    this.view = view;
  }

  async resetCode(): Promise<void> {
    clearTimeout(this.draftTimer);
    this.pendingDraft = null;
    this.code = await this.api.resetCode(this.id, this.language);
  }

  async run(): Promise<void> {
    if (!this.view || this.running || this.submitting) return;
    const invalid = validateCases(this.cases, this.names);
    this.consoleTab = "result";
    if (invalid) {
      this.runError = invalid;
      this.runResult = null;
      return;
    }
    this.running = true;
    this.runError = null;
    try {
      void this.flushDraft();
      this.runResult = await this.api.run(this.id, this.language, this.code, casesToInput(this.cases));
    } catch (e) {
      this.runResult = null;
      this.runError = errorMessage(e);
    } finally {
      this.running = false;
    }
  }

  async submit(): Promise<void> {
    if (!this.view || this.running || this.submitting) return;
    this.submitting = true;
    this.progress = { done: 0, total: this.view.test_count };
    try {
      void this.flushDraft();
      const outcome = await this.api.submit(this.id, this.language, this.code, Math.round(this.timer.elapsed()));
      this.lastSubmit = outcome;
      this.selected = outcome.submission;
      this.leftTab = "result";
      this.submissions = [outcome.submission, ...this.submissions];
      const accepted = outcome.submission.verdict === "Accepted";
      if (accepted) {
        this.timer.pause();
        this.timer.reset(outcome.solve.elapsed_ms);
        this.solveCompleted = true;
        this.solveId = null;
      } else {
        this.solveId = outcome.solve.id;
      }
      this.syncTimer();
      this.hooks.onStatus(this.id, accepted ? "solved" : "attempted");
    } catch (e) {
      this.hooks.notify(errorMessage(e));
    } finally {
      this.submitting = false;
      this.progress = null;
    }
  }

  onProgress(problemId: number, done: number, total: number): void {
    if (problemId === this.id && this.submitting) this.progress = { done, total };
  }

  selectSubmission(s: Submission | null): void {
    this.selected = s;
    if (s) this.leftTab = "result";
  }

  async dispose(): Promise<void> {
    clearTimeout(this.draftTimer);
    await this.flushDraft();
    if (this.timer.running) {
      this.timer.pause();
      this.syncTimer();
      await this.persistTimer(true);
    }
  }
}
