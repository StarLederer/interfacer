import { invoke } from "@tauri-apps/api/tauri";
import { Component, createSignal } from "solid-js";
import router, { Route } from "~/lib/router";
import Button from "~/lib/primitives/Button";
import Progress from "~/lib/primitives/Progress";
import Headerbar from "~/lib/Headerbar";
import Error from "~/lib/Error";
import { stringToHue } from "~/lib/visuals";
import projectStore from "~/stores/projectStore";
import mainStore from "~/stores/mainStore";
import Action from "./components/Action";

const Main: Component<{ path: string }> = (props) => {
  const [err, setErr] = createSignal<any>(undefined);
  const [actions, setActions] = createSignal<{
    name: string;
    active: boolean;
  }[]>([]);
  const [saving, setSaving] = createSignal(false);
  const [localChanges, setLocalChanges] = createSignal(false);
  const [remoteChanges, setRemoteChanges] = createSignal(undefined);

  const hue = () => mainStore.hue() + 180 + 10 * (stringToHue(projectStore.name()) % 3);

  const resetState = () => {
    setActions([]);
    setSaving(false);
    setLocalChanges(false);
    setRemoteChanges(undefined);
  };

  const checkRemote = async () => {
    setRemoteChanges(undefined);
    try {
      setRemoteChanges(await invoke("detect_remote_source_changes"));
    } catch (e) {
      setErr(e);
    }
  };

  const updateCode = async () => {
    try {
      await invoke("download_remote_source_history");
      setRemoteChanges(await invoke("detect_remote_source_changes"));
    } catch (e) {
      setErr(e);
    }
  };

  const saveCode = async () => {
    setSaving(true);

    try {
      await invoke("upload_local_source_history");
      setLocalChanges(await invoke("detect_local_source_changes"));
    } catch (e) {
      setErr(e);
    }

    setSaving(false);
  };

  const onOpen = async () => {
    resetState();
    try {
      await invoke("load_project", { name: projectStore.name() });
      setLocalChanges(await invoke("detect_local_source_changes"));
      setRemoteChanges(await invoke("detect_remote_source_changes"));
      setActions(await invoke("get_actions"));
    } catch (e) {
      setErr(e);
    }
  };

  return (
    <Route path={props.path} onOpen={onOpen}>
      <div class="flex flex-col flex-1" style="overflow: scroll;">
        <Headerbar
          title={projectStore.name()}
          onBack={async () => {
            // await invoke("close_website");
            router.navigate("/projects");
          }}
          titleActions={(
            <div class="flex">
              {!!err && (
                <Button
                  disabled={saving()}
                  style={localChanges() ? "solid" : "half"}
                  onClick={saveCode}
                >
                  {saving() ? (
                    <Progress radius={0.5} />
                  ) : (
                    <div class="i-mdi-content-save text-m0" />
                  )}
                </Button>
              )}
            </div>
          )}
        >
          <div slot="actions" class="flex items-center gap-s">
            {!! err && (() => {
              if (remoteChanges() === undefined) {
                return (<>
                  <span class="text-int-2"> Checking the cloud </span>
                  <Progress radius={0.5} />
                </>);
              } else if (remoteChanges()) {
                return (<>
                  <span class="text-int"> Out of date </span>
                  <Button style="solid" onClick={updateCode}>
                    Update
                    <div class="i-mdi-cloud-download-outline text-m0" />
                  </Button>
                </>);
              } else {
                return (<>
                  <span class="text-int-2"> Up to date </span>
                  <Button style="half" onClick={checkRemote}>
                    Refresh
                    <div class="i-mdi-refresh text-m0" />
                  </Button>
                </>);
              }
            })()}
          </div>
        </Headerbar>

        <div class="flex-1 pd-m0 pd-bs-0">
          {!err ? (
            <Error error={err} />
          ) : (
            <div class="min-height-full gap-s-" style="display: grid; grid-template-columns: repeat(auto-fit, minmax(24rem, 1fr));">
              {actions().map((action, i) => (
                <Action name={action.name} hue={hue()} i={i} />
              ))}
            </div>
          )}
        </div>
      </div>
    </Route>
  );
};

export default Main;
