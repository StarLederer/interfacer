<script lang="ts">
  import Back from "svelte-material-icons/ChevronLeft.svelte";
  import Button from "~/lib/primitives/Button.svelte";

  export let title = null;
  export let back = null;
</script>

<div class="header-bar">
  <div class="title-side">
    {#if back || $$slots.titleActions}
    <div class="title-actions">
      {#if back}
        <Button half on:click={back}>
          <Back />
        </Button>
      {/if}

      <slot name="titleActions" />
    </div>
    {/if}


    {#if title}
      <h2>{title}</h2>
    {/if}
  </div>

  <slot name="actions" />
</div>

<style lang="postcss">
  .header-bar,
  .title-side,
  .title-actions {
    display: flex;
    align-items: center;

  }

  .header-bar {
    @apply pd-m0
      gap-m0
      justify-between;
  }

  .title-side {
    @apply flex1
      gap-m0;
  }

  .title-actions {
    @apply gap-s-;
  }
</style>
