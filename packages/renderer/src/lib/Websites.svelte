<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import route from "./router";
  import { name } from "./server";
  import Website from "./Website.svelte";
  import Progress from "./Progress.svelte";
  import Button from "../primitives/Button.svelte";

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

<h2>Your websites</h2>

{#if status}
  {#if websites}
    <div class="websites">
      {#each websites as website}
        <Website
          label={website}
          on:click={() => {
            name.set(website);
            route.set("/server");
          }}
        />
      {/each}
    </div>
    <Button
      hue={100}
      on:click={async () => {
        route.set("/add-website");
      }}>Add one</Button
    >
  {:else}
    <p>{status}</p>
  {/if}
{:else}
  <Progress />
{/if}

<style>
  h2 {
    margin-block: 2rem;
  }

  .websites {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(18rem, 1fr));
    gap: 0.4rem;

    margin-bottom: 1rem;
  }
</style>
