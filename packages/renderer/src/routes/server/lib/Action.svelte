<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { open } from "@tauri-apps/api/shell";
  import Button from "ui-kit/primitives/Button.svelte";
  import Progress from "ui-kit/primitives/Progress.svelte";

  let error;
  let active = false;
  let loading = false;

  const interact = async () => {
    if (loading) return;

    try {
      loading = true;
      error = null;
      active = true;

      // const timer = setTimeout(() => {}, 1000);
      const before = Date.now();
      const res = (await invoke("interact", { actionI: i })) as {
        name: string;
        active: boolean;
      };

      setTimeout(() => {
        name = res.name;
        active = res.active;
        loading = false;
      }, 1000 - (Date.now() - before));
    } catch (err) {
      error = err;
      loading = false;
    }
  };

  export let name: string;
  export let hue: number;
  export let i: number;
</script>

<Button
  half={!active}
  solid={active}
  colored
  style={{ hue }}
  on:click={interact}
>
  <div class="action-content-container">
    <div class="action-content">
      {#if loading}
        <Progress style={{ borderRadius: 1 }} />
      {:else if error}
        <span class="name">{name}</span>
        <div class="error">
          <div>Error:</div>
          {error}
        </div>
        Try again?
      {:else}
        <span class="name">{name}</span>
      {/if}
    </div>
  </div>
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
    align-items: center;
    gap: 1rem;

    .name {
      font-size: 2rem;
      font-weight: 800;
    }

    .error {
      --border-radius: 1rem;
      --color-s: 40%;
      --color-l: 80%;

      color: hsl(var(--hue), var(--color-s), var(--color-l));
      background: var(--app-background);
      border-radius: var(--border-radius);
      padding: var(--border-radius);
      text-align: start;
      font-weight: 400;

      display: flex;
      flex-direction: column;
      align-items: flex-start;
      gap: 0.4rem;

      div {
        color: hsl(0, var(--color-s), 60%);
        font-weight: 600;
      }
    }
  }
</style>
