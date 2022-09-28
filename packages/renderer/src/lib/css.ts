import nanoCss from "nano-css";
import { addon as addonRule } from "nano-css/addon/rule";

const nano = nanoCss.create();
addonRule(nano);

const { put, rule } = nano;

export {
  put as class,
  rule as inline
}
