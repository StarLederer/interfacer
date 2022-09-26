<script lang="ts">
  import { style as describeStyle } from "$lib/css-engine/style.js";

  export let type = "button";
  export let half = false;
  export let solid = false;
  export let colored = false;
  export let disabled = false;
  export let style: {
    borderRadius?: number;
    hue?: number;
  } = {};
</script>

<button
  class:is-colored={colored}
  class:is-half={half}
  class:is-solid={solid}
  {type}
  {disabled}
  style={describeStyle(style)}
  on:click
>
  <div class="container"><slot /></div>
</button>

<style lang="scss">
  @use "../sass-lib/traits/button-text.scss";
  @use "../sass-lib/traits/focus-indicator.scss";
  @use "../sass-lib/traits/glow.scss";
  @use "../sass-lib/traits/half-transparent.scss";
  @use "../sass-lib/traits/rounded.scss";

  @use "../sass-lib/tokens/lightness.scss";
  @use "../sass-lib/tokens/transition.scss";

  button {
    // Base values
    @include lightness.highlight(--base-highlight-l);
    @include transition.index(--transition);
    @include lightness.index(--base-color-l);
    @include lightness.index(--base-background-l);

    // Computed values
    $h: --hue;
    --highlight-l: 0%;
    --color-l: calc(var(--base-color-l) + var(--highlight-l));
    --outline-color: hsl(var(--hue), var(--color-s), var(--color-l));
    --background-a: 0%;
    --background-s: 0%;
    --background-l: calc(var(--base-background-l) + var(--highlight-l));
    --background-a: 0%;
    --background-hsl: var(#{$h}), var(--background-s), var(--background-l);
    --background: hsla(var(--background-hsl), var(--background-a));
    --color-s: 0%;
    --color-l: calc(var(--base-color-l) + var(--highlight-l));
    --color: hsl(var(#{$h}), var(--color-s), var(--color-l));

    // Traits
    @include button-text.index();
    @include focus-indicator.index(
      --border-radius,
      --outline-color,
      --transition
    );
    @include rounded.index(--border-radius);

    // Properties
    background: var(--background);
    color: var(--color);
    border: none;
    display: flex;
    align-items: stretch;
    transition: var(--transition);

    &:disabled {
      cursor: not-allowed;
    }

    &:not(:disabled) {
      cursor: pointer;

      &:hover,
      &:active {
        --background-a: 20%;
        --highlight-l: var(--base-highlight-l);
        --outline-color: hsl(var(--background-hsl));
      }

      &:focus {
        @include focus-indicator.focus();
      }

      &:active {
        @include focus-indicator.active();
      }
    }

    .container {
      display: flex;
      flex: 1;
      justify-content: center;
      align-items: center;
      gap: var(--border-radius);
    }

    &.is-half {
      @include half-transparent.index(
        background,
        $h,
        --background-s,
        --background-l
      );

      &:not(:disabled) {
        &:hover,
        &:active {
          @include half-transparent.hover();
        }
      }
    }

    &.is-solid {
      @include lightness.highlight-on-solid(--base-highlight-l);
      @include lightness.inverse(--base-color-l);
      --background-a: 100%;
      --outline-color: hsl(var(--background-hsl));
      @include glow.index(--hue, --background-s, --background-l);

      &:hover,
      &:active {
        --background-a: 100%;
      }
    }

    &.is-colored {
      --base-highlight-l: 20%;
      --background-s: 60%;
      --base-background-l: 60%;
      --color-s: 60%;
      --base-color-l: 60%;

      &.is-half {
        @include half-transparent.hover();
      }

      &.is-solid {
        --base-color-l: 10%;
      }
    }

    @media (prefers-color-scheme: light) {
      &.is-colored {
        &:not(.is-solid) {
          --base-highlight-l: -20%;
          --base-color-l: 40%;
        }

        &.is-solid {
          --base-color-l: 10%;
          --base-highlight-l: 10%;
        }
      }
    }
  }
</style>
