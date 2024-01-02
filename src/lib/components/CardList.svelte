<script>
  export let cards;
</script>

<div class="card-list">
  {#each cards as card}
    <div>{card.first_name}</div>
  {/each}
</div>

<style>
  .card-list {
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-gap: 20px;
  }
</style>
