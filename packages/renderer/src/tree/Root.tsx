import type { Component } from "solid-js";
import AddProject from "./branches/AddProject";
import Project from "./branches/Project";
import Projects from "./branches/Projects";
import Setup from "./branches/Setup";
import Settings from "./branches/Settings";

const Root: Component = () => {
  return (
    <>
      <AddProject path="/add-project" />
      <Project path="/project-view" />
      <Projects path="/projects" />
      <Setup path="/setup" />
      <Settings path="/settings" />
    </>
  );
};

export default Root;
