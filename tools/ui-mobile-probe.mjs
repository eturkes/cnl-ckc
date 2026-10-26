// Phone-layout probe for reviewer pages (n22): every .html under the given directories loads
// in a 390x844 mobile-emulated page (mobile viewport, touch, DPR 3); a page fails unless its
// clientWidth is 390 (the viewport meta applies), it has no page-level sideways scroll, and
// every table wider than its box brings its last column fully into view at scrollLeft = max.
// The browser runs over its DevTools pipe (fds 3 + 4, NUL-delimited JSON; tools/cdp-pipe.mjs),
// so the probe needs node alone:  node tools/ui-mobile-probe.mjs <headless browser> <dir>...
//   → rc 0 | 1 (bad pages listed) | 2 (usage, no pages, browser failure)
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { pathToFileURL } from "node:url";
import { open } from "./cdp-pipe.mjs";

const fail = (why) => {
  console.error(`ui-mobile: ${why}`);
  process.exit(2);
};
const [browser, ...dirs] = process.argv.slice(2);
if (!browser || dirs.length === 0) fail("usage: ui-mobile-probe.mjs <headless browser> <dir>...");
const walk = (d) =>
  readdirSync(d).flatMap((n) => {
    const p = join(d, n);
    return statSync(p).isDirectory() ? walk(p) : p.endsWith(".html") ? [p] : [];
  });
const files = dirs.flatMap(walk).sort();
if (files.length === 0) fail("no .html pages under the given directories");

const { send, event, timed, quit, close } = open(browser, "ui-mobile-", fail);

const measure = () => {
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
};

let bad = 0, wide = 0;
try {
  const { targetId } = await send("Target.createTarget", { url: "about:blank" });
  const { sessionId: s } = await send("Target.attachToTarget", { targetId, flatten: true });
  await send("Page.enable", {}, s);
  await send(
    "Emulation.setDeviceMetricsOverride",
    {
      width: 390,
      height: 844,
      deviceScaleFactor: 3,
      mobile: true,
      screenWidth: 390,
      screenHeight: 844,
      screenOrientation: { angle: 0, type: "portraitPrimary" },
    },
    s,
  );
  await send("Emulation.setTouchEmulationEnabled", { enabled: true, maxTouchPoints: 1 }, s);
  for (const f of files) {
    const loaded = event("Page.loadEventFired", s);
    const nav = await send("Page.navigate", { url: pathToFileURL(f).href }, s);
    if (nav.errorText) throw new Error(`${f}: ${nav.errorText}`);
    await timed(loaded, `${f}: load`);
    const { result, exceptionDetails } = await send(
      "Runtime.evaluate",
      { expression: `(${measure})()`, returnByValue: true },
      s,
    );
    if (exceptionDetails) throw new Error(`${f}: ${exceptionDetails.text}`);
    const m = result.value;
    wide += m.wide;
    if (m.cw !== 390 || m.over || m.reached !== m.wide) {
      bad += 1;
      console.log(`bad: ${f} clientWidth=${m.cw} sideways=${m.over} wide=${m.wide} reached=${m.reached}`);
    }
  }
} catch (e) {
  quit(e.message);
}
await close();
console.log(`ui-mobile: ${files.length} pages, ${bad} bad, ${wide} wide tables`);
process.exit(bad === 0 ? 0 : 1);
