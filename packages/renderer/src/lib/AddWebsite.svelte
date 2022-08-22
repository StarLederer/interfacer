<script>
  import Add from "svelte-material-icons/Plus.svelte";
  import Save from "svelte-material-icons/Check.svelte";
  import Back from "svelte-material-icons/ChevronLeft.svelte";
  import Remove from "svelte-material-icons/Close.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Dropdown from "../primitives/Dropdown.svelte";
  import Button from "../primitives/Button.svelte";
  import Input from "../primitives/Input.svelte";
  import route from "./router";
  import Progress from "./Progress.svelte";
  import ButtonList from "../primitives/ButtonList.svelte";

  let loading = false;
  let name;
  let url;
  let environments = ["Your OS", "Docker"];
  let selectedEnvironment = environments[0];
  let actions = [];
</script>

<section class="add-website">
  <ButtonList spread>
    <Button
      half
      on:click={() => {
        route.set("/websites");
      }}
    >
      <Back />
    </Button>

    <Button hue={100}>
      Save
      <Save />
    </Button>
  </ButtonList>

  <h2>Add website</h2>
  <p>
    If you are a content writer, you probably want to call your developer
    for this.
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
                ghost
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
            hue={100}
            half
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
  }

  h2 {
    margin-block: 2rem;
    margin-block-end: 0.4rem;

  }

  p {
    color: hsla(0, 0%, 100%, 0.4);
    margin-block-end: 2rem;

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
