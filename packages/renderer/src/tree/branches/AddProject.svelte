<script>
  import Add from "svelte-material-icons/Plus.svelte";
  import Save from "svelte-material-icons/Check.svelte";
  import Exclimation from "svelte-material-icons/Exclamation.svelte";
  import Remove from "svelte-material-icons/Close.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Dropdown from "~/lib/primitives/Dropdown.svelte";
  import Button from "~/lib/primitives/Button.svelte";
  import Input from "~/lib/primitives/Input.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";
  import { navigate } from "~/lib/router";

  let loading = false;
  let name;
  let url;
  let environments = ["Your OS", "Managed Docker container", "Cloud", "LAN"];
  let selectedEnvironment = environments[0];
  let actions = [];
</script>

<Headerbar
  title="Add website"
  back={() => {
    navigate("/websites");
  }}
>
  <Button solid slot="actions">
    Save
    <Save />
  </Button>
</Headerbar>

<section class="add-website">
  <aside style="--hue: 6; border-radius: 2rem; padding: 0.4rem; display: flex; flex-direction: row; align-items: center; gap: 1rem;" class="text-on-def-2">
    <div style="border-radius: 50%; padding: 0.6rem" class="bg-srf2 text-on-srf">
      <Exclimation size="1rem"/>
    </div>
    <span>
      If you are a content writer, you probably want to call your developer for
      this.
    </span>
  </aside>

  {#if loading}
    <Progress />
  {:else}
    <form
      on:submit|preventDefault={async () => {
        loading = true;
        await invoke("add_website", { name, url });
        navigate("/websites");
      }}
    >
      <div class="flex flex-col gap-s">
        <Input label="Name" bind:value={name} required />
        <Input label="Git URL" bind:value={url} required />
      </div>

      <Dropdown
        label="Environment"
        bind:selected={selectedEnvironment}
        options={environments}
      />

      <fieldset class="actions">
        <legend>Actions</legend>
        {#each actions as _, i}
          <fieldset class="action">
            <Input label="Name" bind:value={actions[i].name} />
            <Input label="Command" bind:value={actions[i].command} />
            <Button
              hue={6}
              on:click={() => {
                actions.splice(i, 1);
                actions = [...actions];
              }}
            >
              <Remove />
            </Button>
          </fieldset>
        {/each}
        <Button
          secondary
          hue={166}
          on:click={() => {
            actions = [...actions, { name: "", command: "" }];
            console.log(actions);
          }}
        >
          Add action <Add />
        </Button>
      </fieldset>
    </form>
  {/if}
</section>

<style lang="postcss">
  .add-website {
    @apply flex
      flex-col
      gap-m
      pad-m;
  }

  form,
  fieldset {
    @apply flex
      flex-col
      gap-s;
  }

  form {
    gap: 1rem;
  }

  legend {
    margin-bottom: 0.4rem;
  }

  .action {
    display: grid;
    grid-template-columns: auto auto min-content;
    gap: 0.4rem;
  }
</style>
