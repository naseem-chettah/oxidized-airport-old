<script>
  import { createEventDispatcher } from "svelte";
  import PassengersStore from "../../stores/PassengersStore";
  import Button from "./Button.svelte";
  import CardDetails from "./CardDetails.svelte";

  const dispatch = createEventDispatcher();

  $: total = $PassengersStore.length;
</script>

<p>total of passengers: {total}</p>
<div class="card-list">
  {#each $PassengersStore as card}
    <div>
      <CardDetails {card} />
    </div>
  {:else}
    no passengers left :/
  {/each}
</div>
<Button on:click={() => dispatch("add")}>Add</Button>

<style>
  .card-list {
    display: grid;
    grid-template-columns: repeat(3, 2fr);
    grid-gap: 20px;
  }
</style>
