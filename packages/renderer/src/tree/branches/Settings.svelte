<script lang="ts">
  import Save from "svelte-material-icons/Check.svelte";
  import Remove from "svelte-material-icons/Close.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  import { Route, navigate } from "~/lib/router";
  import Button from "~/lib/primitives/Button.svelte";
  import Input from "~/lib/primitives/Input.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";
  import { mainStore } from "~/stores";

  let loading = false;
  let error;
  let username;
  let password;

  const resetState = () => {
    loading = false;
    error = undefined;
    username = undefined;
    password = undefined;
  };

  const save = async () => {
    loading = true;
    try {
      let partial = {
        git_username: username,
        git_password: password,
      };
      await invoke("update_user", { partial });
    } catch (err) {
      console.log(err);
    }
    loading = false;
  };

  export let path: string;
</script>

<Route {path} on:open={resetState}>
  <Headerbar
    title="User settings"
    back={() => {
      navigate("/websites");
    }}
  >
    {#if !error}
      <Button
        solid={!loading}
        half={loading}
        slot="actions"
        disabled={loading}
        on:click={save}
      >
        {#if loading}
          <Progress radius={0.5} />
        {:else}
          Save
          <Save />
        {/if}
      </Button>
    {/if}
  </Headerbar>

  <div class="pd-i-m0">
    {#if error}
      <div class="error">
        <p>An error has occured!</p>
        <p class="text-on1">{error}</p>
      </div>
    {:else}
      <form on:submit|preventDefault={save}>
        <fieldset>
          <span class="legend">Appearance</span>
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
          <span class="legend">Git credentials</span>
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
          <p class="text-on3">Previous credentials are not shown.</p>
        </fieldset>
      </form>
    {/if}
  </div>
</Route>

<style lang="postcss">
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

  .legend {
    @apply mg-i-m0 mg-be-m0;
  }

  .field-changer {
    padding: 0;
    @apply gap-s-- flex-row;
  }

  .error {
    --hue: 0;
    @apply bg-srf
      round-m0
      pd-m0;
  }
</style>
