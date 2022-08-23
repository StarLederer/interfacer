<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import Refresh from "svelte-material-icons/Refresh.svelte";

  import route from "~/router";
  import { stringToHue } from "~/lib/visuals";
  import Button from "~/lib/primitives/Button.svelte";
  import ButtonList from "~/lib/primitives/ButtonList.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";

  import { name } from "./server";
  import Action from "./lib/Action.svelte";

  let error;
  let address;

  // $: (async () => {
  //   try {
  //     address = await invoke("open_website", {
  //       name: $name,
  //     });
  //   } catch (err) {
  //     error = err;
  //   }
  // })();
  address = "webwriter.org";

  $: hue = stringToHue($name);

  let activeAction = 2;
  let actions = [
    {
      name: "Code",
      builtIn: true,
      commands: ["git fetch"],
      status: "Up to date",
    },
    {
      name: "Environment",
      status: "No need. Skipped",
      buttons: [
        {
          name: "Refresh",
          commands: [
            "docker build . -t test-vite",
            "docker run --rm -it -v $(pwd):/usr/src/app test-vite pnpm install",
          ],
        },
      ],
    },
    {
      name: "Dev server",
      commands: [
        "docker run --rm -it -v $(pwd):/usr/src/app -p 5173:5173 test-vite pnpm run dev --host --port 5138",
      ],
      status: "Running",
      buttons: ["Open editor", "Stop server"],
    },
    {
      name: "Apply changes",
      commands: ["git add .", 'git commit -m "Make changes with Wrapp"'],
      status: "No changes made",
      buttons: ["Upload changes", "Discard"],
    },
  ];
</script>

<section>
  <Headerbar
    title={$name}
    back={async () => {
      error = undefined;
      address = undefined;
      await invoke("close_website");
      route.set("/websites");
    }}
  >
    <div slot="actions" class="header-bar-actions">
      <span> Up to date </span>
      <Button
        half
        on:click={async () => {
          await open(address);
        }}
      >
        Refresh
        <Refresh />
      </Button>
    </div>
  </Headerbar>

  {#if error}
    <p>An error has occured!</p>
    <p>{error}</p>
  {:else if address}
    <p>A short description maybe?</p>

    <h3>Actions</h3>
    <div class="actions">
      {#each actions as action, i}
        <Action
          {action}
          hue={hue + i * 10}
          {address}
          active={i === activeAction}
        />
      {/each}
    </div>
  {:else}
    <p>Launching the development server</p>
    <Progress />
  {/if}
</section>

<style lang="scss">
  section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
