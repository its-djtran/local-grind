<script lang="ts">
  import {
    compareToReference,
    formatClock,
    formatDateTime,
    formatMemory,
    formatRuntime,
    formatValue,
    verdictLabel,
  } from "../lib/format";
  import { paramNames } from "../lib/testcases";
  import type { Benchmark, Signature, Solve, Submission } from "../lib/types";

  interface Props {
    submission: Submission;
    signature: Signature;
    benchmark?: Benchmark | null;
    solve?: Solve | null;
    onBack?: () => void;
  }

  let { submission: s, signature, benchmark = null, solve = null, onBack }: Props = $props();
  const accepted = $derived(s.verdict === "Accepted");
  const names = $derived(paramNames(signature));

  function bar(value: number, reference: number): number {
    const max = Math.max(value, reference, 1e-9);
    return (100 * value) / max;
  }
</script>

<div class="detail">
  {#if onBack}<button class="ghost back" onclick={onBack}>← All submissions</button>{/if}
  <header>
    <h2 class={accepted ? "ok" : "bad"}>{verdictLabel(s.verdict)}</h2>
    <span class="muted">{s.passed} / {s.total} testcases passed · {formatDateTime(s.created_at)} · {s.language}</span>
  </header>

  {#if accepted}
    <div class="metrics">
      <div class="card metric">
        <span class="muted">Runtime</span>
        <strong>{formatRuntime(s.runtime_ms)}</strong>
        {#if benchmark}
          <div class="bars" aria-label="Runtime compared with the reference solution">
            <div class="barrow"><span>You</span><span class="track"><span class="fill you" style="width: {bar(s.runtime_ms, benchmark.runtime_ms)}%"></span></span></div>
            <div class="barrow"><span>Ref</span><span class="track"><span class="fill ref" style="width: {bar(benchmark.runtime_ms, s.runtime_ms)}%"></span></span></div>
          </div>
          <span class="muted small">Reference {formatRuntime(benchmark.runtime_ms)} (avg of {benchmark.runs} runs) — {compareToReference(s.runtime_ms, benchmark.runtime_ms)}</span>
        {/if}
      </div>
      <div class="card metric">
        <span class="muted">Memory</span>
        <strong>{formatMemory(s.memory_kb)}</strong>
        {#if benchmark}
          <div class="bars" aria-label="Memory compared with the reference solution">
            <div class="barrow"><span>You</span><span class="track"><span class="fill you" style="width: {bar(s.memory_kb, benchmark.memory_kb)}%"></span></span></div>
            <div class="barrow"><span>Ref</span><span class="track"><span class="fill ref" style="width: {bar(benchmark.memory_kb, s.memory_kb)}%"></span></span></div>
          </div>
          <span class="muted small">Reference {formatMemory(benchmark.memory_kb)} — {compareToReference(s.memory_kb, benchmark.memory_kb)}</span>
        {/if}
      </div>
      {#if solve && solve.completed_at}
        <div class="card metric">
          <span class="muted">Solve time</span>
          <strong>{formatClock(solve.elapsed_ms)}</strong>
          <span class="muted small">{solve.errors} wrong submission{solve.errors === 1 ? "" : "s"} before accepting</span>
        </div>
      {/if}
    </div>
  {/if}

  {#if s.error}
    <pre class="error-box">{s.error}</pre>
  {/if}

  {#if s.failure}
    <section class="failure">
      <h3>Failing testcase #{s.failure.index + 1}</h3>
      <div class="kv"><span class="muted">Input</span>
        {#each names as name, j (name)}<div class="value"><span class="muted">{name} =</span> {formatValue(s.failure.args[j])}</div>{/each}
      </div>
      {#if s.failure.stdout}<div class="kv"><span class="muted">Stdout</span><pre class="value">{s.failure.stdout}</pre></div>{/if}
      {#if s.failure.output !== null && s.failure.output !== undefined}
        <div class="kv"><span class="muted">Output</span><div class="value bad">{formatValue(s.failure.output)}</div></div>
      {/if}
      <div class="kv"><span class="muted">Expected</span><div class="value">{formatValue(s.failure.expected)}</div></div>
    </section>
  {/if}

  <section>
    <h3>Code</h3>
    <pre class="code">{s.code}</pre>
  </section>
</div>

<style>
  .detail {
    display: flex;
    flex-direction: column;
    gap: 0.8rem;
  }
  .back {
    align-self: flex-start;
    padding-left: 0;
  }
  header h2 {
    margin: 0;
  }
  .metrics {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 0.7rem;
  }
  .metric {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }
  .metric strong {
    font-size: 1.5em;
  }
  .small {
    font-size: 0.85em;
  }
  .bars {
    display: flex;
    flex-direction: column;
    gap: 3px;
  }
  .barrow {
    display: grid;
    grid-template-columns: 2.5em 1fr;
    align-items: center;
    font-size: 0.8em;
  }
  .track {
    height: 8px;
    background: var(--panel-2);
    border-radius: 4px;
    overflow: hidden;
  }
  .fill {
    display: block;
    height: 100%;
    border-radius: 4px;
  }
  .fill.you {
    background: var(--accent);
  }
  .fill.ref {
    background: var(--muted);
  }
  .kv {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    margin-bottom: 0.6rem;
  }
  .value {
    background: var(--panel-2);
    border-radius: 6px;
    padding: 0.45em 0.7em;
    margin: 0;
    white-space: pre-wrap;
    word-break: break-all;
  }
  .code {
    background: var(--code-bg);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    padding: 0.8em 1em;
    overflow-x: auto;
    margin: 0;
  }
  .error-box {
    background: color-mix(in srgb, var(--bad) 12%, transparent);
    color: var(--bad);
    border-radius: 6px;
    padding: 0.7em;
    white-space: pre-wrap;
    margin: 0;
  }
  h3 {
    margin: 0.4rem 0;
    font-size: 1em;
  }
</style>
