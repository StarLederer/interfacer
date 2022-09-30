<script>
  import Open from "svelte-material-icons/ArrowRight.svelte";
  import Menu from "svelte-material-icons/DotsVertical.svelte";
  import Favorite from "svelte-material-icons/Star.svelte";

  import { stringToHue } from "~/lib/visuals";
  import Button from "~/lib/primitives/Button.svelte";

  export let label = "";
</script>

<div class="panel bg-srf text-on-srf-2 round-m pad-m pad-block-l">
  <article>
    <div class="header">
      <div class="title text-on-srf">{label}</div>
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

    <div class="flex justify-between">
      <div class="flex">
        <Button solid on:click>
          Open
          <Open />
        </Button>
      </div>
      <Button>
        <Menu />
      </Button>
    </div>
  </article>
</div>

<style lang="postcss">
  article {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    & .header {
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
  }
</style>
