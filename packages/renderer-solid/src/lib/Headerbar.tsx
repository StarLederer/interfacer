import { JSXElement, ParentComponent } from "solid-js";
import Button from "~/lib/primitives/Button";

type IMainProps = {
  title?: string;
  titleActions?: JSXElement;
  onBack?: () => {};
};

const Main: ParentComponent<IMainProps> = (props) => {
  return (
    <div class="flex justify-between items-center pd-m0 gap-m0">
      <div class="flex items-center flex1 gam-m0">
        {(props.onBack || props.titleActions) && (
          <div class="flex items-center ga-s-">
            {props.onBack !== undefined && (
              <Button style="half" onClick={props.onBack}>
                <div class="i-mdi-chevron-left text-m--" />
              </Button>
            )}

            {props.titleActions}
          </div>
        )}

        {props.title && (
          <h2>{props.title}</h2>
        )}
      </div>

      {props.children}
    </div>
  );
};

export default Main;
