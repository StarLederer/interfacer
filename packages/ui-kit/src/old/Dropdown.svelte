<script>
  import Arrow from "svelte-material-icons/UnfoldMoreHorizontal.svelte";
  import Interactable from "./Interactable.svelte";
  import {
    Transition,
    Listbox,
    ListboxLabel,
    ListboxButton,
    ListboxOptions,
    ListboxOption,
  } from "@rgossiaux/svelte-headlessui";

  export let label;
  export let selected;
  export let options;
</script>

<Listbox
  value={"Listbox val"}
  class={({ open }) => "listbox interactable" + (open ? " is-open" : "")}
  on:change={(e) => (selected = e.detail)}
>
  <ListboxLabel class="listbox-label">{label}</ListboxLabel>
  <ListboxButton class="listbox-button is-half">
    {selected}
    <div class="icon">
      <Arrow />
    </div>
  </ListboxButton>
  <Transition enter="animate-in" leave="animate-out">
    <ListboxOptions class="listbox-options">
      {#each options as option}
        {#if option !== selected}
          <ListboxOption
            value={option}
            class={({ active }) =>
              "listbox-option button is-ghost" + (active ? " force-hover" : "")}
          >
            {option}
          </ListboxOption>
        {/if}
      {/each}
    </ListboxOptions>
  </Transition>
</Listbox>

<style lang="scss" global>
  .listbox {
    --hue: 0;
    --saturation: 0%;
    --lightness: 100%;

    .listbox-button {
      --color-l: 100%;
      padding-block-start: calc(var(--border-radius) + 1rem);
      height: 4rem;
      position: relative;

      .icon {
        position: absolute;
        right: var(--border-radius);
        top: 1.5rem;
      }
    }

    .listbox-options {
      list-style: none;
      // background: hsla(0, 0, 100%, 0.05);
    }

    .listbox-option {
      margin-block-start: 0.2rem;
      cursor: pointer;
    }

    &.is-open {
      margin-block: 0.4rem;

      &:focus-within {
        &::before {
          border: none;
          --border-width: 0px;
          --background-a: 5%;
          background: var(--background);
        }
      }
    }
  }
</style>
