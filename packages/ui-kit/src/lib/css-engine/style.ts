export type Style = {
  borderRadius?: number;
  hue?: number;
  transitionDuration?: number;
};

export const style = (style: Style) => {
  return `
    ${style.borderRadius !== undefined ? `--border-radius: ${style.borderRadius}rem;` : ''}
    ${style.hue !== undefined ? `--hue: ${style.hue};` : ''}
    ${style.transitionDuration !== undefined ? `--transition-duration: ${style.transitionDuration}ms;` : ''}
  `;
};
