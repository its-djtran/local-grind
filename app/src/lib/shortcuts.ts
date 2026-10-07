// Keyboard shortcuts: one table drives both matching and the `?` help overlay.

export type Action =
  | "help"
  | "focusSearch"
  | "goProblems"
  | "goStats"
  | "goSettings"
  | "nextProblem"
  | "prevProblem"
  | "run"
  | "submit"
  | "rowDown"
  | "rowUp"
  | "random";

export interface Shortcut {
  action: Action;
  keys: string;
  description: string;
  /** Requires Cmd (macOS) or Ctrl. */
  mod?: boolean;
  shift?: boolean;
  /** `KeyboardEvent.key` values that trigger it. */
  key: string[];
  /** Fires even while typing in an input or the editor (only for modifier shortcuts). */
  whileTyping?: boolean;
  scope: "Global" | "Problem list" | "Problem";
}

export const SHORTCUTS: Shortcut[] = [
  { action: "help", keys: "?", description: "Show keyboard shortcuts", key: ["?"], scope: "Global" },
  { action: "goProblems", keys: "⌘1", description: "Go to problems", key: ["1"], mod: true, whileTyping: true, scope: "Global" },
  { action: "goStats", keys: "⌘2", description: "Go to statistics", key: ["2"], mod: true, whileTyping: true, scope: "Global" },
  { action: "goSettings", keys: "⌘,", description: "Go to settings", key: [","], mod: true, whileTyping: true, scope: "Global" },
  { action: "focusSearch", keys: "/", description: "Search problems", key: ["/"], scope: "Problem list" },
  { action: "rowDown", keys: "j", description: "Next problem in the list", key: ["j", "ArrowDown"], scope: "Problem list" },
  { action: "rowUp", keys: "k", description: "Previous problem in the list", key: ["k", "ArrowUp"], scope: "Problem list" },
  { action: "random", keys: "r", description: "Pick a random unsolved problem", key: ["r"], scope: "Problem list" },
  { action: "run", keys: "⌘'", description: "Run testcases", key: ["'"], mod: true, whileTyping: true, scope: "Problem" },
  { action: "submit", keys: "⌘⏎", description: "Submit", key: ["Enter"], mod: true, whileTyping: true, scope: "Problem" },
  { action: "nextProblem", keys: "] or ⌘⇧]", description: "Next problem", key: ["]", "}"], scope: "Problem" },
  { action: "prevProblem", keys: "[ or ⌘⇧[", description: "Previous problem", key: ["[", "{"], scope: "Problem" },
];

export function isTyping(target: EventTarget | null): boolean {
  if (!(target instanceof HTMLElement)) return false;
  return (
    target.isContentEditable ||
    target.closest(".cm-editor") !== null ||
    ["INPUT", "TEXTAREA", "SELECT"].includes(target.tagName)
  );
}

/** Resolves a keydown to an action, honouring modifiers and whether the user is typing. */
export function matchShortcut(e: KeyboardEvent): Action | null {
  const mod = e.metaKey || e.ctrlKey;
  const typing = isTyping(e.target);
  // ⌘⇧] / ⌘⇧[ navigate problems from anywhere, including the editor.
  if (mod && e.shiftKey && (e.key === "]" || e.key === "}")) return "nextProblem";
  if (mod && e.shiftKey && (e.key === "[" || e.key === "{")) return "prevProblem";
  for (const s of SHORTCUTS) {
    if (!s.key.includes(e.key)) continue;
    if (Boolean(s.mod) !== mod) continue;
    if (e.altKey) continue;
    if (typing && !s.whileTyping) continue;
    return s.action;
  }
  return null;
}

/** Page-level actions are broadcast so the active view can handle them. */
export const SHORTCUT_EVENT = "lg-shortcut";

export function dispatchShortcut(action: Action): void {
  window.dispatchEvent(new CustomEvent<Action>(SHORTCUT_EVENT, { detail: action }));
}

export function onShortcut(handler: (action: Action) => void): () => void {
  const listener = (e: Event) => handler((e as CustomEvent<Action>).detail);
  window.addEventListener(SHORTCUT_EVENT, listener);
  return () => window.removeEventListener(SHORTCUT_EVENT, listener);
}
