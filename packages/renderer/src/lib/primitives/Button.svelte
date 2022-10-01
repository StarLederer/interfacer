<script lang="ts">
  export let type = "button";
  export let half = false;
  export let secondary = false;
  export let solid = false;
  export let disabled = false;

  export let hue: number | null = null;
  export let borderRadius: number | null = null;
</script>

<button
  class:is-half={half}
  class:is-secondary={secondary}
  class:is-solid={solid}
  {type}
  {disabled}
  style={`
    ${hue != null ? `--hue: ${hue};` : ""}
    ${borderRadius != null ? `--border-radius: ${borderRadius};` : ""}
  `}
  on:click
>
  <div class="flex items-center justify-center flex-1 gap-m0"><slot /></div>
</button>

<style lang="postcss">
  button {
    @apply flex
      items-stretch
      round-m0
      pad-m0
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
        @apply bg-int3 text-on-int3;
        --highlight: -10%;
      }

      &:active {
        --highlight: -20%;
      }
    }

    &.is-half {
      @apply bg-int3 text-on-int3;
    }

    &.is-secondary {
      @apply bg-int2 text-on-int2;

      &:not(:disabled) {
        &:hover,
        &:active {
          @apply bg-int2 text-on-int2;
        }
      }
    }

    &.is-solid {
      @apply bg-int text-on-int;

      &:not(:disabled) {
        &:hover,
        &:active {
          @apply bg-int text-on-int;
          --highlight: 10%;
        }

        &:active {
          --highlight: 20%;
        }
      }
    }
  }
</style>
