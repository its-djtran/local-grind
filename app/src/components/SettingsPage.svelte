<script lang="ts">
  import { open, save, ask } from "@tauri-apps/plugin-dialog";
  import { api, errorMessage } from "../lib/api";
  import { app } from "../lib/state.svelte";
  import type { CalibrationStatus, Settings, ToolchainStatus } from "../lib/types";
  import { formatDateTime } from "../lib/format";
  import { onMount } from "svelte";

  let draft = $state<Settings>(structuredClone($state.snapshot(app.settings!)));
  let applySettingsOnImport = $state(false);
  let message = $state<string | null>(null);
  let toolchains = $state<ToolchainStatus[] | null>(null);

  async function probe() {
    toolchains = null;
    toolchains = await api.checkToolchains().catch((e) => [
      { language: "python" as const, executable: "", version: null, ok: false, message: errorMessage(e) },
    ]);
  }

  let calibration = $state<CalibrationStatus | null>(null);
  let calibrating = $state(false);
  let calibrationProgress = $state<{ done: number; total: number } | null>(null);
  let calibrationMessage = $state<string | null>(null);

  async function loadCalibration() {
    calibration = await api.calibrationStatus().catch(() => null);
  }

  async function recalibrate() {
    calibrating = true;
    calibrationMessage = null;
    calibrationProgress = null;
    const unlisten = await api.onCalibrationProgress((p) => (calibrationProgress = { done: p.done, total: p.total })).catch(() => null);
    try {
      const s = await api.recalibrate(3);
      calibrationMessage = s.cancelled
        ? `Stopped after ${s.measured} reference solutions.`
        : `Measured ${s.measured} reference solutions${s.failed ? ` (${s.failed} failed)` : ""}.`;
    } catch (e) {
      calibrationMessage = `Recalibration failed: ${errorMessage(e)}`;
    } finally {
      unlisten?.();
      calibrating = false;
      calibrationProgress = null;
      await loadCalibration();
    }
  }

  async function revertBenchmarks() {
    await api.clearLocalBenchmarks();
    calibrationMessage = "Using the bundled benchmarks again.";
    await loadCalibration();
  }

  onMount(() => {
    probe();
    loadCalibration();
  });

  async function commit() {
    await app.saveSettings($state.snapshot(draft));
    draft = structuredClone($state.snapshot(app.settings!));
    await probe();
  }

  async function exportData() {
    const path = await save({ defaultPath: "localgrind-history.json", filters: [{ name: "JSON", extensions: ["json"] }] });
    if (!path) return;
    try {
      await api.exportHistory(path);
      message = `Exported to ${path}`;
    } catch (e) {
      message = errorMessage(e);
    }
  }

  async function importData() {
    const path = await open({ multiple: false, filters: [{ name: "JSON", extensions: ["json"] }] });
    if (!path || Array.isArray(path)) return;
    try {
      const s = await api.importHistory(path, applySettingsOnImport);
      message = `Imported ${s.submissions_added} submissions and ${s.solves_added} solves (${s.submissions_skipped + s.solves_skipped} duplicates skipped).`;
      await app.load();
      draft = structuredClone($state.snapshot(app.settings!));
    } catch (e) {
      message = `Import failed: ${errorMessage(e)}`;
    }
  }

  async function clearData() {
    const ok = await ask("Delete all submissions, solve times and drafts? Settings are kept. Export first if you may want them back.", {
      title: "Clear history",
      kind: "warning",
    });
    if (!ok) return;
    await api.clearHistory();
    await app.load();
    message = "History cleared.";
  }
</script>

<div class="page">
  <h1>Settings</h1>

  <section class="card">
    <h2>Appearance</h2>
    <label class="row">Theme
      <select bind:value={draft.theme} onchange={commit} aria-label="Theme">
        <option value="dark">Dark</option>
        <option value="light">Light</option>
        <option value="system">Match system</option>
      </select>
    </label>
    <label class="row">Interface text size
      <span class="inline"><input type="range" min="10" max="32" bind:value={draft.font_size} onchange={commit} aria-label="Interface text size" /> {draft.font_size}px</span>
    </label>
    <label class="row">Editor text size
      <span class="inline"><input type="range" min="10" max="32" bind:value={draft.editor_font_size} onchange={commit} aria-label="Editor text size" /> {draft.editor_font_size}px</span>
    </label>
    <label class="row">Font family
      <input type="text" bind:value={draft.font_family} onchange={commit} aria-label="Font family" />
    </label>
  </section>

  <section class="card">
    <h2>Editor</h2>
    <label class="row">Default language
      <select bind:value={draft.language} onchange={commit} aria-label="Default language">
        {#each app.boot?.languages ?? [] as l (l.id)}<option value={l.id}>{l.name}</option>{/each}
      </select>
    </label>
    <label class="row">Key bindings
      <select bind:value={draft.keymap} onchange={commit} aria-label="Key bindings">
        <option value="standard">Standard</option>
        <option value="vim">Vim</option>
      </select>
    </label>
    <label class="row">Tab size
      <input type="number" min="1" max="8" bind:value={draft.tab_size} onchange={commit} aria-label="Tab size" />
    </label>
  </section>

  <section class="card">
    <h2>Timer</h2>
    <label class="row">Start the timer on the first keystroke
      <input type="checkbox" bind:checked={draft.auto_start_timer} onchange={commit} aria-label="Auto-start timer" />
    </label>
    <label class="row">Show the timer in the editor
      <input type="checkbox" bind:checked={draft.show_timer} onchange={commit} aria-label="Show timer" />
    </label>
  </section>

  <section class="card">
    <h2>Evaluation</h2>
    <p class="muted small">Tune judging to your machine. Limits apply to the summed solution time across all tests.</p>
    <label class="row">Time limit multiplier
      <input type="number" step="0.25" min="0.25" max="20" bind:value={draft.eval.time_limit_multiplier} onchange={commit} aria-label="Time limit multiplier" />
    </label>
    <label class="row">Memory limit (MB)
      <input type="number" min="64" max="65536" step="64" bind:value={draft.eval.memory_limit_mb} onchange={commit} aria-label="Memory limit" />
    </label>
    <label class="row">Max parallel judge processes
      <input type="number" min="1" max="64" bind:value={draft.eval.max_parallel_jobs} onchange={commit} aria-label="Max parallel jobs" />
    </label>
    <label class="row">Judge at low priority (keeps the machine responsive; timings vary more)
      <input type="checkbox" bind:checked={draft.eval.low_priority} onchange={commit} aria-label="Low priority judging" />
    </label>
    {#each app.boot?.languages ?? [] as l (l.id)}
      <label class="row">{l.name} toolchain path
        <input
          type="text"
          placeholder="auto-detect"
          value={draft.eval.toolchain_paths[l.id] ?? ""}
          onchange={(e) => {
            draft.eval.toolchain_paths = { ...draft.eval.toolchain_paths, [l.id]: e.currentTarget.value };
            commit();
          }}
          aria-label="{l.name} toolchain path"
        />
      </label>
    {/each}
    <ul class="langs" aria-label="Toolchains">
      {#if toolchains === null}
        <li class="muted">Checking toolchains…</li>
      {:else}
        {#each toolchains as t (t.language)}
          <li>
            <span class={t.ok ? "ok" : "bad"}>{t.ok ? "✓" : "✗"}</span>
            {t.version ?? t.language} <span class="muted">{t.executable}</span>
            {#if t.message}<div class="bad small">{t.message}</div>{/if}
          </li>
        {/each}
      {/if}
    </ul>
  </section>

  <section class="card">
    <h2>Benchmarks</h2>
    <p class="muted small">
      Submissions are compared with the optimal reference solution in the same language. The bundled numbers were measured
      on the machine that built the app; recalibrate to measure them on yours (each reference runs 3 times, using up to
      {draft.eval.max_parallel_jobs} judge processes at once).
    </p>
    {#if calibration}
      <p>
        {#if calibration.local_benchmarks > 0}
          Using {calibration.local_benchmarks} benchmarks measured on this machine{calibration.measured_at ? ` (${formatDateTime(calibration.measured_at)})` : ""}.
        {:else}
          Using the bundled benchmarks.
        {/if}
      </p>
    {/if}
    <div class="buttons">
      {#if calibrating}
        <span class="progress" aria-label="Recalibration progress">
          <span class="track"><span class="fill" style="width: {calibrationProgress ? (100 * calibrationProgress.done) / Math.max(1, calibrationProgress.total) : 0}%"></span></span>
          {calibrationProgress ? `${calibrationProgress.done}/${calibrationProgress.total}` : "Starting…"}
        </span>
        <button onclick={() => api.cancelRecalibration()}>Stop</button>
      {:else}
        <button onclick={recalibrate}>Recalibrate on this machine</button>
        {#if calibration && calibration.local_benchmarks > 0}
          <button onclick={revertBenchmarks}>Use bundled benchmarks</button>
        {/if}
      {/if}
    </div>
    {#if calibrationMessage}<p role="status">{calibrationMessage}</p>{/if}
  </section>

  <section class="card">
    <h2>Data</h2>
    <p class="muted small">Export everything (submissions, solve times, drafts, settings) to one JSON file, or import a previous export.</p>
    <div class="buttons">
      <button onclick={exportData}>Export history…</button>
      <button onclick={importData}>Import…</button>
      <label><input type="checkbox" bind:checked={applySettingsOnImport} /> also apply imported settings</label>
      <button class="danger" onclick={clearData}>Clear history…</button>
    </div>
    {#if message}<p role="status">{message}</p>{/if}
    {#if app.boot?.catalog_built_at}<p class="muted small">Problem catalog built {app.boot.catalog_built_at}.</p>{/if}
  </section>
</div>

<style>
  .page {
    height: 100%;
    overflow-y: auto;
    padding: 1.2rem max(1.2rem, calc((100% - 760px) / 2));
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  h1 {
    font-size: 1.5em;
    margin: 0;
  }
  h2 {
    font-size: 1.1em;
    margin: 0 0 0.6rem;
  }
  .row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
    padding: 0.4rem 0;
    border-bottom: 1px solid var(--border);
  }
  .row:last-of-type {
    border-bottom: none;
  }
  .row input[type="number"] {
    width: 7em;
  }
  .row input[type="text"] {
    width: 18em;
  }
  .small {
    font-size: 0.85em;
  }
  .inline {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
  }
  .langs {
    list-style: none;
    padding: 0;
    margin: 0.6rem 0 0;
  }
  .buttons {
    display: flex;
    flex-wrap: wrap;
    gap: 0.6rem;
    align-items: center;
  }
  .progress {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
  }
  .track {
    width: 200px;
    height: 6px;
    background: var(--panel-2);
    border-radius: 3px;
    overflow: hidden;
  }
  .fill {
    display: block;
    height: 100%;
    background: var(--accent);
  }
  .danger {
    margin-left: auto;
    color: var(--bad);
  }
</style>
