import type { Component } from "solid-js";
import { Route } from "~/lib/router";
import Progress from "~/lib/primitives/Progress";

const Main: Component<{ path: string }> = (props) => {
  return (
    <Route path={props.path}>
      <div class="flex flex-col gap-m0 justify-center items-center flex-1">
        <div>Adding project</div>
        <div class="text-on2">Please wait</div>
        <Progress />
      </div>
    </Route>
  );
};

export default Main;
