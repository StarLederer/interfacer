<script lang="ts">
  export let hue: number = null;
  export let padding: number | [number, number] = 1;
</script>

<div
  class="panel"
  style={`
    --hue: ${hue};
    --padding-block: ${padding[0] ?? padding}rem;
    --padding-inline: ${padding[1] ?? padding}rem;
    ${hue === null ? "--background-s: 0%" : ""}
  `}
>
  <slot />
</div>

<style lang="scss" global>
  .panel {
    @mixin use-color {
      --color: hsl(var(--hue), var(--color-s), var(--color-l), var(--color-a));
    }

    --background-s: 20%;
    --background-l: 20%;
    --background-a: 20%;

    --border-width: 1px;
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
    padding-block: calc(var(--padding-block) - var(--border-width));
    padding-inline: calc(var(--padding-inline) - var(--border-width));

    @include use-color;
    color: var(--color);

    .title {
      --color-l: 80%;
      --color-s: 40%;
      --color-a: 100%;
      @include use-color;
      color: var(--color);

      font-weight: 600;
    }
  }
</style>
