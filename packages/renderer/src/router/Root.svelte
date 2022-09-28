<script lang="ts">
  import { Route } from "~/router";

  import Projects from "./routes/projects/Projects.svelte";
  import AddProject from "./routes/AddProject.svelte";
  import Server from "./routes/server/Server.svelte";
  import UserSettings from "./routes/UserSettings.svelte";
  import Setup from "./routes/setup/Setup.svelte";
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
