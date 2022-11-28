import { Component, createSignal, Signal } from "solid-js";
import router, { Route } from "~/lib/router";
import Button from "~/lib/primitives/Button";
import Input from "~/lib/primitives/Input";
import Toggle from "~/lib/primitives/Toggle";
import Headerbar from "~/lib/Headerbar";
import type { IHookKey, IHooks } from "../hooks";
import { availableHookKeys } from "../hooks";
import { ISaveCallback } from "..";

type IMainProps = {
  path: string;
  onSave: ISaveCallback;
};

type IHookSignals = Partial<Record<IHookKey, Signal<string>[]>>;

const Main: Component<IMainProps> = (props) => {
  let [name, setName] = createSignal("");
  let [url, setUrl] = createSignal("");
  let [hooks, setHooks] = createSignal<IHookSignals>({});
  let [actions, setActions] = createSignal<{
    idle_name: Signal<string>;
    active_name: Signal<string>;
    command: Signal<string>;
    user_terminated: Signal<boolean>;
  }[]>([]);

  const save = () => {
    props.onSave(url(), {
      hooks: (() => {
        let out: IHooks = {};
        hookKeys(usedHooks()).forEach((key) => {
          let commandSignals = usedHooks()[key];
          if (commandSignals) { out[key] = commandSignals.map(([command]) => command()); }
        })
        return out;
      })(),
      actions: actions().map((action) => ({
        idle_name: action.idle_name[0](),
        active_name: action.active_name[0](),
        command: action.command[0](),
        user_terminated: action.user_terminated[0](),
      }))
    });
  };

  // Computed hooks

  const filteredHooks = () => {
    const usedHooks: IHookSignals = {};
    const unusedHooks: IHookSignals = {};

    availableHookKeys.forEach((key) => {
      if (hooks()[key] !== undefined) {
        usedHooks[key] = hooks()[key];
      } else {
        unusedHooks[key] = hooks()[key];
      }
    });

    return {
      usedHooks,
      unusedHooks,
    }
  };

  const usedHooks = (): IHookSignals => filteredHooks().usedHooks;
  const unusedHooks = (): IHookSignals => filteredHooks().unusedHooks;

  const hookKeys = (hooks: IHookSignals): IHookKey[] => Object.keys(hooks) as IHookKey[];

  // Render

  const styles = {
    fieldset: "flex flex-col gap-s",
    legend: "mg-i-m0 mg-be-m0 font-bold",
    label: "text-fg-2 text-s++",
  };

  return (
    <Route path={props.path}>
      <Headerbar
        title="Add project"
        onBack={() => {
          router.navigate("/projects");
        }}
      >
        <Button style="solid" onClick={save}>
          Save
          <div class="i-mdi-check text-m0" />
        </Button>
      </Headerbar>

      <section style="overflow: auto">
        <div class="flex flex-col gap-m0 pd-m0 pd-bs-0">
          <aside
            style="--hue: 6;"
            class="flex flex-row items-center gap-m0 bg-srf text-on1 pd-s- round-m--"
          >
            <div style="border-radius: 50%;" class="bg-srf2 text-on1 pd-s+">
              <div class="i-mdi-exclamation text-m0" />
            </div>
            <span>
              If you are a content writer, you probably want to call your developer
              for this.
            </span>
          </aside>

          <form class="flex flex-col gap-m0" onSubmit={save}>
            <div class={styles.fieldset}>
              <Input label="Local name" required signal={[name, setName]} />
              <Input label="Git URL" required signal={[url, setUrl]} />
            </div>

            {(hookKeys(usedHooks()).length > 0) && (
              <div class={styles.fieldset}>
                <span class={styles.legend}>Hooks</span>
                {hookKeys(usedHooks()).map((key) => (
                  <fieldset class="panel bg-srf flex flex-col gap-s">
                    <div class="flex justify-between items-center">
                      <span class="mg-i-m0">{key.replaceAll("_", " ")}</span>
                      <Button
                        hue={0}
                        onClick={() => {
                          setHooks((prev) => {
                            prev[key] = undefined;
                            return { ...prev };
                          });
                        }}
                      >
                        <div class="i-mdi-close text-m0" />
                      </Button>
                    </div>

                    {hooks()[key]?.map((hook: Signal<string>, i: number) => (
                      <div class="flex gap-s-- flex-row">
                        <Input signal={hook} label="Command" />
                        <Button
                          hue={0}
                          onClick={() => {
                            setHooks((prev) => {
                              prev[key]?.splice(i, 1);
                              return { ...prev };
                            });
                          }}
                        >
                          <div class="i-mdi-close text-m0" />
                        </Button>
                      </div>
                    ))}

                    <Button
                      style="secondary"
                      onClick={() => {
                        setHooks((prev) => {
                          const out = [];
                          const prevHook = prev[key];
                          if (prevHook) {
                            prev[key] = [...prevHook, createSignal("")];
                          } else {
                            prev[key] = [createSignal("")];
                          }
                          return { ...prev };
                        });
                      }}
                    >
                      Add command
                      <div class="i-mdi-plus text-m0" />
                    </Button>
                  </fieldset>
                ))}
              </div>
            )}

            {(hookKeys(unusedHooks()).length > 0) && (
              <div class={styles.fieldset}>
                <span class={styles.legend}>Add hooks</span>
                <div class="flex flex-wrap gap-s-">
                  {hookKeys(unusedHooks()).map((key) => (
                    <Button
                      style="half"
                      onClick={() => {
                        setHooks((prev) => {
                          prev[key] = [createSignal("")];
                          return { ...prev };
                        });
                      }}
                    >
                      {key.replaceAll("_", " ")}
                      <div class="i-mdi-plus text-m0" />
                    </Button>
                  ))}
                </div>
              </div>
            )}

            <div class={styles.fieldset}>
              <span class={styles.legend}>Actions</span>
              {actions().map((action, i) => (
                <fieldset class="panel bg-srf flex flex-col gap-s--">
                  <Input label="Idle name" signal={action.idle_name} />
                  <Input label="Active name" signal={action.active_name} />
                  <Input label="Command" signal={action.command} />
                  <div class="flex items-start flex-col pd-i-m0 gap-s">
                    <div class={styles.label}>User terminated</div>
                    <Toggle label="User terminated" signal={action.user_terminated as Signal<boolean | undefined>} />
                  </div>
                  <Button
                    hue={0}
                    style="secondary"
                    onClick={() => {
                      setActions((prev) => {
                        prev.splice(i, 1);
                        return [...prev];
                      })
                    }}
                  >
                    Remove this action
                    <div class="i-mdi-close text-m0" />
                  </Button>
                </fieldset>
              ))}
              <Button
                style="secondary"
                hue={166}
                onClick={() => {
                  setActions((prev) => [
                    ...prev,
                    {
                      idle_name: createSignal(""),
                      active_name: createSignal(""),
                      command: createSignal(""),
                      user_terminated: createSignal(false),
                    },
                  ]);
                }}
              >
                Add action
                <div class="i-mdi-plus text-m0" />
              </Button>
            </div>
          </form>
        </div>
      </section>
    </Route>
  );
};

export default Main;
