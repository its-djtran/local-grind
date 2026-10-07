<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { EditorState } from "@codemirror/state";
  import { EditorView } from "@codemirror/view";
  import { Compartments, createExtensions, indentExtension, loadKeymap, loadLanguage, replaceDocument } from "../lib/editor";
  import type { LanguageId, Settings } from "../lib/types";

  interface Props {
    value: string;
    language: LanguageId;
    keymap: Settings["keymap"];
    tabSize: number;
    onChange?: (code: string) => void;
    onRun?: () => void;
    onSubmit?: () => void;
  }

  let {
    value,
    language,
    keymap,
    tabSize,
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
        extensions: createExtensions(compartments, { tabSize }, {
          onChange: (code) => onChange(code),
          onRun: () => onRun(),
          onSubmit: () => onSubmit(),
        }),
      }),
    });
  });

  onDestroy(() => view?.destroy());

  // Replace the document when the value changes from outside (reset, language switch).
  $effect(() => {
    const next = value;
    if (view && view.state.doc.toString() !== next) {
      replaceDocument(view, compartments, next);
    }
  });

  let mounted = $state(false);
  onMount(() => (mounted = true));

  // Language packs and the vim keymap load on demand; a stale load is ignored if the prop has
  // changed again meanwhile.
  $effect(() => {
    const wanted = language;
    if (!mounted) return;
    loadLanguage(wanted).then((ext) => {
      if (view && wanted === language) view.dispatch({ effects: compartments.language.reconfigure(ext) });
    });
  });
  $effect(() => {
    const wanted = keymap;
    if (!mounted) return;
    loadKeymap(wanted).then((ext) => {
      if (view && wanted === keymap) view.dispatch({ effects: compartments.keymap.reconfigure(ext) });
    });
  });
  $effect(() => {
    view?.dispatch({ effects: compartments.indent.reconfigure(indentExtension(tabSize)) });
  });
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
