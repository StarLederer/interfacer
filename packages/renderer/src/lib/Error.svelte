<script lang="ts">
  export let error;
</script>

<div class="error">
  <p>An error has occured!</p>
  <p class="text-on1">{error}</p>
</div>

<style lang="postcss">
  .error {
    --hue: 0;
    @apply bg-srf
      round-m0
      pd-m0;
  }
</style>
