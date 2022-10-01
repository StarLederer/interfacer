import { writable } from "svelte/store";

const mainStore = writable(40);

export { mainStore };
