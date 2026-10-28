// The JavaScript lint of `just eslint`: the review page's script (rust/ckc-spec/src/ui.rs
// script_html, extracted by the recipe) + the browser probes. Rules = dynamic-code + DOM-sink
// classes; everything else is out of this gate's scope.
import noUnsanitized from "eslint-plugin-no-unsanitized";

export default [
  {
    files: ["**/*.js", "**/*.mjs"],
    plugins: { "no-unsanitized": noUnsanitized },
    languageOptions: { ecmaVersion: "latest" },
    linterOptions: { reportUnusedDisableDirectives: "error" },
    rules: {
      "no-eval": "error",
      "no-implied-eval": "error",
      "no-new-func": "error",
      "no-script-url": "error",
      "no-unsanitized/method": "error",
      "no-unsanitized/property": "error",
    },
  },
];
