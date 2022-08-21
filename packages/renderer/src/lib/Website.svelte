<script>
  import Play from "svelte-material-icons/Play.svelte";
  import Menu from "svelte-material-icons/DotsVertical.svelte";
  import Favorite from "svelte-material-icons/Star.svelte";
  import { stringToHue } from "./visuals";
  import Button from "../primitives/Button.svelte";
  import ButtonList from "../primitives/ButtonList.svelte";

  export let label = "";
</script>

<article style={`--hue: ${stringToHue(label)}`}>
  <div class="header">
    <div class="title">{label}</div>
    <ButtonList>
      <Button hue={stringToHue(label)} ghost>
        <Favorite />
      </Button>
      <!-- <Button hue={stringToHue(label)} ghost>
        <Menu />
      </Button> -->
    </ButtonList>
  </div>
  <!-- <p>A short description maybe?</p> -->
  <table>
    <thead>
      <tr>
        <th>Team</th>
        <th>Last modified</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td class="team-members">
          {#each Array(Math.floor(Math.random() * 3 + 1)) as member}
            <div class="team-member" style={`--hue: ${Math.random() * 360}`} />
          {/each}
        </td>
        <td>Recently</td>
      </tr>
    </tbody>
  </table>

  <ButtonList spread>
    <Button hue={stringToHue(label)} on:click>
      Launch
      <Play />
    </Button>
    <Button hue={stringToHue(label)} ghost>
      <Menu />
    </Button>
  </ButtonList>
</article>

<style lang="scss">
  article {
    display: flex;
    flex-direction: column;
    gap: 1rem;

    background: hsla(var(--hue), 20%, 20%, 20%);
    --border-width: 1px;
    border-style: solid;
    border-width: var(--border-width);
    border-color: hsla(var(--hue), 20%, 40%, 20%);
    border-radius: 1rem;
    padding: calc(1rem - var(--border-width));

    * {
      --color-l: 60%;
      --color-hsl: var(--hue), 40%, var(--color-l);
      --color-a: 80%;
      --color: hsla(var(--color-hsl), var(--color-a));
      color: var(--color);
    }

    .header {
      display: flex;
      justify-content: space-between;
      align-items: center;
    }

    .title {
      --color-l: 80%;
      --color-a: 100%;
      font-weight: 600;
    }

    table {
      font-weight: 400;
    }

    th {
      width: 50%;
      --color-a: 40%;
      text-align: start;
      font-weight: 400;
    }

    .team-members {
      display: flex;
      // gap: 0.2rem;
    }
    .team-member {
      width: 1.4rem;
      aspect-ratio: 1/1;
      background: hsl(var(--hue), 40%, 50%);
      border-radius: 50%;
    }
  }
</style>
