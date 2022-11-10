<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";

  import { navigate } from "~/lib/router";
  import { mainStore } from "~/stores";

  import Root from "./tree/Root.svelte";

  onMount(async () => {
    const has_user = await invoke("load_user") as boolean;

    if (has_user) {
      navigate("/websites");
    }
    else {
      navigate("/setup/user")
    }
  });
</script>

<main class="bg-def transition" style="--hue: {$mainStore}">
  <Root />
</main>

<style>
  main {
    position: fixed;
    inset: 0;
  }
</style>
