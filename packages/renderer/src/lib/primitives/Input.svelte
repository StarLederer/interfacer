<script lang="ts">
  export let value: string;
  export let label: string;
  export let required = false;
</script>

<label class="input" {required}>
  <div class="label">{label}</div>
  <input type="text" bind:value />
</label>

<style lang="postcss">
  .input {
    @apply round-m border-color-int3 text-on-int3 transition;
    border-width: 1px;
    border-style: solid;
    width: 100%;
    cursor: text;
    display: flex;
    position: relative;

    &:hover {
      --highlight: -10%;
    }

    &:focus-within {
      @apply border-color-int;
    }

    & .label {
      @apply text-on-def-2;
      font-size: 0.8rem;
      position: absolute;
      top: 0;
      left: 1rem;
      line-height: 2.5rem;
    }

    & input {
      min-width: 0;
      flex: 1;
      color: inherit;
      border-radius: inherit;
      @apply pad-m int-text;
      padding-block-start: 2rem;
      outline: none;
      top: 0;
    }
  }
</style>
