<script lang="ts">
  import { animateIn, animateOut } from "~/lib/visuals";
  import { route } from "~/lib/router";

  const voidTransition = { duration: 0, delay: 0 };

  const routeIn = (node) => {
    if ($route.firstDifferent[1] === pathLast) {
      return animateIn(node);
    } else {
      return voidTransition;
    }
  };

  const routeOut = (node) => {
    if ($route.firstDifferent[0] === pathLast) {
      return animateOut(node);
    } else {
      return voidTransition;
    }
  };

  $: pathLast = path.split("/").pop()

  export let path: string;
</script>

{#if $route.current.startsWith(path)}
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
