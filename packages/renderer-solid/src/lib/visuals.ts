const stringToHue = (str: string) => {
  var hash = 0;
  for (var i = 0; i < str.length; i++) {
    hash = str.charCodeAt(i) + ((hash << 5) - hash);
  }
  return hash;
}

const stringToColour = (str: string, s: number, l: number) => {
  return `hsl(${stringToHue(str)}, ${s}%, ${l}%)`;
}

export {
  stringToHue,
  stringToColour,
};
