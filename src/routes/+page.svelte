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
</main>
<Footer />

<style>
  main {
    max-width: 960px;
    margin: 40px auto;
  }
</style>
