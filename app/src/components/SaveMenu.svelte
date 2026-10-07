<script lang="ts">
  import { api, errorMessage } from "../lib/api";
  import { app } from "../lib/state.svelte";

  let { problemId }: { problemId: number } = $props();

  let open = $state(false);
  let newName = $state("");
  const lists = $derived(app.boot?.user_lists ?? []);
  const saved = $derived(lists.some((l) => l.problem_ids.includes(problemId)));

  async function toggle(listId: number, member: boolean) {
    try {
      app.setUserLists(await api.setInUserList(listId, problemId, member));
    } catch (e) {
      app.notify(errorMessage(e));
    }
  }

  async function create() {
    const name = newName.trim();
    if (!name) return;
    try {
      const updated = await api.createUserList(name);
      const created = updated.find((l) => l.name === name);
      app.setUserLists(created ? await api.setInUserList(created.id, problemId, true) : updated);
      newName = "";
    } catch (e) {
      app.notify(errorMessage(e));
    }
  }
</script>

<span class="save">
  <button class="tag linkish" class:saved aria-expanded={open} onclick={() => (open = !open)}>{saved ? "★ Saved" : "☆ Save"}</button>
  {#if open}
    <div class="menu card" role="dialog" aria-label="Save to lists">
      {#each lists as l (l.id)}
        <label class="row">
          <input type="checkbox" checked={l.problem_ids.includes(problemId)} onchange={(e) => toggle(l.id, e.currentTarget.checked)} />
          {l.name}
        </label>
      {:else}
        <p class="muted small">No lists yet. Create one to save problems for later.</p>
      {/each}
      <form class="new" onsubmit={(e) => (e.preventDefault(), create())}>
        <input type="text" placeholder="New list…" bind:value={newName} aria-label="New list name" maxlength="60" />
        <button type="submit" disabled={!newName.trim()}>Add</button>
      </form>
    </div>
  {/if}
</span>

<style>
  .save {
    position: relative;
  }
  .linkish {
    cursor: pointer;
  }
  .saved {
    color: var(--accent);
  }
  .menu {
    position: absolute;
    top: 1.8em;
    left: 0;
    z-index: 20;
    min-width: 220px;
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    padding: 0.7rem;
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.3);
  }
  .row {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    cursor: pointer;
  }
  .new {
    display: flex;
    gap: 0.4rem;
    margin-top: 0.3rem;
  }
  .new input {
    flex: 1;
    min-width: 0;
  }
  .small {
    font-size: 0.85em;
    margin: 0;
  }
</style>
