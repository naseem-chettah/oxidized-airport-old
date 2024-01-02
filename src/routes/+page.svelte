<script>
  //import { invoke } from "@tauri-apps/api/tauri";
  import Header from "../lib/components/Header.svelte";
  import Footer from "../lib/components/Footer.svelte";
  import Tabs from "../lib/components/Tabs.svelte";
  import AddPassengerForm from "../lib/components/AddPassengerForm.svelte";

  //tabs
  let tabs = ["Passengers", "Flights", "Airports"];
  let activeTab = "Passengers";

  //passenger
  let passengers = [
    {
      id: 1,
      first_name: 'seth',
      last_name: 'ossidian',
      date_of_birth: '2003-12-31'
    },
  ];

  const tabChange = (e) => {
    activeTab = e.detail;
  };

  const handleAddPassenger = (e) => {
    const passenger = e.detail;
    passengers = [ passenger, ...passengers];
    console.log(passengers);
    // activeTab = "Passenger";
  }
</script>

<Header />
<main>
  <Tabs {activeTab} {tabs} on:tabChange={tabChange} />
  {#if activeTab === "Passengers"}
    <AddPassengerForm on:add={handleAddPassenger} />
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
