type Percentage = "12.5%"
  | "25%"
  | "50%"
  | "75%"
  | "100%";

const stringToHue = (str) => {
  var hash = 0;
  for (var i = 0; i < str.length; i++) {
    hash = str.charCodeAt(i) + ((hash << 5) - hash);
  }
  return hash;
}

const stringToColour = (str, s: Percentage, l: Percentage) => {
  return `hsl(${stringToHue(str)}, ${s}, ${l})`;
}

const routeIn = (node, { delay = 100, duration = 100 }) => {
  const opacity = +getComputedStyle(node).opacity;

  return {
    delay,
    duration,
    css: (t) => `
    opacity: ${t * opacity};
    transform: scale(${1 - (0.025 * (1 - t))});
    `,
  };
}

const routeOut = (node, { delay = 0, duration = 100 }) => {
  const opacity = +getComputedStyle(node).opacity;

  return {
    delay,
    duration,
    css: (t) => `
    opacity: ${t * opacity};
    transform: scale(${1 + (0.025 * (1 - t))});
    `,
  };
}

export {
  stringToHue,
  stringToColour,
  routeIn,
  routeOut,
}
