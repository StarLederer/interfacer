<script lang="ts">
  import { Route } from "~/lib/router";
  import User from "./branches/UserSetup.svelte";
</script>

<Route path="/setup/user">
  <User />
</Route>
<Route path="/setup/done">
  <h1>All done!</h1>
</Route>
