import { writable } from "svelte/store";

type IRoute = "/"
  | "/add-website"
  | "/server"
  | "/settings"
  | "/setup"
  | "/setup/done"
  | "/setup/user"
  | "/websites"

const route = writable<IRoute>("/");

const navigate = (to: IRoute) => {
  route.set(to);
}

export { default as Root } from "./Root.svelte";
export { default as Route } from "./components/Route.svelte";
export { route, navigate };
export type { IRoute }

