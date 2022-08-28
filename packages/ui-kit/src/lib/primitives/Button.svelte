<script lang="ts">
  import { style as describeStyle } from "$lib/css-engine/style.js";

  export let type = "button";
  export let half = false;
  export let solid = false;
  export let colored = false;
  export let style: {
    borderRadius?: number;
    hue?: number;
  } = {};
</script>

<button
  class="interactable"
  class:is-colored={colored}
  class:is-half={half}
  class:is-solid={solid}
  {type}
  style={describeStyle(style)}
  on:click
>
  <div class="container"><slot /></div>
</button>

<style lang="scss">
  @use "../sass-resources/interactable";

  @mixin glow() {
    --glow-color-a: 40%;
    --glow-color: hsla(var(--background-hsl), var(--glow-color-a));
    --glow: 0 0 6rem var(--glow-color);
    --shadow: 0 0 0.4rem rgba(0, 0, 0, 0.6);

    box-shadow: var(--glow), var(--shadow);

    @media (prefers-color-scheme: light) {
      --glow-color-a: 60%;
      --shadow: 0 0 0 transparent;
    }
  }

  button {
    @include interactable.baseColors;
    @include interactable.highlightable;
    @include interactable.transition;
    @include interactable.outline(
      --border-radius,
      --background-hsl,
      40%,
      --transition
    );
    @include interactable.buttonShape($border-radius: --border-radius);
    @include interactable.buttonColors(
      $hue: --hue,
      $base-background-l: --base-background-l,
      $base-color-l: --base-color-l,
      $highlight-l: --highlight-l
    );

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

    &.is-half {
      @include interactable.half;
    }

    &.is-solid {
      --base-color-l: 0%;
      --background-a: 100%;
      @include glow();
    }

    &.is-colored {
      --base-highlight-l: 20%;
      --background-s: 60%;
      --base-background-l: 60%;
      --color-s: 60%;
      --base-color-l: 60%;

      &.is-half {
        --background-a: 20%;
      }

      &.is-solid {
        --base-color-l: 10%;
      }
    }

    @media (prefers-color-scheme: light) {
      &.is-solid {
        --base-highlight-l: 10%;
        --base-color-l: 90%;
      }

      &.is-colored {
        --base-highlight-l: -20%;
        --base-color-l: 40%;

        &.is-solid {
          --base-color-l: 10%;
          --base-highlight-l: 10%;
        }

        &.is-solid {
          --base-background-l: 60%;
        }
      }
    }
  }
</style>
