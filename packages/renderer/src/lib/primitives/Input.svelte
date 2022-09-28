<script lang="ts">
  export let value: string;
  export let label: string;
  export let required = false;
</script>

<label class="input" {required}>
  <div class="label">{label}</div>
  <input type="text" bind:value />
</label>

<style lang="scss">
  @use "../sass-lib/traits/focus-indicator.scss";
  @use "../sass-lib/traits/half-transparent.scss";
  @use "../sass-lib/traits/label.scss";

  @use "../sass-lib/tokens/lightness.scss";
  @use "../sass-lib/tokens/transition.scss";

  .input {
    // Base values
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
    @include focus-indicator.index(
      --border-radius,
      --outline-color,
      --transition
    );
    @include half-transparent.index(background, $h, $s, --l);

    // Properties
    transition: var(--transition);
    border-radius: var(--border-radius);
    height: 4rem;
    width: 100%;
    color: hsl(var($h), var($s), var(--l));
    cursor: text;
    display: flex;

    &:hover,
    &:active,
    &:focus-within {
      --highlight-l: var(--base-highlight-l);
      @include half-transparent.hover();
    }

    &:focus-within {
      @include focus-indicator.focus();
    }

    &:active {
      @include focus-indicator.active();
    }

    .label {
      @include label.index(--border-radius);
    }

    input {
      min-width: 0;
      flex: 1;
      border-radius: inherit;
      padding: var(--border-radius);
      color: inherit;
      padding-block-start: calc(var(--border-radius) + 1rem);
      font-size: 1rem;
      font-weight: 600;
      outline: none;
      border: none;
      background: none;
    }
  }
</style>
