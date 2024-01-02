<script>
  //import { invoke } from "@tauri-apps/api/tauri";
  import Header from "../lib/components/Header.svelte";
  import Footer from "../lib/components/Footer.svelte";
  import Tabs from "../lib/components/Tabs.svelte";
  import AddPassengerForm from "../lib/components/AddPassengerForm.svelte";

  //tabs
  let tabs = ["Passengers", "Flights", "Airports"];
  let activeTab = "Passengers";

  const tabChange = (e) => {
    activeTab = e.detail;
  };
</script>

<Header />
<main>
  <Tabs {activeTab} {tabs} on:tabChange={tabChange} />
  {#if activeTab === "Passengers"}
    <AddPassengerForm />
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
