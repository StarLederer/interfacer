<script lang="ts">
  import { style as describeStyle } from "$lib/css-engine/style.js";

  export let style: {
    borderRadius?: number;
  } = {};
</script>

<div style={describeStyle(style)} />

<style lang="scss">
  div {
    --base-l: 100%;
    --color-1: hsla(0, 0%, var(--base-l), 10%);
    --color-2: hsla(0, 0%, var(--base-l), 40%);

    border: 0.2rem solid var(--color-1);
    border-top: 0.2rem solid var(--color-2);
    border-radius: 50%;
    width: calc(var(--border-radius) * 2);
    height: calc(var(--border-radius) * 2);
    animation: spin 2s linear infinite;

    @media (prefers-color-scheme: light) {
      --base-l: 0%;
    }
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
