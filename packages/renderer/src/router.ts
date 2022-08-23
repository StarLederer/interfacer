import { writable } from "svelte/store";

type Routes = "/"
  | "/websites"
  | "/add-website"
  | "/server"

const route = writable<Routes>("/");

export default route;
export type { Routes }
