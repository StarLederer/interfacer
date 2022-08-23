<script lang="ts">
  import { fade } from "svelte/transition";
  import route from "~/router";
  import Projects from "~/routes/projects/Projects.svelte";
  import AddProject from "~/routes/AddProject.svelte";
  import Server from "~/routes/server/Server.svelte";

  route.set("/websites");
</script>

<main>
  {#if $route.startsWith("/websites")}
    <section transition:fade>
      <Projects />
    </section>
  {:else if $route.startsWith("/add-website")}
    <section transition:fade>
      <AddProject />
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
    display: block;

    position: absolute;
    width: 100%;
    /* height: 100%; */
  }
</style>
