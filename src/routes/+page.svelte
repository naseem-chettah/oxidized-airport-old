<script>
  //import { invoke } from "@tauri-apps/api/tauri";
  import Header from "../lib/components/Header.svelte";
  import Footer from "../lib/components/Footer.svelte";
  import Tabs from "../lib/components/Tabs.svelte";
  import CardList from "../lib/components/CardList.svelte";
  import PassengerForm from "../lib/components/PassengerForm.svelte";
  import FormView from "../lib/components/FormView.svelte";

  import PassengersStore from "../stores/PassengersStore";

  //tabs
  let tabs = ["Passengers", "Flights", "Airports"];
  let activeTab = "Passengers";

  let hideAddPassenger = true;

  const toggleAddPassenger = () => {
    hideAddPassenger = !hideAddPassenger;
  };

  const tabChange = (e) => {
    activeTab = e.detail;
  };

  const pushPassenger = (e) => {
    $PassengersStore = [e.detail, ...$PassengersStore];
    hideAddPassenger = !hideAddPassenger;
  };
</script>

<FormView hidden={hideAddPassenger} on:add={toggleAddPassenger}>
  <PassengerForm on:pushPassenger={pushPassenger} />
</FormView>

<Header />
<main>
  <Tabs {activeTab} {tabs} on:tabChange={tabChange} />
  {#if activeTab === "Passengers"}
    <CardList on:add={toggleAddPassenger} />
  {:else if activeTab === "Flights"}
    <p>nth here yet</p>
  {:else if activeTab === "Airports"}
    <p>nth here yet</p>
  {/if}
</main>
<Footer />

<style>
  main {
    max-width: 960px;
    margin: 40px auto;
  }
</style>
