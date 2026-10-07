<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { EditorState } from "@codemirror/state";
  import { EditorView } from "@codemirror/view";
  import {
    Compartments,
    createExtensions,
    indentExtension,
    keymapExtension,
    languageExtension,
  } from "../lib/editor";
  import type { LanguageId, Settings } from "../lib/types";

  interface Props {
    value: string;
    language: LanguageId;
    keymap: Settings["keymap"];
    tabSize: number;
    readonly?: boolean;
    onChange?: (code: string) => void;
    onRun?: () => void;
    onSubmit?: () => void;
  }

  let {
    value,
    language,
    keymap,
    tabSize,
    readonly = false,
    onChange = () => {},
    onRun = () => {},
    onSubmit = () => {},
  }: Props = $props();

  let host: HTMLDivElement;
  let view: EditorView | undefined;
  const compartments = new Compartments();

  onMount(() => {
    view = new EditorView({
      parent: host,
      state: EditorState.create({
        doc: value,
        extensions: [
          ...createExtensions(compartments, { language, keymap, tabSize }, {
            onChange: (code) => onChange(code),
            onRun: () => onRun(),
            onSubmit: () => onSubmit(),
          }),
          EditorState.readOnly.of(readonly),
          EditorView.editable.of(!readonly),
        ],
      }),
    });
  });

  onDestroy(() => view?.destroy());

  // Replace the document when the value changes from outside (reset, language switch).
  $effect(() => {
    const next = value;
    if (view && view.state.doc.toString() !== next) {
      view.dispatch({ changes: { from: 0, to: view.state.doc.length, insert: next } });
    }
  });

  $effect(() => {
    view?.dispatch({ effects: compartments.language.reconfigure(languageExtension(language)) });
  });
  $effect(() => {
    view?.dispatch({ effects: compartments.keymap.reconfigure(keymapExtension(keymap)) });
  });
  $effect(() => {
    view?.dispatch({ effects: compartments.indent.reconfigure(indentExtension(tabSize)) });
  });

  export function focus() {
    view?.focus();
  }
</script>

<div class="editor" bind:this={host} data-testid="code-editor"></div>

<style>
  .editor {
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }
  .editor :global(.cm-editor) {
    height: 100%;
  }
</style>
