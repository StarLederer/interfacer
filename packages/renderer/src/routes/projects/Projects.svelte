<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import Add from "svelte-material-icons/Git.svelte";
  import AddLocal from "svelte-material-icons/FolderPlus.svelte";
  import route from "~/router";
  import { name } from "~/routes/server/server";
  import Project from "./lib/Project.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Button from "~/lib/primitives/Button.svelte";
  import ButtonList from "~/lib/primitives/ButtonList.svelte";

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

<div class="header">
  <h2>Your projects</h2>
  {#if status}
    <ButtonList>
      <Button
        hue={100}
        ghost
        on:click={async () => {
          route.set("/add-website");
        }}
      >
        Add local
        <AddLocal />
      </Button>
      <Button
        half
        hue={100}
        on:click={async () => {
          route.set("/add-website");
        }}
      >
        Add from Git
        <Add />
      </Button>
    </ButtonList>
  {/if}
</div>

{#if status}
  {#if websites}
    <div class="websites">
      {#each websites as website}
        <Project
          label={website}
          on:click={() => {
            name.set(website);
            route.set("/server");
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

<style>
  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-block-end: 1rem;
  }

  .websites {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(18rem, 1fr));
    gap: 0.4rem;
  }
</style>
