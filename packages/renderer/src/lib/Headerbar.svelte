<script lang="ts">
  import Back from "svelte-material-icons/ChevronLeft.svelte";
  import Button from "~/lib/primitives/Button.svelte";

  export let title = null;
  export let back = null;
</script>

<div class="header-bar">
  <div class="title-side">
    <div class="title-actions">
      {#if back}
        <Button half on:click={back}>
          <Back />
        </Button>
      {/if}

      <slot name="title-actions" />
    </div>

    {#if title}
      <h2>{title}</h2>
    {/if}
  </div>

  <slot name="actions" />
</div>

<style>
  .header-bar,
  .title-side,
  .title-actions {
    display: flex;
    align-items: center;

  }

  .header-bar {
    gap: 1rem;
    justify-content: space-between;
    padding: var(--border-radius);
  }

  .title-side {
    flex:1;
    gap: 1rem;
  }

  .title-actions {
    gap: 0.4rem;
  }
</style>
