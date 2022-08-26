<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/shell";
  import Refresh from "svelte-material-icons/Refresh.svelte";
  import Save from "svelte-material-icons/ContentSave.svelte";

  import route from "~/router";
  import { stringToHue } from "~/lib/visuals";
  import Button from "~/lib/primitives/Button.svelte";
  import ButtonList from "~/lib/primitives/ButtonList.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";

  import { name } from "./server";
  import Action from "./lib/Action.svelte";
  import type { IActionStatus, IAction } from "./lib/Action.svelte";

  let error;

  $: hue = stringToHue($name);

  const actions: IAction[] = [
    {
      names: {
        idle: "Start server",
        activating: "Starting the server...",
        active: "Stop server",
        terminated: "Server stopped",
      },
      duration: "user-terminated",
    },
    {
      names: {
        idle: "Open editor",
        active: "Opening editor in your browser",
        terminated: "Editor opened",
      },
      duration: "instant",
      depends: [
        {
          action: 0,
          status: "active",
        },
      ],
    },
    {
      names: {
        idle: "Deploy",
        active: "Deploying...",
        terminated: "Deploy successful",
      },
      duration: "self-terminated",
    },
  ];
  const actionStati: IActionStatus[] = Array(actions.length).fill("idle");
</script>

<section>
  <Headerbar
    title={$name}
    back={async () => {
      error = undefined;
      // await invoke("close_website");
      route.set("/websites");
    }}
  >
    <ButtonList slot="title-actions">
      <Button half>
        <Save />
      </Button>
    </ButtonList>

    <div slot="actions" class="header-bar-actions">
      <span> Up to date </span>
      <Button half>
        Refresh
        <Refresh />
      </Button>
    </div>
  </Headerbar>

  <main>
    {#if error}
      <p>An error has occured!</p>
      <p>{error}</p>
    {:else}
      <div class="actions">
        {#each actions as action, i}
          <Action
            {action}
            {hue}
            status={actionStati[i]}
            on:click={async () => {
              if (action.duration === "user-terminated") {
                if (actionStati[i] === "idle") {
                  actionStati[i] = "activating";
                  await invoke("start_action", {name: $name})
                  actionStati[i] = "active";
                } else if (actionStati[i] === "active") {
                  actionStati[i] = "terminated";
                  setTimeout(() => {
                    actionStati[i] = "idle";
                  }, 1000);
                }
              } else if (action.duration === "self-terminated") {
                if (actionStati[i] === "idle") {
                  actionStati[i] = "active";
                  setTimeout(() => {
                    actionStati[i] = "terminated";
                  }, 1000);
                  setTimeout(() => {
                    actionStati[i] = "idle";
                  }, 2000);
                }
              } else if (action.duration === "instant") {
                if (actionStati[i] === "idle") {
                  actionStati[i] = "active";
                  setTimeout(() => {
                    actionStati[i] = "terminated";
                  }, 1000);
                  setTimeout(() => {
                    actionStati[i] = "idle";
                  }, 2000);
                }
              }
            }}
          />
        {/each}
      </div>
    {/if}
  </main>
</section>

<style lang="scss">
  section {
    height: 100%;
    display: grid;
    grid-template-rows: min-content auto;
  }

  main {
    padding: var(--border-radius);
    padding-block-start: 0;
  }

  .header-bar-actions {
    display: flex;
    align-items: center;
    gap: 0.8rem;

    span {
      opacity: 0.6;
    }
  }

  p {
    color: hsla(0, 0%, 100%, 0.8);
  }

  .actions {
    height: 100%;
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(24rem, 1fr));
    gap: 0.4rem;
    --border-radius: 3rem;
  }
</style>
