<script lang="ts">
  import { Route } from "~/lib/router";
  import User from "./branches/UserSetup.svelte";
</script>

<User path="/setup/user" />
<Route path="/setup/done">
  <h1>All done!</h1>
</Route>
