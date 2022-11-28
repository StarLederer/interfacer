import type { Component } from "solid-js";
import Button from "~/lib/primitives/Button";
import { stringToHue } from "~/lib/visuals";
import mainStore from "~/stores/mainStore";

type IMainProps = {
  label: string;
  onClick: () => void;
};

const Main: Component<IMainProps> = (props) => {
  return (
    <div
      style={`--hue: ${mainStore.hue() + 180 + 10 * (stringToHue(props.label) % 3)};`}
      class="panel bg-srf text-on2 flex flex-col gap-m0"
    >
      <div class="flex flex-col gap-s pd-bs-m0">
        <div class="header">
          <div class="title text-on0 font-semibold">{props.label}</div>
        </div>
        <p>A short description maybe?</p>
      </div>

      <div class="flex gap-s- justify-between">
        <div class="flex">
          <Button style="solid" onClick={props.onClick}>
            Open
            <div class="i-mdi-arrow-right text-m0" />
          </Button>
        </div>
        <Button style="half">
          <div class="i-mdi-dots-vertical text-m0" />
        </Button>
      </div>
    </div>
  );
};

export default Main;
