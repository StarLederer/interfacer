import type { IThemeColor } from "./IThemeColors"

type ITheme = {
  wrapp: {
    colors: Record<string, IThemeColor>;
    sizes: Record<string, string>;
  };
};

export type { ITheme };
