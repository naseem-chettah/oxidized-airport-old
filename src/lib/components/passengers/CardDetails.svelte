<script>
  import PassengersStore from "../../../stores/PassengersStore";
  import Button from "../../shared/Button.svelte";
  export let card = "plus";

  const handleDelete = (id) => {
    PassengersStore.update((currentPassengers) => {
      return currentPassengers.filter((card) => card.id != id);
    });
  };
</script>

{#if card != "plus"}
  <div class="card">
    <div>
      <h3>Passenger {card.id}</h3>
      <p><b>full name:</b> {card.firstName} {card.lastName}</p>
      <p><b>date of birth:</b> {card.dateOfBirth} <b>gender:</b> {card.gender}</p>
    </div>
    <div style="text-align: right">
      <Button on:click={() => handleDelete(card.id)}>x</Button>
    </div>
  </div>
{:else}
  <div on:click class="card plus-card">
    <p>+</p>
  </div>
{/if}

<style>
  .card {
    padding: 20px;
    border-radius: 20px;
    box-shadow: 0px 2px 4px rgba(0, 0, 0, 0.2);
    transition: 0.3s ease;
    display: flex;
    justify-content: space-between;
  }

  .plus-card {
    padding: 0px;
    text-align: center;
    justify-content: center;
    font-size: 45px;
    font-weight: bold;
  }

  .plus-card:hover {
    color: white;
    background: #d91b42;
    cursor: pointer;
  }

  h3 {
    margin: 0 auto;
    color: #555;
  }
</style>
