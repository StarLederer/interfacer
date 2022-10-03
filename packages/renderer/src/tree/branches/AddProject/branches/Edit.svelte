<script lang="ts" context="module">
  type IHook = {
    name: string;
    commands?: string[];
  };

  type IAction = {
    idleName: string;
    activeName: string;
    command: string;
    userTerminated: boolean;
  };

  export type { IHook, IAction };
</script>

<script lang="ts">
  import Add from "svelte-material-icons/Plus.svelte";
  import Save from "svelte-material-icons/Check.svelte";
  import Exclimation from "svelte-material-icons/Exclamation.svelte";
  import Remove from "svelte-material-icons/Close.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Route } from "~/lib/router";
  import Button from "~/lib/primitives/Button.svelte";
  import Input from "~/lib/primitives/Input.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Toggle from "~/lib/primitives/Toggle.svelte";
  import { navigate } from "~/lib/router";

  let usedHooks: (IHook & { key: string })[] = [];
  let unusedHooks: (IHook & { key: string })[] = [];

  $: {
    usedHooks = [];
    unusedHooks = [];

    Object.keys(hooks).forEach((key) => {
      const hook = hooks[key];

      if (hook.commands != undefined) {
        usedHooks.push({ key, ...hook });
      } else {
        unusedHooks.push({ key, ...hook });
      }
    });
  }

  let loading = false;

  export let path;

  export let name;
  export let url;
  export let remote;
  export let hooks: Record<string, IHook>;
  export let actions: IAction[];
</script>

<Route {path} absolute={false}>
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
          <div class="fieldset">
            <Input label="Local name" bind:value={name} required />
            <Input label="Git URL" bind:value={url} required />
            <Input label="Push remote" bind:value={remote} required />
          </div>

          {#if usedHooks.length > 0}
            <div class="fieldset">
              <span class="legend">Hooks</span>
              {#each usedHooks as hook}
                <fieldset class="panel bg-srf text-on-srf flex flex-col gap-s">
                  <div class="flex justify-between items-center">
                    <span class="mg-i-m0">{hook.name}</span>
                    <Button
                      hue={0}
                      on:click={() => {
                        hooks[hook.key].commands = undefined;
                      }}
                    >
                      <Remove />
                    </Button>
                  </div>

                  {#each hook.commands as _, i}
                    <div class="flex gap-s-- flex-row">
                      <Input
                        label="Command"
                        bind:value={hooks[hook.key].commands[i]}
                      />
                      <Button
                        hue={0}
                        on:click={() => {
                          hooks[hook.key].commands = undefined;
                        }}
                      >
                        <Remove />
                      </Button>
                    </div>
                  {/each}

                  <Button
                    secondary
                    on:click={() => {
                      hooks[hook.key].commands = [
                        ...hooks[hook.key].commands,
                        "",
                      ];
                    }}
                  >
                    Add command
                    <Add />
                  </Button>
                </fieldset>
              {/each}
            </div>
          {/if}

          {#if unusedHooks.length > 0}
            <div class="fieldset">
              <span class="legend">Add hooks</span>
              <div class="flex flex-wrap gap-s-">
                {#each unusedHooks as hook}
                  <Button
                    half
                    on:click={() => {
                      hooks[hook.key].commands = [""];
                    }}
                  >
                    {hook.name}
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
                <Input label="Idle name" bind:value={actions[i].idleName} />
                <Input label="Active name" bind:value={actions[i].idleName} />
                <Input label="Command" bind:value={actions[i].command} />
                <div class="flex items-start flex-col pd-i-m0 gap-s">
                  <div class="label">User terminated</div>
                  <Toggle label="User terminated" />
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
                    idleName: "",
                    activeName: "",
                    command: "",
                    userTerminated: false,
                  },
                ];
              }}
            >
              Add action <Add />
            </Button>
          </div>
        </form>
      {/if}
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
