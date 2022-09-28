<script>
  import Save from "svelte-material-icons/Check.svelte";
  import Remove from "svelte-material-icons/Close.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Button from "ui-kit/primitives/Button.svelte";
  import Input from "ui-kit/primitives/Input.svelte";
  import Progress from "ui-kit/primitives/Progress.svelte";
  import Flex from "ui-kit/helpers/Flex.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";
  import { navigate } from "~/router";

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
    colored
    style={{ hue: 100 }}
    slot="actions"
    disabled={loading}
    on:click={save}
  >
    {#if loading}
      <Progress style={{ borderRadius: 0.5 }} />
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
      <p>Previous credentials are not shown.</p>
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
    </fieldset>
  </form>
</div>

<style lang="scss">
  .body {
    padding-inline: 1rem;
  }

  p {
    color: hsla(0, 0%, 100%, 0.4);
  }

  form,
  fieldset {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  fieldset {
    padding: 0.4rem;
    p {
      padding: 0.4rem;
    }
  }

  .field-changer {
    padding: 0;
    border: none;
    flex-direction: row;
  }
</style>
