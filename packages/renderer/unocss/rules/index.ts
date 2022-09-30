import type { Rule } from "@unocss/core";
import { colorRules } from "./colors";

const rules: Rule<{}>[] = [
  // Colors
  ['bg-none', { background: 'none' }],
  ...colorRules('border-color', 'border-color'),
  ...colorRules('bg', 'background'),
  ...colorRules('text', 'color'),

  // Shape
  ['round-m', { 'border-radius': '1rem' }],
  ['round-l', { 'border-radius': '2rem' }],

  ['pad-xxs', { 'padding': '0.1rem' }],
  ['pad-xs', { 'padding': '0.2rem' }],
  ['pad-s', { 'padding': '0.4rem' }],
  ['pad-s+', { 'padding': '0.6rem' }],
  ['pad-m', { 'padding': '1rem' }],
  ['pad-l', { 'padding': '2rem' }],

  ['pad-inline-m', { 'padding-inline': '1rem' }],
  ['pad-block-l', { 'padding-block': '2rem' }],

  ['gap-xxs', { 'gap': '0.1rem' }],
  ['gap-xs', { 'gap': '0.2rem' }],
  ['gap-s', { 'gap': '0.4rem' }],
  ['gap-s+', { 'gap': '0.6rem' }],
  ['gap-m', { 'gap': '1rem' }],
  ['gap-l', { 'gap': '2rem' }],
];

export default rules;
