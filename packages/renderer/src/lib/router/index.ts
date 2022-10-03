import { writable } from "svelte/store";

type IRouteStore = {
  previous: string;
  current: string;
  firstDifferent: string[]; // fist path segments that is different between previous and current
};

const resolve = (path: string) => {
  let resolvedSegments = [];

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

const route = writable<IRouteStore>({
  previous: "/",
  current: "/",
  firstDifferent: [undefined, undefined],
});

const navigate = (to: string) => {
  const resolvedTo = resolve(to);

  route.update((oldState) => {
    const previous = oldState.current;
    const newState = oldState;
    Object.assign(newState, {
      previous,
      current: resolvedTo,
      firstDifferent: findFirstDifferent(previous, resolvedTo)
    });
    return newState;
  });
}

export { default as Route } from "./Route.svelte";
export { route, navigate };
