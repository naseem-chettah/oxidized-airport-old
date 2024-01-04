<script>
  import PassengersStore from "../../../stores/PassengersStore";
  import CardList from "./CardList.svelte";
  import FormView from "../FormView.svelte";
  import AddNewForm from "./AddNewForm.svelte";

  let hidden = true;
</script>

<FormView on:hide={() => { hidden = !hidden }} {hidden}>
  <AddNewForm on:hideForm={() => { hidden = !hidden }} />
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
