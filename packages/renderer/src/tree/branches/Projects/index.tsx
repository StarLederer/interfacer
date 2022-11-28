import { invoke } from "@tauri-apps/api/tauri";
import { Component, createResource, createSignal } from "solid-js";
import router, { Route } from "~/lib/router";
import Button from "~/lib/primitives/Button";
import Progress from "~/lib/primitives/Progress";
import Headerbar from "~/lib/Headerbar";
import Error from "~/lib/Error";
import projectStore from "~/stores/projectStore";

// import { projectStore } from "~/stores/";
import Project from "./components/Project";

const getProjects = async () => (await invoke("get_websites") as any[]);

const Main: Component<{ path: string }> = (props) => {
  const { navigate } = router;

  const [err, setErr] = createSignal<any>();
  const [projects, { refetch }] = createResource(getProjects);

  return (
    <Route path={props.path} onOpen={() => { refetch() }}>
      <Headerbar title="Your projects">
        <div class="flex gap-s-">
          {!err() && (
            <>
              <Button
                style="half"
                onClick={async () => {
                  navigate("/add-project/edit");
                }}
              >
                Add project
                <div class="i-mdi-git text-m0" />
              </Button>
              <Button
                style="half"
                onClick={async () => {
                  navigate("/settings");
                }}
              >
                <div class="i-mdi-cog text-m0" />
              </Button>
            </>
          )}
        </div>
      </Headerbar>

      <div style="overflow: auto">
        {!projects.loading ? (
          <div class="pd-m0 pd-bs-0">
            {!projects.error ? (
              // TODO: Use grid class when unocss-preset supports that
              <div class="gap-s" style="display: grid; grid-template-columns: repeat(auto-fill, minmax(24rem, 1fr));">
                {projects()?.map((project) => (
                  <Project
                    label={project}
                    onClick={() => {
                      projectStore.setName(project)
                      navigate("/project-view");
                    }}
                  />
                ))}
              </div>
            ) : (
              <Error error={projects.error} />
            )}
          </div>
        ) : (
          <Progress />
        )}
      </div>
    </Route>
  );
};

export default Main;
