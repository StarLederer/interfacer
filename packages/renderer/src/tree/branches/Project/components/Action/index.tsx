import { invoke } from "@tauri-apps/api/tauri";
import { Component, createSignal } from "solid-js";
import router, { Route } from "~/lib/router";
import Button from "~/lib/primitives/Button";
import Progress from "~/lib/primitives/Progress";
import styles from "./style.module.css";

type IMainProps = {
  name: string;
  i: number;
  hue: number;
};

const Main: Component<IMainProps> = (props) => {
  const [err, setErr] = createSignal<any>();
  const [nam, setNam] = createSignal(props.name);
  const [active, setActive] = createSignal(false);
  const [loading, setLoading] = createSignal(false);

  const interact = async () => {
    if (loading()) return;

    try {
      setErr(undefined);
      setLoading(true);
      setActive(true);

      const before = Date.now();
      const res = (await invoke("interact", { actionI: props.i })) as {
        name: string;
        active: boolean;
      };

      setTimeout(() => {
        setNam(res.name);
        setActive(res.active);
        setLoading(false);
      }, 1000 - (Date.now() - before));
    } catch (e) {
      setErr(e);
      setLoading(false);
    }
  };

  return (
    <Button
      style={active() ? "solid" : "secondary"}
      disabled={loading()}
      class={`round-m- position-relative ${styles.action}`}
      onClick={interact}
      hue={props.hue}
    >
      <div class="pd-m- text-fg-1">
        {(() => {
          if (loading()) {
            return (<>
              <div
                // in:animateIn
                // out:animateOut
                class="route flex items-center justify-center"
              >
                <Progress />
              </div>
            </>)
          } else if (err()) {
            return (<>
              <div
                // in:animateIn
                // out:animateOut
                class="route"
              >
                <span class={styles.name}>{nam()}</span>
                <div class="error">
                  <div>Error:</div>
                  {err()}
                </div>
                Try again?
              </div>
            </>)
          } else {
            return (<>
              <div
                // in:animateIn
                // out:animateOut
                class={`${styles.rotue} ${styles.nameLine} ${active() && styles.hasCog}`}
              >
                <div class={`i-mdi-cog ${styles.icon}`}></div>
                <span class={styles.name}>{nam()}</span>
              </div>
            </>)
          }
        })()}
      </div>
    </Button>
  );
};

export default Main;
