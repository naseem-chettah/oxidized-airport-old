<script>
  import { scale } from "svelte/transition";
  import { flip } from "svelte/animate";
  import { createEventDispatcher } from "svelte";
  import PassengersStore from "../../stores/PassengersStore";
  import CardDetails from "./CardDetails.svelte";

  const dispatch = createEventDispatcher();

  $: total = $PassengersStore.length;
</script>

<p>total of passengers: {total}</p>
<div class="card-list">
  {#each $PassengersStore as card (card.id)}
    <div out:scale|local animate:flip={{ duration: 500 }}>
      <CardDetails {card} />
    </div>
  {/each}
  <div>
    <CardDetails
      on:click={() => {
        dispatch("add");
      }}
    />
  </div>
</div>

<style>
  .card-list {
    display: grid;
    grid-template-columns: repeat(3, 2fr);
    grid-gap: 20px;
  }
</style>
