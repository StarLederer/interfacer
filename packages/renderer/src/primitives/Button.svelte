<script lang="ts">
  export let hue: number;
</script>

<button class="button"
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
  }
</style>
