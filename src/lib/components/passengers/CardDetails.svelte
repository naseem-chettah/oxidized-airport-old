<script>
  import PassengersStore from "../../../stores/PassengersStore";
  import Button from "../../shared/Button.svelte";
  export let card = "plus";
  let showMore = false;

  const handleDelete = (id) => {
    PassengersStore.update((currentPassengers) => {
      return currentPassengers.filter((card) => card.id != id);
    });
  };
</script>

{#if card != "plus"}
  {#if !showMore}
    <div
      class="card"
      on:click={() => {
        showMore = !showMore;
      }}
    >
      <div>
        <h3>{card.firstName} {card.lastName}</h3>
        <p>
          <b>passport number:</b>
          {card.passportNumber}
        </p>
      </div>
      <div style="text-align: right">
        <Button on:click={() => handleDelete(card.id)}>x</Button>
      </div>
    </div>
  {:else}
    <div
      class="card"
      on:click={() => {
        showMore = !showMore;
      }}
    >
      <div>
        <h3>{card.firstName} {card.lastName}</h3>
        <p>
          <b>date of birth:</b>
          {card.dateOfBirth} <br>
          <b>gender:</b>
          {card.gender} <br>
          <b>phone number:</b>
          {card.phoneNumber} <br>
          <b>email:</b>
          {card.email} <br>
          <b>nationality:</b>
          {card.nationality} <br>
        <p>
          <b>passport number:</b>
          {card.passportNumber}
        </p>
      </div>
      <div style="text-align: right">
        <Button on:click={() => handleDelete(card.id)}>x</Button>
      </div>
    </div>
  {/if}
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
    cursor: pointer;
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
  }

  h3 {
    margin: 0 auto;
    color: #555;
  }
</style>
