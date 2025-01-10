import eslintConfigPrettier from "eslint-config-prettier";
import pluginVue from "eslint-plugin-vue";
import globals from "globals";

import withNuxt from "./.nuxt/eslint.config.mjs";

export default withNuxt(
  eslintConfigPrettier,
  ...pluginVue.configs["flat/essential"],
  { languageOptions: { globals: { ...globals.browser, ...globals.node } } },
  {
    files: ["**/*.{js,mjs,cjs,ts,vue}"],
    ignores: [
      "node_modules",
      "dist",
      "public",
      ".nuxt",
      "build",
      "docs",
      "static",
      "coverage",
    ],
    rules: {
      camelcase: "off",
      "default-case": 0,
      "comma-dangle": [0],
      "linebreak-style": 0,
      "consistent-return": 0,
      "import/imports-first": 0,
      "import/no-unresolved": 0,
      "prefer-destructuring": 0,
      "implicit-arrow-linebreak": 0,
      "import/prefer-default-export": 0,
      "vue/no-watch-after-await": "off",
      complexity: ["error", { max: 30 }],
      "max-nested-callbacks": ["error", 4],
      "new-cap": ["error", { capIsNew: true }],
      "arrow-body-style": ["error", "as-needed"],
      "vue/html-closing-bracket-spacing": "error",
      "vue/html-closing-bracket-newline": ["error", { multiline: "always" }],
      "max-lines": [
        "error",
        { max: 350, skipComments: true, skipBlankLines: true },
      ],
      "vue/html-self-closing": [
        "error",
        { html: { normal: "any", void: "always" } },
      ],
      "no-console": "off",
      "no-debugger": "off",
      "vue/html-indent": "off",
      "import/extensions": "off",
      "no-param-reassign": "off",
      "operator-linebreak": "off",
      "vue/script-setup-uses-vars": "off",
      "vue/max-attributes-per-line": "off",
      "vue/multi-word-component-names": "off",
      "vue/no-reserved-component-names": "off",
      "vue/singleline-html-element-content-newline": "off",
    },
  }
);
