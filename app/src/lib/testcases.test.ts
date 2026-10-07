import { describe, expect, it } from "vitest";
import { casesFromExamples, casesToInput, paramNames, splitInput, validateCases } from "./testcases";
import type { Signature } from "./types";

const fn: Signature = {
  kind: "function",
  name: "twoSum",
  params: [
    { name: "nums", type: "int[]" },
    { name: "target", type: "int" },
  ],
  returns: "int[]",
};
const design: Signature = { kind: "design", class: "C", constructor: [], methods: [] };

describe("testcases", () => {
  it("names parameters", () => {
    expect(paramNames(fn)).toEqual(["nums", "target"]);
    expect(paramNames(design)).toEqual(["operations", "arguments"]);
    const codec: Signature = {
      kind: "codec",
      class: "Codec",
      encode: { name: "serialize", params: [{ name: "root", type: "TreeNode" }], returns: "string" },
      decode: { name: "deserialize", params: [{ name: "data", type: "string" }], returns: "TreeNode" },
    };
    expect(paramNames(codec)).toEqual(["root"]);
  });

  it("splits and joins", () => {
    expect(splitInput("[1,2]\n3\n\n[4]\n 5 ", 2)).toEqual([["[1,2]", "3"], ["[4]", "5"]]);
    expect(splitInput("", 0)).toEqual([[]]);
    expect(casesToInput([["[1]", " 2 "], ["[3]", "4"]])).toBe("[1]\n2\n[3]\n4");
  });

  it("builds cases from examples", () => {
    const cases = casesFromExamples([{ input: "[2,7]\n9", output: "[0,1]" }, { input: "[3,3]\n6", output: "[0,1]" }], fn);
    expect(cases).toEqual([["[2,7]", "9"], ["[3,3]", "6"]]);
  });

  it("validates", () => {
    expect(validateCases([["[1]", "2"]], ["nums", "target"])).toBeNull();
    expect(validateCases([["[1]", ""]], ["nums", "target"])).toBe("Case 1: target is empty");
    expect(validateCases([["[1]", "2"], ["[1", "2"]], ["nums", "target"])).toBe("Case 2: nums is not valid JSON");
    expect(validateCases([["[1]"]], ["nums", "target"])).toBe("Case 1: target is empty");
  });
});
