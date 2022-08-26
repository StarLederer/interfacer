<script lang="ts">
  export let type = "button";
  export let ghost = false;
  export let half = false;
  export let hue: number = null;
</script>

<button
  class="button interactable"
  class:is-colored={hue !== null}
  class:is-ghost={ghost}
  class:is-half={half}
  on:click
  {type}
  style={`
    --hue: ${hue ?? 0};
  `}
>
  <div class="container"><slot /></div>
</button>

<style lang="scss" global>
  .button,
  button {
    --background-s: 0%;
    --background-l: 100%;
    --background-hsl: var(--hue), var(--background-s), var(--background-l);
    --background-a: 100%;
    --background: hsla(var(--background-hsl), var(--background-a));
    --color-s: 0%;
    --color-l: 10%;
    --color: hsl(var(--hue), var(--color-s), var(--color-l));
    --glow-color-a: 40%;
    --glow-color: hsla(var(--background-hsl), var(--glow-color-a));
    --glow: 0 0 6rem var(--glow-color);
    --shadow: 0 0 0.4rem rgba(0, 0, 0, 0.6);

    background: var(--background);
    color: var(--color);
    position: relative;
    border-radius: var(--border-radius);
    box-shadow: var(--glow), var(--shadow);
    font-weight: 600;
    padding: 1rem;

    display: flex;
    align-items: stretch;
    /* justify-content: stretch; */
    transition: var(--transition);

    .container {
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      gap: var(--border-radius);
    }

    &.is-ghost {
      --background-l: 100%;
      --background-a: 0;
      --color-l: 80%;
      --glow: 0 0 0 transparent;
      --shadow: 0 0 0 transparent;

      &:hover,
      &.force-hover,
      &:active {
        --background-a: 0.1;
        --color-l: 100%;
      }
    }

    &.is-half {
      --background-l: 100%;
      --background-a: 5%;
      --color-l: 80%;
      --glow: 0 0 0 transparent;
      --shadow: 0 0 0 transparent;

      &:hover,
      &.force-hover,
      &:active {
        --background-a: 10%;
        --color-l: 100%;
      }
    }

    &:hover,
    &.force-hover,
    &:active {
      --background-l: 100%;
      --color-l: 10%;
    }

    &:focus {
      --shadow: 0 0 0 transparent;
      outline: none;

      &::before {
        --offset: 0.4rem;
        --background-a: 20%;
      }
    }

    &.is-colored {
      --background-s: 60%;
      --background-l: 60%;
      --color-s: 20%;
      --color-l: 10%;

      &:hover,
      &.force-hover,
      &:active {
        --background-s: 60%;
        --background-l: 80%;
        --color-s: 100%;
        --color-l: 10%;
      }

      &.is-ghost {
        --color-s: 60%;
        --color-l: 60%;

        &:hover,
        &.force-hover,
        &:active {
          --background-a: 0.2;
          --color-l: 80%;
        }
      }

      &.is-half {
        --background-a: 0.2;
        --color-s: 60%;
        --color-l: 60%;

        &:hover,
        &.force-hover,
        &:active {
          --color-l: 80%;
        }
      }
    }
  }
</style>
