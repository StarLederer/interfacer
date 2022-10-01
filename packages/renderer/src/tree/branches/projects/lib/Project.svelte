<script>
  import Open from "svelte-material-icons/ArrowRight.svelte";
  import Menu from "svelte-material-icons/DotsVertical.svelte";
  import Favorite from "svelte-material-icons/Star.svelte";

  import { stringToHue } from "~/lib/visuals";
  import Button from "~/lib/primitives/Button.svelte";

  export let label = "";
</script>

<div class="panel bg-srf text-on-srf-2 flex flex-col gap-m0">
  <div class="flex flex-col gap-s pd-bs-m0">
    <div class="header">
      <div class="title text-on-srf font-semibold">{label}</div>
    </div>
    <p>A short description maybe?</p>
  </div>

  <div class="flex gap-s- justify-between">
    <div class="flex">
      <Button solid on:click>
        Open
        <Open />
      </Button>
    </div>
    <Button secondary>
      <Menu />
    </Button>
  </div>
</div>
