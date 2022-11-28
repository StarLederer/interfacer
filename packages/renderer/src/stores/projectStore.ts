import { createRoot, createSignal } from "solid-js";

const createMainStore = () => {
  const [name, setName] = createSignal("");
  return { name, setName };
}

export default createRoot(createMainStore);
