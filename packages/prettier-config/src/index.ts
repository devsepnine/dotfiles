import type { Options } from "prettier";

const config: Options = {
  printWidth: 100,
  tabWidth: 2,
  useTabs: false,
  semi: true,
  singleQuote: true,
  jsxSingleQuote: false,
  trailingComma: "all", // “To reduce git diff.”
  bracketSpacing: true,
  bracketSameLine: false, // “Change to false for improved readability.”
  arrowParens: "always", // “Enhance consistency.”
  endOfLine: "lf", // “Prevent different line endings across operating systems.”
};

export default config;
