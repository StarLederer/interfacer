<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Refresh from "svelte-material-icons/Refresh.svelte";
  import Download from "svelte-material-icons/CloudDownloadOutline.svelte";
  import Save from "svelte-material-icons/ContentSave.svelte";

  import { Route, navigate } from "~/lib/router";
  import { stringToHue } from "~/lib/visuals";
  import Button from "~/lib/primitives/Button.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";
  import Error from "~/lib/Error.svelte";

  import { projectStore } from "~/stores";
  import Action from "./lib/Action.svelte";

  $: hue = stringToHue($projectStore);

  let error;
  let actions: {
    name: string;
    active: boolean;
  }[] = [];
  let saving;
  let localChanges;
  let remoteChanges;

  const resetState = () => {
    actions = [];
    saving = false;
    localChanges = false;
    remoteChanges = undefined;
  };

  const checkRemote = async () => {
    try {
      remoteChanges = await invoke("detect_remote_source_changes");
    } catch (err) {
      error = err;
    }
  };

  const updateCode = async () => {
    try {
      await invoke("download_remote_source_history");
      remoteChanges = await invoke("detect_remote_source_changes");
    } catch (err) {
      error = err;
    }
  };

  const saveCode = async () => {
    saving = true;

    try {
      await invoke("upload_local_source_history");
      localChanges = await invoke("detect_local_source_changes");
    } catch (err) {
      error = err;
    }

    saving = false;
  };

  const onOpen = async () => {
    resetState();
    try {
      await invoke("load_project", { name: $projectStore });
      localChanges = await invoke("detect_local_source_changes");
      remoteChanges = await invoke("detect_remote_source_changes");
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
        {#if !error}
          <Button
            disabled={saving}
            half={!localChanges}
            solid={localChanges}
            on:click={saveCode}
          >
            {#if saving}
              <Progress radius={0.5} />
            {:else}
              <Save />
            {/if}
          </Button>
        {/if}
      </div>

      <div slot="actions" class="header-bar-actions">
        {#if !error}
          {#if remoteChanges === undefined}
            <span class="text-int-2"> Checking the cloud </span>
            <Progress radius={0.5} />
          {:else if remoteChanges}
            <span class="text-int"> Out of date </span>
            <Button solid on:click={updateCode}>
              Update
              <Download />
            </Button>
          {:else}
            <span class="text-int-2"> Up to date </span>
            <Button half on:click={checkRemote}>
              Refresh
              <Refresh />
            </Button>
          {/if}
        {/if}
      </div>
    </Headerbar>

    <div class="main">
      {#if error}
        <Error {error} />
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
</style>
