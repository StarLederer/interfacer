<script lang="ts">
  import {Route} from "~/lib/router";
  import Progress from "~/lib/primitives/Progress.svelte";

  export let path: string;
</script>

<Route {path}>
  <div class="flex flex-col gap-m0 justify-center items-center flex-1">
    <div>Adding project</div>
    <div class="text-on-def-3">Please wait</div>
    <Progress />
  </div>
</Route>
