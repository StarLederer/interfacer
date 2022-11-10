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

let duration = 150;
let delay = 0;

const animateIn = (node, _) => {
  const opacity = +getComputedStyle(node).opacity;

  return {
    delay,
    duration,
    css: (t) => `
      opacity: ${t * opacity};
      transform: scale(${1 - 0.025 * (1 - t)});
    `,
  };
};

const animateOut = (node, _) => {
  const opacity = +getComputedStyle(node).opacity;

  return {
    delay,
    duration,
    css: (t) => `
      opacity: ${t * opacity};
      transform: scale(${1 + 0.025 * (1 - t)});
    `,
  };
};

export {
  stringToHue,
  stringToColour,
  animateIn,
  animateOut,
};
