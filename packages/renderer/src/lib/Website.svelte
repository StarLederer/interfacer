<script>
  import Play from "svelte-material-icons/Play.svelte";
  import Edit from "svelte-material-icons/Pencil.svelte";
  import { stringToHue } from "./visuals";
  import Button from "../primitives/Button.svelte";
  export let label = "";
</script>



<article style={`--hue: ${stringToHue(label)}`}>
  <span class="title">{label}</span>
  <Button hue={stringToHue(label)} on:click >
    <Play/>
  </Button>
  <Button hue={stringToHue(label)} >
    <Edit/>
  </Button>
</article>

<style>
  article {
    display: grid;
    gap: 0.4rem;
    grid-template-columns: auto 3rem 3rem;
    align-items: center;

    background: hsla(var(--hue), 20%, 20%, 20%);
    border-radius: 1rem;
    padding: 0.2rem;
    padding-inline-start: 1.2rem;
  }

  .title {
    color: hsl(var(--hue), 20%, 80%);
  }
</style>
