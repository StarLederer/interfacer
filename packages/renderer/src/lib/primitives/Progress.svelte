<script lang="ts">
  export let radius: number = 1;
</script>

<div
  style="--radius: {radius}rem"
/>

<style lang="postcss">
  div {
    /* TODO: replace --fg with @apply when unocss-preset adds support */
    border: 0.2rem solid var(--fg0);
    border-top: 0.2rem solid var(--fg1);
    border-radius: 50%;
    width: calc(var(--radius) * 2);
    height: calc(var(--radius) * 2);
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
