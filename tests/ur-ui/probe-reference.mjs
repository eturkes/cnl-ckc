// Probe calibration only: independent D7 state machine + frozen pre-unit highlight chrome.
// node probe-reference.mjs <pre-unit-highlight-golden.html> <output-dir>
// The product is always graded separately on its unmodified rendered pages.
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const [oldPage, out] = process.argv.slice(2);
if (!oldPage || !out || process.argv.length !== 4) throw new Error("usage: <old-highlight-golden> <output-dir>");
const old = readFileSync(oldPage, "utf8");
const block = (start, end) => old.split(start)[1].split(end)[0];
const css = block("<style>\n", "</style>") + '\nbutton.rw { margin: 0; padding: 0; border: 0; font: inherit; color: inherit; background: none; cursor: pointer; text-decoration: underline solid #1d4ed8 1px; text-underline-offset: 0.35em; }\nbutton.rw[aria-pressed="true"] { text-decoration-thickness: 3px; }\np.reading { min-height: 1.55em; margin: 0.5rem 0 0; overflow-wrap: anywhere; }\n';
const oldScript = block("<script>", "</script>");
if (oldScript.includes('closest("button.rw")')) throw new Error("reference requires the pre-unit highlight-only script");
const readingScript = `
const selectReading = (lemma) => {
  for (const span of document.querySelectorAll("p.reading span[data-w]")) span.hidden = span.dataset.w !== lemma;
  for (const button of document.querySelectorAll("button.rw")) button.setAttribute("aria-pressed", String(lemma !== "" && button.dataset.w === lemma));
};
document.addEventListener("click", function (ev) {
  const button = ev.target.closest("button.rw");
  if (button === null) return;
  selectReading(button.getAttribute("aria-pressed") === "true" ? "" : button.dataset.w);
});
document.addEventListener("keydown", function (ev) {
  if (ev.key === "Escape") selectReading("");
});
`;
const line = '<p class="reading" aria-live="polite"><span data-w="">Select a word with a solid underline to see how the compiler reads it.</span><span data-w="usually" hidden>In this guideline, the compiler reads usually as a qualifier: How often.</span><span data-w="ideally" hidden>In this guideline, the compiler reads ideally as a qualifier: Best case.</span></p>';
const button = (lemma, content) => `<button type="button" class="rw" aria-pressed="false" data-w="${lemma}">${content}</button>`;
mkdirSync(out, { recursive: true });
for (const name of ["reading-only", "highlight-only", "combined"]) {
  const readings = name !== "highlight-only", highlights = name !== "reading-only";
  const source = highlights ? '<mark class="t0">linked-first</mark> <mark class="t1">linked-second</mark>' : 'usually and ideally appear in the source.';
  const ace = readings
    ? button("usually", highlights ? '<mark class="t0">usual</mark>ly' : "usually") + ',' + button("ideally", highlights ? '<mark class="t1">ideally</mark>' : "ideally") + ' ' + button("usually", "usually") + '.\n'
    : '<mark class="t0">domain</mark> <mark class="t1">label</mark>.\n';
  const html = `<!doctype html>\n<html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>Probe reference</title><style>\n${css}</style></head><body><main id="main">${highlights ? '<p class="hl-note"><label><input type="checkbox" class="hl-toggle" checked> Highlighting</label></p>' : ''}<script>${oldScript}${readingScript}</script><section><pre class="prose">${source}</pre></section><section><pre class="prose">${ace}</pre>${readings ? line : ''}</section><label for="reviewer">Reviewer name</label><input id="reviewer" type="text"></main></body></html>\n`;
  writeFileSync(join(out, `${name}.html`), html);
}
console.log("ur-probe-reference: three independent transition-control pages");
