<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { ask } from "@tauri-apps/plugin-dialog";
  import { api } from "../lib/api";
  import { app } from "../lib/state.svelte";
  import { router } from "../lib/router.svelte";
  import { Workspace, type LeftTab } from "../lib/workspace.svelte";
  import { onShortcut } from "../lib/shortcuts";
  import type { LanguageId } from "../lib/types";
  import SplitPane from "./SplitPane.svelte";
  import CodeEditor from "./CodeEditor.svelte";
  import Console from "./Console.svelte";
  import Timer from "./Timer.svelte";
  import SubmissionDetail from "./SubmissionDetail.svelte";
  import SubmissionList from "./SubmissionList.svelte";

  let { id }: { id: number } = $props();

  // svelte-ignore state_referenced_locally
  const ws = new Workspace(id, api, {
    settings: () => app.settings,
    onStatus: (pid, status) => app.setStatus(pid, status),
    notify: (m) => app.notify(m),
  });

  let showHints = $state(false);
  let solutionLanguage = $state<LanguageId | null>(null);
  let showTopics = $state(false);
  let showCompanies = $state(false);
  let ticker: ReturnType<typeof setInterval>;
  let unlisten: (() => void) | undefined;
  let offShortcuts: (() => void) | undefined;

  const settings = $derived(app.settings);
  const view = $derived(ws.view);
  const order = $derived(app.boot?.problems.map((p) => p.id) ?? []);
  const position = $derived(order.indexOf(id));
  const benchmark = $derived(view?.benchmarks[ws.language] ?? null);

  onMount(() => {
    ws.load();
    ticker = setInterval(() => ws.tick(), 1000);
    api
      .onSubmitProgress((p) => ws.onProgress(p.problem_id, p.done, p.total))
      .then((u) => (unlisten = u))
      .catch(() => {});
    window.addEventListener("beforeunload", persist);
    offShortcuts = onShortcut((action) => {
      if (action === "run") ws.run();
      else if (action === "submit") ws.submit();
      else if (action === "nextProblem") go(1);
      else if (action === "prevProblem") go(-1);
    });
  });

  function persist() {
    void ws.dispose();
  }

  onDestroy(() => {
    clearInterval(ticker);
    unlisten?.();
    offShortcuts?.();
    window.removeEventListener("beforeunload", persist);
    persist();
  });

  function go(offset: number) {
    const next = order[position + offset];
    if (next !== undefined) router.go({ name: "problem", id: next });
  }

  function setTab(t: LeftTab) {
    ws.leftTab = t;
    if (t === "submissions") ws.selectSubmission(null);
  }

  async function resetCode() {
    const ok = await ask("Reset the editor to the starter code? Your current code will be discarded.", {
      title: "Reset code",
      kind: "warning",
    });
    if (ok) await ws.resetCode();
  }
</script>

{#if ws.loadError}
  <div class="center"><div class="card"><p class="bad">{ws.loadError}</p><a href="#/problems">Back to problems</a></div></div>
{:else if !view}
  <div class="center muted">Loading problem…</div>
{:else}
  <div class="workspace">
    <SplitPane direction="horizontal" initial={0.45} storageKey="lg-split-main">
      {#snippet first()}
        <section class="panel left">
          <div class="tabs" role="tablist">
            {#each [["description", "Description"], ["editorial", "Editorial"], ["submissions", "Submissions"], ["notes", "Notes"]] as [key, label] (key)}
              <button role="tab" aria-selected={ws.leftTab === key} class:active={ws.leftTab === key} onclick={() => setTab(key as LeftTab)}>
                {label}{#if key === "notes" && ws.note.trim()}<span class="dot" aria-label="has notes"> •</span>{/if}
              </button>
            {/each}
            {#if ws.selected}
              <button role="tab" aria-selected={ws.leftTab === "result"} class:active={ws.leftTab === "result"} onclick={() => setTab("result")}>Result</button>
            {/if}
            <span class="spacer"></span>
            <button class="ghost nav" onclick={() => go(-1)} disabled={position <= 0} aria-label="Previous problem">‹</button>
            <button class="ghost nav" onclick={() => go(1)} disabled={position < 0 || position >= order.length - 1} aria-label="Next problem">›</button>
          </div>
          <div class="scroll">
            {#if ws.leftTab === "description"}
              <h1>{view.id}. {view.title}</h1>
              <div class="meta">
                <span class="tag {view.difficulty.toLowerCase()}">{view.difficulty}</span>
                {#if view.status === "solved"}<span class="tag ok">Solved ✓</span>{/if}
                {#each view.lists as l (l)}<span class="tag">{l}</span>{/each}
                <button class="tag linkish" onclick={() => (showTopics = !showTopics)}>Topics</button>
                {#if view.companies.length}<button class="tag linkish" onclick={() => (showCompanies = !showCompanies)}>Companies</button>{/if}
                {#if view.hints.length}<button class="tag linkish" onclick={() => (showHints = !showHints)}>Hints</button>{/if}
              </div>
              {#if showTopics}
                <div class="reveal">{#each view.topics as t (t)}<span class="tag">{t}</span> {/each}</div>
              {/if}
              {#if showCompanies}
                <div class="reveal">
                  {#each view.companies as c (c.company)}
                    <span class="tag" title="frequency {c.frequency.toFixed(1)} · last seen {c.recency}">{c.company}</span>
                  {/each}
                </div>
              {/if}
              <article class="prose">{@html view.description_html}</article>
              {#if showHints}
                <ol class="hints">{#each view.hints as h, i (i)}<li>{h}</li>{/each}</ol>
              {/if}
            {:else if ws.leftTab === "editorial"}
              <article class="prose">{@html view.editorial_html}</article>
              {#if view.solutions.length}
                {@const shown = view.solutions.find((s) => s.language === (solutionLanguage ?? ws.language)) ?? view.solutions[0]}
                <section class="references">
                  <h2>Reference solutions</h2>
                  <div class="lang-tabs" role="tablist" aria-label="Reference solution language">
                    {#each view.solutions as s (s.language)}
                      <button
                        role="tab"
                        aria-selected={s.language === shown.language}
                        class:active={s.language === shown.language}
                        onclick={() => (solutionLanguage = s.language)}
                      >
                        {app.boot?.languages.find((l) => l.id === s.language)?.name ?? s.language}
                      </button>
                    {/each}
                  </div>
                  <div class="prose">{@html shown.html}</div>
                </section>
              {/if}
            {:else if ws.leftTab === "notes"}
              <textarea
                class="notes"
                aria-label="Notes"
                placeholder="Your notes for this problem: the key idea, pitfalls, what to review next time…"
                value={ws.note}
                oninput={(e) => ws.editNote(e.currentTarget.value)}
              ></textarea>
              <p class="muted small-note">Saved automatically. Notes are included when you export your history.</p>
            {:else if ws.leftTab === "submissions"}
              <SubmissionList submissions={ws.submissions} onSelect={(s) => ws.selectSubmission(s)} />
            {:else if ws.selected}
              <SubmissionDetail
                submission={ws.selected}
                signature={view.signature}
                benchmark={view.benchmarks[ws.selected.language] ?? null}
                solve={ws.lastSubmit?.submission.id === ws.selected.id ? ws.lastSubmit.solve : null}
                onBack={() => setTab("submissions")}
                onUseTestcase={(args) => ws.useTestcase(args)}
              />
            {/if}
          </div>
        </section>
      {/snippet}
      {#snippet second()}
        <SplitPane direction="vertical" initial={0.62} storageKey="lg-split-editor">
          {#snippet first()}
            <section class="panel">
              <div class="toolbar">
                <select
                  aria-label="Language"
                  value={ws.language}
                  onchange={(e) => ws.changeLanguage(e.currentTarget.value as LanguageId)}
                >
                  {#each (app.boot?.languages ?? []).filter((l) => view.languages.includes(l.id)) as l (l.id)}
                    <option value={l.id}>{l.name}{l.available ? "" : " (not found)"}</option>
                  {/each}
                </select>
                <span class="spacer"></span>
                {#if settings?.show_timer !== false}
                  <Timer
                    elapsed={ws.elapsed}
                    running={ws.timerRunning}
                    completed={ws.solveCompleted}
                    onToggle={() => ws.toggleTimer()}
                    onReset={() => ws.resetTimer()}
                  />
                {/if}
                <button class="ghost" onclick={resetCode} title="Reset to starter code" aria-label="Reset code">⟲</button>
              </div>
              <CodeEditor
                value={ws.code}
                language={ws.language}
                keymap={settings?.keymap ?? "standard"}
                tabSize={settings?.tab_size ?? 4}
                onChange={(c) => ws.edit(c)}
                onRun={() => ws.run()}
                onSubmit={() => ws.submit()}
              />
            </section>
          {/snippet}
          {#snippet second()}
            <section class="panel">
              <Console
                names={ws.names}
                cases={ws.cases}
                result={ws.runResult}
                running={ws.running}
                tab={ws.consoleTab}
                error={ws.runError}
                onTab={(t) => (ws.consoleTab = t)}
                onCasesChange={(c) => ws.setCases(c)}
                onResetCases={() => ws.resetCases()}
              />
              <div class="actions">
                {#if ws.progress}
                  <span class="progress" aria-label="Submission progress">
                    <span class="track"><span class="fill" style="width: {(100 * ws.progress.done) / Math.max(1, ws.progress.total)}%"></span></span>
                    {ws.progress.done}/{ws.progress.total}
                  </span>
                {/if}
                <span class="spacer"></span>
                <span class="muted hint">⌘' run · ⌘⏎ submit</span>
                <button onclick={() => ws.run()} disabled={ws.running || ws.submitting}>{ws.running ? "Running…" : "Run"}</button>
                <button class="primary" onclick={() => ws.submit()} disabled={ws.running || ws.submitting}>
                  {ws.submitting ? "Judging…" : "Submit"}
                </button>
              </div>
            </section>
          {/snippet}
        </SplitPane>
      {/snippet}
    </SplitPane>
  </div>
{/if}

<style>
  .workspace {
    height: 100%;
    padding: 8px;
  }
  .center {
    display: grid;
    place-items: center;
    height: 100%;
  }
  .panel {
    display: flex;
    flex-direction: column;
    height: 100%;
    min-height: 0;
    background: var(--panel);
    border: 1px solid var(--border);
    border-radius: var(--radius);
    overflow: hidden;
  }
  .tabs,
  .toolbar {
    display: flex;
    align-items: center;
    gap: 0.3rem;
    padding: 0.3rem 0.5rem;
    border-bottom: 1px solid var(--border);
    background: var(--panel-2);
    flex-shrink: 0;
  }
  .tabs {
    overflow-x: auto;
    scrollbar-width: none;
  }
  .tabs::-webkit-scrollbar {
    display: none;
  }
  .tabs button[role="tab"] {
    background: transparent;
    border: none;
    color: var(--muted);
    padding: 0.3em 0.55em;
    white-space: nowrap;
    flex-shrink: 0;
  }
  .tabs button.active {
    color: var(--text);
    font-weight: 700;
  }
  .nav {
    font-size: 1.2em;
    padding: 0 0.5em;
    flex-shrink: 0;
  }
  .spacer {
    flex: 1;
  }
  .scroll {
    flex: 1;
    overflow-y: auto;
    padding: 1rem 1.2rem 2rem;
  }
  h1 {
    font-size: 1.45em;
    margin: 0 0 0.6rem;
  }
  .meta,
  .reveal {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
    margin-bottom: 0.6rem;
  }
  .linkish {
    cursor: pointer;
  }
  .notes {
    width: 100%;
    min-height: 60vh;
    resize: vertical;
    line-height: 1.6;
    font-family: var(--font-family);
  }
  .small-note {
    font-size: 0.85em;
  }
  .dot {
    color: var(--accent);
  }
  .references h2 {
    font-size: 1.2em;
    margin: 1.6em 0 0.6em;
  }
  .lang-tabs {
    display: flex;
    gap: 0.3rem;
    margin-bottom: 0.4rem;
  }
  .lang-tabs button {
    background: transparent;
    border-color: transparent;
    color: var(--muted);
  }
  .lang-tabs button.active {
    color: var(--text);
    border-color: var(--border);
    background: var(--panel-2);
  }
  .hints {
    border-top: 1px solid var(--border);
    padding-top: 0.8rem;
  }
  .hints li {
    margin-bottom: 0.4rem;
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.5rem 0.7rem;
    border-top: 1px solid var(--border);
    flex-shrink: 0;
  }
  .hint {
    font-size: 0.85em;
  }
  .progress {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.85em;
  }
  .track {
    width: 140px;
    height: 6px;
    background: var(--panel-2);
    border-radius: 3px;
    overflow: hidden;
  }
  .fill {
    display: block;
    height: 100%;
    background: var(--accent);
    transition: width 0.1s linear;
  }
</style>
