// Phone-layout probe for reviewer pages (n22): every .html under the given directories loads
// in a 390x844 mobile-emulated page (mobile viewport, touch, DPR 3); a page fails unless its
// clientWidth is 390 (the viewport meta applies), it has no page-level sideways scroll, and
// every table wider than its box brings its last column fully into view at scrollLeft = max.
// The browser runs over its DevTools pipe (fds 3 + 4, NUL-delimited JSON), so the probe needs
// node alone:  node tools/ui-mobile-probe.mjs <headless browser> <dir>...
//   → rc 0 | 1 (bad pages listed) | 2 (usage, no pages, browser failure)
import { spawn } from "node:child_process";
import { mkdtempSync, readdirSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { pathToFileURL } from "node:url";

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

// --hide-scrollbars = Playwright's headless default: a classic scrollbar would narrow
// clientWidth. --no-sandbox: hosts that restrict unprivileged user namespaces (Ubuntu 24.04
// runners) cannot start the sandbox; the inputs are this repo's own rendered pages.
const profile = mkdtempSync(join(tmpdir(), "ui-mobile-"));
const chrome = spawn(
  browser,
  [
    "--headless",
    "--no-sandbox",
    "--hide-scrollbars",
    "--no-first-run",
    "--no-startup-window",
    "--remote-debugging-pipe",
    `--user-data-dir=${profile}`,
  ],
  { stdio: ["ignore", "ignore", "pipe", "pipe", "pipe"] },
);
let stderr = "";
chrome.stderr.on("data", (b) => (stderr += b));
const quit = (why) => {
  chrome.kill("SIGKILL");
  rmSync(profile, { recursive: true, force: true });
  fail(`${why}${stderr ? `\n${stderr.trimEnd()}` : ""}`);
};
for (const x of [chrome, chrome.stdio[3], chrome.stdio[4]]) x.on("error", (e) => quit(`browser: ${e.message}`));
chrome.on("exit", (code, signal) => quit(`browser exited (${signal ?? code})`));

let next = 0, buffer = "";
const pending = new Map(), waiters = [];
chrome.stdio[4].on("data", (b) => {
  buffer += b;
  for (let i; (i = buffer.indexOf("\0")) >= 0; ) {
    let m;
    try {
      m = JSON.parse(buffer.slice(0, i));
    } catch (e) {
      quit(`browser: ${e.message}`);
    }
    buffer = buffer.slice(i + 1);
    if (m.id !== undefined) {
      const [ok, ko, method] = pending.get(m.id);
      pending.delete(m.id);
      m.error ? ko(new Error(`${method}: ${m.error.message}`)) : ok(m.result);
    } else {
      const k = waiters.findIndex((w) => w.method === m.method && w.session === m.sessionId);
      if (k >= 0) waiters.splice(k, 1)[0].ok(m.params);
    }
  }
});
const timed = (p, what) =>
  Promise.race([p, new Promise((_, ko) => setTimeout(() => ko(new Error(`${what}: timeout`)), 60000).unref())]);
const send = (method, params = {}, sessionId) =>
  timed(
    new Promise((ok, ko) => {
      const id = ++next;
      pending.set(id, [ok, ko, method]);
      chrome.stdio[3].write(JSON.stringify({ id, method, params, ...(sessionId && { sessionId }) }) + "\0");
    }),
    method,
  );
const event = (method, session) => new Promise((ok) => waiters.push({ method, session, ok }));

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
for (const x of [chrome, chrome.stdio[3], chrome.stdio[4]]) x.removeAllListeners("error").on("error", () => {});
chrome.removeAllListeners("exit");
const exited = new Promise((ok) => chrome.once("exit", ok));
await send("Browser.close").catch(() => {});
await timed(exited, "browser close").catch(() => chrome.kill("SIGKILL"));
rmSync(profile, { recursive: true, force: true });
console.log(`ui-mobile: ${files.length} pages, ${bad} bad, ${wide} wide tables`);
process.exit(bad === 0 ? 0 : 1);
