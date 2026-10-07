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

export type LeftTab = "description" | "editorial" | "submissions" | "notes" | "result";
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
  note = $state("");

  private timer: SolveTimer;
  private solveId: number | null = null;
  /** Bumped on reset so a pending solve start knows its attempt was discarded. */
  private resets = 0;
  private draftTimer: ReturnType<typeof setTimeout> | undefined;
  private pendingDraft: string | null = null;
  private lastPersist = 0;
  private noteTimer: ReturnType<typeof setTimeout> | undefined;
  private pendingNote: string | null = null;

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
      this.note = view.note;
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

  /** Appends a judged test's arguments (e.g. a failed submission's input) to the console. */
  useTestcase(args: unknown[]): void {
    const fields = this.names.map((_, j) => JSON.stringify(args[j] ?? null));
    this.setCases([...this.cases, fields]);
    this.consoleTab = "testcase";
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

  /** Called on every change in the notes panel; saved after a short pause. */
  editNote(body: string): void {
    this.note = body;
    this.pendingNote = body;
    clearTimeout(this.noteTimer);
    this.noteTimer = setTimeout(() => void this.flushNote(), DRAFT_SAVE_DELAY_MS);
  }

  async flushNote(): Promise<void> {
    clearTimeout(this.noteTimer);
    if (this.pendingNote === null) return;
    const body = this.pendingNote;
    this.pendingNote = null;
    try {
      await this.api.saveNote(this.id, body);
    } catch (e) {
      this.hooks.notify(`Could not save note: ${errorMessage(e)}`);
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
      const resets = this.resets;
      try {
        const solve = await this.api.startSolve(this.id);
        // Reset meanwhile: the user discarded this attempt, so drop its solve.
        if (resets !== this.resets) {
          await this.api.discardSolve(solve.id).catch(() => {});
          return;
        }
        this.solveId = solve.id;
        // Continuing an existing solve: add its time, keeping the clock as the user left it
        // (running, or paused meanwhile).
        if (solve.elapsed_ms > 0) {
          const running = this.timer.running;
          this.timer.reset(solve.elapsed_ms + this.timer.elapsed());
          if (running) this.timer.start();
        }
        if (!this.timer.running) await this.persistTimer(true);
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
    this.resets++;
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
    try {
      const view = await this.api.problem(this.id, language);
      this.language = view.language;
      this.code = view.code;
      this.view = view;
    } catch (e) {
      this.hooks.notify(`Could not switch language: ${errorMessage(e)}`);
    }
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
      // A finished solve's time is still on the clock; it must not count towards a new one.
      const elapsed = this.solveCompleted ? 0 : Math.round(this.timer.elapsed());
      const outcome = await this.api.submit(this.id, this.language, this.code, elapsed);
      this.lastSubmit = outcome;
      this.selected = outcome.submission;
      this.leftTab = "result";
      this.submissions = [outcome.submission, ...this.submissions];
      const accepted = outcome.submission.verdict === "Accepted";
      if (outcome.solve && accepted) {
        this.timer.pause();
        this.timer.reset(outcome.solve.elapsed_ms);
        this.solveCompleted = true;
        this.solveId = null;
      } else if (outcome.solve) {
        this.solveId = outcome.solve.id;
      }
      this.syncTimer();
      const status = accepted || this.view.status === "solved" ? "solved" : "attempted";
      this.view = { ...this.view, status };
      this.hooks.onStatus(this.id, status);
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
    await this.flushNote();
    if (this.timer.running) {
      this.timer.pause();
      this.syncTimer();
      await this.persistTimer(true);
    }
  }
}
