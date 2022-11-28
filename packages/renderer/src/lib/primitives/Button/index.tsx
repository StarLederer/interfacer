import { ParentComponent } from "solid-js";
import styles from './style.module.css';

type IMainProps = {
  type?: "submit" | "reset" | "button";
  style?: "ghost" | "half" | "secondary" | "solid";
  // half?: boolean;
  // secondary?: boolean;
  // solid?: boolean;
  disabled?: boolean;
  hue?: number;
  class?: string;
  onClick?: () => void;
};

const Main: ParentComponent<IMainProps> = (props) => {
  return (
    <button
      class={[
        props.class ?? "round-m0 pd-m0",
        styles.button,
        (() => {
          if (props.style === "solid") return styles.isSolid;
          if (props.style === "secondary") return styles.isSecondary;
          if (props.style === "half") return styles.isHalf;
          return styles.isGhost;
        })(),
      ].join(" ")}
      type={props.type ?? "button"}
      disabled={props.disabled}
      style={`${props.hue != null ? `--hue: ${props.hue};` : ""}`}
      onClick={props.onClick}
    >
      <div class="flex items-center justify-center flex-1 gap-m0">
        {props.children}
      </div>
    </button>
  );
};

export default Main;
