<script lang="ts">
  import { onMount } from "svelte";
  import { app } from "./lib/state.svelte";
  import { router } from "./lib/router.svelte";
  import ProblemList from "./components/ProblemList.svelte";
  import ProblemView from "./components/ProblemView.svelte";
  import StatsPage from "./components/StatsPage.svelte";
  import SettingsPage from "./components/SettingsPage.svelte";

  onMount(() => {
    app.load();
  });

  const theme = $derived(app.settings?.theme);
</script>

<div class="shell">
  <header class="topbar">
    <a class="brand" href="#/problems" aria-label="Local Grind home">
      <span class="chev">&gt;</span><span class="cursor">_</span> Local Grind
    </a>
    <nav>
      <a href="#/problems" class:active={router.route.name === "problems" || router.route.name === "problem"}>Problems</a>
      <a href="#/stats" class:active={router.route.name === "stats"}>Stats</a>
      <a href="#/settings" class:active={router.route.name === "settings"}>Settings</a>
    </nav>
    <div class="spacer"></div>
    <button class="ghost" title="Toggle theme" aria-label="Toggle theme" onclick={() => app.toggleTheme()} data-theme-setting={theme}>
      ◐
    </button>
  </header>
  <main>
    {#if app.error}
      <div class="center">
        <div class="card error">
          <h2>Local Grind could not start</h2>
          <p>{app.error}</p>
          <button onclick={() => app.load()}>Retry</button>
        </div>
      </div>
    {:else if !app.boot}
      <div class="center muted">Loading…</div>
    {:else if router.route.name === "problem"}
      {#key router.route.id}
        <ProblemView id={router.route.id} />
      {/key}
    {:else if router.route.name === "stats"}
      <StatsPage />
    {:else if router.route.name === "settings"}
      <SettingsPage />
    {:else}
      <ProblemList />
    {/if}
  </main>
  {#if app.toast}
    <div class="toast" role="status">{app.toast}</div>
  {/if}
</div>

<style>
  .shell {
    display: flex;
    flex-direction: column;
    height: 100%;
  }
  .topbar {
    display: flex;
    align-items: center;
    gap: 1.5rem;
    height: 48px;
    padding: 0 1rem;
    background: var(--panel);
    border-bottom: 1px solid var(--border);
    flex-shrink: 0;
  }
  .brand {
    font-weight: 700;
    font-size: 1.1em;
  }
  .chev {
    color: var(--accent);
  }
  .cursor {
    color: var(--ok);
    margin-right: 0.3em;
  }
  nav {
    display: flex;
    gap: 1rem;
  }
  nav a {
    color: var(--muted);
    padding: 0.2em 0.2em;
  }
  nav a.active,
  nav a:hover {
    color: var(--text);
  }
  nav a.active {
    border-bottom: 2px solid var(--accent);
  }
  .spacer {
    flex: 1;
  }
  main {
    flex: 1;
    min-height: 0;
    overflow: hidden;
  }
  .center {
    display: grid;
    place-items: center;
    height: 100%;
  }
  .error {
    max-width: 560px;
  }
  .toast {
    position: fixed;
    bottom: 1.2rem;
    left: 50%;
    transform: translateX(-50%);
    background: var(--panel-2);
    border: 1px solid var(--border);
    padding: 0.6em 1.2em;
    border-radius: var(--radius);
    box-shadow: 0 6px 24px rgba(0, 0, 0, 0.3);
    z-index: 50;
  }
</style>
