<script lang="ts">
  import { formatRuntime, formatValue, verdictLabel } from "../lib/format";
  import type { RunOutcome } from "../lib/types";

  interface Props {
    names: string[];
    cases: string[][];
    result: RunOutcome | null;
    running: boolean;
    tab: "testcase" | "result";
    error: string | null;
    onTab: (t: "testcase" | "result") => void;
    onCasesChange: (cases: string[][]) => void;
    onResetCases: () => void;
  }

  let { names, cases, result, running, tab, error, onTab, onCasesChange, onResetCases }: Props = $props();

  let selected = $state(0);
  let resultSelected = $state(0);
  const current = $derived(Math.min(selected, Math.max(0, cases.length - 1)));
  const shown = $derived(result?.cases[Math.min(resultSelected, Math.max(0, (result?.cases.length ?? 1) - 1))]);

  function update(i: number, j: number, v: string) {
    const next = cases.map((c) => [...c]);
    next[i][j] = v;
    onCasesChange(next);
  }

  function addCase() {
    const base = cases[current] ?? names.map(() => "");
    onCasesChange([...cases.map((c) => [...c]), [...base]]);
    selected = cases.length;
  }

  function removeCase(i: number) {
    if (cases.length <= 1) return;
    onCasesChange(cases.filter((_, k) => k !== i));
    if (selected >= i && selected > 0) selected--;
  }

  function argText(args: unknown[], j: number) {
    return formatValue(args[j]);
  }
</script>

<div class="console">
  <div class="tabs" role="tablist">
    <button role="tab" class:active={tab === "testcase"} aria-selected={tab === "testcase"} onclick={() => onTab("testcase")}>Testcase</button>
    <button role="tab" class:active={tab === "result"} aria-selected={tab === "result"} onclick={() => onTab("result")}>Test Result</button>
  </div>

  <div class="body">
    {#if tab === "testcase"}
      <div class="chips">
        {#each cases as _, i (i)}
          <span class="chip" class:active={i === current}>
            <button class="ghost" onclick={() => (selected = i)}>Case {i + 1}</button>
            {#if cases.length > 1}
              <button class="ghost x" aria-label="Remove case {i + 1}" onclick={() => removeCase(i)}>×</button>
            {/if}
          </span>
        {/each}
        <button class="ghost" onclick={addCase} aria-label="Add testcase" title="Add a testcase (copies the current one)">+</button>
        <button class="ghost reset" onclick={onResetCases} title="Restore the example testcases">Reset</button>
      </div>
      {#if cases[current]}
        {#each names as name, j (name)}
          <label class="field">
            <span class="muted">{name} =</span>
            <textarea rows="1" spellcheck="false" value={cases[current][j] ?? ""} oninput={(e) => update(current, j, e.currentTarget.value)}></textarea>
          </label>
        {/each}
      {/if}
    {:else if running}
      <p class="muted">Running…</p>
    {:else if error}
      <pre class="error-box">{error}</pre>
    {:else if !result}
      <p class="muted">Run your code to see results here.</p>
    {:else}
      {@const finishedOnly = result.verdict === "Accepted" && !result.expected_available}
      <h3 class="verdict" class:ok={result.verdict === "Accepted" && !finishedOnly} class:bad={result.verdict !== "Accepted"}>
        {finishedOnly ? "Finished" : verdictLabel(result.verdict)}
        {#if result.verdict === "Accepted" || result.verdict === "WrongAnswer"}
          <span class="muted runtime">Runtime: {formatRuntime(result.runtime_ms)}</span>
        {/if}
      </h3>
      {#if result.error}
        <pre class="error-box">{result.error}</pre>
      {/if}
      {#if result.cases.length}
        <div class="chips">
          {#each result.cases as c, i (i)}
            <button class="ghost chip" class:active={i === resultSelected} onclick={() => (resultSelected = i)}>
              <span class={c.passed ? "ok" : "bad"}>●</span> Case {i + 1}
            </button>
          {/each}
        </div>
        {#if shown}
          <div class="section"><span class="muted">Input</span>
            {#each names as name, j (name)}
              <div class="value"><span class="muted">{name} =</span> {argText(shown.args, j)}</div>
            {/each}
          </div>
          {#if shown.stdout}
            <div class="section"><span class="muted">Stdout</span><pre class="value">{shown.stdout}</pre></div>
          {/if}
          <div class="section"><span class="muted">Output</span><div class="value" class:bad={!shown.passed && result.expected_available}>{formatValue(shown.output)}</div></div>
          {#if result.expected_available}
            <div class="section"><span class="muted">Expected</span><div class="value">{formatValue(shown.expected)}</div></div>
          {/if}
        {/if}
      {/if}
      {#if result.stderr}
        <div class="section"><span class="muted">Stderr</span><pre class="value">{result.stderr}</pre></div>
      {/if}
    {/if}
  </div>
</div>

<style>
  .console {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-height: 0;
  }
  .tabs {
    display: flex;
    gap: 0.2rem;
    padding: 0.3rem 0.5rem;
    border-bottom: 1px solid var(--border);
    background: var(--panel-2);
  }
  .tabs button {
    background: transparent;
    border: none;
    color: var(--muted);
  }
  .tabs button.active {
    color: var(--text);
    font-weight: 700;
  }
  .body {
    flex: 1;
    overflow-y: auto;
    padding: 0.7rem 1rem;
  }
  .chips {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-bottom: 0.7rem;
    align-items: center;
  }
  .chip {
    display: inline-flex;
    align-items: center;
    border-radius: 6px;
    background: transparent;
  }
  .chip.active {
    background: var(--hover);
  }
  .chip .x {
    padding: 0 0.4em;
    color: var(--muted);
  }
  .reset {
    margin-left: auto;
    color: var(--muted);
  }
  .field {
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
    margin-bottom: 0.6rem;
  }
  .field textarea {
    resize: vertical;
    min-height: 2.2em;
    background: var(--panel-2);
    font-family: var(--font-family);
  }
  .verdict {
    margin: 0 0 0.6rem;
    font-size: 1.25em;
  }
  .runtime {
    font-size: 0.7em;
    font-weight: 400;
    margin-left: 0.8em;
  }
  .section {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    margin-bottom: 0.7rem;
  }
  .value {
    background: var(--panel-2);
    border-radius: 6px;
    padding: 0.45em 0.7em;
    margin: 0;
    white-space: pre-wrap;
    word-break: break-all;
  }
  .error-box {
    background: color-mix(in srgb, var(--bad) 12%, transparent);
    color: var(--bad);
    border-radius: 6px;
    padding: 0.7em;
    white-space: pre-wrap;
    margin: 0 0 0.7rem;
  }
</style>
