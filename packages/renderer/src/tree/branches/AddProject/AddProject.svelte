<script lang="ts">
  import { Route } from "~/lib/router";
  import Button from "~/lib/primitives/Button.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import { navigate } from "~/lib/router";
  import Edit from "./branches/Edit.svelte";

  export let path;
</script>

<Route {path}>
  <div class="container">
    <Edit path="{path}/edit" />

    <Route path="{path}/loading">
      <div class="flex flex-col gap-m0 justify-center items-center flex-1">
        <div>Adding project</div>
        <div class="text-on-def-3">Please wait</div>
        <Progress />
      </div>
    </Route>

    <Route path="{path}/success">
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

    <Route path="{path}/error">
      <div class="flex flex-col gap-m0 justify-center items-center flex-1">
        <div>An error occured!</div>
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
  </div>
</Route>

<style>
  .container {
    flex: 1;
    position: relative;
  }
</style>
