import { DynamicRule } from "@unocss/core";
import { ITheme } from "../theme/types";

const sizeRule = (prefix: string, property: string): DynamicRule<ITheme> => {
  return [
    new RegExp(`^(${prefix})-(.+)$`),
    (match, { theme }) => {
      const rule = {};
      rule[property] = theme.wrapp.sizes[match[2]]
      return rule;
    }
  ];
};

export default sizeRule;
