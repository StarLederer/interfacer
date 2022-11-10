<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { animateIn, animateOut } from "~/lib/visuals";
  import { route } from "~/lib/router";

  const dispatch = createEventDispatcher();

  // Transitions

  const voidTransition = { duration: 0, delay: 0 };

  const routeIn = (node, options) => {
    if ($route.firstDifferent[1] === pathLast) {
      return animateIn(node, options);
    } else {
      return voidTransition;
    }
  };

  const routeOut = (node, options) => {
    if ($route.firstDifferent[0] === pathLast) {
      return animateOut(node, options);
    } else {
      return voidTransition;
    }
  };

  $: pathLast = path.split("/").pop();

  // State

  $: isOpen = (() => {
    let open = false;

    if (strict) {
      open = $route.current === path;
    } else {
      open = $route.current.startsWith(path);
    }

    if (open) dispatch("open");
    else dispatch("close");

    return open;
  })();

  // Attributes

  export let strict: boolean = false;
  export let path: string;
  let cls: string = "flex flex-col items-stretch";
  export { cls as class };
</script>

{#if isOpen}
  <section class={cls} in:routeIn out:routeOut>
    <slot />
  </section>
{/if}

<style>
  section {
    position: absolute;
    inset: 0;
    overflow: hidden;
  }
</style>
