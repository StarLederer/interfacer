<script>
  export let hue;
  export let saturation;
  export let lightness;
</script>

<div
  class="interactable"
  style={`
  --hue: ${hue};
  --saturation: ${saturation};
  --lightness: ${lightness};
`}
>
  <slot />
</div>

<style lang="scss" global>
  .interactable {
    position: relative;
    border-radius: var(--border-radius);
    display: flex;
    flex-direction: column;
    transition: var(--transition);

    * {
      outline: none;
    }

    &::before {
      content: "";

      --offset: 0rem;
      --background-a: 0%;
      --background-hsl: var(--hue), var(--saturation), var(--lightness);
      --background-a: 0%;
      --background: hsla(var(--background-hsl), var(--background-a));
      --border-width: 0px;

      display: block;
      position: absolute;
      top: calc(var(--offset) * -1);
      left: calc(var(--offset) * -1);
      width: calc(100% + var(--offset) * 2 - var(--border-width) * 2);
      height: calc(100% + var(--offset) * 2 - var(--border-width) * 2);
      border-radius: calc(var(--border-radius) + var(--offset));
      border-width: var(--border-width);
      border-style: solid;

      border-color: var(--background);
      transition: var(--transition);
    }

    .force-focus,
    &:focus,
    &:focus-within {
      &::before {
        --offset: 0.4rem;
        --background-a: 20%;
        --border-width: 2px;
      }
    }

    &:active {
      &::before {
        --offset: 0.2rem;
        --border-width: 1px;
      }
    }
  }
</style>
