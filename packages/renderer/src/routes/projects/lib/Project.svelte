<script>
  import Open from "svelte-material-icons/ArrowRight.svelte";
  import Menu from "svelte-material-icons/DotsVertical.svelte";
  import Favorite from "svelte-material-icons/Star.svelte";

  import { stringToHue } from "~/lib/visuals";
  import Button from "~/lib//primitives/Button.svelte";
  import Panel from "~/lib//primitives/Panel.svelte";
  import ButtonList from "~/lib//primitives/ButtonList.svelte";

  export let label = "";
</script>

<Panel hue={stringToHue(label)}>
  <article>
    <div class="header">
      <div class="title">{label}</div>
      <ButtonList>
        <Button hue={stringToHue(label)} ghost>
          <Favorite />
        </Button>
      </ButtonList>
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

    <ButtonList spread>
      <ButtonList>
        <Button hue={stringToHue(label)} on:click>
          Open
          <Open />
        </Button>
      </ButtonList>
      <Button hue={stringToHue(label)} ghost>
        <Menu />
      </Button>
    </ButtonList>
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
