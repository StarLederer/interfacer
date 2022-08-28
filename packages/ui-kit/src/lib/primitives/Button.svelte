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
  @forward "../sass-resources/interactable.scss";

  button {
    --hue: 0;
    --hover-highlight: 10%;

    --highlight: 0%;
    --background-s: 0%;
    --background-l: 90%;
    --background-hsl: var(--hue), var(--background-s),
      calc(var(--background-l) + var(--highlight));
    --background-a: 100%;
    --background: hsla(var(--background-hsl), var(--background-a));
    --color-s: 0%;
    --color-l: 0%;
    --color: hsl(
      var(--hue),
      var(--color-s),
      calc(var(--color-l) + var(--highlight))
    );
    --glow-color-a: 40%;
    --glow-color: hsla(var(--background-hsl), var(--glow-color-a));
    --glow: 0 0 6rem var(--glow-color);
    --shadow: 0 0 0.4rem rgba(0, 0, 0, 0.6);
    --transition: 100ms;

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
    transition: var(--transition);
    cursor: pointer;

    &::before {
      --background-a: 40%;
    }

    .container {
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      gap: var(--border-radius);
    }

    &:hover,
    &:active {
      --highlight: var(--hover-highlight);
    }

    &.is-ghost,
    &.is-half {
      --glow: 0 0 0 transparent;
      --shadow: 0 0 0 transparent;
      --color-s: var(--background-s);
      --color-l: var(--background-l);

      &:hover,
      &:active {
        --background-a: 10%;
      }
    }

    &.is-ghost {
      --background-a: 0%;
    }

    &.is-half {
      --background-a: 5%;
    }

    &.is-colored {
      --hover-highlight: 20%;
      --background-s: 60%;
      --background-l: 60%;
      --color-s: 60%;
      --color-l: 10%;

      &.is-ghost {
        --color-l: 60%;
        &:hover,
        &:active {
          --background-a: 20%;
        }
      }

      &.is-half {
        --color-l: 60%;
        --background-a: 20%;
      }
    }

    @media (prefers-color-scheme: light) {
      --background-l: 0%;
      --color-l: 100%;
      --glow-color-a: 60%;
      --shadow: 0 0 0 transparent;

      &.is-ghost,
      &.is-half {
        --hover-highlight: -10%;
        --color-l: 10%;
      }

      &.is-colored {
        --hover-highlight: 10%;

        &.is-ghost,
        &.is-half {
          --hover-highlight: -20%;
          --color-l: 40%;
        }
      }
    }
  }
</style>
