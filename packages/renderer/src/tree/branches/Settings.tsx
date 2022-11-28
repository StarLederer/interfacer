import { invoke } from "@tauri-apps/api/tauri";
import { Component, createResource, createSignal } from "solid-js";
import router, { Route } from "~/lib/router";
import Button from "~/lib/primitives/Button";
import Input from "~/lib/primitives/Input";
import Progress from "~/lib/primitives/Progress";
import Headerbar from "~/lib/Headerbar";
import Error from "~/lib/Error";
import mainStore from "~/stores/mainStore";

const updateUser = async (partial: any) => (await invoke("update_user", { partial }) as any[]);

const Main: Component<{ path: string }> = (props) => {
  const { navigate } = router;

  const [username, setUsername] = createSignal('');
  const [password, setPassword] = createSignal('');

  const [save] = createResource({
    git_username: username() ? username() : undefined,
    git_password: password() ? password() : undefined,
  }, updateUser);

  const resetState = () => {
    setUsername('');
    setPassword('');
  };

  const styles = {
    legend: "mg-i-m0 mg-be-m0",
    fieldChanger: "pd-0 gap-s-- flex",
    fieldset: "flex flex-col gap-s-",
  };

  return (
    <Route path={props.path} onOpen={resetState}>
      <Headerbar
        title="User settings"
        onBack={() => {
          navigate("/projects");
        }}
      >
        <div slot="actions">
          {save.error === undefined && (
            <Button
              style={save.loading ? "half" : "solid"}
              disabled={save.loading}
              onClick={() => { save() }}
            >
              {save.loading ? (
                <Progress radius={0.5} />
              ) : (
                <>
                  Save
                  <div class="i-mdi-check text-m0" />
                </>
              )}
            </Button>
          )}
        </div>
      </Headerbar>

      <div class="pd-i-m0">
        {save.error ? (
          <Error error={save.error} />
        ) : (
          <form class="flex flex-col gap-m0" onSubmit={save}>
            <fieldset class={styles.fieldset}>
              <span class={styles.legend}>Appearance</span>
              <div class="flex gap-s-">
                {[["red", 0], ["orange", 40], ["teal", 150], ["blue", 210]].map((color) => (
                  <Button
                    style="secondary"
                    hue={color[1] as number}
                    onClick={() => {
                      mainStore.setHue(color[1] as number);
                    }}
                  >
                    {color[0]}
                  </Button>
                ))}
              </div>
            </fieldset>
            <fieldset class={styles.fieldset}>
              <span class={styles.legend}>Git credentials</span>
              <fieldset class={styles.fieldChanger}>
                {/* <Input label="New username" bind:value={username} required /> */}
                <Input label="New username" required signal={[username, setUsername]} />
                <Button
                  style="half"
                  onClick={() => {
                    setUsername('');
                  }}
                >
                  <div class="i-mdi-close text-m0" />
                </Button>
              </fieldset>
              <fieldset class={styles.fieldChanger}>
                {/* <Input label="New password" bind:value={password} required /> */}
                <Input label="New password" required signal={[password, setPassword]} />
                <Button
                  style="half"
                  onClick={() => {
                    setPassword('');
                  }}
                >
                  <div class="i-mdi-close text-m0" />
                </Button>
              </fieldset>
              <p class="text-fg-3 text-center">Previous credentials are not shown.</p>
            </fieldset>
          </form>
        )}
      </div>
    </Route>
  );
};

export default Main;
