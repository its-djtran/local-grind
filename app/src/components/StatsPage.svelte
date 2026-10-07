<script lang="ts">
  import { onMount } from "svelte";
  import { api, errorMessage } from "../lib/api";
  import { formatDuration, percent } from "../lib/format";
  import { hasData, severity, sortReview, trend, type ReviewSort } from "../lib/review";
  import type { Stats } from "../lib/types";
  import Heatmap from "./Heatmap.svelte";
  import TrendChart from "./TrendChart.svelte";

  let stats = $state<Stats | null>(null);
  let error = $state<string | null>(null);
  let loading = $state(false);
  let recentN = $state(10);
  let minDays = $state(7);
  let sort = $state<ReviewSort>("score");

  async function load() {
    loading = true;
    try {
      stats = await api.stats(Math.max(1, recentN || 1), Math.max(0, minDays || 0), -new Date().getTimezoneOffset());
      error = null;
    } catch (e) {
      error = errorMessage(e);
    } finally {
      loading = false;
    }
  }

  onMount(load);

  const review = $derived(stats ? sortReview(stats.review, sort) : []);
  const acceptance = $derived(
    stats && stats.totals.submissions ? (100 * stats.totals.accepted_submissions) / stats.totals.submissions : NaN,
  );
  const topics = $derived(
    stats ? [...stats.topics].sort((a, b) => b.available - a.available || a.topic.localeCompare(b.topic)) : [],
  );
</script>

<div class="page" class:loading>
  <div class="controls">
    <h1>Statistics</h1>
    <label>Recent window <input type="number" min="1" max="500" bind:value={recentN} onchange={load} aria-label="Recent window" /> solves</label>
    <label>Review after <input type="number" min="0" max="365" bind:value={minDays} onchange={load} aria-label="Review after days" /> days</label>
  </div>

  {#if error}
    <p class="bad">{error}</p>
  {:else if !stats}
    <p class="muted">Crunching numbers…</p>
  {:else}
    <section class="tiles">
      <div class="card tile"><span class="muted">Solved</span><strong>{stats.totals.solved}<small class="muted"> / {stats.totals.available}</small></strong></div>
      <div class="card tile"><span class="muted">Timed solves</span><strong>{stats.totals.solves}</strong></div>
      <div class="card tile"><span class="muted">Acceptance</span><strong>{percent(acceptance)}</strong></div>
      <div class="card tile"><span class="muted">Current streak</span><strong>{stats.totals.current_streak}<small class="muted"> days</small></strong></div>
      <div class="card tile"><span class="muted">Longest streak</span><strong>{stats.totals.longest_streak}<small class="muted"> days</small></strong></div>
    </section>

    <section class="card"><Heatmap days={stats.heatmap} /></section>

    <section class="two">
      <div class="card">
        <TrendChart
          title="Average solve time"
          points={stats.weekly.map((w) => ({ label: w.week_start, value: w.avg_ms === null ? null : w.avg_ms / 60000 }))}
          format={(minutes) => formatDuration(minutes * 60000)}
          tickFormat={(minutes) => `${minutes}m`}
        />
      </div>
      <div class="card">
        <TrendChart
          title="Error rate"
          points={stats.weekly.map((w) => ({ label: w.week_start, value: w.error_rate }))}
          format={(v) => percent(v)}
        />
      </div>
    </section>

    <section class="card">
      <h2>By difficulty</h2>
      <table>
        <thead>
          <tr>
            <th>Difficulty</th><th>Solved</th><th>Solves</th><th>Avg time</th><th>Fastest</th><th>Slowest</th><th>Error rate</th>
            <th>Last {stats.recent_n}: avg time</th><th>Last {stats.recent_n}: error rate</th>
          </tr>
        </thead>
        <tbody>
          {#each stats.by_difficulty as d (d.difficulty)}
            {@const t = trend(d.recent.avg_ms, d.overall.avg_ms, true, ["faster", "slower"])}
            {@const e = trend(d.recent.error_rate, d.overall.error_rate, true, ["fewer errors", "more errors"])}
            <tr>
              <td class={d.difficulty.toLowerCase()}>{d.difficulty}</td>
              <td>
                {d.solved} / {d.available}
                <span class="minibar"><span style="width: {(100 * d.solved) / Math.max(1, d.available)}%"></span></span>
              </td>
              <td>{d.overall.solves}</td>
              {#if hasData(d.overall)}
                <td>{formatDuration(d.overall.avg_ms)}</td>
                <td>{formatDuration(d.overall.min_ms)}</td>
                <td>{formatDuration(d.overall.max_ms)}</td>
                <td>{percent(d.overall.error_rate)}</td>
              {:else}
                <td class="muted">—</td><td class="muted">—</td><td class="muted">—</td><td class="muted">—</td>
              {/if}
              {#if hasData(d.recent)}
                <td>{formatDuration(d.recent.avg_ms)} <span class={t.tone}>{t.text}</span></td>
                <td>{percent(d.recent.error_rate)} <span class={e.tone}>{e.text}</span></td>
              {:else}
                <td class="muted">—</td><td class="muted">—</td>
              {/if}
            </tr>
          {/each}
        </tbody>
      </table>
    </section>

    <section class="card">
      <div class="row-head">
        <h2>Review queue</h2>
        <label>Sort by
          <select bind:value={sort} aria-label="Sort review queue">
            <option value="score">Priority</option>
            <option value="passRate">Lowest pass rate</option>
            <option value="time">Slowest</option>
            <option value="stale">Longest since solved</option>
          </select>
        </label>
      </div>
      <p class="muted small">Ranks problems you solved at least {minDays} days ago: 45% pass rate, 35% time versus your own average for that difficulty, 20% staleness.</p>
      {#if review.length === 0}
        <p class="muted">Nothing due yet. Solve problems and they will come back here for review.</p>
      {:else}
        <ul class="review">
          {#each review as r (r.slug)}
            <li>
              <span class="score sev-{severity(r.score)}" title="Review priority">{r.score}</span>
              <span class="what">
                {#if r.problem_id !== null}<a href="#/problems/{r.problem_id}">{r.title}</a>{:else}{r.title}{/if}
                <span class="{r.difficulty.toLowerCase()} small">{r.difficulty}</span>
                <span class="reasons">{#each r.reasons as reason (reason.text)}<span class="tag {reason.severity}">{reason.text}</span>{/each}</span>
              </span>
              <span class="muted small">{r.solves} solve{r.solves === 1 ? "" : "s"} · avg {formatDuration(r.avg_ms)} · {Math.floor(r.days_since)}d ago</span>
            </li>
          {/each}
        </ul>
      {/if}
    </section>

    <div class="two">
      <section class="card">
        <h2>Lists</h2>
        {#each stats.lists as l (l.id)}
          <div class="progress-row">
            <span>{l.name}</span>
            <span class="minibar wide"><span style="width: {(100 * l.solved) / Math.max(1, l.total)}%"></span></span>
            <span class="muted">{l.solved} / {l.total}</span>
          </div>
        {:else}
          <p class="muted">No lists in this catalog.</p>
        {/each}
      </section>
      <section class="card">
        <h2>Topics</h2>
        <table>
          <thead><tr><th>Topic</th><th>Solved</th><th>Avg time</th><th>Error rate</th></tr></thead>
          <tbody>
            {#each topics as t (t.topic)}
              <tr>
                <td>{t.topic}</td>
                <td>{t.solved} / {t.available}</td>
                <td>{t.solves ? formatDuration(t.avg_ms) : "—"}</td>
                <td>{t.solves ? percent(t.error_rate) : "—"}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </section>
    </div>
  {/if}
</div>

<style>
  .page {
    height: 100%;
    overflow-y: auto;
    padding: 1.2rem max(1.2rem, calc((100% - 1100px) / 2));
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
  .page.loading {
    opacity: 0.7;
  }
  .controls {
    display: flex;
    flex-wrap: wrap;
    gap: 1.2rem;
    align-items: center;
  }
  .controls h1 {
    margin: 0 auto 0 0;
    font-size: 1.5em;
  }
  .controls input {
    width: 4.5em;
  }
  .tiles {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    gap: 0.8rem;
  }
  .tile {
    display: flex;
    flex-direction: column;
  }
  .tile strong {
    font-size: 1.8em;
  }
  .tile small {
    font-size: 0.5em;
  }
  h2 {
    font-size: 1.1em;
    margin: 0 0 0.6rem;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-variant-numeric: tabular-nums;
  }
  th {
    text-align: left;
    color: var(--muted);
    font-weight: 400;
    border-bottom: 1px solid var(--border);
    padding: 0.35rem 0.4rem;
  }
  td {
    padding: 0.4rem;
    border-bottom: 1px solid var(--border);
  }
  .minibar {
    display: inline-block;
    width: 60px;
    height: 6px;
    background: var(--panel-2);
    border-radius: 3px;
    overflow: hidden;
    vertical-align: middle;
    margin-left: 0.4em;
  }
  .minibar.wide {
    width: 100%;
    margin: 0;
  }
  .minibar span {
    display: block;
    height: 100%;
    background: var(--ok);
    border-radius: 3px;
  }
  .row-head {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .small {
    font-size: 0.85em;
  }
  .review {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  .review li {
    display: grid;
    grid-template-columns: 3rem 1fr auto;
    gap: 0.8rem;
    align-items: center;
    padding: 0.5rem 0;
    border-bottom: 1px solid var(--border);
  }
  .review a:hover {
    color: var(--accent);
  }
  .reasons {
    display: inline-flex;
    flex-wrap: wrap;
    gap: 0.3rem;
    margin-left: 0.5rem;
  }
  .tag.bad {
    color: var(--bad);
  }
  .tag.warn {
    color: var(--warn);
  }
  .score {
    text-align: center;
    font-weight: 700;
    border-radius: 6px;
    padding: 0.2em 0;
    border: 1px solid var(--border);
  }
  .sev-high {
    color: var(--bad);
  }
  .sev-med {
    color: var(--warn);
  }
  .sev-low {
    color: var(--muted);
  }
  .two {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
    gap: 1rem;
  }
  .progress-row {
    display: grid;
    grid-template-columns: 7rem 1fr 4.5rem;
    gap: 0.6rem;
    align-items: center;
    margin-bottom: 0.5rem;
  }
</style>
