<script lang="ts">
  import { buildCalendar, monthLabels } from "../lib/calendar";
  import type { DayCount } from "../lib/types";

  let { days, today = new Date() }: { days: DayCount[]; today?: Date } = $props();

  const CELL = 11;
  const GAP = 2;
  const columns = $derived(buildCalendar(days, today));
  const months = $derived(monthLabels(columns));
  const total = $derived(days.reduce((a, d) => a + d.submissions, 0));
  const width = $derived(columns.length * (CELL + GAP) + 28);
</script>

<figure class="heatmap">
  <figcaption><strong>{total}</strong> submissions in the last year</figcaption>
  <svg viewBox="0 0 {width} {7 * (CELL + GAP) + 18}" role="img" aria-label="Submission activity calendar">
    {#each months as m (m.index)}
      <text x={28 + m.index * (CELL + GAP)} y="10" class="label">{m.label}</text>
    {/each}
    {#each [["Mon", 1], ["Wed", 3], ["Fri", 5]] as [d, row] (d)}
      <text x="0" y={18 + (row as number) * (CELL + GAP) + CELL - 2} class="label">{d}</text>
    {/each}
    {#each columns as col, i (i)}
      {#each col as cell, j (cell.date)}
        <rect
          x={28 + i * (CELL + GAP)}
          y={18 + j * (CELL + GAP)}
          width={CELL}
          height={CELL}
          rx="2"
          class="l{cell.level}"
          aria-label="{cell.date}: {cell.submissions} submissions, {cell.accepted} accepted"
        >
          <title>{cell.date}: {cell.submissions} submission{cell.submissions === 1 ? "" : "s"}, {cell.accepted} accepted</title>
        </rect>
      {/each}
    {/each}
  </svg>
  <div class="legend muted" aria-hidden="true">
    Less
    {#each [0, 1, 2, 3, 4] as l (l)}<span class="swatch l{l}"></span>{/each}
    More
  </div>
</figure>

<style>
  .heatmap {
    margin: 0;
  }
  figcaption {
    margin-bottom: 0.4rem;
  }
  svg {
    width: 100%;
    max-width: 760px;
    display: block;
  }
  .label {
    fill: var(--muted);
    font-size: 9px;
    font-family: var(--font-family);
  }
  rect:hover {
    stroke: var(--text);
    stroke-width: 1;
  }
  .l0 {
    fill: var(--heat-0);
    background: var(--heat-0);
  }
  .l1 {
    fill: var(--heat-1);
    background: var(--heat-1);
  }
  .l2 {
    fill: var(--heat-2);
    background: var(--heat-2);
  }
  .l3 {
    fill: var(--heat-3);
    background: var(--heat-3);
  }
  .l4 {
    fill: var(--heat-4);
    background: var(--heat-4);
  }
  .legend {
    display: flex;
    align-items: center;
    gap: 3px;
    justify-content: flex-end;
    font-size: 0.8em;
    max-width: 760px;
  }
  .swatch {
    width: 11px;
    height: 11px;
    border-radius: 2px;
    display: inline-block;
  }
</style>
