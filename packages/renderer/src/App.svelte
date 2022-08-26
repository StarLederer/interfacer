<script lang="ts">
  import route from "~/router";
  import Projects from "~/routes/projects/Projects.svelte";
  import AddProject from "~/routes/AddProject.svelte";
  import Server from "~/routes/server/Server.svelte";
  import { routeIn, routeOut } from "~/lib/visuals";

  route.set("/websites");
</script>

<main>
  {#if $route.startsWith("/websites")}
    <section in:routeIn out:routeOut>
      <Projects />
    </section>
  {:else if $route.startsWith("/add-website")}
    <section in:routeIn out:routeOut>
      <AddProject />
    </section>
  {:else if $route.startsWith("/server")}
    <section in:routeIn out:routeOut>
      <Server />
    </section>
  {/if}
</main>

<style>
  main {
    position: fixed;
    inset: 0;
  }

  section {
    position: absolute;
    inset: 0;
    overflow: scroll;
    overflow: overlay;
  }
</style>
