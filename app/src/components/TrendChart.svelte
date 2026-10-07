<script lang="ts">
  import { labelIndices, linePath, nearestIndex, niceScale, segments, shortDate, ticks, type Point } from "../lib/chart";

  interface Props {
    title: string;
    points: Point[];
    format: (v: number) => string;
    /** Axis tick formatter; defaults to `format`. */
    tickFormat?: (v: number) => string;
  }

  let { title, points, format, tickFormat }: Props = $props();

  const W = 560;
  const H = 180;
  const LEFT = 52;
  const RIGHT = 16;
  const TOP = 12;
  const BOTTOM = 26;
  const plotW = W - LEFT - RIGHT;
  const plotH = H - TOP - BOTTOM;

  let active = $state<number | null>(null);
  let showTable = $state(false);
  let svg = $state<SVGSVGElement>();

  const values = $derived(points.map((p) => p.value).filter((v): v is number => v !== null && Number.isFinite(v)));
  const scale = $derived(niceScale(Math.max(0, ...values)));
  const yTicks = $derived(ticks(scale.max, scale.step));
  const runs = $derived(segments(points));
  const xLabels = $derived(labelIndices(points.length));
  const lastIndex = $derived(runs.length ? runs[runs.length - 1].at(-1)!.index : null);
  const fmtTick = $derived(tickFormat ?? format);

  const x = (i: number) => LEFT + (points.length <= 1 ? plotW / 2 : (i / (points.length - 1)) * plotW);
  const y = (v: number) => TOP + plotH - (v / scale.max) * plotH;

  function onMove(e: PointerEvent) {
    const rect = svg!.getBoundingClientRect();
    if (rect.width <= 0) return;
    const px = ((e.clientX - rect.left) / rect.width) * W;
    active = nearestIndex(px, LEFT, plotW, points.length);
  }

  function onKey(e: KeyboardEvent) {
    if (!points.length) return;
    if (e.key === "ArrowRight") active = Math.min(points.length - 1, (active ?? -1) + 1);
    else if (e.key === "ArrowLeft") active = Math.max(0, (active ?? points.length) - 1);
    else if (e.key === "Escape") active = null;
    else return;
    e.preventDefault();
  }
</script>

<figure class="trend">
  <figcaption>
    <span>{title}</span>
    <button class="ghost small" onclick={() => (showTable = !showTable)} aria-pressed={showTable}>
      {showTable ? "Chart" : "Table"}
    </button>
  </figcaption>
  {#if showTable}
    <table>
      <thead><tr><th>Week of</th><th>{title}</th></tr></thead>
      <tbody>
        {#each points as p (p.label)}
          <tr><td>{shortDate(p.label)}</td><td>{p.value === null ? "—" : format(p.value)}</td></tr>
        {/each}
      </tbody>
    </table>
  {:else if !values.length}
    <p class="muted empty">No timed solves in this period yet.</p>
  {:else}
    <div class="plot">
      <!-- Keyboard-explorable chart: arrow keys move the crosshair, matching pointer hover. -->
      <!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
      <svg
        bind:this={svg}
        viewBox="0 0 {W} {H}"
        role="img"
        aria-label="{title} by week"
        tabindex="0"
        onpointermove={onMove}
        onpointerleave={() => (active = null)}
        onkeydown={onKey}
        onblur={() => (active = null)}
      >
        {#each yTicks as t (t)}
          <line class="grid" x1={LEFT} x2={W - RIGHT} y1={y(t)} y2={y(t)} />
          <text class="tick" x={LEFT - 8} y={y(t) + 4} text-anchor="end">{fmtTick(t)}</text>
        {/each}
        {#each xLabels as i (i)}
          <text class="tick" x={x(i)} y={H - 6} text-anchor="middle">{shortDate(points[i].label)}</text>
        {/each}
        {#each runs as run, k (k)}
          <path class="line" d={linePath(run, x, y)} />
          {#each run as p (p.index)}<circle class="dot" cx={x(p.index)} cy={y(p.value)} r="2.5" />{/each}
        {/each}
        {#if lastIndex !== null && points[lastIndex].value !== null}
          <circle class="end" cx={x(lastIndex)} cy={y(points[lastIndex].value!)} r="4" />
        {/if}
        {#if active !== null}
          <line class="crosshair" x1={x(active)} x2={x(active)} y1={TOP} y2={TOP + plotH} />
          {#if points[active].value !== null}
            <circle class="end" cx={x(active)} cy={y(points[active].value!)} r="4" />
          {/if}
        {/if}
      </svg>
      {#if active !== null}
        <div class="tooltip" role="status" style="left: {(x(active) / W) * 100}%">
          <strong>{points[active].value === null ? "No solves" : format(points[active].value!)}</strong>
          <span class="muted">Week of {shortDate(points[active].label)}</span>
        </div>
      {/if}
    </div>
  {/if}
</figure>

<style>
  .trend {
    margin: 0;
  }
  figcaption {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.3rem;
    color: var(--text-2);
  }
  .small {
    font-size: 0.85em;
    padding: 0.1em 0.5em;
  }
  .plot {
    position: relative;
  }
  svg {
    width: 100%;
    display: block;
    outline: none;
  }
  svg:focus-visible {
    box-shadow: 0 0 0 2px var(--accent);
    border-radius: 4px;
  }
  .grid {
    stroke: var(--border);
    stroke-width: 1;
  }
  .tick {
    fill: var(--muted);
    font-size: 10px;
    font-family: var(--font-family);
    font-variant-numeric: tabular-nums;
  }
  .line {
    fill: none;
    stroke: var(--chart-1);
    stroke-width: 2;
    stroke-linejoin: round;
    stroke-linecap: round;
  }
  .dot {
    fill: var(--chart-1);
  }
  .end {
    fill: var(--chart-1);
    stroke: var(--panel);
    stroke-width: 2;
  }
  .crosshair {
    stroke: var(--muted);
    stroke-width: 1;
  }
  .tooltip {
    position: absolute;
    top: 0;
    transform: translateX(-50%);
    background: var(--panel-2);
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 0.25em 0.6em;
    display: flex;
    flex-direction: column;
    pointer-events: none;
    white-space: nowrap;
    font-size: 0.85em;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-variant-numeric: tabular-nums;
  }
  th,
  td {
    text-align: left;
    padding: 0.2rem 0.4rem;
    border-bottom: 1px solid var(--border);
  }
  th {
    color: var(--muted);
    font-weight: 400;
  }
  .empty {
    padding: 1rem 0;
  }
</style>
