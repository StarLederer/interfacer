<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import route from "./router";
  import Website from "./Website.svelte";
  import Progress from "./Progress.svelte";

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
<div class="websites">
  {#if status}
    {#if websites}
      {#each websites as website}
        <Website label={website} />
      {/each}
      <Website
        label="Add one"
        on:click={async () => {
          await invoke("add_website", { name: "testwebwriterwebsite" });
          reloadSites();
          // route.set("/add-website");
        }}
      />
    {:else}
      <p>{status}</p>
    {/if}
  {:else}
    <Progress />
  {/if}
</div>

<style>
  .websites {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));
  }
</style>
