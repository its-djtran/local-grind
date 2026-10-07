<script lang="ts">
  import { formatMemory, formatRelative, formatRuntime, verdictLabel } from "../lib/format";
  import type { Submission } from "../lib/types";

  interface Props {
    submissions: Submission[];
    onSelect: (s: Submission) => void;
  }

  let { submissions, onSelect }: Props = $props();
</script>

{#if submissions.length === 0}
  <p class="muted">No submissions yet. Your history for this problem will appear here.</p>
{:else}
  <table>
    <thead>
      <tr><th>Status</th><th>Language</th><th>Runtime</th><th>Memory</th><th>When</th></tr>
    </thead>
    <tbody>
      {#each submissions as s (s.id)}
        <tr onclick={() => onSelect(s)} tabindex="0" onkeydown={(e) => e.key === "Enter" && onSelect(s)}>
          <td class={s.verdict === "Accepted" ? "ok" : "bad"}>
            {verdictLabel(s.verdict)}
            {#if s.verdict !== "Accepted"}<span class="muted"> ({s.passed}/{s.total})</span>{/if}
          </td>
          <td>{s.language}</td>
          <td>{s.verdict === "Accepted" ? formatRuntime(s.runtime_ms) : "—"}</td>
          <td>{s.verdict === "Accepted" ? formatMemory(s.memory_kb) : "—"}</td>
          <td class="muted">{formatRelative(s.created_at)}</td>
        </tr>
      {/each}
    </tbody>
  </table>
{/if}

<style>
  table {
    width: 100%;
    border-collapse: collapse;
  }
  th {
    text-align: left;
    color: var(--muted);
    font-weight: 400;
    padding: 0.4rem 0.5rem;
    border-bottom: 1px solid var(--border);
  }
  td {
    padding: 0.5rem;
    border-bottom: 1px solid var(--border);
  }
  tbody tr {
    cursor: pointer;
  }
  tbody tr:hover {
    background: var(--hover);
  }
</style>
