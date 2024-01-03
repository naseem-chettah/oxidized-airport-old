<script>
  import PassengersStore from "../stores/PassengersStore";
  import CardList from "../lib/components/CardList.svelte";
  import FormView from "../lib/components/FormView.svelte";
  import PassengerForm from "../lib/components/PassengerForm.svelte";

  let hidden = true;
</script>

<FormView on:hide={() => { hidden = !hidden }} {hidden}>
  <PassengerForm />
</FormView>

<div>
  <CardList
    on:add={() => {
      hidden = !hidden;
    }}
    items="passengers"
    cards={$PassengersStore}
  />
</div>
