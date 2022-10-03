<script lang="ts">
  export let radius: number = 1;
</script>

<div
  style="--radius: {radius}rem"
/>

<style lang="postcss">
  div {
    border: 0.2rem solid black;
    border-top: 0.2rem solid white;
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
