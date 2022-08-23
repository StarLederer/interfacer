<script lang="ts">
  export let value;
  export let label: string;
  export let required = false;
</script>

<label class="input interactable" {required}>
  <div class="label">{label}</div>
  <input type="text" bind:value />
</label>

<style lang="scss">
  .input {
    --hue: 0;
    --saturation: 0%;
    --lightness: 100%;
    --background-a: 5%;

    height: 4rem;
    width: 100%;
    background: hsl(0, 0%, 100%, var(--background-a));
    border-radius: var(--border-radius);
    cursor: text;

    .label {
      z-index: 1;
    }

    input {
      padding: var(--border-radius);
      padding-block-start: calc(var(--border-radius) + 1rem);
      font-size: 1rem;
      font-weight: 600;
      outline: none;
      border: none;
      background: none;
    }

    &:hover {
      --background-a: 10%;
    }
  }
</style>
