<script lang="ts">
  import Arrow from "svelte-material-icons/UnfoldMoreHorizontal.svelte";
  import {
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

<style lang="scss" global>
  @use "../sass-resources/label";

  @use "../sass-lib/traits/button-text.scss";
  @use "../sass-lib/traits/focus-indicator.scss";
  @use "../sass-lib/traits/half-transparent.scss";
  @use "../sass-lib/traits/rounded.scss";

  @use "../sass-lib/tokens/lightness.scss";
  @use "../sass-lib/tokens/transition.scss";

  .listbox {
    // Base-values
    --base-hue: 0;
    --base-s: 0%;
    @include transition.index(--transition);
    @include lightness.index(--base-l);
    @include lightness.highlight(--base-highlight-l);

    // Computed values
    $h: --base-hue;
    $s: --base-s;
    --highlight-l: 0%;
    --l: calc(var(--base-l) + var(--highlight-l));
    --outline-color: hsl(var($h), var($s), var(--l));

    // Traits

    // Properties
    border-radius: var(--border-radius);
    position: relative;
    flex: 1;
    gap: 0.4rem;
    display: flex;
    flex-direction: column;

    .listbox-label {
      @include label.label;
    }

    .clickable {
      @include half-transparent.index(background, $h, $s, --l);
      @include button-text.index();
      @include rounded.index(--border-radius);

      color: hsl(var($h), var($s), var(--l));
      border: none;
      transition: var(--transition);
      cursor: pointer;
    }

    .listbox-button {
      // Traits
      @include focus-indicator.index(
        --border-radius,
        --outline-color,
        --transition
      );

      transition: var(--transition);
      height: 4rem;
      padding-block-start: calc(var(--border-radius) + 1rem);
      text-align: left;
      font-size: 1rem;

      &:hover,
      &:active {
        @include half-transparent.hover();
      }

      &:focus {
        @include focus-indicator.focus();
      }

      &:active {
        @include focus-indicator.active();
      }

      .icon {
        position: absolute;
        right: var(--border-radius);
        top: 1.5rem;
      }
    }

    .listbox-options {
      @include focus-indicator.index(
        --border-radius,
        --outline-color,
        --transition
      );

      list-style: none;
      display: flex;
      flex-direction: column;
      padding: 0;
      margin: 0;

      &:focus {
        @include focus-indicator.focus();
      }
    }

    .listbox-option {
      &:first-child {
        border-radius: var(--border-radius) var(--border-radius) 0 0;
      }

      &:last-child {
        border-radius: 0 0 var(--border-radius) var(--border-radius);
      }

      &.active {
        @include half-transparent.hover();
      }
    }

    // &.is-open {
    //   .listbox-button {
    //     border-radius: var(--border-radius) var(--border-radius) 0 0;
    //   }
    // }
  }
</style>
