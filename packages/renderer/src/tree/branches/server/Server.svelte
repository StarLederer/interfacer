<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import Refresh from "svelte-material-icons/Refresh.svelte";
  import Save from "svelte-material-icons/ContentSave.svelte";

  import {navigate} from "~/lib/router";
  import { stringToHue } from "~/lib/visuals";
  import Button from "~/lib/primitives/Button.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";

  import { projectStore } from "~/stores";
  import Action from "./lib/Action.svelte";

  $: hue = stringToHue($projectStore);

  let error;
  let actions: {
    name: string;
    active: boolean;
  }[] = [];

  onMount(async () => {
    try {
      await invoke("load_project", { name: $projectStore });
      actions = await invoke("get_actions");
    } catch (err) {
      error = err;
    }
  });
</script>

<section>
  <Headerbar
    title={$projectStore}
    back={async () => {
      error = undefined;
      // await invoke("close_website");
      navigate("/websites");
    }}
  >
    <div class="flex" slot="titleActions">
      <Button half>
        <Save />
      </Button>
    </div>

    <div slot="actions" class="header-bar-actions">
      <span class="text-int-3"> Up to date </span>
      <Button half>
        Refresh
        <Refresh />
      </Button>
    </div>
  </Headerbar>

  <main>
    {#if error}
      <p>An error has occured!</p>
      <p>{error}</p>
    {:else}
      <div class="actions">
        {#each actions as action, i}
          <Action name={action.name} {hue} {i} />
        {/each}
      </div>
    {/if}
  </main>
</section>

<style lang="postcss">
  section {
    height: 100%;
    display: grid;
    grid-template-rows: min-content auto;
    overflow: auto
  }

  main {
    @apply pd-m0
      pd-bs-0;
  }

  .header-bar-actions {
    display: flex;
    align-items: center;
    @apply gap-m0;
  }

  .actions {
    height: 100%;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(24rem, 1fr));
    gap: 0.4rem;
    --border-radius: 3rem;
  }
</style>
