<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Refresh from "svelte-material-icons/Refresh.svelte";
  import Save from "svelte-material-icons/ContentSave.svelte";

  import { Route, navigate } from "~/lib/router";
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

  const onOpen = async () => {
    actions = [];
    try {
      await invoke("load_project", { name: $projectStore });
      actions = await invoke("get_actions");
    } catch (err) {
      error = err;
    }
  };

  export let path: string;
</script>

<Route {path} on:open={onOpen}>
  <div class="wrapper flex flex-col flex-1">
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

    <div class="main">
      {#if error}
        <div class="error">
          <p>An error has occured!</p>
          <p class="text-on1">{error}</p>
        </div>
      {:else}
        <div class="actions">
          {#each actions as action, i}
            <Action name={action.name} {hue} {i} />
          {/each}
        </div>
      {/if}
    </div>
  </div>
</Route>

<style lang="postcss">
  .wrapper {
    overflow: scroll;
  }

  .header-bar-actions {
    display: flex;
    align-items: center;
    @apply gap-m0;
  }

  .main {
    @apply flex-1
      pd-m0
      pd-bs-0;
  }

  .actions {
    min-height: 100%;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(24rem, 1fr));
    @apply gap-s-;
  }

  .error {
    --hue: 0;
    @apply bg-srf
      round-m0
      pd-m0;
  }
</style>
