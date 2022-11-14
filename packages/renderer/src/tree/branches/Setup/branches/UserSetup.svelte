<script lang="ts">
  import Git from "svelte-material-icons/Git.svelte";
  import Save from "svelte-material-icons/Check.svelte";
  import { invoke } from "@tauri-apps/api/tauri";

  import { Route, navigate } from "~/lib/router";
  import Button from "~/lib/primitives/Button.svelte";
  import Input from "~/lib/primitives/Input.svelte";
  import Error from "~/lib/Error.svelte";

  let loading;
  let error;
  let username;
  let password;

  const resetState = () => {
    loading = false;
    error = undefined;
    username = undefined;
    password = undefined;
  };

  const save = async () => {
    loading = true;
    try {
      let partial = {
        git_username: username,
        git_password: password,
      };
      await invoke("update_user", { partial });
      navigate("/websites");
    } catch (err) {
      error = err;
      console.log(error);
    }
  };

  const skip = () => {
    navigate("/websites");
  };

  export let path: string;
</script>

<Route {path} on:open={resetState}>
  <div class="container flex items-center justify-center">
    <div class="flex flex-col gap-m0">
      <div class="header">
        <div class="git bg-srf">
          <Git size="2rem" />
        </div>

        <div class="flex flex-col gap-s-">
          <span class="text-on2"> Welcome to Interfacer! </span>

          <span class="text-on1">
            Let's start by setting up your Git credentials
          </span>
        </div>
      </div>

      {#if error}
        <Error {error} />
      {:else}
        <div class="flex flex-col gap-s--">
          <Input label="Username" bind:value={username} />
          <Input label="Token" bind:value={password} />
        </div>
        <div class="flex justify-between">
          <Button on:click={skip}>Skip</Button>
          <Button solid on:click={save}>Save</Button>
        </div>
      {/if}
    </div>
  </div>
</Route>

<style>
  .container {
    max-width: 48rem;
    margin: auto;
  }

  .header {
    margin-block-end: 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 2rem;
  }

  .git {
    padding: 2rem;
    display: flex;
    border-radius: 50%;
  }
</style>
