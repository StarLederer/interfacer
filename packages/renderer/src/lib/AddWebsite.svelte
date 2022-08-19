<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import route from "./router";
  import Progress from "./Progress.svelte";

  let loading = false;
  let name = "";
  let url = "";
</script>

<button
  on:click={() => {
    route.set("/websites");
  }}
>
  back
</button>

{#if loading}
  <Progress />
{:else}
  <form
    on:submit|preventDefault={async () => {
      loading = true;
      await invoke("add_website", { name, url });
      route.set("/websites");
    }}
  >
    <p>You probably want to ask your developer to do this step for you</p>

    <label>
      <span>Name</span>
      <input type="text" bind:value={name} />
    </label>
    <label>
      <span>Git URL</span>
      <input type="url" bind:value={url} />
    </label>

    <input type="submit" value="Add website" />
  </form>
{/if}

<style>
  form {
    max-width: 20rem;
    display: flex;
    flex-direction: column;
  }
</style>
