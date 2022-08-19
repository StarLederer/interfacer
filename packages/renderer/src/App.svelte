<script lang="ts">
  import route from "./lib/router";
  import Websites from "./lib/Websites.svelte";
  import AddWebsite from "./lib/AddWebsite.svelte";

  route.set("/websites");
</script>

{#if $route.startsWith("/websites")}
  <Websites/>
{:else if $route.startsWith("/add-website")}
  <AddWebsite/>
{/if}

<style>

</style>
