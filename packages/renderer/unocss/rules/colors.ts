// Generates rules that
const colorRules = (prefix, property) => {
  const formulas: [(color: string) => string, string[]][] = [
    [
      (color) => `hsl(var(--hue), var(--col-${color}-s), var(--col-${color}-l));`,
      [
        'abs',
        'on-abs',
        'def',
        'on-def',
        'on-def-2',
        'on-def-3',
        'srf',
        'on-srf',
        'on-srf-2',
        'srf2',
        'on-srf2',
      ]
    ],
    [
      (color) => `hsl(var(--hue), var(--col-${color}-s), calc(var(--col-${color}-l) + var(--highlight)));`,
      [
        'int',
        'on-int',
        'int2',
        'on-int2',
        'int3',
        'on-int3',
      ]
    ]
  ]

  const rules = [];

  formulas.forEach(([formula, colors]) => {
    colors.forEach((color) => {
      const className = `${prefix}-${color}`;

      const css = {};
      css[property] = formula(color);

      rules.push([className, css]);
    });
  });

  return rules;
}

export { colorRules }
