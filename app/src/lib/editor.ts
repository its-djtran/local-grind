// CodeMirror setup kept out of the component so it can be unit tested.
import { autocompletion, closeBrackets, closeBracketsKeymap, completionKeymap } from "@codemirror/autocomplete";
import { defaultKeymap, history, historyKeymap, indentWithTab } from "@codemirror/commands";
import { python } from "@codemirror/lang-python";
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
import { vim } from "@replit/codemirror-vim";
import type { LanguageId, Settings } from "./types";

export interface EditorHandlers {
  onChange: (code: string) => void;
  onRun: () => void;
  onSubmit: () => void;
}

export function languageExtension(lang: LanguageId): Extension {
  switch (lang) {
    case "python":
      return python();
  }
}

export function keymapExtension(mode: Settings["keymap"]): Extension {
  return mode === "vim" ? vim() : [];
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
}

export function createExtensions(
  c: Compartments,
  opts: { language: LanguageId; keymap: Settings["keymap"]; tabSize: number },
  h: EditorHandlers,
): Extension[] {
  return [
    c.keymap.of(keymapExtension(opts.keymap)),
    Prec.highest(
      keymap.of([
        { key: "Mod-'", run: () => (h.onRun(), true) },
        { key: "Mod-Enter", run: () => (h.onSubmit(), true) },
      ]),
    ),
    lineNumbers(),
    highlightActiveLineGutter(),
    foldGutter(),
    history(),
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
    c.language.of(languageExtension(opts.language)),
    c.indent.of(indentExtension(opts.tabSize)),
    baseTheme,
    EditorView.updateListener.of((u) => {
      if (u.docChanged) h.onChange(u.state.doc.toString());
    }),
  ];
}
