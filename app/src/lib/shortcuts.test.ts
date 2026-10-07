import { describe, expect, it, vi } from "vitest";
import { dispatchShortcut, isTyping, matchShortcut, onShortcut, SHORTCUTS } from "./shortcuts";

function key(k: string, opts: Partial<KeyboardEventInit> = {}, target?: HTMLElement) {
  const e = new KeyboardEvent("keydown", { key: k, bubbles: true, ...opts });
  if (target) Object.defineProperty(e, "target", { value: target });
  return e;
}

describe("shortcuts", () => {
  it("matches plain and modified keys", () => {
    expect(matchShortcut(key("?"))).toBe("help");
    expect(matchShortcut(key("/"))).toBe("focusSearch");
    expect(matchShortcut(key("j"))).toBe("rowDown");
    expect(matchShortcut(key("ArrowUp"))).toBe("rowUp");
    expect(matchShortcut(key("1", { metaKey: true }))).toBe("goProblems");
    expect(matchShortcut(key("2", { ctrlKey: true }))).toBe("goStats");
    expect(matchShortcut(key(",", { metaKey: true }))).toBe("goSettings");
    expect(matchShortcut(key("Enter", { metaKey: true }))).toBe("submit");
    expect(matchShortcut(key("]"))).toBe("nextProblem");
    expect(matchShortcut(key("}", { metaKey: true, shiftKey: true }))).toBe("nextProblem");
    expect(matchShortcut(key("{", { metaKey: true, shiftKey: true }))).toBe("prevProblem");
    expect(matchShortcut(key("1"))).toBeNull();
    expect(matchShortcut(key("j", { altKey: true }))).toBeNull();
    expect(matchShortcut(key("x"))).toBeNull();
  });

  it("ignores plain keys while typing but keeps modifier shortcuts", () => {
    const input = document.createElement("input");
    expect(isTyping(input)).toBe(true);
    expect(matchShortcut(key("j", {}, input))).toBeNull();
    expect(matchShortcut(key("'", { metaKey: true }, input))).toBe("run");
    const editor = document.createElement("div");
    editor.className = "cm-editor";
    const line = document.createElement("div");
    editor.appendChild(line);
    expect(isTyping(line)).toBe(true);
    expect(matchShortcut(key("]", { metaKey: true, shiftKey: true }, line))).toBe("nextProblem");
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
