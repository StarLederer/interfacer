<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import route from "./router";
  import { name } from "./server";
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

<button
  on:click={() => {
    route.set("/websites");
  }}
>
  back
</button>
<h1>{$name}</h1>
{#if error}
  <p>An error has occured!</p>
  <p>{error}</p>
{:else if address}
  <p>
    We've launched the development server for you!
    You can now edit the website
  </p>
  <a href={address} target="_blank">Open in browser</a>
{:else}
  <p>Launching the development server</p>
  <Progress />
{/if}
