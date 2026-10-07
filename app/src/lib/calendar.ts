import type { DayCount } from "./types";

export interface Cell {
  date: string;
  submissions: number;
  accepted: number;
  level: 0 | 1 | 2 | 3 | 4;
}

function ymd(d: Date): string {
  return `${d.getFullYear()}-${String(d.getMonth() + 1).padStart(2, "0")}-${String(d.getDate()).padStart(2, "0")}`;
}

/** Quantizes a count into 5 sequential steps relative to the busiest day. */
export function level(count: number, max: number): Cell["level"] {
  if (count <= 0 || max <= 0) return 0;
  return Math.min(4, Math.max(1, Math.ceil((4 * count) / max))) as Cell["level"];
}

/**
 * Lays out the last `weeks` weeks ending today as columns of 7 days (Sunday first), the
 * GitHub contribution-graph shape. Days after today are omitted from the final column.
 */
export function buildCalendar(days: DayCount[], today: Date, weeks = 53): Cell[][] {
  const counts = new Map(days.map((d) => [d.date, d]));
  const max = Math.max(0, ...days.map((d) => d.submissions));
  const start = new Date(today.getFullYear(), today.getMonth(), today.getDate());
  start.setDate(start.getDate() - start.getDay() - (weeks - 1) * 7);
  const columns: Cell[][] = [];
  const cursor = new Date(start);
  const end = ymd(today);
  for (let w = 0; w < weeks; w++) {
    const col: Cell[] = [];
    for (let d = 0; d < 7; d++) {
      const key = ymd(cursor);
      if (key > end) break;
      const c = counts.get(key);
      col.push({
        date: key,
        submissions: c?.submissions ?? 0,
        accepted: c?.accepted ?? 0,
        level: level(c?.submissions ?? 0, max),
      });
      cursor.setDate(cursor.getDate() + 1);
    }
    columns.push(col);
  }
  return columns;
}

export const MONTHS = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

/** Month labels positioned at the first column whose first day starts a new month. */
export function monthLabels(columns: Cell[][]): { index: number; label: string }[] {
  const out: { index: number; label: string }[] = [];
  let last = -1;
  columns.forEach((col, i) => {
    const first = col[0];
    if (!first) return;
    const month = Number(first.date.slice(5, 7)) - 1;
    if (month !== last) {
      if (last !== -1 || i === 0) out.push({ index: i, label: MONTHS[month] });
      last = month;
    }
  });
  return out;
}
