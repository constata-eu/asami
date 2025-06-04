import globals from "globals";
import pluginJs from "@eslint/js";
import tseslint from "typescript-eslint";


/** @type {import('eslint').Linter.Config[]} */
export default [
  {
    files: ["**/*.{js,mjs,cjs,ts}"],
  },
  {
    languageOptions: {
      globals: globals.browser,
    }
  },
  pluginJs.configs.recommended,
  ...tseslint.configs.recommended,
  {
    rules: {
        "@typescript-eslint/no-unused-vars": "error",
    }
  },
  {
    ignores: [
      "node_modules",
      "/packages/**/node_modules",
      "/packages/**/lib",
    ]
  }
];