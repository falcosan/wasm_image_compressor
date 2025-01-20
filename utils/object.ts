/* eslint-disable @typescript-eslint/no-explicit-any */

export const filterObject = (
  elements: any,
  filters: string | string[],
  rule: boolean,
  strict = false
) => {
  const filterFn = (k: string) => {
    if (strict) {
      if (Array.isArray(filters)) {
        return rule
          ? filters.every((filter) => k === filter)
          : filters.every((filter) => k !== filter);
      } else {
        return rule ? k === filters : k !== filters;
      }
    } else {
      const filterRegex = new RegExp(
        Array.isArray(filters) ? filters.join("|") : filters
      );
      return rule ? filterRegex.test(k) : !filterRegex.test(k);
    }
  };
  return Object.entries(elements).reduce((acc: any, [k, v]) => {
    if (filterFn(k)) acc[k] = v;
    return acc;
  }, {});
};
