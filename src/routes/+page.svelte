<script>
  //import { invoke } from "@tauri-apps/api/tauri";
  import Header from "../lib/components/Header.svelte";
  import Footer from "../lib/components/Footer.svelte";
  import Tabs from "../lib/components/Tabs.svelte";
  import CreateItemForm from "../lib/components/CreateItemForm.svelte";

  //tabs
  let tabs = ["Passengers", "Flights", "Airports"];
  let activeTab = "Passengers";

  const tabChange = (e) => {
    activeTab = e.detail;
  }
</script>

<Header />
<main>
  <Tabs {activeTab} {tabs} on:tabChange={tabChange} />
  {#if activeTab === 'Passengers'}
    <CreateItemForm {activeTab} />
  {:else if activeTab === 'Flights'}
    <CreateItemForm {activeTab} />
  {:else if activeTab === 'Airports'}
    <CreateItemForm {activeTab} />
  {/if}
</main>
<Footer />

<style>
  main {
    max-width: 960px;
    margin: 40px auto;
  }
</style>
