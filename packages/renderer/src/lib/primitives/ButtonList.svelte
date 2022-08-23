<script>
  export let spread = false;
</script>

<div style={`justify-content: ${spread ? 'space-between' : 'unset'}`}>
  <slot />
</div>

<style>
  div {
    display: flex;
    flex-wrap: wrap;
    gap: 0.4rem;
  }
</style>
