import type { Example, Signature } from "./types";

/** Labels for the per-parameter inputs in the testcase editor. */
export function paramNames(sig: Signature): string[] {
  switch (sig.kind) {
    case "function":
      return sig.params.map((p) => p.name);
    case "codec":
      return [sig.encode.params[0].name];
    default:
      return ["operations", "arguments"];
  }
}

/** Splits LeetCode-style testcase text into cases of `arity` lines each. */
export function splitInput(text: string, arity: number): string[][] {
  const lines = text
    .split("\n")
    .map((l) => l.trim())
    .filter((l) => l.length > 0);
  if (arity <= 0) return [[]];
  const cases: string[][] = [];
  for (let i = 0; i < lines.length; i += arity) cases.push(lines.slice(i, i + arity));
  return cases;
}

export function casesFromExamples(examples: Example[], sig: Signature): string[][] {
  const arity = paramNames(sig).length;
  return examples.flatMap((e) => splitInput(e.input, arity));
}

export function casesToInput(cases: string[][]): string {
  return cases.map((c) => c.map((v) => v.trim()).join("\n")).join("\n");
}

/** Validates that every field holds JSON; returns the first problem found. */
export function validateCases(cases: string[][], names: string[]): string | null {
  for (let i = 0; i < cases.length; i++) {
    for (let j = 0; j < names.length; j++) {
      const v = (cases[i][j] ?? "").trim();
      if (!v) return `Case ${i + 1}: ${names[j]} is empty`;
      try {
        JSON.parse(v);
      } catch {
        return `Case ${i + 1}: ${names[j]} is not valid JSON`;
      }
    }
  }
  return null;
}
