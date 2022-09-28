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

export {
  stringToHue,
  stringToColour
}
