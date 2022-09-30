import type { ITheme } from "./types/ITheme";

const theme: ITheme = {
  wrapp: {
    colors: {
      'abs': { dark: { s: 0, l: 0 } },
      'on-abs': { dark: { s: 10, l: 100 } },

      'def': { dark: { s: 10, l: 5 } },
      'on-def': { dark: { s: 10, l: 90 } },
      'on-def-2': { dark: { s: 20, l: 60 } },
      'on-def-3': { dark: { s: 20, l: 40 } },

      'srf': { dark: { s: 20, l: 10 } },
      'on-srf': { dark: { s: 100, l: 80 } },
      'on-srf-2': { dark: { s: 40, l: 60 } },

      'srf2': { dark: { s: 20, l: 20 } },
      'on-srf2': { dark: { s: 20, l: 80 } },

      'int': { dark: { s: 100, l: 60 }, light: { l: 60 } },
      'on-int': { dark: { s: 100, l: 10 }, light: { l: 10 } },

      'int2': { dark: { s: 60, l: 20 }, light: { s: 90 } },
      'on-int2': { dark: { s: 80, l: 90 } },

      'int3': { dark: { s: 10, l: 10 } },
      'on-int3': { dark: { s: 10, l: 80 } },
    },
  }
};

export default theme;
