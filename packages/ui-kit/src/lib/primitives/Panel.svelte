<script lang="ts">
  import { style as describeStyle } from "$lib/css-engine/style.js";

  let titleStyle = `
    color: hsl(var(--hue), 20%, var(--color-l));
    font-weight: 600;
  `;

  export let style: {
    hue: number;
    borderRadius?: number;
  };
</script>

<div class="panel" style={describeStyle(style)}>
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
