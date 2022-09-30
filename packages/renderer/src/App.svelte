<script lang="ts">
  import { navigate } from "~/lib/router";
  import Root from "./tree/Root.svelte";

  navigate("/websites");
</script>

<main class="bg-def text-on-def">
  <Root />
</main>

<style>
  main {
    --hue: 40;
    position: fixed;
    inset: 0;
  }
</style>
