<script lang="ts">
  export let inline = false;
  export let block = false;
</script>

<div
  style={`
    overflow-inline: ${inline};
    overflow-block: ${block};
  `}
>
  <slot />
</div>
