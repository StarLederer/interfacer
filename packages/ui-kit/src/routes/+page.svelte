<script lang="ts">
  import Icon from "svelte-material-icons/Check.svelte";

  import { style } from "$lib/css-engine/style";
  import Button from "$lib/primitives/Button.svelte";
</script>

<main
  style={style({
    borderRadius: 1,
  })}
>
  <header>
    <h1>Ui Library</h1>
  </header>
  <article>
    <h2>Buttons</h2>
    <div class="table">
      <div class="row">
        <div class="demo">
          <div>
            <Button />
          </div>
          <div>Empty</div>
        </div>
        <div class="demo">
          <div>
            <Button><Icon /></Button>
          </div>
          <div>Icon</div>
        </div>
        <div class="demo">
          <div>
            <Button>Button</Button>
          </div>
          <div>Text</div>
        </div>
        <div class="demo">
          <div>
            <Button>Button <Icon /></Button>
          </div>
          <div>Text & icon</div>
        </div>
      </div>

      <div class="row">
        <div class="demo">
          <div>
            <Button ghost>Button</Button>
          </div>
          <div>Ghost</div>
        </div>
        <div class="demo">
          <div>
            <Button half>Button</Button>
          </div>
          <div>Half</div>
        </div>
        <div class="demo">
          <div>
            <Button colored ghost style={{ hue: 100 }}>Button</Button>
          </div>
          <div>Colored ghost</div>
        </div>
        <div class="demo">
          <div>
            <Button colored half style={{ hue: 100 }}>Button</Button>
          </div>
          <div>Colored half</div>
        </div>
        <div class="demo">
          <div>
            <Button colored style={{ hue: 100 }}>Button</Button>
          </div>
          <div>Colored</div>
        </div>
      </div>

      <div class="row">
        <div class="demo">
          <div>
            <Button style={{ borderRadius: 2 }}>Button<Icon /></Button>
          </div>
          <div>Resized</div>
        </div>
        <div class="demo">
          <div>
            <Button colored style={{ hue: 100, borderRadius: 2 }} ><Icon /></Button>
          </div>
          <div>Resized</div>
        </div>
        <div class="demo">
          <div>
            <Button colored style={{ hue: 100, borderRadius: 2 }} ><div style="position: absolute;"><Icon /></div></Button>
          </div>
          <div>Ghost contnet</div>
        </div>
      </div>
    </div>
  </article>
  <article>
    <h2>Dropdown</h2>
    <div class="table">
      <div class="row">
        <div class="demo">
          <div></div>
          <div></div>
        </div>
      </div>
    </div>
  </article>
  <article>
    <h2>Progress</h2>
    <div class="table">
      <div class="row">
        <div class="demo">
          <div></div>
          <div></div>
        </div>
      </div>
    </div>
  </article>
</main>

<style global>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-weight: 400;

    color-scheme: light dark;
    color: white;
    background: black;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }

  @media (prefers-color-scheme: light) {
    :root {
      color: black;
      background-color: white;
    }
  }

  body {
    margin: 1rem;
  }

  main {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .table {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .row {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
  }

  .demo {
    display: flex;
    gap: 0.4rem;
    flex-direction: column;
  }

  .demo > div {
    display: flex;
    justify-content: center;
    align-items: center;
  }

  .demo > div:first-child {
    flex: 1;
  }
</style>
