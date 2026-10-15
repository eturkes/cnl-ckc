// Phone-layout probe for reviewer pages (n22 P4): every .html under the given
// directories loads in a 390x844 mobile-emulated page (isMobile, DPR 3); a page fails
// unless its clientWidth is 390 (the viewport meta applies), it has no page-level
// sideways scroll, and every table wider than its box brings its last column fully
// into view at scrollLeft = max. Runs outside the gate (needs a browser):
//   NODE_PATH=<dir with playwright-core> CKC_CHROMIUM=<chromium binary> \
//     node tools/ui-mobile-probe.mjs <dir>...   → rc 0 | 1 (bad pages listed)
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { createRequire } from "node:module";
import { pathToFileURL } from "node:url";

const { chromium } = createRequire(join(process.cwd(), "noop.js"))("playwright-core");
const walk = (d) =>
  readdirSync(d).flatMap((n) => {
    const p = join(d, n);
    return statSync(p).isDirectory() ? walk(p) : p.endsWith(".html") ? [p] : [];
  });
const files = process.argv.slice(2).flatMap(walk).sort();
if (files.length === 0) {
  console.error("ui-mobile: no .html pages under the given directories");
  process.exit(2);
}
const browser = await chromium.launch({ executablePath: process.env.CKC_CHROMIUM, headless: true });
const page = await browser.newPage({
  viewport: { width: 390, height: 844 },
  isMobile: true,
  hasTouch: true,
  deviceScaleFactor: 3,
});
let bad = 0, wide = 0;
for (const f of files) {
  await page.goto(pathToFileURL(f).href, { waitUntil: "load" });
  const m = await page.evaluate(() => {
    const de = document.documentElement;
    let wide = 0, reached = 0;
    for (const t of document.querySelectorAll("table")) {
      if (t.scrollWidth <= t.clientWidth) continue;
      wide += 1;
      t.scrollLeft = t.scrollWidth;
      const box = t.getBoundingClientRect();
      const cells = [...t.querySelectorAll("tr > :last-child")];
      const inside = (r) => r.left >= box.left - 1 && r.right <= box.right + 1;
      if (cells.every((c) => inside(c.getBoundingClientRect()))) reached += 1;
    }
    return { cw: de.clientWidth, over: de.scrollWidth > de.clientWidth, wide, reached };
  });
  wide += m.wide;
  if (m.cw !== 390 || m.over || m.reached !== m.wide) {
    bad += 1;
    console.log(`bad: ${f} clientWidth=${m.cw} sideways=${m.over} wide=${m.wide} reached=${m.reached}`);
  }
}
await browser.close();
console.log(`ui-mobile: ${files.length} pages, ${bad} bad, ${wide} wide tables`);
process.exit(bad === 0 ? 0 : 1);
