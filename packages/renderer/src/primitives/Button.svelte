<script lang="ts">
  export let ghost = false;
  export let half = false;
  export let hue: number;
</script>

<button
  class="button"
  class:is-ghost={ghost}
  class:is-half={half}
  on:click
  style={`
    --hue: ${hue};
  `}
>
  <div><slot /></div>
</button>

<style>
  button div {
    position: relative;
    display: flex;
    gap: var(--border-radius);
  }
</style>
