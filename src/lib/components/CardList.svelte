<script>
  import { createEventDispatcher } from "svelte";
  import Button from "./Button.svelte";
  import CardDetails from "./CardDetails.svelte";

  export let cards;
  $: total = cards.length;
</script>

<div>
  <p>total of passengers: {total}</p>
  <div class="card-list">
    {#each cards as card}
      <div>
        <CardDetails {card} />
      </div>
    {:else}
      no passengers left :/
    {/each}
  </div>
  <Button on:click={() => console.log("button clicked")}>Add</Button>
</div>

<style>
  .card-list {
    display: grid;
    grid-template-columns: repeat(3, 2fr);
    grid-gap: 20px;
  }
</style>
