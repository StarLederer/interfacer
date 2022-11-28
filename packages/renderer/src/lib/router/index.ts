import { createSignal, createRoot } from "solid-js";

type Path = string | undefined;

type IRouteStore = {
  previous: string;
  current: string;
  firstDifferent: Path[]; // fist path segments that is different between previous and current
};

const resolve = (path: string) => {
  let resolvedSegments: string[] = [];

  path.split("/").forEach((segment) => {
    if (segment === "..") {
      resolvedSegments.pop();
    } else {
      resolvedSegments.push(segment);
    }
  })

  return `${resolvedSegments.join("/")}`;
}

const findFirstDifferent = (a: string, b: string) => {
  const c = a.split("/"); // a but split
  const d = b.split("/"); // b but split

  // Yes, we do go outside one of the array bounds
  // It is very convenient that javascript gives us
  // undefined if we do so.
  // /a/b/undefined (undefined is first different)
  // /a/b/c/d (c is first differenc)
  const maxLength = Math.max(c.length, d.length);
  for (let i = 0; i < maxLength; ++i) {
    if (c[i] != d[i]) {
      return [c[i], d[i]];
    }
  }

  // If paths are the same return undefined as
  // first different for both paths
  return [undefined, undefined];
}

const createRouter = () => {
  const [route, setRoute] = createSignal<IRouteStore>({
    previous: "/",
    current: "/",
    firstDifferent: [undefined, undefined],
  });

  const navigate = (to: string) => {
    const previous = route().current;
    const current = resolve(to);

    setRoute({
      previous,
      current,
      firstDifferent: findFirstDifferent(previous, current)
    });
  };

  return { route, navigate };
}

export default createRoot(createRouter);
export { default as Route } from "./Route";
