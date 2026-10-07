// CodeMirror setup kept out of the component so it can be unit tested.
import { autocompletion, closeBrackets, closeBracketsKeymap, completionKeymap } from "@codemirror/autocomplete";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import {
  bracketMatching,
  foldGutter,
  indentOnInput,
  indentUnit,
  syntaxHighlighting,
} from "@codemirror/language";
import { highlightSelectionMatches, searchKeymap } from "@codemirror/search";
import { Compartment, EditorState, Prec, type Extension } from "@codemirror/state";
import {
  drawSelection,
  EditorView,
  highlightActiveLine,
  highlightActiveLineGutter,
  keymap,
  lineNumbers,
  rectangularSelection,
} from "@codemirror/view";
import { classHighlighter } from "@lezer/highlight";
import type { LanguageId, Settings } from "./types";

export interface EditorHandlers {
  onChange: (code: string) => void;
  onRun: () => void;
  onSubmit: () => void;
}

/** Loads a language pack on first use (code-split out of the initial bundle). */
export async function loadLanguage(lang: LanguageId): Promise<Extension> {
  switch (lang) {
    case "python":
      return (await import("@codemirror/lang-python")).python();
    case "javascript":
      return (await import("@codemirror/lang-javascript")).javascript();
    case "cpp":
      return (await import("@codemirror/lang-cpp")).cpp();
    case "rust":
      return (await import("@codemirror/lang-rust")).rust();
  }
}

/** Loads the vim keymap only when it is selected. */
export async function loadKeymap(mode: Settings["keymap"]): Promise<Extension> {
  return mode === "vim" ? (await import("@replit/codemirror-vim")).vim() : [];
}

export function indentExtension(tabSize: number): Extension {
  return [EditorState.tabSize.of(tabSize), indentUnit.of(" ".repeat(tabSize))];
}

export const baseTheme = EditorView.theme({
  "&": { height: "100%", fontSize: "var(--editor-font-size)", backgroundColor: "var(--code-bg)", color: "var(--text)" },
  ".cm-scroller": { fontFamily: "var(--font-family)", lineHeight: "1.55" },
  ".cm-gutters": { backgroundColor: "var(--code-bg)", color: "var(--muted)", border: "none" },
  ".cm-activeLine": { backgroundColor: "color-mix(in srgb, var(--hover) 55%, transparent)" },
  ".cm-activeLineGutter": { backgroundColor: "transparent", color: "var(--text)" },
  "&.cm-focused .cm-selectionBackground, .cm-selectionBackground, ::selection": {
    backgroundColor: "var(--selection) !important",
  },
  ".cm-cursor": { borderLeftColor: "var(--accent)" },
  "&.cm-focused": { outline: "none" },
});

export class Compartments {
  language = new Compartment();
  keymap = new Compartment();
  indent = new Compartment();
  history = new Compartment();
}

/**
 * Replaces the whole document from outside (language switch, reset) and starts a fresh undo
 * history, so undo cannot bring back another language's code.
 */
export function replaceDocument(view: EditorView, c: Compartments, text: string): void {
  view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: text }, effects: c.history.reconfigure([]) });
  view.dispatch({ effects: c.history.reconfigure(history()) });
}

/** Base extensions; language and keymap compartments start empty and are filled once loaded. */
export function createExtensions(c: Compartments, opts: { tabSize: number }, h: EditorHandlers): Extension[] {
  return [
    c.keymap.of([]),
    Prec.highest(
      keymap.of([
        { key: "Mod-'", run: () => (h.onRun(), true) },
        { key: "Mod-Enter", run: () => (h.onSubmit(), true) },
      ]),
    ),
    lineNumbers(),
    highlightActiveLineGutter(),
    foldGutter(),
    c.history.of(history()),
    drawSelection(),
    indentOnInput(),
    bracketMatching(),
    closeBrackets(),
    autocompletion(),
    rectangularSelection(),
    highlightActiveLine(),
    highlightSelectionMatches(),
    syntaxHighlighting(classHighlighter),
    keymap.of([...closeBracketsKeymap, ...defaultKeymap, ...searchKeymap, ...historyKeymap, ...completionKeymap, indentWithTab]),
    c.language.of([]),
    c.indent.of(indentExtension(opts.tabSize)),
    baseTheme,
    EditorView.updateListener.of((u) => {
      if (u.docChanged) h.onChange(u.state.doc.toString());
    }),
  ];
}
