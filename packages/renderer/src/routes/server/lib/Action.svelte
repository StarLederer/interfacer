<script lang="ts" context="module">
  type IActionStatus = "idle" | "activating" | "active" | "terminated";

  type IAction = {
    duration: "instant" | "self-terminated" | "user-terminated";
    names: { [K in IActionStatus]: string };
    depends?: {
      action: number;
      status: IActionStatus;
    }[];
  };

  export type { IActionStatus, IAction };
</script>

<script lang="ts">
  import Button from "~/lib/primitives/Button.svelte";

  export let action: IAction;
  export let hue: number;
  export let status: IActionStatus;
</script>

<Button {hue} half={status !== "active"} on:click>
  <div class="action-content">
    <span class="name">{action.names[status]}</span>
    <!-- {#if status === "active"}
      <div class="info">
        <table class="status">
          <tr>
            <th>Status: </th>
            <td>Running</td>
          </tr>
        </table>
        <div class="output">
        </div>
      </div>
    {/if} -->
  </div>
</Button>

<style lang="scss">
  @mixin use-background {
    --background: hsl(
      var(--hue),
      var(--background-s),
      var(--background-l),
      var(--background-a)
    );
  }

  .action-content {
    width: 100%;
    height: 100%;

    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: 1rem;

    .name {
      font-size: 2rem;
      font-weight: 800;
    }

    // .info {
    //   --border-radius: 1rem;

    //   color: white;
    //   background: var(--app-background);
    //   padding: var(--border-radius);
    //   border-radius: var(--border-radius);

    //   display: flex;
    //   flex-direction: column;
    //   gap: 1rem;

    //   .status {
    //     text-align: left;

    //     tr {
    //       display: flex;
    //       gap: 1rem;
    //     }
    //   }

    //   .output {
    //     max-height: 6rem;
    //     overflow: scroll;
    //     overflow: overlay;
    //     text-align: start;
    //   }
    // }
  }
</style>
