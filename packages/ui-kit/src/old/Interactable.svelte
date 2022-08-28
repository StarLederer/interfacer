<script>
  export let hue;
  export let saturation;
  export let lightness;
</script>

<div
  class="interactable"
>
  <slot />
</div>

<style lang="scss">
  @forward "../lib/sass-resources/interactable.scss";
</style>
