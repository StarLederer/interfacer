<script lang="ts">
  import { Route } from "~/lib/router";
  import Edit from "./branches/Edit.svelte";
  import Loading from "./branches/Loading.svelte";
  import Success from "./branches/Success.svelte";
  import Error from "./branches/Error.svelte";

  export let path;
</script>

<Route {path}>
  <Edit path="{path}/edit" />
  <Loading path="{path}/loading" />
  <Success path="{path}/success" />
  <Error path="{path}/error" />
</Route>
