<script lang="ts">
  import { Route } from "~/lib/router";

  import Projects from "./branches/Pprojects/Projects.svelte";
  import AddProject from "./branches/AddProject/AddProject.svelte";
  import Project from "./branches/Project/Project.svelte";
  import UserSettings from "./branches/Settings.svelte";
  import Setup from "./branches/Setup/Setup.svelte";
</script>

<Route path="/websites">
  <Projects />
</Route>
<AddProject path="/add-website" />
<Project path="/project"/>
<Route path="/settings">
  <UserSettings />
</Route>
<Route path="/setup">
  <Setup />
</Route>
