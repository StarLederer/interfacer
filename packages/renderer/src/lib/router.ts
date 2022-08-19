import { writable } from "svelte/store";

type Routes = "/"
  | "/websites"
  | "/add-website"

const route = writable<Routes>("/");

export default route;
export type { Routes }
