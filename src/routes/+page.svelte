<script>
  //import { invoke } from "@tauri-apps/api/tauri";
  import Header from "../lib/components/Header.svelte";
  import Footer from "../lib/components/Footer.svelte";
  import Tabs from "../lib/components/Tabs.svelte";
  import CreateItemForm from "../lib/components/CreateItemForm.svelte";

  //tabs
  let tabs = ["Passengers", "Flights", "Airports"];
  let activeTab = "Passengers";

  //data
  let data = [
    {
      "first name": "text",
      "last name": "text",
      "date of birth": "date",
    },
    {
      "departure time": "datetime",
      "arrival time": "datetime",
    },
    {
      "airport name": "text",
      "city": "text",
    },
  ];

  const tabChange = (e) => {
    activeTab = e.detail;
  };
</script>

<Header />
<main>
  <Tabs {activeTab} {tabs} on:tabChange={tabChange} />
  {#if activeTab === "Passengers"}
    <CreateItemForm {activeTab} {data} {tabs} />
  {:else if activeTab === "Flights"}
    <CreateItemForm {activeTab} {data} {tabs} />
  {:else if activeTab === "Airports"}
    <CreateItemForm {activeTab} {data} {tabs} />
  {/if}
</main>
<Footer />

<style>
  main {
    max-width: 960px;
    margin: 40px auto;
  }
</style>
