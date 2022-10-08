<script lang="ts">
  import {Route, navigate} from "~/lib/router";
  import Button from "~/lib/primitives/Button.svelte";

  export let path: string;
</script>

<Route {path}>
  <div class="flex flex-col gap-m0 justify-center items-center flex-1">
    <div>Project added!</div>
    <Button
      solid
      on:click={() => {
        navigate("/websites");
      }}
    >
      Go to projects
    </Button>
  </div>
</Route>
