<script>
  export let card;
</script>

<div class="card">
  <h3>Passenger {card.id}</h3>
  <p><b>full name:</b> {card.first_name} {card.last_name}</p>
  <p><b>date of birth:</b> {card.date_of_birth}</p>
</div>

<style>
  .card {
    padding: 20px;
    border-radius: 20px;
    box-shadow: 0px 2px 4px rgba(0, 0, 0, .2);
  }

  h3 {
    margin: 0 auto;
    color: #555;
  }
</style>
