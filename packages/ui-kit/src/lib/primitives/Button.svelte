<script lang="ts">
  import { style as describeStyle } from "$lib/css-engine/style.js";

  export let type = "button";
  export let ghost = false;
  export let half = false;
  export let colored = false;
  export let style: {
    borderRadius?: number;
    hue?: number;
  } = {};
</script>

<button
  class="interactable"
  class:is-colored={colored}
  class:is-ghost={ghost}
  class:is-half={half}
  {type}
  style={describeStyle(style)}
  on:click
>
  <div class="container"><slot /></div>
</button>

<style lang="scss">
  @use "../sass-resources/half";
  @use "../sass-resources/interactable.scss";

  button {
    --hue: 0;
    --base-background-l: 90%;
    --base-color-l: 0%;
    @include interactable.highlightable;
    @include interactable.transition;
    @include interactable.interactable(--border-radius, --background-hsl, 40%, --transition);

    --background-s: 0%;
    --background-l: calc(var(--base-background-l) + var(--base-highlight-l));
    --background-hsl: var(--hue), var(--background-s),
      calc(var(--base-background-l) + var(--highlight-l));
    --background-a: 100%;
    --background: hsla(var(--background-hsl), var(--background-a));
    --color-s: 0%;
    --color-l: calc(var(--base-color-l) + var(--highlight-l));
    --color: hsl(var(--hue), var(--color-s), var(--color-l));
    --glow-color-a: 40%;
    --glow-color: hsla(var(--background-hsl), var(--glow-color-a));
    --glow: 0 0 6rem var(--glow-color);
    --shadow: 0 0 0.4rem rgba(0, 0, 0, 0.6);

    position: relative;
    padding: var(--border-radius);

    background: var(--background);
    color: var(--color);
    border: none;
    border-radius: var(--border-radius);
    box-shadow: var(--glow), var(--shadow);
    line-height: 1;
    font-weight: 600;

    display: flex;
    align-items: stretch;
    cursor: pointer;

    .container {
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      gap: var(--border-radius);
    }

    &.is-ghost,
    &.is-half {
      --base-color-l: var(--base-background-l);
      --glow: 0 0 0 transparent;
      --shadow: 0 0 0 transparent;
      --color-s: var(--background-s);
    }

    &.is-ghost {
      --background-a: 0%;

      &:hover,
      &:active {
        --background-a: 10%;
      }
    }

    &.is-half {
      @include half.half;
    }

    &.is-colored {
      --base-color-l: 10%;
      --base-highlight-l: 20%;
      --background-s: 60%;
      --base-background-l: 60%;
      --color-s: 60%;

      &.is-ghost {
        --base-color-l: 60%;
        &:hover,
        &:active {
          --background-a: 20%;
        }
      }

      &.is-half {
        --base-color-l: 60%;
        --background-a: 20%;
      }
    }

    @media (prefers-color-scheme: light) {
      --base-background-l: 0%;
      --base-color-l: 100%;
      --glow-color-a: 60%;
      --shadow: 0 0 0 transparent;

      &.is-ghost,
      &.is-half {
        --base-highlight-l: -10%;
        --base-color-l: 10%;
      }

      &.is-colored {
        --base-highlight-l: 10%;

        &.is-ghost,
        &.is-half {
          --base-highlight-l: -20%;
          --base-color-l: 40%;
        }
      }
    }
  }
</style>
