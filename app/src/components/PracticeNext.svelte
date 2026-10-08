<script lang="ts">
  import { onMount } from "svelte";
  import { api } from "../lib/api";
  import { nextInLists } from "../lib/filter";
  import { severity } from "../lib/review";
  import type { ProblemList, ProblemListing, ReviewItem } from "../lib/types";

  let { problems, lists }: { problems: ProblemListing[]; lists: ProblemList[] } = $props();

  let review = $state<ReviewItem[]>([]);
  const next = $derived(nextInLists(problems, lists));

  onMount(async () => {
    try {
      review = await api.reviewQueue(7, 3);
    } catch {
      review = [];
    }
  });
</script>

{#if review.length || next.length}
  <section class="practice" aria-label="Practice next">
    {#each next as n (n.listId)}
      <a class="item card" href="#/problems/{n.problem.id}">
        <span class="muted small">Next in {n.listName} · {n.group}</span>
        <strong>{n.problem.id}. {n.problem.title}</strong>
        <span class="small"><span class={n.problem.difficulty.toLowerCase()}>{n.problem.difficulty}</span> · {n.remaining} left</span>
      </a>
    {/each}
    {#each review as r (r.slug)}
      <a class="item card" href="#/problems/{r.problem_id}">
        <span class="muted small">Due for review · priority <span class="sev-{severity(r.score)}">{r.score}</span></span>
        <strong>{r.title}</strong>
        <span class="small muted">{r.reasons[0]?.text}</span>
      </a>
    {/each}
  </section>
{/if}

<style>
  .practice {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
    gap: 0.8rem;
    margin-bottom: 1rem;
  }
  .item {
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
    padding: 0.7rem 0.9rem;
  }
  .item:hover {
    border-color: var(--accent);
  }
  .small {
    font-size: 0.85em;
  }
  .sev-high {
    color: var(--bad);
  }
  .sev-med {
    color: var(--warn);
  }
</style>
