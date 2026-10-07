import type { CompanyEntry, Difficulty, ProblemList, ProblemListing, Status, UserList } from "./types";

export type SortKey = "id" | "title" | "difficulty" | "frequency";

export interface Filters {
  query: string;
  listId: string;
  difficulty: Difficulty | "";
  status: Status | "";
  topic: string;
  company: string;
  sort: SortKey;
}

export const emptyFilters = (): Filters => ({
  query: "",
  listId: "",
  difficulty: "",
  status: "",
  topic: "",
  company: "",
  sort: "id",
});

const DIFFICULTY_ORDER: Record<Difficulty, number> = { Easy: 0, Medium: 1, Hard: 2 };

export function matchesQuery(p: ProblemListing, query: string): boolean {
  const q = query.trim().toLowerCase();
  if (!q) return true;
  if (/^\d+$/.test(q)) return String(p.id).startsWith(q);
  return (
    p.title.toLowerCase().includes(q) ||
    p.slug.includes(q) ||
    p.topics.some((t) => t.toLowerCase() === q)
  );
}

export interface Group {
  name: string;
  problems: ProblemListing[];
}

/**
 * Applies filters. When a list is selected, problems keep the list's order and are grouped by
 * the list's groups (topic for Blind 75, week for Grind 75); otherwise one group is returned.
 */
export function filterProblems(
  problems: ProblemListing[],
  f: Filters,
  lists: ProblemList[],
  companies: CompanyEntry[],
): Group[] {
  const byId = new Map(problems.map((p) => [p.id, p]));
  const company = f.company ? companies.find((c) => c.name === f.company) : undefined;
  const companyIds = company ? new Set(company.problem_ids) : null;
  const keep = (p: ProblemListing) =>
    matchesQuery(p, f.query) &&
    (!f.difficulty || p.difficulty === f.difficulty) &&
    (!f.status || p.status === f.status) &&
    (!f.topic || p.topics.includes(f.topic)) &&
    (!companyIds || companyIds.has(p.id));

  const list = f.listId ? lists.find((l) => l.id === f.listId) : undefined;
  if (list && f.sort === "id") {
    const groups: Group[] = [];
    for (const item of list.items) {
      const p = byId.get(item.problem_id);
      if (!p || !keep(p)) continue;
      let g = groups[groups.length - 1];
      if (!g || g.name !== item.group) {
        g = { name: item.group, problems: [] };
        groups.push(g);
      }
      g.problems.push(p);
    }
    return groups;
  }
  const inList = list ? new Set(list.items.map((i) => i.problem_id)) : null;
  const rows = problems.filter((p) => keep(p) && (!inList || inList.has(p.id)));
  const companyRank = new Map(company?.problem_ids.map((id, i) => [id, i]) ?? []);
  rows.sort((a, b) => {
    switch (f.sort) {
      case "title":
        return a.title.localeCompare(b.title);
      case "difficulty":
        return DIFFICULTY_ORDER[a.difficulty] - DIFFICULTY_ORDER[b.difficulty] || a.id - b.id;
      case "frequency":
        return (companyRank.get(a.id) ?? 1e9) - (companyRank.get(b.id) ?? 1e9) || a.id - b.id;
      default:
        return a.id - b.id;
    }
  });
  return [{ name: "", problems: rows }];
}

export function countByStatus(problems: ProblemListing[]): Record<Status, number> {
  const out: Record<Status, number> = { solved: 0, attempted: 0, todo: 0 };
  for (const p of problems) out[p.status]++;
  return out;
}

/** A random unsolved problem from the visible set (falls back to any visible problem). */
export function pickRandom(groups: Group[], rand: () => number = Math.random): ProblemListing | null {
  const all = groups.flatMap((g) => g.problems);
  const pool = all.filter((p) => p.status !== "solved");
  const from = pool.length ? pool : all;
  return from.length ? from[Math.floor(rand() * from.length)] : null;
}

export interface NextUp {
  listId: string;
  listName: string;
  group: string;
  problem: ProblemListing;
  remaining: number;
}

/** The first unsolved problem of each list, in list order (lists with nothing left are skipped). */
export function nextInLists(problems: ProblemListing[], lists: ProblemList[]): NextUp[] {
  const byId = new Map(problems.map((p) => [p.id, p]));
  const out: NextUp[] = [];
  for (const list of lists) {
    const open = list.items.filter((i) => byId.get(i.problem_id) && byId.get(i.problem_id)!.status !== "solved");
    if (!open.length) continue;
    out.push({
      listId: list.id,
      listName: list.name,
      group: open[0].group,
      problem: byId.get(open[0].problem_id)!,
      remaining: open.length,
    });
  }
  return out;
}

/** User lists in the same shape as bundled lists (ids prefixed `user:`), so filters treat them alike. */
export function userListsAsProblemLists(lists: UserList[]): ProblemList[] {
  return lists.map((l) => ({
    id: `user:${l.id}`,
    name: l.name,
    description: "",
    items: l.problem_ids.map((problem_id) => ({ problem_id, group: "" })),
  }));
}
