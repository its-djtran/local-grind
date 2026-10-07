/**
 * Solve timer. Tracks active time spent on a problem; the clock is injectable for tests.
 * Elapsed time survives restarts because the owner persists `elapsed()` periodically.
 */
export class SolveTimer {
  private base: number;
  private startedAt: number | null = null;

  constructor(
    initialMs = 0,
    private now: () => number = () => Date.now(),
  ) {
    this.base = Math.max(0, initialMs);
  }

  get running(): boolean {
    return this.startedAt !== null;
  }

  elapsed(): number {
    return this.base + (this.startedAt === null ? 0 : this.now() - this.startedAt);
  }

  start(): void {
    if (this.startedAt === null) this.startedAt = this.now();
  }

  pause(): void {
    if (this.startedAt !== null) {
      this.base += this.now() - this.startedAt;
      this.startedAt = null;
    }
  }

  reset(ms = 0): void {
    this.base = Math.max(0, ms);
    this.startedAt = null;
  }
}
