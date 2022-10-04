<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import Add from "svelte-material-icons/Git.svelte";
  import AddLocal from "svelte-material-icons/FolderPlus.svelte";
  import Settings from "svelte-material-icons/Cog.svelte";
  import { navigate } from "~/lib/router";
  import { projectStore } from "~/stores";
  import Project from "./lib/Project.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Button from "~/lib/primitives/Button.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";

  let status;
  let websites;

  const reloadSites = async () => {
    try {
      websites = await invoke("get_websites");
      status = "Loaded";
    } catch (err) {
      status = err;
    }
  };

  onMount(async () => {
    await reloadSites();
  });
</script>

<section>
  <Headerbar title="Your projects">
    <div class="flex gap-s-" slot="actions">
      {#if status}
        <Button
          half
          on:click={async () => {
            navigate("/add-website/edit");
          }}
        >
          Add project
          <AddLocal />
        </Button>
        <Button
          half
          on:click={async () => {
            navigate("/settings");
          }}
        >
          <Settings />
        </Button>
      {/if}
    </div>
  </Headerbar>

  {#if status}
    {#if websites}
      <div class="websites">
        {#each websites as website}
          <Project
            label={website}
            on:click={() => {
              projectStore.set(website);
              navigate("/project");
            }}
          />
        {/each}
      </div>
    {:else}
      <p>{status}</p>
    {/if}
  {:else}
    <Progress />
  {/if}
</section>

<style lang="postcss">
  section {
    overflow: auto;
  }

  .websites {
    @apply gap-m0
      pd-m0
      pd-bs-0;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(24rem, 1fr));
  }
</style>
