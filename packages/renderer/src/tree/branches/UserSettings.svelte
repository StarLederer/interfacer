<script>
  import Save from "svelte-material-icons/Check.svelte";
  import Remove from "svelte-material-icons/Close.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Button from "~/lib/primitives/Button.svelte";
  import Input from "~/lib/primitives/Input.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";
  import { navigate } from "~/lib/router";

  let loading = false;
  let username;
  let password;

  const save = () => {
    loading = true;
    // await invoke("add_website", { username, url });
    loading = false;
  };
</script>

<Headerbar
  title="User settings"
  back={() => {
    navigate("/websites");
  }}
>
  <Button
    solid={!loading}
    half={loading}
    slot="actions"
    disabled={loading}
    on:click={save}
  >
    {#if loading}
      <Progress borderRadius={0.5} />
    {:else}
      Save
      <Save />
    {/if}
  </Button>
</Headerbar>

<div class="body">
  <form on:submit|preventDefault={async () => {}}>
    <fieldset>
      <legend>Change Git credentials</legend>
      <fieldset class="field-changer">
        <Input label="New username" bind:value={username} required />
        <Button
          half
          on:click={() => {
            username = undefined;
          }}
        >
          <Remove />
        </Button>
      </fieldset>
      <fieldset class="field-changer">
        <Input label="New password" bind:value={password} required />
        <Button
          half
          on:click={() => {
            password = undefined;
          }}
        >
          <Remove />
        </Button>
      </fieldset>
      <p class="text-on-def-3">Previous credentials are not shown.</p>
    </fieldset>
  </form>
</div>

<style lang="postcss">
  .body {
    padding-inline: 1rem;
  }

  form,
  fieldset {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  fieldset {
    & p {
      text-align: center;
    }
  }

  .field-changer {
    padding: 0;
    border: none;
    flex-direction: row;
  }
</style>
