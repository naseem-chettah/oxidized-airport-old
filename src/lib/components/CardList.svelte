<script>
  import { createEventDispatcher } from "svelte";
  import CardDetails from "./CardDetails.svelte";

  const dispatch = createEventDispatcher();

  export let items;
  export let cards;

  $: total = cards.length;
</script>

<p>total of {items} : {total}</p>
<div class="card-list">
  {#each cards as card (card.id)}
    <CardDetails {card} />
  {/each}
  <CardDetails
    on:click={() => {
      dispatch("add");
    }}
  />
</div>

<style>
  .card-list {
    display: grid;
    grid-template-columns: repeat(3, 2fr);
    grid-gap: 20px;
  }
</style>
