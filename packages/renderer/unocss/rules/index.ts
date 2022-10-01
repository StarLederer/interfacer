import type { Rule } from "@unocss/core";

import { ITheme } from "../theme/types";
import { colorRules } from "./colors";
import sizeRule from "./sizes";

const rules: Rule<ITheme>[] = [
  // Colors
  ['bg-none', { background: 'none' }],
  ...colorRules('border-color', 'border-color'),
  ...colorRules('bg', 'background'),
  ...colorRules('text', 'color'),

  // Shapesand sizes
  sizeRule('round', 'border-radius'),
  sizeRule('pad', 'padding'),
  sizeRule('pad-i', 'padding-inline'),
  sizeRule('pad-b', 'padding-block'),
  sizeRule('gap', 'gap'),
];

export default rules;
