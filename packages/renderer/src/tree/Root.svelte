<script lang="ts">
  import { Route } from "~/lib/router";

  import Projects from "./branches/projects/Projects.svelte";
  import AddProject from "./branches/AddProject.svelte";
  import Server from "./branches/server/Server.svelte";
  import UserSettings from "./branches/Settings.svelte";
  import Setup from "./branches/setup/Setup.svelte";
</script>

<Route path="/websites">
  <Projects />
</Route>
<Route path="/add-website">
  <AddProject />
</Route>
<Route path="/server">
  <Server />
</Route>
<Route path="/settings">
  <UserSettings />
</Route>
<Route path="/setup">
  <Setup />
</Route>
