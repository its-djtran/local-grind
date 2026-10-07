import { describe, expect, it, vi } from "vitest";
import { render, screen } from "@testing-library/svelte";
import { EditorView } from "@codemirror/view";
import CodeEditor from "./CodeEditor.svelte";
import { runScopeHandlers } from "@codemirror/view";

function view(): EditorView {
  const el = screen.getByTestId("code-editor").querySelector(".cm-editor") as HTMLElement;
  return EditorView.findFromDOM(el)!;
}

describe("CodeEditor", () => {
  it("emits changes, runs shortcuts and follows prop updates", async () => {
    const onChange = vi.fn();
    const onRun = vi.fn();
    const onSubmit = vi.fn();
    const r = render(CodeEditor, { value: "x = 1", language: "python", keymap: "standard", tabSize: 4, onChange, onRun, onSubmit });
    const v = view();
    expect(v.state.doc.toString()).toBe("x = 1");
    v.dispatch({ changes: { from: 0, insert: "#" } });
    expect(onChange).toHaveBeenCalledWith("#x = 1");

    const key = (k: string) => new KeyboardEvent("keydown", { key: k, ctrlKey: true });
    runScopeHandlers(v, key("'"), "editor");
    runScopeHandlers(v, key("Enter"), "editor");
    expect(onRun).toHaveBeenCalled();
    expect(onSubmit).toHaveBeenCalled();

    await r.rerender({ value: "y = 2", language: "javascript", keymap: "vim", tabSize: 2, onChange, onRun, onSubmit });
    expect(view().state.doc.toString()).toBe("y = 2");
    expect(view().state.tabSize).toBe(2);
  });

  it("supports read-only mode with default handlers", () => {
    render(CodeEditor, { value: "z", language: "python", keymap: "standard", tabSize: 4, readonly: true });
    const v = view();
    expect(v.state.readOnly).toBe(true);
    v.dispatch({ changes: { from: 0, insert: "a" } });
    const key = (k: string) => new KeyboardEvent("keydown", { key: k, ctrlKey: true });
    expect(runScopeHandlers(v, key("'"), "editor")).toBe(true);
    expect(runScopeHandlers(v, key("Enter"), "editor")).toBe(true);
  });
});
