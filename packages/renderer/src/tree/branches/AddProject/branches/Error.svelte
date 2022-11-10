<script lang="ts">
  import {Route, navigate} from "~/lib/router";
  import Button from "~/lib/primitives/Button.svelte";
  import { error } from "../store";

  export let path: string;
</script>

<Route {path}>
  <div class="flex flex-col gap-m0 justify-center items-center flex-1">
    <div>An error occured!</div>
    <div class="text-on1 text-center">{$error}</div>
    <Button
      solid
      on:click={() => {
        navigate("/add-website/edit");
      }}
    >
      Go back
    </Button>
  </div>
</Route>
