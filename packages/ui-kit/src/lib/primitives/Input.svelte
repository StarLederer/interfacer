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
  @use "../sass-resources/half";

  .input {
    --base-background-l: 100%;
    @include half.half;
    @include interactable.highlightable;
    @include interactable.transition;
    @include interactable.interactable(--border-radius, --background-hsl, 40%, --transition);

    --background-l: calc(var(--base-background-l) + var(--base-highlight-l));
    --highlight-l: 0%

    height: 4rem;
    width: 100%;
    background: hsl(0, 0%, var(--background-l), var(--background-a));
    border-radius: var(--border-radius);
    cursor: text;
    display: flex;

    .label {
      @include label.label;
    }

    input {
      min-width: 0;
      padding: var(--border-radius);
      padding-block-start: calc(var(--border-radius) + 1rem);
      font-size: 1rem;
      font-weight: 600;
      outline: none;
      border: none;
      background: none;
    }

    @media (prefers-color-scheme: light) {
      --base-background-l: 0%;
    }
  }
</style>
