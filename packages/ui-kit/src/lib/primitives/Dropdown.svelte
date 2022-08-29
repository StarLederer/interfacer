<script lang="ts">
  import Arrow from "svelte-material-icons/UnfoldMoreHorizontal.svelte";
  import {
    Transition,
    Listbox,
    ListboxLabel,
    ListboxButton,
    ListboxOptions,
    ListboxOption,
  } from "@rgossiaux/svelte-headlessui";
  import { style as describeStyle } from "$lib/css-engine/style.js";

  export let label: string;
  export let selected: string;
  export let options: string[];
  export let style: {
    borderRadius?: number;
  } = {};
</script>

<Listbox
  value={"Listbox val"}
  class={({ open }) => "listbox" + (open ? " is-open force-focus" : "")}
  style={describeStyle(style)}
  on:change={(e) => (selected = e.detail)}
>
  <ListboxLabel class="listbox-label">{label}</ListboxLabel>
  <ListboxButton class="listbox-button">
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
              "listbox-option button" + (active ? " force-hover" : "")}
          >
            {option}
          </ListboxOption>
        {/if}
      {/each}
    </ListboxOptions>
  </Transition>
</Listbox>

<style lang="scss" global>
  @use "../sass-resources/label";
  @use "../sass-resources/interactable";

  .listbox {
    --hue: 0;
    @include interactable.baseColors;
    @include interactable.transition;
    @include interactable.outline(
      --border-radius,
      --background-hsl,
      40%,
      --transition
    );
    @include interactable.buttonColors(
      $hue: --hue,
      $base-background-l: --base-background-l,
      $base-color-l: --base-color-l,
      $highlight-l: --highlight-l
    );
    --background-a: 0%;

    border-radius: var(--border-radius);

    position: relative;
    flex: 1;
    display: flex;
    flex-direction: column;

    .listbox-label {
      @include label.label;
    }

    .listbox-button {
      @include interactable.highlightable;
      @include interactable.transition;
      @include interactable.buttonShape($border-radius: --border-radius);
      @include interactable.buttonColors(
        $hue: --hue,
        $base-background-l: --base-background-l,
        $base-color-l: --base-color-l,
        $highlight-l: --highlight-l
      );
      @include interactable.half;

      height: 4rem;
      padding-block-start: calc(var(--border-radius) + 1rem);

      cursor: pointer;
      text-align: left;
      font-size: 1rem;

      .icon {
        position: absolute;
        right: var(--border-radius);
        top: 1.5rem;
      }
    }

    .listbox-options {
      list-style: none;
      display: flex;
      flex-direction: column;
      padding: 0;
      margin: 0;
      border-radius: 0 0 var(--border-radius) var(--border-radius);
      overflow: hidden;
    }

    .listbox-option {
      @include interactable.transition;
      @include interactable.baseColors;
      @include interactable.highlightable;
      @include interactable.buttonColors(
        $hue: --hue,
        $base-background-l: --base-background-l,
        $base-color-l: --base-color-l,
        $highlight-l: --highlight-l
      );
      @include interactable.buttonShape($border-radius: --border-radius);
      @include interactable.half;
      border-radius: 0;
      cursor: pointer;
    }

    &.is-open {
      .listbox-button {
        border-radius: var(--border-radius) var(--border-radius)  0 0;
      }
    }
  }
</style>
