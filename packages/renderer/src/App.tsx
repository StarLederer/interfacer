import { Component, createEffect, createResource } from 'solid-js';
import { invoke } from "@tauri-apps/api/tauri";
import Root from "./tree/Root";
import router from "~/lib/router";
import mainStore from "~/stores/mainStore";

const App: Component = () => {
  createEffect(async () => {
    let hasUser = await invoke("load_user") as boolean;

    if (hasUser) {
      router.navigate("/projects");
    }
    else {
      router.navigate("/setup/user");
    }
  });

  return (
    <main class="bg-def transition fixed inset-0" style={`--hue: ${mainStore.hue()}`}>
      <Root />
    </main>
  );
};

export default App;
