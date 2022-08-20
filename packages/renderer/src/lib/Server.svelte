<script>
  import { open } from "@tauri-apps/api/shell";
  import { invoke } from "@tauri-apps/api/tauri";
  import route from "./router";
  import { name } from "./server";
  import Button from "../primitives/Button.svelte";
  import Progress from "./Progress.svelte";

  let error;
  let address;

  $: (async () => {
    console.log($name);
    try {
      address = await invoke("open_website", {
        name: $name,
      });
    } catch (err) {
      error = err;
    }
  })();
</script>

<Button
  on:click={async () => {
    error = undefined;
    address = undefined;
    await invoke("close_website");
    route.set("/websites");
  }}
  hue={0}
>
  back
</Button>
<h2>{$name}</h2>
{#if error}
  <p>An error has occured!</p>
  <p>{error}</p>
{:else if address}
  <p>
    We've launched the development server. You can start editing the website
  </p>
  <Button
    hue={240}
    on:click={async () => {
      await open(address);
    }}>Open in browser</Button
  >
{:else}
  <p>Launching the development server</p>
  <Progress />
{/if}

<style>
  h2 {
    margin-block: 2rem;
  }

  p {
    margin-block-end: 1rem;
    color: hsla(0, 0%, 100%, 0.8);
  }
</style>
