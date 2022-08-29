<script lang="ts">
  export let value: string;
  export let label: string;
  export let required = false;
</script>

<label class="input interactable" {required}>
  <div class="label">{label}</div>
  <input type="text" bind:value />
</label>

<style lang="scss">
  @use "../sass-resources/label";
  @use "../sass-resources/interactable";

  .input {
    --hue: 0;
    @include interactable.baseColors;
    @include interactable.highlightable;
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
    @include interactable.half;

    border-radius: var(--border-radius);
    height: 4rem;
    width: 100%;
    cursor: text;
    display: flex;

    .label {
      @include label.label;
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
