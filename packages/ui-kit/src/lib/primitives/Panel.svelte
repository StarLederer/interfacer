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
  .panel {
    --background-s: 40%;
    --background-l: 20%;
    --background-a: 20%;

    --border-width: 2px;
    --border-s: var(--background-s);
    --border-l: 40%;
    --border-a: 20%;

    --color-s: 60%;
    --color-l: 60%;
    --color-a: 80%;

    background: hsla(
      var(--hue),
      var(--background-s),
      var(--background-l),
      var(--background-a)
    );
    border-style: solid;
    border-width: var(--border-width);
    border-color: hsla(
      var(--hue),
      var(--border-s),
      var(--border-l),
      var(--border-a)
    );
    border-radius: var(--border-radius);
    padding: calc(var(--border-radius) - var(--border-width));

    --color: hsl(var(--hue), var(--color-s), var(--color-l), var(--color-a));
    color: var(--color);

    @media (prefers-color-scheme: light) {
      --background-l: 80%;
      --color-l: 40%;
    }
  }
</style>
