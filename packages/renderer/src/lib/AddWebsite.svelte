<script>
  import route from "./router";

  let gitUrl = "";
</script>

<button
  on:click={() => {
    route.set("/websites");
  }}
>
  back</button
>

<form on:submit|preventDefault={() => {

}}>
  <p>You probably want to ask your developer to do this step for you</p>
  <label>
    <span>Git URL</span>
    <input type="url" bind:value={gitUrl}>
  </label>

  <input type="submit" value="Add website">
  <input type="button" value="Add locally">
</form>

<style>
  form {
    max-width: 20rem;
    display: flex;
    flex-direction: column;
  }
</style>
