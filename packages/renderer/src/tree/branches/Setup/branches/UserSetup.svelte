<script lang="ts">
  import Git from "svelte-material-icons/Git.svelte";
  import Save from "svelte-material-icons/Check.svelte";
  import Button from "~/lib/primitives/Button.svelte";
  import Input from "~/lib/primitives/Input.svelte";

  let username;
  let password;
</script>

<div class="container flex items-center justify-center">
  <div class="flex flex-col gap-m0">
    <div class="header">
      <div class="git bg-srf text-on-srf">
        <Git size="2rem" />
      </div>

      <span class="text-on-def-2">Let's start by setting up your Git credentials</span>
    </div>

    <div class="flex flex-col gap-s--">
      <Input label="Username" bind:value={username} />
      <Input label="Token" bind:value={password} />
    </div>
    <div class="flex justify-between">
      <Button>Skip</Button>
      <Button solid>Save</Button>
    </div>
  </div>
</div>

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
