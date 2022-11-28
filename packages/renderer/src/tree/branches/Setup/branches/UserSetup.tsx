import { Component, createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";

import router, { Route } from "~/lib/router";
import Error from "~/lib/Error";
import Input from "~/lib/primitives/Input";
import Button from "~/lib/primitives/Button";

const Main: Component<{ path: string }> = (props) => {
  const { navigate } = router;

  let [loading, setLoading] = createSignal(false);
  let [error, setError] = createSignal(undefined);
  let [username, setUsername] = createSignal('');
  let [password, setPassword] = createSignal('');

  const resetState = () => {
    setLoading(false);
    setError(undefined);
    setUsername('');
    setPassword('');
  };

  const save = async () => {
    setLoading(true);
    try {
      let partial = {
        git_username: username,
        git_password: password,
      };
      await invoke("update_user", { partial });
      navigate("/websites");
    } catch (err: any) {
      setError(err);
      console.log(error);
    }
  };

  const skip = () => {
    navigate("/projects");
  };

  return (
    <Route path={props.path} onOpen={resetState}>
      <div class="flex flex-col gap-m0" style="max-width: 48rem; margin: auto;">
        <div class="flex flex-col items-center text-center gap-m--">
          <div class="pd-m-- round-full bg-srf">
            <div class="i-mdi-git text-m--" />
          </div>

          <div class="flex flex-col gap-s-">
            <span class="text-fg-2">
              Welcome to Interfacer!
            </span>
            <span class="text-fg-1">
              Let's start by setting up your Git credentials
            </span>
          </div>
        </div>

        {error() ? (
          <Error error={error} />
        ) : (
          <>
            <div class="flex flex-col gap-s--">
              {/* <Input label="Username" bind:value={username} />
                <Input label="Token" bind:value={password} /> */}
              <Input label="Username" signal={[username, setUsername]} />
              <Input label="Token" signal={[password, setPassword]} />
            </div>
            <div class="flex justify-between">
              <Button onClick={skip}>Skip</Button>
              <Button style="solid" onClick={save}>Save</Button>
            </div>
          </>
        )}
      </div>
    </Route>
  );
};

export default Main;
