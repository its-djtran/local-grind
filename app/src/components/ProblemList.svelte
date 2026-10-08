<script lang="ts">
  import { app } from "../lib/state.svelte";
  import { router } from "../lib/router.svelte";
  import { countByStatus, emptyFilters, filterProblems, pickRandom, userListsAsProblemLists, type Filters } from "../lib/filter";
  import { api, errorMessage } from "../lib/api";
  import { ask } from "@tauri-apps/plugin-dialog";
  import PracticeNext from "./PracticeNext.svelte";
  import { onMount } from "svelte";
  import { onShortcut } from "../lib/shortcuts";

  let search: HTMLInputElement | undefined = $state();
  let rowsEl: HTMLElement | undefined = $state();

  function moveRow(delta: number) {
    const rows = [...(rowsEl?.querySelectorAll<HTMLAnchorElement>("a.row") ?? [])];
    if (!rows.length) return;
    const current = rows.indexOf(document.activeElement as HTMLAnchorElement);
    const next = current < 0 ? (delta > 0 ? 0 : rows.length - 1) : Math.min(rows.length - 1, Math.max(0, current + delta));
    rows[next].focus();
  }

  onMount(() =>
    onShortcut((action) => {
      if (action === "focusSearch") search?.focus();
      else if (action === "rowDown") moveRow(1);
      else if (action === "rowUp") moveRow(-1);
      else if (action === "random") random();
    }),
  );

  const boot = $derived(app.boot!);
  let filters = $state<Filters>(loadFilters());

  function loadFilters(): Filters {
    try {
      return { ...emptyFilters(), ...JSON.parse(sessionStorage.getItem("lg-filters") ?? "{}") };
    } catch {
      return emptyFilters();
    }
  }

  $effect(() => {
    sessionStorage.setItem("lg-filters", JSON.stringify(filters));
  });

  const userLists = $derived(userListsAsProblemLists(boot.user_lists));
  const allLists = $derived([...boot.lists, ...userLists]);
  const groups = $derived(filterProblems(boot.problems, filters, allLists, boot.companies));
  const visible = $derived(groups.flatMap((g) => g.problems));
  const counts = $derived(countByStatus(visible));
  const activeList = $derived(allLists.find((l) => l.id === filters.listId));
  let creating = $state(false);
  let newListName = $state("");

  async function createList() {
    const name = newListName.trim();
    if (!name) return;
    try {
      app.setUserLists(await api.createUserList(name));
      newListName = "";
      creating = false;
    } catch (e) {
      app.notify(errorMessage(e));
    }
  }

  async function deleteList(listId: string, name: string) {
    const ok = await ask(`Delete the list "${name}"? The problems themselves are not affected.`, { title: "Delete list", kind: "warning" });
    if (!ok) return;
    try {
      app.setUserLists(await api.deleteUserList(Number(listId.slice(5))));
      if (filters.listId === listId) filters.listId = "";
    } catch (e) {
      app.notify(errorMessage(e));
    }
  }

  function random() {
    const p = pickRandom(groups);
    if (p) router.go({ name: "problem", id: p.id });
  }

  function clear() {
    filters = emptyFilters();
  }
</script>

<div class="page">
  <section class="lists">
    <button class="list-card" class:selected={!filters.listId} onclick={() => (filters.listId = "")}>
      <strong>All problems</strong>
      <span class="muted">{boot.problems.length} problems</span>
    </button>
    {#each allLists as list (list.id)}
      {@const solved = list.items.filter((i) => boot.problems.find((p) => p.id === i.problem_id)?.status === "solved").length}
      <div class="list-wrap">
        <button class="list-card" class:selected={filters.listId === list.id} onclick={() => (filters.listId = list.id)}>
          <strong>{list.name}</strong>
          <span class="muted">{solved} / {list.items.length} solved</span>
          <span class="bar"><span style="width: {(100 * solved) / Math.max(1, list.items.length)}%"></span></span>
        </button>
        {#if list.id.startsWith("user:")}
          <button class="ghost delete" aria-label="Delete list {list.name}" title="Delete list" onclick={() => deleteList(list.id, list.name)}>×</button>
        {/if}
      </div>
    {/each}
    {#if creating}
      <form class="list-card new-list" onsubmit={(e) => (e.preventDefault(), createList())}>
        <input type="text" placeholder="List name" bind:value={newListName} aria-label="List name" maxlength="60" />
        <span class="buttons">
          <button type="submit" disabled={!newListName.trim()}>Create</button>
          <button type="button" class="ghost" onclick={() => (creating = false)}>Cancel</button>
        </span>
      </form>
    {:else}
      <button class="list-card add" onclick={() => (creating = true)}>
        <strong>+ New list</strong>
        <span class="muted">Save problems to review later</span>
      </button>
    {/if}
  </section>

  <PracticeNext problems={boot.problems} lists={boot.lists} />

  <section class="filters">
    <input class="search" type="search" placeholder="Search by title, number or topic  ( / )" bind:value={filters.query} aria-label="Search problems" bind:this={search} />
    <select bind:value={filters.difficulty} aria-label="Difficulty">
      <option value="">Difficulty</option>
      <option>Easy</option>
      <option>Medium</option>
      <option>Hard</option>
    </select>
    <select bind:value={filters.status} aria-label="Status">
      <option value="">Status</option>
      <option value="todo">Todo</option>
      <option value="attempted">Attempted</option>
      <option value="solved">Solved</option>
    </select>
    <select bind:value={filters.topic} aria-label="Topic">
      <option value="">Topic</option>
      {#each boot.topics as t (t)}<option>{t}</option>{/each}
    </select>
    <select bind:value={filters.company} aria-label="Company">
      <option value="">Company</option>
      {#each boot.companies as c (c.name)}<option value={c.name}>{c.name} ({c.problem_ids.length})</option>{/each}
    </select>
    <select bind:value={filters.sort} aria-label="Sort">
      <option value="id">{activeList ? "List order" : "Number"}</option>
      <option value="title">Title</option>
      <option value="difficulty">Difficulty</option>
      {#if filters.company}<option value="frequency">Company frequency</option>{/if}
    </select>
    <button onclick={clear}>Reset</button>
    <button onclick={random} title="Open a random unsolved problem from this view">Pick one</button>
    <span class="summary muted">
      <span class="ok">{counts.solved}</span> solved · <span class="warn">{counts.attempted}</span> attempted · {visible.length} shown
    </span>
  </section>

  <section class="table" role="table" aria-label="Problems" bind:this={rowsEl}>
    {#each groups as group (group.name)}
      {#if group.name}<h3 class="group">{group.name}</h3>{/if}
      {#each group.problems as p (p.id)}
        <a class="row" role="row" href="#/problems/{p.id}">
          <span class="status {p.status}" title={p.status} aria-label={p.status}>
            {p.status === "solved" ? "✓" : p.status === "attempted" ? "◐" : ""}
          </span>
          <span class="title">{p.id}. {p.title}</span>
          <span class="topics">
            {#each p.topics.slice(0, 3) as t (t)}<span class="tag">{t}</span>{/each}
          </span>
          <span class="difficulty {p.difficulty.toLowerCase()}">{p.difficulty}</span>
        </a>
      {/each}
    {:else}
      <p class="muted empty">No problems match these filters.</p>
    {/each}
  </section>
</div>

<style>
  .page {
    height: 100%;
    overflow-y: auto;
    padding: 1.2rem max(1.2rem, calc((100% - 1100px) / 2));
  }
  .lists {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 0.8rem;
    margin-bottom: 1rem;
  }
  .list-card {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.2rem;
    padding: 0.8rem 1rem;
    background: var(--panel);
    text-align: left;
  }
  .list-wrap {
    position: relative;
    display: flex;
  }
  .list-wrap .list-card {
    flex: 1;
  }
  .delete {
    position: absolute;
    top: 0.3rem;
    right: 0.3rem;
    padding: 0 0.45em;
    color: var(--muted);
  }
  .add {
    border-style: dashed;
  }
  .new-list .buttons {
    display: flex;
    gap: 0.4rem;
  }
  .new-list input {
    width: 100%;
  }
  .list-card.selected {
    border-color: var(--accent);
  }
  .bar {
    display: block;
    width: 100%;
    height: 4px;
    background: var(--panel-2);
    border-radius: 2px;
    overflow: hidden;
  }
  .bar span {
    display: block;
    height: 100%;
    background: var(--ok);
  }
  .filters {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    align-items: center;
    margin-bottom: 0.8rem;
  }
  .search {
    flex: 1 1 240px;
  }
  .summary {
    margin-left: auto;
  }
  .table {
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
  }
  .group {
    margin: 0;
    padding: 0.6rem 1rem;
    font-size: 0.95em;
    background: var(--panel-2);
    border-bottom: 1px solid var(--border);
  }
  .row {
    display: grid;
    grid-template-columns: 2rem 1fr auto 5rem;
    gap: 0.8rem;
    align-items: center;
    padding: 0.55rem 1rem;
    border-bottom: 1px solid var(--border);
  }
  .row:nth-child(even) {
    background: var(--panel-2);
  }
  .row:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: -2px;
  }
  .row:hover {
    background: var(--hover);
  }
  .status.solved {
    color: var(--ok);
  }
  .status.attempted {
    color: var(--warn);
  }
  .topics {
    display: flex;
    gap: 0.3rem;
  }
  .empty {
    padding: 1rem;
  }
</style>
