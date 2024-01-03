<script>
  //import { invoke } from "@tauri-apps/api/tauri";
  import Header from "../lib/components/Header.svelte";
  import Footer from "../lib/components/Footer.svelte";
  import Tabs from "../lib/components/Tabs.svelte";
  import CardList from "../lib/components/CardList.svelte";
    import AddPassengerForm from "../lib/components/AddPassengerForm.svelte";

  //tabs
  let tabs = ["Passengers", "Flights", "Airports"];
  let activeTab = "Passengers";

  //passenger
  let passengers = [
    {
      id: 1,
      first_name: "seth",
      last_name: "ossidian",
      date_of_birth: "2003-12-31",
    },
  ];

  let hideAddPassenger = true;

  const toggleAddPassenger = () => {
    console.log('toggled');
    hideAddPassenger = !hideAddPassenger;
  }

  const tabChange = (e) => {
    activeTab = e.detail;
  };
</script>

<AddPassengerForm hidden={hideAddPassenger} />
<Header />
<main>
  <Tabs {activeTab} {tabs} on:tabChange={tabChange} />
  {#if activeTab === "Passengers"}
    <CardList cards={passengers} on:add={toggleAddPassenger} />
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
