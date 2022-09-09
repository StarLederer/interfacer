<script lang="ts">
  import Icon from "svelte-material-icons/Check.svelte";

  import { style } from "$lib/css-engine/style";
  import Button from "$lib/primitives/Button.svelte";
  import Dropdown from "$lib/primitives/Dropdown.svelte";
  import Input from "$lib/primitives/Input.svelte";
  import Panel from "$lib/primitives/Panel.svelte";
  import Progress from "$lib/primitives/Progress.svelte";

  import Flex from "$lib/helpers/Flex.svelte";
</script>

<main
  style={style({
    hue: 0,
    borderRadius: 1,
  })}
>
  <header>
    <h1>UI Library</h1>
  </header>
  <article>
    <h2>Buttons</h2>
    <div class="table">
      <div class="row">
        <div class="demo">
          <div>
            <Button solid />
          </div>
          <div>Empty</div>
        </div>
        <div class="demo">
          <div>
            <Button solid><Icon /></Button>
          </div>
          <div>Icon</div>
        </div>
        <div class="demo">
          <div>
            <Button solid>Button</Button>
          </div>
          <div>Text</div>
        </div>
        <div class="demo">
          <div>
            <Button solid>Button <Icon /></Button>
          </div>
          <div>Text & icon</div>
        </div>
      </div>

      <div class="row">
        <div class="demo">
          <div>
            <Button>Button</Button>
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
            <Button colored style={{ hue: 100 }}>Button</Button>
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
            <Button colored solid style={{ hue: 100 }}>Button</Button>
          </div>
          <div>Colored</div>
        </div>
      </div>

      <div class="row">
        <div class="demo">
          <div>
            <Button solid style={{ borderRadius: 2 }}>Button<Icon /></Button>
          </div>
          <div>Resized</div>
        </div>
        <div class="demo">
          <div>
            <Button colored solid style={{ hue: 100, borderRadius: 2 }}
              ><Icon /></Button
            >
          </div>
          <div>Resized</div>
        </div>
        <div class="demo">
          <div>
            <Button colored solid style={{ hue: 100, borderRadius: 2 }}>
              <div style="position: absolute;"><Icon /></div>
            </Button>
          </div>
          <div>Ghost contnet</div>
        </div>
      </div>

      <div class="row">
        <div class="demo">
          <div>
            <Button disabled>Inactive</Button>
          </div>
          <div>Disabled</div>
        </div>
        <div class="demo">
          <div>
            <Button half disabled>Inactive</Button>
          </div>
          <div>Disabled</div>
        </div>
        <div class="demo">
          <div>
            <Button solid disabled>Inactive</Button>
          </div>
          <div>Disabled</div>
        </div>
      </div>
    </div>
  </article>

  <article class="form-inputs">
    <h2>Form inputs</h2>
    <div class="table">
      <div class="demo">
        <div>
          <Input value="" label="Input" />
        </div>
        <div>Text input</div>
      </div>
      <div class="demo">
        <div>
          <Input value="Prefilled" label="Input" />
        </div>
        <div>Prefilled input</div>
      </div>
      <div class="demo">
        <div>
          <Dropdown
            label="Input"
            selected="First"
            options={["First", "Second", "Third"]}
          />
        </div>
        <div>Dropdown</div>
      </div>
    </div>
  </article>
  <article>
    <h2>Panels</h2>
    <div class="table">
      <div class="row">
        <div class="demo">
          <div>
            <Panel style={{ hue: 140 }}>This one has simple text inside.</Panel>
          </div>
          <div>Panel</div>
        </div>
        <div class="demo">
          <div>
            <Panel let:titleStyle style={{ hue: 180 }}>
              <Flex direction="column">
                <span style={titleStyle}>This one has a title</span>
                <span>On top of some text.</span>
                <span>Within some alignment.</span>
              </Flex>
            </Panel>
          </div>
          <div>Panel</div>
        </div>
        <div class="demo">
          <div>
            <Panel style={{ hue: 220 }} let:titleStyle>
              <Flex direction="column" gap={1}>
                <Flex justify="space-between" align="center">
                  <span style={titleStyle}>This one</span>
                  <Button half colored><Icon /></Button>
                </Flex>
                <Flex direction="column">
                  <span>Has signifincatly more content.</span>
                  <span>Even buttons!</span>
                </Flex>
                <Button solid colored>Button</Button>
              </Flex>
            </Panel>
          </div>
          <div>Panel</div>
        </div>
      </div>
    </div>
  </article>
  <article>
    <h2>Progress</h2>
    <div class="table">
      <div class="row">
        <div class="demo">
          <div><Progress /></div>
          <div>Spinner</div>
        </div>
        <div class="demo">
          <div><Progress style={{ borderRadius: 2 }} /></div>
          <div>Resized</div>
        </div>
        <div class="demo">
          <div><Progress style={{ borderRadius: 3 }} /></div>
          <div>Resized</div>
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
