<script lang="ts">
  import Arrow from "svelte-material-icons/UnfoldMoreHorizontal.svelte";
  import {
    Listbox,
    ListboxLabel,
    ListboxButton,
    ListboxOptions,
    ListboxOption,
  } from "@rgossiaux/svelte-headlessui";

  export let label: string;
  export let selected: string;
  export let options: string[];

  export let borderRadius: number | null = null;
</script>

<Listbox
  value={"Listbox val"}
  class={({ open }) => "listbox" + (open ? " is-open force-focus" : "")}
  style={`
    ${borderRadius != null ? `--border-radius: ${borderRadius};` : ""}
  `}
  on:change={(e) => (selected = e.detail)}
>
  <ListboxLabel class="listbox-label">{label}</ListboxLabel>
  <ListboxButton class="listbox-button clickable">
    {selected}
    <div class="icon">
      <Arrow />
    </div>
  </ListboxButton>
  <ListboxOptions class="listbox-options">
    {#each options as option}
      {#if option !== selected}
        <ListboxOption
          value={option}
          class={({ active }) =>
            "listbox-option clickable" + (active ? " active" : "")}
        >
          {option}
        </ListboxOption>
      {/if}
    {/each}
  </ListboxOptions>
</Listbox>

<style lang="postcss" global>
  .listbox {
    @apply flex
      flex-col
      gap-s--
      flex-1
      border-color-int3
      text-on0
      rund-m;
    position: relative;

    &:hover {
      @apply highlight;
    }

    & .listbox-label {
      @apply text-on1;
      position: absolute;
      left: 1rem;
      line-height: 2.5rem;
      z-index: 1;
      font-size: 0.8rem;
      pointer-events: none;
    }

    & .clickable {
      @apply int-text
        transition
        pd-m0;
      border-color: inherit;
      cursor: pointer;
    }

    & .listbox-button {
      @apply round-m0;
      border-width: 1px;
      border-style: solid;
      color: inherit;
      padding-block-start: 2rem;
      text-align: left;

      & .icon {
        position: absolute;
        right: 1rem;
        top: 1.5rem;
      }
    }

    & .listbox-options {
      @apply round-m0;
      border-color: inherit;
      border-width: 1px;
      border-style: solid;
      list-style: none;
      display: flex;
      flex-direction: column;
      overflow: hidden;
    }

    & .listbox-option {
      &.active {
        @apply bg-int2 highlight;
      }
    }

    &:focus-within,
    &.is-open {
      & .listbox-options {
        @apply border-color-int;
      }
    }

    &.is-open {
      @apply border-color-int;

      & .listbox-button {
        border-end-start-radius: 0.4rem;
        border-end-end-radius: 0.4rem;
      }

      & .listbox-options {
        border-start-start-radius: 0.4rem;
        border-start-end-radius: 0.4rem;
      }
    }
  }
</style>
