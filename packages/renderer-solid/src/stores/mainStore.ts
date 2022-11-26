import { createRoot, createSignal } from "solid-js";

const createMainStore = () => {
  const [hue, setHue] = createSignal(40);
  return { hue, setHue };
}

export default createRoot(createMainStore);
