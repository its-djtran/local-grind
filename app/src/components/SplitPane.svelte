<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    direction?: "horizontal" | "vertical";
    /** Initial fraction of space given to the first pane. */
    initial?: number;
    min?: number;
    storageKey?: string;
    first: Snippet;
    second: Snippet;
  }

  let { direction = "horizontal", initial = 0.5, min = 0.15, storageKey, first, second }: Props = $props();

  function stored(): number {
    const v = storageKey ? Number(localStorage.getItem(storageKey)) : NaN;
    return Number.isFinite(v) && v > 0 ? v : initial;
  }

  let ratio = $state(stored());
  let container: HTMLDivElement;
  let dragging = $state(false);

  export function clampRatio(r: number): number {
    return Math.min(1 - min, Math.max(min, r));
  }

  function onPointerDown(e: PointerEvent) {
    dragging = true;
    (e.target as HTMLElement).setPointerCapture?.(e.pointerId);
  }

  function onPointerMove(e: PointerEvent) {
    if (!dragging) return;
    const rect = container.getBoundingClientRect();
    const size = direction === "horizontal" ? rect.width : rect.height;
    if (size <= 0) return;
    const pos = direction === "horizontal" ? e.clientX - rect.left : e.clientY - rect.top;
    ratio = clampRatio(pos / size);
  }

  function onPointerUp() {
    if (!dragging) return;
    dragging = false;
    if (storageKey) localStorage.setItem(storageKey, String(ratio));
  }

  function onKeyDown(e: KeyboardEvent) {
    const step = e.shiftKey ? 0.1 : 0.02;
    const back = direction === "horizontal" ? "ArrowLeft" : "ArrowUp";
    const fwd = direction === "horizontal" ? "ArrowRight" : "ArrowDown";
    if (e.key === back) ratio = clampRatio(ratio - step);
    else if (e.key === fwd) ratio = clampRatio(ratio + step);
    else return;
    e.preventDefault();
    if (storageKey) localStorage.setItem(storageKey, String(ratio));
  }
</script>

<div class="split {direction}" class:dragging bind:this={container}>
  <div class="pane" style="flex: {ratio} 1 0">{@render first()}</div>
  <!-- A focusable separator is the ARIA "window splitter" pattern. -->
  <!-- svelte-ignore a11y_no_noninteractive_tabindex, a11y_no_noninteractive_element_interactions -->
  <div
    class="divider"
    role="separator"
    aria-orientation={direction === "horizontal" ? "vertical" : "horizontal"}
    aria-valuenow={Math.round(ratio * 100)}
    tabindex="0"
    onpointerdown={onPointerDown}
    onpointermove={onPointerMove}
    onpointerup={onPointerUp}
    onkeydown={onKeyDown}
  ></div>
  <div class="pane" style="flex: {1 - ratio} 1 0">{@render second()}</div>
</div>

<style>
  .split {
    display: flex;
    width: 100%;
    height: 100%;
    min-width: 0;
    min-height: 0;
  }
  .split.vertical {
    flex-direction: column;
  }
  .split.dragging {
    user-select: none;
  }
  .pane {
    min-width: 0;
    min-height: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }
  .divider {
    flex: 0 0 8px;
    background: transparent;
    position: relative;
  }
  .horizontal > .divider {
    cursor: col-resize;
  }
  .vertical > .divider {
    cursor: row-resize;
  }
  .divider::after {
    content: "";
    position: absolute;
    inset: 0;
    margin: auto;
    border-radius: 2px;
    background: var(--border);
  }
  .horizontal > .divider::after {
    width: 2px;
    height: 32px;
  }
  .vertical > .divider::after {
    height: 2px;
    width: 32px;
  }
  .divider:hover::after,
  .divider:focus-visible::after,
  .dragging > .divider::after {
    background: var(--accent);
  }
  .horizontal > .divider:hover::after,
  .dragging.horizontal > .divider::after {
    height: 100%;
  }
  .vertical > .divider:hover::after,
  .dragging.vertical > .divider::after {
    width: 100%;
  }
</style>
