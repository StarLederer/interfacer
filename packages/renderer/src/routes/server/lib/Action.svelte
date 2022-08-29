<script lang="ts" context="module">
  type IAction = {
    name: string;
    commands: {
      lang: "bash" | "built-in";
      command: string;
    }[];
  };

  export type { IAction };
</script>

<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/shell";
  import Button from "ui-kit/primitives/Button.svelte";
  import Progress from "ui-kit/primitives/Progress.svelte";

  let active = false;
  let loading = false;

  export let action: IAction;
  export let hue: number;
  export let i: number;
</script>

<Button
  half={!active}
  solid={active}
  colored
  style={{ hue }}
  on:click={async () => {
    if (loading) return;

    if (!active) {
      try {
        loading = true;
        active = true;
        await invoke("start_action", { i });
        loading = false;
      } catch (err) {
        active = false;
        loading = false;
        alert(err);
      }
    } else {
      try {
        loading = true;
        await invoke("stop_action");
        loading = false;
        active = false;
      } catch (err) {
        alert(err);
      }
    }
  }}
>
  {#if loading}
    <Progress />
  {:else}
    <div class="action-content">
      <span class="name">{action.name}</span>
    </div>
  {/if}
</Button>

<style lang="scss">
  @mixin use-background {
    --background: hsl(
      var(--hue),
      var(--background-s),
      var(--background-l),
      var(--background-a)
    );
  }

  .action-content {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 1rem;

    .name {
      font-size: 2rem;
      font-weight: 800;
    }
  }
</style>
