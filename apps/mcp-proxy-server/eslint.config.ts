import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";
import pluginPrettier from "eslint-config-prettier";

const fixedBrowserGlobals = Object.fromEntries(
  Object.entries({ ...globals.browser, ...globals.node }).map(
    ([key, value]) => [key.trim(), value],
  ),
);

/** @type {import('eslint').Linter.Config[]} */
export default [
  { files: ["**/*.{js,mjs,cjs,ts,jsx,tsx}"] },
  {
    ignores: [],
  },
  { languageOptions: { globals: fixedBrowserGlobals } },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  pluginPrettier,
  {
    rules: {
      "@typescript-eslint/no-explicit-any": "warn",
      "@typescript-eslint/no-unused-vars": "warn",
    },
  },
];
