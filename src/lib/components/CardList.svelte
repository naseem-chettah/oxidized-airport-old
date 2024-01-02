<script>
  import CardDetails from "./CardDetails.svelte";
  export let cards;
  $: total = cards.length;
</script>

<p>total of passengers: {total}</p>
<div class="card-list">
  {#each cards as card}
    <div>
      <CardDetails {card} />
    </div>
  {/each}
</div>

<style>
  .card-list {
    display: grid;
    grid-template-columns: 1fr 4fr;
    grid-gap: 20px;
  }
</style>
