<script>
  import Add from "svelte-material-icons/Plus.svelte";
  import Save from "svelte-material-icons/Check.svelte";
  import Back from "svelte-material-icons/ChevronLeft.svelte";
  import Remove from "svelte-material-icons/Close.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Dropdown from "ui-kit/primitives/Dropdown.svelte";
  import Button from "ui-kit/primitives/Button.svelte";
  import Input from "ui-kit/primitives/Input.svelte";
  import route from "../router";
  import Progress from "ui-kit/primitives/Progress.svelte";
  import Flex from "ui-kit/helpers/Flex.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";

  let loading = false;
  let name;
  let url;
  let environments = ["Your OS", "Managed Docker container"];
  let selectedEnvironment = environments[0];
  let actions = [];
</script>

<Headerbar
  title="Add website"
  back={() => {
    route.set("/websites");
  }}
>
  <Button solid colored style={{ hue: 100 }} slot="actions">
    Save
    <Save />
  </Button>
</Headerbar>

<section class="add-website">
  <p>
    If you are a content writer, you probably want to call your developer for
    this.
  </p>

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
      <Input label="Name" bind:value={name} required />
      <Input label="Git URL" bind:value={url} required />

      <fieldset>
        <legend>Setup</legend>
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
            half
            colored
            style={{ hue: 100 }}
            on:click={() => {
              actions = [...actions, { name: "", command: "" }];
              console.log(actions);
            }}
          >
            Add action <Add />
          </Button>
        </fieldset>
      </fieldset>
    </form>
  {/if}
</section>

<style lang="scss">
  .add-website {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    padding: var(--border-radius);
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

  .action {
    display: grid;
    grid-template-columns: auto auto min-content;
    padding: 0.4rem;
    gap: 0.4rem;
  }
</style>
