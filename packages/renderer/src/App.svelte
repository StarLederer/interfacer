<script lang="ts">
  import { fade } from "svelte/transition";
  import route from "./lib/router";
  import Websites from "./lib/Websites.svelte";
  import AddWebsite from "./lib/AddWebsite.svelte";
  import Server from "./lib/Server.svelte";

  route.set("/websites");
</script>

<main>
  {#if $route.startsWith("/websites")}
    <section transition:fade>
      <Websites />
    </section>
  {:else if $route.startsWith("/add-website")}
    <section transition:fade>
      <AddWebsite />
    </section>
  {:else if $route.startsWith("/server")}
    <section transition:fade>
      <Server />
    </section>
  {/if}
</main>

<style>
  section {
    padding: 1rem;

    position: absolute;
    width: 100%;
    height: 100%;
  }
</style>
