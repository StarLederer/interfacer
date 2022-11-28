import type { Component } from "solid-js";
import { Route } from "~/lib/router";
import UserSetup from "./branches/UserSetup";

const Main: Component<{ path: string }> = (props) => {
  return (
    <Route path={props.path} >
      <UserSetup path={`${props.path}/user`} />
      <Route path={`${props.path}/done`} >
        <h1>All done!</h1>
      </Route>
    </Route>
  );
};

export default Main;
