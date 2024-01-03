<script>
  import { fade } from "svelte/transition";
  import { createEventDispatcher } from "svelte";
  const dispatch = createEventDispatcher();
  export let hidden;
</script>

{#if !hidden}
  <div transition:fade class="back-drop" on:click|self={() => dispatch("add")}>
    <slot {hidden} />
  </div>
{/if}

<style>
  .back-drop {
    width: 100%;
    height: 100%;
    position: fixed;
    background: rgba(0, 0, 0, 0.2);
  }
</style>
