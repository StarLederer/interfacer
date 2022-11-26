import { ParentComponent } from "solid-js";

type IMainProps = {
  error: any;
};

const Main: ParentComponent<IMainProps> = (props) => {
  return (
    <div class="hue-0 bg-srf round-m0 pd-m0">
      <p>An error has occured!</p>
      <p class="text-on1">{props.error}</p>
    </div>
  );
};

export default Main;
