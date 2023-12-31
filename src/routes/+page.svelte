<script>
  //import { invoke } from "@tauri-apps/api/tauri";
  import Header from "../lib/components/Header.svelte";
  import Footer from "../lib/components/Footer.svelte";
  import Tabs from "../lib/components/Tabs.svelte";

  //tabs
  let items = ["Passengers", "Flights", "Airports"];
  let activeItem = "Passengers";

  const tabChange = (e) => {
    activeItem = e.detail;
  }
</script>

<Header />
<main>
  <Tabs {activeItem} {items} on:tabChange={tabChange} />
  {#if activeItem === 'Passengers'}
    <p>this is the list of Passengers</p>
  {:else if activeItem === 'Flights'}
    <p>this is the list of Flights</p>
  {:else if activeItem === 'Airports'}
    <p>this is the list of Airports</p>
  {/if}
</main>
<Footer />

<style>
  main {
    max-width: 960px;
    margin: 40px auto;
  }
</style>
