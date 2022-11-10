<script lang="ts">
  export let type = "button";
  export let half = false;
  export let secondary = false;
  export let solid = false;
  export let disabled = false;

  export let hue: number | null = null;
  let cls: string = "round-m0 pd-m0";
  export { cls as class };
</script>

<button
  class={cls}
  class:is-half={half}
  class:is-secondary={secondary}
  class:is-solid={solid}
  {type}
  {disabled}
  style={`
    ${hue != null ? `--hue: ${hue};` : ""}
  `}
  on:click
>
  <div class="flex items-center justify-center flex-1 gap-m0"><slot /></div>
</button>

<style lang="postcss">
  button {
    @apply flex
      items-stretch
      int-text
      transition
      bg-none
      text-int
      border-none;

    font-weight: 600;

    &:disabled {
      cursor: not-allowed;
    }

    &:not(:disabled) {
      cursor: pointer;

      &:hover,
      &:active {
        @apply bg-int4 highlight;
      }

      &:active {
        @apply highlight+;
      }
    }

    &.is-half {
      @apply bg-int4;
    }

    &.is-secondary {
      @apply bg-int2;

      &:not(:disabled) {
        &:hover,
        &:active {
          @apply bg-int2;
        }
      }
    }

    &.is-solid {
      @apply bg-int;

      &:not(:disabled) {
        &:hover,
        &:active {
          @apply bg-int highlight;
        }

        &:active {
          @apply highlight+;
        }
      }
    }
  }
</style>
