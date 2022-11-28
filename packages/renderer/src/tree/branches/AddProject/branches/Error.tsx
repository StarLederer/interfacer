import type { Component } from "solid-js";
import router, { Route } from "~/lib/router";
import Button from "~/lib/primitives/Button";

type IMainProps = {
  path: string;
  err: any;
};

const Main: Component<IMainProps> = (props) => {
  return (
    <Route path={props.path}>
      <div class="flex flex-col gap-m0 justify-center items-center flex-1">
        <div>An error occured!</div>
        <div class="text-on1 text-center">{props.err}</div>
        <Button
          style="solid"
          onClick={() => {
            router.navigate(`${props.path}/../edit`);
          }}
        >
          Go back
        </Button>
      </div>
    </Route>
  );
};

export default Main;
