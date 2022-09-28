<script lang="ts">
  export let borderRadius: number | null = null;
</script>

<div
  style={`${borderRadius != null ? `--borderRadius : ${borderRadius}` : ""}`}
/>

<style lang="scss">
  @use "../sass-lib/tokens/lightness.scss";

  div {
    @include lightness.index(--base-l);
    --color-1: hsla(0, 0%, var(--base-l), 10%);
    --color-2: hsla(0, 0%, var(--base-l), 40%);

    border: 0.2rem solid var(--color-1);
    border-top: 0.2rem solid var(--color-2);
    border-radius: 50%;
    width: calc(var(--border-radius) * 2);
    height: calc(var(--border-radius) * 2);
    animation: spin 2s linear infinite;
  }

  @keyframes spin {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(360deg);
    }
  }
</style>
