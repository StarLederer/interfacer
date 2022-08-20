<script lang="ts">
  import route from "./lib/router";
  import Websites from "./lib/Websites.svelte";
  import AddWebsite from "./lib/AddWebsite.svelte";
  import Server from "./lib/Server.svelte";

  route.set("/websites");
</script>

{#if $route.startsWith("/websites")}
  <Websites />
{:else if $route.startsWith("/add-website")}
  <AddWebsite />
{:else if $route.startsWith("/server")}
  <Server />
{/if}

<style>
</style>
