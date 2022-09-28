<script lang="ts">
  import { style } from "ui-kit/css-engine/style";
  import { navigate, Root } from "~/router";

  navigate("/setup/user");
</script>

<main
  style={style({
    hue: 0,
    borderRadius: 1,
  })}
>
  <Root />
</main>

<style>
  main {
    position: fixed;
    inset: 0;
  }
</style>
