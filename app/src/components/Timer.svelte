<script lang="ts">
  import { formatClock } from "../lib/format";

  interface Props {
    elapsed: number;
    running: boolean;
    completed: boolean;
    onToggle: () => void;
    onReset: () => void;
  }

  let { elapsed, running, completed, onToggle, onReset }: Props = $props();
</script>

<div class="timer" class:running class:completed>
  <button class="ghost" onclick={onToggle} title={running ? "Pause timer" : "Start timer"} aria-label={running ? "Pause timer" : "Start timer"}>
    {running ? "❚❚" : "▶"}
  </button>
  <span class="clock" aria-label="Elapsed time">{formatClock(elapsed)}</span>
  <button class="ghost" onclick={onReset} title="Discard this attempt's timer" aria-label="Reset timer">↺</button>
</div>

<style>
  .timer {
    display: inline-flex;
    align-items: center;
    gap: 0.2rem;
    border: 1px solid var(--border);
    border-radius: 6px;
    padding: 0 0.2rem;
  }
  .timer button {
    padding: 0.1em 0.4em;
  }
  .clock {
    font-variant-numeric: tabular-nums;
    min-width: 3.6em;
    text-align: center;
  }
  .running .clock {
    color: var(--accent);
  }
  .completed .clock {
    color: var(--ok);
  }
</style>
