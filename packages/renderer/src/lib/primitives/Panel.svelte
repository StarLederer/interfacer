<script lang="ts">
  let titleStyle = `
    color: hsl(var(--hue), 20%, var(--color-l));
    font-weight: 600;
  `;

  export let hue: number | null = null;
  export let borderRadius: number | null = null;
</script>

<div
  class="panel"
  style={`
  ${hue != null ? `--hue: ${hue};` : ""}
  ${borderRadius != null ? `--border-radius: ${borderRadius};` : ""}
`}
>
  <slot {titleStyle} />
</div>

<style lang="scss">
  @use "../sass-lib/tokens/lightness.scss";
  @use "../sass-lib/traits/rounded.scss";

  .panel {
    // Base
    --background-s: 40%;
    @include lightness.index(--background-l, lightness.$grade-20i);
    --background-a: 20%;

    --color-s: 60%;
    @include lightness.index(--color-l, lightness.$grade-40);
    --color-a: 80%;

    // Computed
    --color: hsl(var(--hue), var(--color-s), var(--color-l), var(--color-a));

    // Traits
    @include rounded.index(--border-radius);

    // Props
    background: hsla(
      var(--hue),
      var(--background-s),
      var(--background-l),
      var(--background-a)
    );
    color: var(--color);
  }
</style>
