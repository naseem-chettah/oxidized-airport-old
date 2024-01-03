<script>
  import { createEventDispatcher } from "svelte";

  const dispatch = createEventDispatcher();

  export let hidden;
</script>

{#if !hidden}
  <div
    class="back-drop"
    on:click|self={() => {
      dispatch("hide");
    }}
  >
    <slot {hidden} />
  </div>
{/if}

<style>
  .back-drop {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 9999;
    background: rgba(0, 0, 0, 0.3);
  }
</style>
