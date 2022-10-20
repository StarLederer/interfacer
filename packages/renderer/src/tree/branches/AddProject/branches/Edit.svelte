<script lang="ts">
  import type { IHooks, IAction } from "../hooks";
  import { availableHookKeys } from "../hooks";
  import { invoke } from "@tauri-apps/api/tauri";
  import Add from "svelte-material-icons/Plus.svelte";
  import Save from "svelte-material-icons/Check.svelte";
  import Exclimation from "svelte-material-icons/Exclamation.svelte";
  import Remove from "svelte-material-icons/Close.svelte";
  import { Route } from "~/lib/router";
  import Headerbar from "~/lib/Headerbar.svelte";
  import Button from "~/lib/primitives/Button.svelte";
  import Input from "~/lib/primitives/Input.svelte";
  import Toggle from "~/lib/primitives/Toggle.svelte";
  import { navigate } from "~/lib/router";
  import { error } from "../store";

  const save = async () => {
    navigate(`${path}/../loading`);

    try {
      const config = {
        hooks,
        actions,
      };

      console.log(JSON.stringify(config));

      await invoke("add_project", {
        name,
        gitUrl: url,
        config: JSON.stringify(config),
      });
      navigate(`${path}/../success`);
    } catch (err) {
      error.set(err);
      navigate(`${path}/../error`);
    }
  };

  let usedHooks: IHooks = {};
  let unusedHooks: IHooks = {};

  $: {
    usedHooks = {};
    unusedHooks = {};

    availableHookKeys.forEach((key) => {
      if (hooks[key] !== undefined) {
        usedHooks[key] = hooks[key];
      } else {
        unusedHooks[key] = hooks[key];
      }
    });
  }

  let name = "";
  let url = "";
  let hooks: IHooks = {};
  let actions: IAction[] = [];

  export let path: string;
</script>

<Route {path}>
  <Headerbar
    title="Add project"
    back={() => {
      navigate("/websites");
    }}
  >
    <Button solid slot="actions" on:click={save}>
      Save
      <Save />
    </Button>
  </Headerbar>

  <section>
    <div class="add-website">
      <aside
        style="--hue: 6;"
        class="flex flex-row items-center gap-m0 bg-srf text-on-srf-2 pd-s- round-m--"
      >
        <div style="border-radius: 50%;" class="bg-srf2 text-on-srf pd-s+">
          <Exclimation size="1rem" />
        </div>
        <span>
          If you are a content writer, you probably want to call your developer
          for this.
        </span>
      </aside>

      <form on:submit|preventDefault={save}>
        <div class="fieldset">
          <Input label="Local name" bind:value={name} required />
          <Input label="Git URL" bind:value={url} required />
        </div>

        {#if Object.keys(usedHooks).length > 0}
          <div class="fieldset">
            <span class="legend">Hooks</span>
            {#each Object.keys(usedHooks) as key}
              <fieldset class="panel bg-srf text-on-srf flex flex-col gap-s">
                <div class="flex justify-between items-center">
                  <span class="mg-i-m0">{key.replaceAll("_", " ")}</span>
                  <Button
                    hue={0}
                    on:click={() => {
                      hooks[key] = undefined;
                    }}
                  >
                    <Remove />
                  </Button>
                </div>

                {#each hooks[key] as _, i}
                  <div class="flex gap-s-- flex-row">
                    <Input label="Command" bind:value={hooks[key][i]} />
                    <Button
                      hue={0}
                      on:click={() => {
                        hooks[key].splice(i, 1);
                        hooks[key] = hooks[key];
                      }}
                    >
                      <Remove />
                    </Button>
                  </div>
                {/each}

                <Button
                  secondary
                  on:click={() => {
                    hooks[key] = [...hooks[key], ""];
                  }}
                >
                  Add command
                  <Add />
                </Button>
              </fieldset>
            {/each}
          </div>
        {/if}

        {#if Object.keys(unusedHooks).length > 0}
          <div class="fieldset">
            <span class="legend">Add hooks</span>
            <div class="flex flex-wrap gap-s-">
              {#each Object.keys(unusedHooks) as key}
                <Button
                  half
                  on:click={() => {
                    hooks[key] = [""];
                  }}
                >
                  {key.replaceAll("_", " ")}
                  <Add />
                </Button>
              {/each}
            </div>
          </div>
        {/if}

        <div class="fieldset">
          <span class="legend">Actions</span>
          {#each actions as _, i}
            <fieldset class="panel bg-srf text-on-srf flex flex-col gap-s--">
              <Input label="Idle name" bind:value={actions[i].idle_name} />
              <Input label="Active name" bind:value={actions[i].active_name} />
              <Input label="Command" bind:value={actions[i].command} />
              <div class="flex items-start flex-col pd-i-m0 gap-s">
                <div class="label">User terminated</div>
                <Toggle
                  label="User terminated"
                  bind:on={actions[i].user_terminated}
                />
              </div>
              <Button
                hue={0}
                secondary
                on:click={() => {
                  actions.splice(i, 1);
                  actions = [...actions];
                }}
              >
                Remove this action <Remove />
              </Button>
            </fieldset>
          {/each}
          <Button
            secondary
            hue={166}
            on:click={() => {
              actions = [
                ...actions,
                {
                  idle_name: "",
                  active_name: "",
                  command: "",
                  user_terminated: false,
                },
              ];
            }}
          >
            Add action <Add />
          </Button>
        </div>
      </form>
    </div>
  </section>
</Route>

<style lang="postcss">
  section {
    overflow: auto;
  }

  .add-website {
    @apply flex
      flex-col
      gap-m0
      pd-m0
      pd-bs-0;
  }

  form {
    @apply flex
      flex-col
      gap-m0;
  }

  .fieldset {
    @apply flex
      flex-col
      gap-s;
  }

  .legend {
    @apply mg-i-m0 mg-be-m0;
    font-size: 1.2rem;
  }

  .label {
    font-size: 0.8rem;
    @apply text-on-def-2;
  }
</style>
