<script lang="ts">
  import Git from "svelte-material-icons/Git.svelte"
  import Save from "svelte-material-icons/Check.svelte"
  import Panel from "~/lib/primitives/Panel.svelte"
  import Button from "~/lib/primitives/Button.svelte"
  import Input from "~/lib/primitives/Input.svelte"
  import Flex from "~/lib/helpers/Flex.svelte"

  import { inline } from "~/lib/css";

  let username;
  let password;

  const styles = {
    root: inline({ flex: 1, margin: "1rem", marginBlock: "2rem"}),
  };
</script>


<div class="container">
  <Panel hue={100}>
    <Flex direction="column" justify="space-between" class={styles.root} gap={1}>

      <div class="header">
        <div class="git">
          <Git size="2rem"/>
        </div>

        <span>Let's start by setting up your Git credentials</span>
      </div>

      <Flex direction="column" class={inline({width: "100%"})}>
        <Input label="Username" bind:value={username} />
        <Input label="Token" bind:value={password} />
      </Flex>
      <Flex direction="row" justify="space-between">
        <Button>Skip</Button>
        <Button solid>Save</Button>
      </Flex>
    </Flex>
  </Panel>
</div>

<style lang="scss">
  .container {
    max-width: 48rem;
    height: auto;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: auto
  }

  .header {
    margin-block-end: 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 1rem;

    span {
      max-width: 60%;
    }
  }

  .git {
    padding: 2rem;
    display: flex;
    background: hsl(var(--hue), 20%, 20%);
    border-radius: 50%;
  }
</style>
