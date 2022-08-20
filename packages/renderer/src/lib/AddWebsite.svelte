<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import Button from "../primitives/Button.svelte";
  import route from "./router";
  import Progress from "./Progress.svelte";

  let loading = false;
  let name = "";
  let url = "";
</script>

<Button
  hue={0}
  on:click={() => {
    route.set("/websites");
  }}
>
  back
</Button>

<h2>Add website</h2>

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
    <p>You probably want to ask your developer to do this step for you.</p>

    <label>
      <span>Name</span>
      <input type="text" bind:value={name} required />
    </label>
    <label>
      <span>Git URL</span>
      <input type="url" bind:value={url} required />
    </label>

    <Button hue={100}>Add website</Button>
  </form>
{/if}

<style>
  h2 {
    margin-block: 2rem;
  }

  p {
    color: hsla(0, 0%, 100%, 0.4);
  }

  form {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    gap: 1rem;
  }

  label {
    display: flex;
    gap: 0.4rem;
    flex-direction: column;
  }

  input {
    /* width: 100%; */
  }
</style>
