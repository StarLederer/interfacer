import { invoke } from "@tauri-apps/api/tauri";
import { Component, createSignal } from "solid-js";
import router, { Route } from "~/lib/router";
import Edit from "./branches/Edit";
import Loading from "./branches/Loading";
import Success from "./branches/Success";
import Error from "./branches/Error";
import { IAction, IHooks } from "./hooks";

type ISaveCallback = (
  url: string,
  config: {
    hooks: IHooks,
    actions: IAction[],
  }
) => void;

const Main: Component<{ path: string }> = (props) => {
  const [err, setErr] = createSignal<any>();

  const save: ISaveCallback = async (url, config) => {
    router.navigate(`${props.path}/loading`);

    try {
      await invoke("add_project", {
        name,
        gitUrl: url,
        config: JSON.stringify(config),
      });
      router.navigate(`${props.path}/success`);
    } catch (e) {
      setErr(e);
      router.navigate(`${props.path}/error`);
    }
  };

  return (
    <Route path={props.path}>
      <Edit path={`${props.path}/edit`} onSave={save} />
      <Loading path={`${props.path}/loading`} />
      <Success path={`${props.path}/success`} />
      <Error path={`${props.path}/error`} err={err()}/>
    </Route>
  );
};

export default Main;
export type { ISaveCallback };
