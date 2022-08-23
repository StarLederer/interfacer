<script>
  import Check from "svelte-material-icons/Check.svelte";
  import { open } from "@tauri-apps/api/shell";
  import Button from "~/lib/primitives/Button.svelte";
  import ButtonList from "~/lib/primitives/ButtonList.svelte";

  export let active = false;
  export let action;
  export let hue;
  export let address;
</script>

<div class="action" style={`--hue: ${hue}`}>
  <div class="icon">
    <Check />
  </div>
  <div class="content">
    <span class="name">{action.name}</span>
    <span class="status">{action.status}</span>
    {#if action.buttons}
      {#each action.buttons as button}
        <Button
          {hue}
          half={!active}
          on:click={async () => {
            await open(address);
          }}>{button.name}</Button
        >
      {/each}
    {/if}
  </div>
</div>

<style lang="scss">
  @mixin use-color {
    --color: hsl(var(--hue), var(--color-s), var(--color-l), var(--color-a));
  }

  @mixin use-background {
    --background: hsl(
      var(--hue),
      var(--background-s),
      var(--background-l),
      var(--background-a)
    );
  }

  .action {
    --color-s: 60%;
    --color-l: 60%;
    --color-a: 1;
    --background-s: 40%;
    --background-l: 60%;
    --background-a: 0.1;

    @include use-background;
    background: var(--background);
    border-radius: var(--border-radius);
    border-style: solid;
    border-width: var(--border-width);
    border-radius: var(--border-radius);
    border-color: hsla(var(--hue), 60%, 80%, 10%);
    padding-block: calc(2rem - var(--border-width));
    padding-inline: calc(1rem - var(--border-width));

    display: flex;
    gap: 1rem;

    .dot {
      @include use-color;

      width: 1rem;
      height: 1rem;

      background: var(--color);
      border-radius: 50%;
    }

    .content {
      --background-a: 5%;

      display: flex;
      flex-direction: column;
      align-items: flex-start;
      gap: 0.4rem;

      .name {
        --color-l: 80%;
        --color-s: 40%;
        --color-a: 1;
        @include use-color;

        color: var(--color);
        font-weight: 600;
      }

      .status {
        --color-l: 60%;
        --color-s: 40%;
        --color-a: 0.8;
        @include use-color;

        color: var(--color);
      }
    }
  }
</style>
