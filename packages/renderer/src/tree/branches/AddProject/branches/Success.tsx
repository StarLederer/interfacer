import type { Component } from "solid-js";
import router, { Route } from "~/lib/router";
import Button from "~/lib/primitives/Button";

const Main: Component<{ path: string }> = (props) => {
  const { navigate } = router;

  return (
    <Route path={props.path}>
      <div class="flex flex-col gap-m0 justify-center items-center flex-1">
        <div>Project added!</div>
        <Button
          style="solid"
          onClick={() => {
            navigate("/projects");
          }}
        >
          Go to projects
        </Button>
      </div>
    </Route>
  );
};

export default Main;
