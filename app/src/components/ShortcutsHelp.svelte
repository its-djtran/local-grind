<script lang="ts">
  import { SHORTCUTS } from "../lib/shortcuts";

  let { onClose }: { onClose: () => void } = $props();
  const scopes = ["Global", "Problem list", "Problem"] as const;
  let dialog: HTMLElement | undefined = $state();

  // Take focus so keys go to the dialog rather than the page behind it.
  $effect(() => dialog?.focus());
</script>

<div class="backdrop" onclick={(e) => e.target === e.currentTarget && onClose()} role="presentation">
  <div class="dialog card" role="dialog" aria-modal="true" aria-label="Keyboard shortcuts" tabindex="-1" bind:this={dialog}>
    <header>
      <h2>Keyboard shortcuts</h2>
      <button class="ghost" onclick={onClose} aria-label="Close">×</button>
    </header>
    {#each scopes as scope (scope)}
      <h3>{scope}</h3>
      <dl>
        {#each SHORTCUTS.filter((s) => s.scope === scope) as s (s.action)}
          <dt><kbd>{s.keys}</kbd></dt>
          <dd>{s.description}</dd>
        {/each}
      </dl>
    {/each}
    <p class="muted small">⌘ is Ctrl on Windows and Linux.</p>
  </div>
</div>

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.45);
    display: grid;
    place-items: center;
    z-index: 60;
  }
  .dialog {
    width: min(520px, 92vw);
    max-height: 85vh;
    overflow-y: auto;
    outline: none;
  }
  header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  h2 {
    margin: 0;
    font-size: 1.2em;
  }
  h3 {
    font-size: 0.95em;
    color: var(--muted);
    margin: 1rem 0 0.4rem;
  }
  dl {
    display: grid;
    grid-template-columns: 9rem 1fr;
    gap: 0.35rem 0.8rem;
    margin: 0;
  }
  dd {
    margin: 0;
  }
  kbd {
    background: var(--panel-2);
    border: 1px solid var(--border);
    border-radius: 4px;
    padding: 0.05em 0.45em;
    font-family: var(--font-family);
  }
  .small {
    font-size: 0.85em;
  }
</style>
