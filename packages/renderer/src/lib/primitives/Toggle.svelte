<script lang="ts">
  import { Switch } from "@rgossiaux/svelte-headlessui";

  export let on = false;
  export let label: string = undefined;
</script>

<Switch
  checked={on}
  on:change={(e) => (on = e.detail)}
  class="wrapp-switch {on ? 'on' : ''}"
>
  <div class="text-representation">Toggle {label}: {on ? "on" : "off"}</div>
  <div class="toggle" />
</Switch>

<style lang="postcss" global>
  .wrapp-switch {
    height: 2rem;
    min-width: 4rem;
    border: solid 1px;
    cursor: pointer;

    @apply bg-int3
      border-color-int3
      transition
      transition-all
      round-m0
      pd-s--;

    &:hover {
      @apply highlight;
    }

    &:active {
      @apply pd-s- highlight+;
    }

    & .text-representation {
      display: none;
    }

    & .toggle {
      height: 100%;
      width: 50%;

      position: relative;
      inset-inline-start: 0;
      transition: inherit;
      opacity: 60%;


      @apply round-m0 bg-int;
    }

    &:is(.on) {
      @apply border-color-int2;

      & .toggle {
        inset-inline-start: 50%;
        opacity: 100%;
      }
    }
  }
</style>
