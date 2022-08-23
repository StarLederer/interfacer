<script lang="ts">
  import Back from "svelte-material-icons/ChevronLeft.svelte";
  import Button from "~/lib/primitives/Button.svelte";
  import route from "~/router";

  export let title = null;
  export let back = null;
</script>

<div class="header-bar">
  <div class="action-bar">
    {#if back}
      <Button
        half
        on:click={back}
      >
        <Back />
      </Button>
    {/if}

    <slot name="actions" />
  </div>
  {#if title}
    <h2>{title}</h2>
  {/if}
</div>

<style>
  .header-bar {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .action-bar {
    display: flex;
    justify-content: space-between;
  }
</style>
