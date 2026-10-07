import { describe, expect, it } from "vitest";
import { countByStatus, emptyFilters, filterProblems, matchesQuery, nextInLists, pickRandom } from "./filter";
import type { CompanyEntry, ProblemList, ProblemListing } from "./types";

const problems: ProblemListing[] = [
  { id: 1, slug: "two-sum", title: "Two Sum", difficulty: "Easy", topics: ["Array", "Hash Table"], status: "solved" },
  { id: 20, slug: "valid-parentheses", title: "Valid Parentheses", difficulty: "Easy", topics: ["Stack"], status: "todo" },
  { id: 206, slug: "reverse-linked-list", title: "Reverse Linked List", difficulty: "Easy", topics: ["Linked List"], status: "attempted" },
  { id: 42, slug: "trapping-rain-water", title: "Trapping Rain Water", difficulty: "Hard", topics: ["Array", "Stack"], status: "todo" },
  { id: 3, slug: "longest-substring", title: "Longest Substring", difficulty: "Medium", topics: ["String"], status: "todo" },
];
const lists: ProblemList[] = [
  {
    id: "blind75",
    name: "Blind 75",
    description: "",
    items: [
      { problem_id: 206, group: "Linked List" },
      { problem_id: 1, group: "Array" },
      { problem_id: 42, group: "Array" },
      { problem_id: 999, group: "Array" },
    ],
  },
];
const companies: CompanyEntry[] = [{ name: "Google", problem_ids: [42, 1] }];

const ids = (g: ReturnType<typeof filterProblems>) => g.flatMap((x) => x.problems.map((p) => p.id));

describe("filterProblems", () => {
  it("defaults to id order", () => {
    expect(ids(filterProblems(problems, emptyFilters(), lists, companies))).toEqual([1, 3, 20, 42, 206]);
  });

  it("matches queries by number, title, slug and topic", () => {
    expect(matchesQuery(problems[0], "")).toBe(true);
    expect(matchesQuery(problems[1], "2")).toBe(true);
    expect(matchesQuery(problems[0], "2")).toBe(false);
    expect(matchesQuery(problems[0], "SUM")).toBe(true);
    expect(matchesQuery(problems[2], "linked-list")).toBe(true);
    expect(matchesQuery(problems[1], "stack")).toBe(true);
    expect(matchesQuery(problems[1], "sta")).toBe(false);
  });

  it("filters by every facet", () => {
    const f = { ...emptyFilters(), difficulty: "Easy" as const, topic: "Array" };
    expect(ids(filterProblems(problems, f, lists, companies))).toEqual([1]);
    expect(ids(filterProblems(problems, { ...emptyFilters(), status: "todo" }, lists, companies))).toEqual([3, 20, 42]);
    expect(ids(filterProblems(problems, { ...emptyFilters(), company: "Google" }, lists, companies))).toEqual([1, 42]);
    expect(ids(filterProblems(problems, { ...emptyFilters(), company: "Nobody" }, lists, companies))).toHaveLength(5);
  });

  it("groups by list order", () => {
    const groups = filterProblems(problems, { ...emptyFilters(), listId: "blind75" }, lists, companies);
    expect(groups.map((g) => g.name)).toEqual(["Linked List", "Array"]);
    expect(ids(groups)).toEqual([206, 1, 42]);
    const hard = filterProblems(problems, { ...emptyFilters(), listId: "blind75", difficulty: "Hard" }, lists, companies);
    expect(ids(hard)).toEqual([42]);
  });

  it("sorts", () => {
    const by = (sort: "title" | "difficulty" | "frequency", extra = {}) =>
      ids(filterProblems(problems, { ...emptyFilters(), sort, ...extra }, lists, companies));
    expect(by("title")).toEqual([3, 206, 42, 1, 20]);
    expect(by("difficulty")).toEqual([1, 20, 206, 3, 42]);
    expect(by("frequency", { company: "Google" })).toEqual([42, 1]);
    expect(by("frequency")).toEqual([1, 3, 20, 42, 206]);
    expect(by("title", { listId: "blind75" })).toEqual([206, 42, 1]);
  });

  it("counts and picks", () => {
    expect(countByStatus(problems)).toEqual({ solved: 1, attempted: 1, todo: 3 });
    const groups = filterProblems(problems, emptyFilters(), lists, companies);
    expect(pickRandom(groups, () => 0)?.status).not.toBe("solved");
    const solvedOnly = [{ name: "", problems: [problems[0]] }];
    expect(pickRandom(solvedOnly, () => 0.5)?.id).toBe(1);
    expect(pickRandom([], () => 0)).toBeNull();
    expect(pickRandom(groups)).not.toBeNull();
  });
});

describe("nextInLists", () => {
  it("finds the first unsolved item per list", () => {
    const next = nextInLists(problems, lists);
    expect(next).toHaveLength(1);
    expect(next[0]).toMatchObject({ listId: "blind75", group: "Linked List", remaining: 2 });
    expect(next[0].problem.id).toBe(206);
    const allSolved = problems.map((p) => ({ ...p, status: "solved" as const }));
    expect(nextInLists(allSolved, lists)).toEqual([]);
  });
});
