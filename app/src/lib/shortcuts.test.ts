import { describe, expect, it, vi } from "vitest";
import { dispatchShortcut, isTyping, matchShortcut, onShortcut, SHORTCUTS } from "./shortcuts";

function key(k: string, opts: Partial<KeyboardEventInit> = {}, target?: HTMLElement) {
  const e = new KeyboardEvent("keydown", { key: k, bubbles: true, ...opts });
  if (target) Object.defineProperty(e, "target", { value: target });
  return e;
}

describe("shortcuts", () => {
  it("matches plain and modified keys", () => {
    expect(matchShortcut(key("?"), null)).toBe("help");
    expect(matchShortcut(key("/"), "Problem list")).toBe("focusSearch");
    expect(matchShortcut(key("j"), "Problem list")).toBe("rowDown");
    expect(matchShortcut(key("ArrowUp"), "Problem list")).toBe("rowUp");
    expect(matchShortcut(key("1", { metaKey: true }), "Problem")).toBe("goProblems");
    expect(matchShortcut(key("2", { ctrlKey: true }), null)).toBe("goStats");
    expect(matchShortcut(key(",", { metaKey: true }), null)).toBe("goSettings");
    expect(matchShortcut(key("Enter", { metaKey: true }), "Problem")).toBe("submit");
    expect(matchShortcut(key("]"), "Problem")).toBe("nextProblem");
    expect(matchShortcut(key("}", { metaKey: true, shiftKey: true }), "Problem")).toBe("nextProblem");
    expect(matchShortcut(key("{", { metaKey: true, shiftKey: true }), "Problem")).toBe("prevProblem");
    expect(matchShortcut(key("1"), null)).toBeNull();
    expect(matchShortcut(key("j", { altKey: true }), "Problem list")).toBeNull();
    expect(matchShortcut(key("x"), "Problem list")).toBeNull();
  });

  it("only matches shortcuts of the current page", () => {
    // Arrow keys scroll normally outside the problem list.
    expect(matchShortcut(key("ArrowDown"), null)).toBeNull();
    expect(matchShortcut(key("ArrowDown"), "Problem")).toBeNull();
    expect(matchShortcut(key("Enter", { metaKey: true }), "Problem list")).toBeNull();
    expect(matchShortcut(key("}", { metaKey: true, shiftKey: true }), null)).toBeNull();
    expect(matchShortcut(key("r"), "Problem")).toBeNull();
  });

  it("ignores plain keys while typing but keeps modifier shortcuts", () => {
    const input = document.createElement("input");
    expect(isTyping(input)).toBe(true);
    expect(matchShortcut(key("j", {}, input), "Problem list")).toBeNull();
    expect(matchShortcut(key("'", { metaKey: true }, input), "Problem")).toBe("run");
    const editor = document.createElement("div");
    editor.className = "cm-editor";
    const line = document.createElement("div");
    editor.appendChild(line);
    expect(isTyping(line)).toBe(true);
    expect(matchShortcut(key("]", { metaKey: true, shiftKey: true }, line), "Problem")).toBe("nextProblem");
    expect(isTyping(null)).toBe(false);
    expect(isTyping(document.createElement("div"))).toBe(false);
  });

  it("broadcasts actions to listeners", () => {
    const handler = vi.fn();
    const off = onShortcut(handler);
    dispatchShortcut("run");
    off();
    dispatchShortcut("run");
    expect(handler).toHaveBeenCalledTimes(1);
    expect(handler).toHaveBeenCalledWith("run");
    expect(new Set(SHORTCUTS.map((s) => s.action)).size).toBe(SHORTCUTS.length);
  });
});
