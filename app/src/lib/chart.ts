// Pure helpers for the SVG trend charts (kept out of components for unit testing).

export interface Point {
  label: string;
  value: number | null;
}

/** A clean axis maximum and tick step (1, 2 or 5 × 10^k) covering `max` in about four steps. */
export function niceScale(max: number, targetTicks = 4): { max: number; step: number } {
  if (!(max > 0) || !Number.isFinite(max)) return { max: 1, step: 0.25 };
  const raw = max / targetTicks;
  const mag = 10 ** Math.floor(Math.log10(raw));
  const step = [1, 2, 5, 10].map((m) => m * mag).find((s) => s >= raw)!;
  return { max: Math.ceil(max / step) * step, step };
}

export function ticks(scaleMax: number, step: number): number[] {
  const out: number[] = [];
  for (let v = 0; v <= scaleMax + step / 1e6; v += step) out.push(Math.round(v * 1e9) / 1e9);
  return out;
}

/** Splits points into runs of consecutive non-null values so gaps break the line. */
export function segments(points: Point[]): { index: number; value: number }[][] {
  const runs: { index: number; value: number }[][] = [];
  let current: { index: number; value: number }[] = [];
  points.forEach((p, index) => {
    if (p.value === null || !Number.isFinite(p.value)) {
      if (current.length) runs.push(current);
      current = [];
    } else {
      current.push({ index, value: p.value });
    }
  });
  if (current.length) runs.push(current);
  return runs;
}

export function linePath(run: { index: number; value: number }[], x: (i: number) => number, y: (v: number) => number): string {
  return run.map((p, k) => `${k ? "L" : "M"}${x(p.index).toFixed(1)},${y(p.value).toFixed(1)}`).join("");
}

/** Index of the data position nearest to pixel `px` for `n` evenly spaced points. */
export function nearestIndex(px: number, left: number, width: number, n: number): number {
  if (n <= 1) return 0;
  const i = Math.round(((px - left) / width) * (n - 1));
  return Math.min(n - 1, Math.max(0, i));
}

const MONTHS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

/** `2026-03-02` -> `Mar 2`. */
export function shortDate(ymd: string): string {
  const [, m, d] = ymd.split("-").map(Number);
  return m >= 1 && m <= 12 ? `${MONTHS[m - 1]} ${d}` : ymd;
}

/** Indices for about `count` evenly spaced x-axis labels, always including the last point. */
export function labelIndices(n: number, count = 6): number[] {
  if (n <= 0) return [];
  if (n <= count) return [...Array(n).keys()];
  const step = (n - 1) / (count - 1);
  return [...new Set([...Array(count).keys()].map((k) => Math.round(k * step)))];
}
