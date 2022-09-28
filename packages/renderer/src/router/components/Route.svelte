<script lang="ts">
  import { route } from "~/router";
  import type { IRoute } from "~/router";

  let duration = 100;
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

  export let path: IRoute;
</script>

{#if $route.startsWith(path)}
  <section in:routeIn out:routeOut>
    <slot />
  </section>
{/if}

<style lang="scss">
  section {
    position: absolute;
    inset: 0;
    overflow: hidden;
    display: flex;
    align-items: stretch;
  }
</style>
