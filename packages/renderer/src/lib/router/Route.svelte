<script lang="ts">
  import { route } from "~/lib/router";

  let duration = 150;
  let delay = 0;

  const routeIn = (node) => {
    const opacity = +getComputedStyle(node).opacity;

    return {
      delay,
      duration,
      css: (t) => `
        opacity: ${t * opacity};
        transform: scale(${1 - 0.025 * (1 - t)});
      `,
    };
  };

  const routeOut = (node) => {
    const opacity = +getComputedStyle(node).opacity;

    return {
      delay,
      duration,
      css: (t) => `
        opacity: ${t * opacity};
        transform: scale(${1 + 0.025 * (1 - t)});
      `,
    };
  };

  export let path: string;
</script>

{#if $route.startsWith(path)}
  <section in:routeIn out:routeOut>
    <slot />
  </section>
{/if}

<style>
  section {
    position: absolute;
    inset: 0;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    align-items: stretch;
  }
</style>
