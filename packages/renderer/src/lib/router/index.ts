import { writable } from "svelte/store";

const route = writable<string>("/");

const navigate = (to: string) => {
  route.set(to);
}

export { default as Route } from "./Route.svelte";
export { route, navigate };
