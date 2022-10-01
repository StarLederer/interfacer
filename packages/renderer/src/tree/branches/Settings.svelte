<script>
  import Save from "svelte-material-icons/Check.svelte";
  import Remove from "svelte-material-icons/Close.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Button from "~/lib/primitives/Button.svelte";
  import Input from "~/lib/primitives/Input.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";
  import { navigate } from "~/lib/router";
  import { mainStore } from "~/stores";

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

<section>
  <form on:submit|preventDefault={async () => {}}>
    <fieldset>
      <legend>Appearance</legend>
      <div class="flex gap-s-">
        {#each [["red", 0], ["orange", 40], ["teal", 150], ["blue", 210]] as color}
          <Button
            secondary
            hue={color[1]}
            on:click={() => {
              mainStore.set(color[1]);
            }}
          >
            {color[0]}
          </Button>
        {/each}
      </div>
    </fieldset>
    <fieldset>
      <legend>Git credentials</legend>
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
</section>

<style lang="postcss">
  section {
    padding-inline: 1rem;
  }

  form,
  fieldset {
    @apply flex
      flex-col
      gap-s-;
  }

  form {
    @apply gap-m0;
  }

  fieldset {
    & p {
      text-align: center;
    }
  }

  legend {
    @apply mg-i-m0 mg-be-m0;
  }

  .field-changer {
    padding: 0;
    @apply gap-s-- flex-row;
  }
</style>
