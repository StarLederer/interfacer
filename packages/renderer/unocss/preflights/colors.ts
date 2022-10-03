import { PreflightContext } from "@unocss/core";
import { ITheme, IThemeColor } from "../theme/types";

/**
 * Turns IThemeColor into CSS string
 *
 * @param name custom CSS property name partial (--col-NAME)
 * @param dark
 * @param light
 * @returns a tuple of (s)aturation and (l)ightness custom CSS properties for dark (default) and light modes
 */
const getColorCSS = (
  name: string,
  color: IThemeColor
): [string, string] => {
  const { dark, light } = color;

  const cssDark = `
    --col-${name}-s: ${dark.s}%;
    --col-${name}-l: ${dark.l}%;
  `;

  // We could generate this the same as cssDark
  // but we can optimize if we don't add light variatns
  // when they are the same as dark.
  // This at least removes the need for --col-NAME-s
  // light variant for almost all the colors
  let cssLight = '';
  if (light?.s !== undefined) {
    cssLight += `--col-${name}-s: ${light.s}%;`
  }

  if (light?.l !== undefined) {
    cssLight += `--col-${name}-l: ${light.l}%;`
  }
  else {
    cssLight += `--col-${name}-l: ${100 - dark.l}%;`
  }

  return [cssDark, cssLight];
}

/**
 * An preflight getter for UnoCSS
 *
 * @returns :root CSS containing various color variables for dark (default) and light (@media) modes. The variables are intended for internal use by this UnoCSS preset
 */
const getCSS = ({ theme: { wrapp } }: PreflightContext<ITheme>): string => {
  const colors = Object.keys(wrapp.colors).map((color) => {
    return getColorCSS(color, wrapp.colors[color]);
  });

  return `
    :root {
      --hue: 0;
      --highlight: 0%;

      ${colors.map(([dark, _]) => {
        return dark;
      }).join('\n')}
    }

    @media (prefers-color-scheme: light) {
      :root {
        ${colors.map(([_, light]) => {
          return light;
        }).join('\n')}
      }
    }
  `;
}

export default getCSS;