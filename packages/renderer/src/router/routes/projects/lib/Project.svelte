<script>
  import Open from "svelte-material-icons/ArrowRight.svelte";
  import Menu from "svelte-material-icons/DotsVertical.svelte";
  import Favorite from "svelte-material-icons/Star.svelte";

  import { stringToHue } from "~/lib/visuals";
  import Button from "ui-kit/primitives/Button.svelte";
  import Panel from "ui-kit/primitives/Panel.svelte";
  import Flex from "ui-kit/helpers/Flex.svelte";

  export let label = "";
</script>

<Panel style={{ hue: stringToHue(label) }}>
  <article>
    <div class="header">
      <div class="title">{label}</div>
      <Flex>
        <Button colored style={{ hue: stringToHue(label) }}>
          <Favorite />
        </Button>
      </Flex>
    </div>
    <p>A short description maybe?</p>
    <!-- <table>
      <thead>
        <tr>
          <th>Team</th>
          <th>Git status</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td class="team-members">
            {#each Array(Math.floor(Math.random() * 3 + 1)) as member}
              <div class="team-member" style={`--hue: ${Math.random() * 360}`} />
            {/each}
          </td>
          <td>Up to date</td>
        </tr>
      </tbody>
    </table> -->

    <Flex justify="space-between">
      <Flex>
        <Button solid colored style={{ hue: stringToHue(label) }} on:click>
          Open
          <Open />
        </Button>
      </Flex>
      <Button colored style={{ hue: stringToHue(label) }}>
        <Menu />
      </Button>
    </Flex>
  </article>
</Panel>

<style lang="scss">
  article {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    .header {
      display: flex;
      justify-content: space-between;
      align-items: center;
    }

    // table {
    //   font-weight: 400;
    // }

    // th {
    //   width: 50%;
    //   --color-a: 40%;
    //   text-align: start;
    //   font-weight: 400;
    // }

    // .team-members {
    //   display: flex;
    // }
    // .team-member {
    //   width: 1.4rem;
    //   aspect-ratio: 1/1;
    //   background: hsl(var(--hue), 40%, 50%);
    //   border-radius: 50%;
    // }
  }
</style>
