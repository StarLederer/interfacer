<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Active from "svelte-material-icons/Cog.svelte";
  import Button from "~/lib/primitives/Button.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";

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
  disabled={loading}
  {hue}
  on:click={interact}
>
  <div
    class="action-content"
    class:is-active={active && !loading}
    class:is-loading={loading}
  >
    <div class="progress-container">
      <Progress />
    </div>

    <div class="name-line">
      <div class="icon"><Active size={"100%"} /></div>
      <span class="name">{name}</span>
    </div>
    {#if error}
      <span class="name">{name}</span>
      <div class="error">
        <div>Error:</div>
        {error}
      </div>
      Try again?
    {/if}
  </div>
</Button>

<style lang="postcss">
  @keyframes rotate {
    from {
      transform: rotate(0deg);
    }
    to {
      transform: rotate(360deg);
    }
  }

  .action-content {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 1rem;

    & .progress-container {
      position: absolute;
      inset: 0;
      display: flex;
      justify-content: center;
      align-items: center;
      transition: var(--transition);

      opacity: 0;
      --border-radius: 1rem;
    }

    & .name-line {
      display: flex;
      align-items: center;
      transition: var(--transition);

      & .name {
        font-weight: 800;
        font-size: 2rem;
      }

      & .icon {
        width: 0rem;
        display: flex;
        animation: 10s rotate infinite;
        margin-inline-start: 0rem;
        transition: var(--transition);
      }
    }

    & .error {
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

      & div {
        color: hsl(0, var(--color-s), 60%);
        font-weight: 600;
      }
    }

    &.is-loading {
      & .name-line {
        opacity: 0;
      }

      & .progress-container {
        opacity: 1;
      }
    }

    &.is-active {
      & .icon {
        width: 2rem;
        margin-inline-end: 1rem;
      }
    }
  }
</style>
