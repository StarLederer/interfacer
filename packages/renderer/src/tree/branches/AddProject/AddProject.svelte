<script lang="ts">
  import Save from "svelte-material-icons/Check.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { Route } from "~/lib/router";
  import Button from "~/lib/primitives/Button.svelte";
  import Progress from "~/lib/primitives/Progress.svelte";
  import Headerbar from "~/lib/Headerbar.svelte";
  import { navigate } from "~/lib/router";

  import type { IHook, IAction } from "./branches/Edit.svelte";
  import Edit from "./branches/Edit.svelte";

  let name;
  let url;
  let remote = "origin";
  let hooks: Record<string, IHook> = {
    before_each_action: {
      name: "Before each action",
      commands: undefined,
    },
    after_code_downlaod: {
      name: "After code download",
      commands: undefined,
    },
    before_code_upload: {
      name: "Before code upload",
      commands: undefined,
    },
  };
  let actions: IAction[] = [];

  export let path;
</script>

<Route {path}>
  <Headerbar
    title="Add project"
    back={() => {
      navigate("/websites");
    }}
  >
    <Button
      solid
      slot="actions"
      on:click={() => {
        navigate(`${path}/loading`);
        setTimeout(() => {
          navigate(`${path}/success`);
        }, 2000);
      }}
    >
      Save
      <Save />
    </Button>
  </Headerbar>

  <div class="container">
    <Edit
      path="{path}/edit"
      bind:name
      bind:url
      bind:remote
      bind:hooks
      bind:actions
    />

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
  </div>
</Route>

<style>
  .container {
    flex: 1;
    position: relative;
  }
</style>
