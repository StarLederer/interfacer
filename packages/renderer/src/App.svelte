<script lang="ts">
  import { navigate } from "~/lib/router";
  import { mainStore } from "~/stores";

  import Root from "./tree/Root.svelte";

  navigate("/websites");
</script>

<main class="bg-def text-on-def transition" style="--hue: {$mainStore}">
  <Root />
</main>

<style>
  main {
    position: fixed;
    inset: 0;
  }
</style>
