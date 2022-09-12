<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from "svelte";
  import Refresh from "svelte-material-icons/Refresh.svelte";
  import Save from "svelte-material-icons/ContentSave.svelte";

  import route from "~/router";
  import { stringToHue } from "~/lib/visuals";
  import Button from "ui-kit/primitives/Button.svelte";
  import Flex from "ui-kit/helpers/Flex.svelte";
  import Progress from "ui-kit/primitives/Progress.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";

  import { name } from "./server";
  import Action from "./lib/Action.svelte";

  $: hue = stringToHue($name);

  let error;
  let actions: {
    name: string;
    active: boolean;
  }[] = [];

  onMount(async () => {
    try {
      await invoke("load_project", { name: $name });
      actions = await invoke("get_actions");
    } catch (err) {
      error = err;
    }
  });
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
    <Flex slot="title-actions">
      <Button half>
        <Save />
      </Button>
    </Flex>

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
          <Action name={action.name} {hue} {i} />
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
