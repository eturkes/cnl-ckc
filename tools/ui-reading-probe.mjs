// UR P6: native pointer/touch/keyboard transitions over the three tests/ur-ui pages.
// node ui-reading-probe.mjs <pinned headless browser> <render-dir>...
// node ui-reading-probe.mjs --strip-handler <render-dir> <red-dir>
// rc 0 = pass; 1 = failed transitions; 2 = usage, filesystem, browser, or strip-anchor failure.
import { cpSync, existsSync, readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { basename, join } from "node:path";
import { pathToFileURL } from "node:url";
import { open } from "./cdp-pipe.mjs";

const fail = (why) => { console.error(`ui-reading: ${why}`); process.exit(2); };
const walk = (dir) => readdirSync(dir).flatMap((name) => {
  const path = join(dir, name);
  return statSync(path).isDirectory() ? walk(path) : path.endsWith(".html") ? [path] : [];
});
const anchor = 'closest("button.rw")';
const names = ["reading-only", "highlight-only", "combined"];
const args = process.argv.slice(2);

function stripHandler(html, path) {
  const open = html.indexOf("<script>"), close = html.indexOf("</script>");
  if (open < 0 || close < open) fail(`${path}: missing script before strip`);
  const body = html.slice(open + 8, close);
  if (body.split(anchor).length !== 2) fail(`${path}: reading-handler anchor must occur once before strip`);
  const at = body.indexOf(anchor);
  const starts = [...body.matchAll(/document\.addEventListener\("click",\s*function\s*\([^)]*\)\s*\{/g)];
  let selected;
  for (const match of starts) {
    let depth = 1, quote = "", escaped = false, end = match.index + match[0].length;
    for (; end < body.length && depth > 0; end += 1) {
      const char = body[end];
      if (quote) {
        if (escaped) escaped = false;
        else if (char === "\\") escaped = true;
        else if (char === quote) quote = "";
      } else if (char === '"' || char === "'") quote = char;
      else if (char === "{") depth += 1;
      else if (char === "}") depth -= 1;
    }
    if (depth !== 0) fail(`${path}: unbalanced handler`);
    const suffix = body.slice(end).match(/^\s*\);/);
    if (!suffix) fail(`${path}: handler terminator missing`);
    end += suffix[0].length;
    if (match.index <= at && at < end) selected = [match.index, end];
  }
  if (!selected) fail(`${path}: separate reading click handler not found`);
  const result = html.slice(0, open + 8) + body.slice(0, selected[0]) + body.slice(selected[1]) + html.slice(close);
  if (result.includes(anchor)) fail(`${path}: reading-handler anchor remains after strip`);
  return result;
}

if (args[0] === "--strip-handler") {
  if (args.length !== 3) fail("usage: --strip-handler <render-dir> <red-dir>");
  const [, source, target] = args;
  if (existsSync(target)) fail("strip target must be absent");
  try {
    cpSync(source, target, { recursive: true, errorOnExist: true });
    const pages = walk(target);
    let changed = 0;
    for (const name of names) {
      const found = pages.filter((p) => basename(p) === `${name}.html`);
      if (found.length !== 1) fail(`strip: expected one ${name}.html, found ${found.length}`);
      const path = found[0];
      writeFileSync(path, stripHandler(readFileSync(path, "utf8"), path));
      changed += 1;
    }
    console.log(`ui-reading: stripped reading click handler from ${changed} pages; anchors present before, absent after`);
    process.exit(0);
  } catch (error) { fail(error.message); }
}

const [browser, ...dirs] = args;
if (!browser || dirs.length === 0) fail("usage: ui-reading-probe.mjs <headless browser> <render-dir>...");
let files;
try { files = dirs.flatMap(walk).sort(); } catch (error) { fail(error.message); }
if (files.length === 0) fail("no .html pages under the given directories");
const selected = names.map((name) => [name, files.filter((p) => basename(p) === `${name}.html`)]);
const missing = selected.filter(([, paths]) => paths.length !== 1);
if (missing.length) {
  for (const [name, paths] of missing) console.log(`bad: ${name}: expected one fixture page, found ${paths.length}`);
  console.log(`ui-reading: 0 transitions, ${missing.length} failures`);
  process.exit(1);
}

const { send, event, timed, quit, close } = open(browser, "ui-reading-", fail);
let session, transitions = 0;
const failures = [];
class TransitionFailure extends Error {}
const evaluate = async (expression) => {
  const { result, exceptionDetails } = await send("Runtime.evaluate", { expression, returnByValue: true, awaitPromise: true }, session);
  if (exceptionDetails) throw new Error(exceptionDetails.exception?.description ?? exceptionDetails.text);
  return result.value;
};
const snap = () => evaluate(`(() => {
  const buttons = [...document.querySelectorAll("button.rw")];
  const reading = document.querySelector("p.reading");
  const pre = [...document.querySelectorAll("pre.prose")][1];
  return {
    buttons: buttons.map((b) => ({ w: b.dataset.w, text: b.textContent, pressed: b.getAttribute("aria-pressed") })),
    spans: reading ? [...reading.querySelectorAll("span[data-w]")].map((s) => ({ w: s.dataset.w, shown: !s.hidden && getComputedStyle(s).display !== "none", text: s.textContent })) : [],
    live: reading?.getAttribute("aria-live") ?? null,
    picked: [...document.querySelectorAll("mark")].map((m) => ({ group: [...m.classList].find((c) => /^t[0-9]+$/.test(c)) ?? "", picked: m.classList.contains("hl-pick") })),
    click: document.querySelector("main")?.classList.contains("hl-click") ?? false,
    toggle: document.querySelector("input.hl-toggle")?.checked ?? null,
    active: buttons.indexOf(document.activeElement),
    outline: getComputedStyle(document.activeElement).outlineStyle,
    outlineWidth: parseFloat(getComputedStyle(document.activeElement).outlineWidth),
    box: pre ? [pre.offsetLeft, pre.offsetTop, pre.offsetWidth, pre.offsetHeight] : null,
    scripts: document.querySelectorAll("script").length,
    prohibited: document.querySelectorAll("[tabindex], [style]").length
  };
})()`);
const require = (condition, label, detail) => { if (!condition) failures.push(`${label}: ${detail}`); };
const state = async (label, lemma, group, expectedBox, toggle, hasReading = true) => {
  transitions += 1;
  const s = await snap();
  const shown = s.spans.filter((x) => x.shown).map((x) => x.w);
  require(JSON.stringify(s.buttons.map((b) => b.w)) === JSON.stringify(hasReading ? ["usually", "ideally", "usually"] : []), label, "reading button inventory changed");
  require(JSON.stringify(s.spans.map((x) => x.w)) === JSON.stringify(hasReading ? ["", "usually", "ideally"] : []), label, "reading span inventory changed");
  require(s.live === (hasReading ? "polite" : null), label, "live reading-line semantics changed");
  require(s.picked.length === (toggle === null ? 0 : 4), label, "aligned mark inventory changed");
  require(JSON.stringify(shown) === JSON.stringify(hasReading ? [lemma] : []), label, `visible reading: expected ${hasReading ? JSON.stringify(lemma) : "no line"}, got ${JSON.stringify(shown)}`);
  for (const [i, b] of s.buttons.entries()) require(b.pressed === String(lemma !== "" && b.w === lemma), label, `aria-pressed button ${i} (${b.w}): ${b.pressed}`);
  require(s.click === (group !== ""), label, `highlight click state: expected ${group !== ""}, got ${s.click}`);
  for (const [i, m] of s.picked.entries()) require(m.picked === (group !== "" && m.group === group), label, `highlight mark ${i} (${m.group}): expected group ${JSON.stringify(group)}, picked=${m.picked}`);
  require(s.toggle === toggle, label, `highlight checkbox: expected ${toggle}, got ${s.toggle}`);
  require(JSON.stringify(s.box) === JSON.stringify(expectedBox), label, `ACE box moved: expected ${JSON.stringify(expectedBox)}, got ${JSON.stringify(s.box)}`);
  require(s.prohibited === 0, label, "tabindex or style attribute appeared");
  return s;
};
const key = async (key, code, virtual, text) => {
  await send("Input.dispatchKeyEvent", { type: "keyDown", key, code, windowsVirtualKeyCode: virtual, nativeVirtualKeyCode: virtual, ...(text && { text, unmodifiedText: text }) }, session);
  await send("Input.dispatchKeyEvent", { type: "keyUp", key, code, windowsVirtualKeyCode: virtual, nativeVirtualKeyCode: virtual }, session);
};
const tab = () => key("Tab", "Tab", 9);
const enter = () => key("Enter", "Enter", 13, "\r");
const space = () => key(" ", "Space", 32, " ");
const escape = () => key("Escape", "Escape", 27);
const location = async (selector, unmarked = false) => evaluate(`(() => {
  const el = document.querySelector(${JSON.stringify(selector)});
  if (!el) return null;
  el.scrollIntoView({ block: "center", inline: "nearest" });
  let box = el.getBoundingClientRect();
  if (${unmarked}) {
    const walker = document.createTreeWalker(el, NodeFilter.SHOW_TEXT);
    let node;
    while ((node = walker.nextNode())) {
      if (!node.textContent.trim() || node.parentElement.closest("mark")) continue;
      const range = document.createRange();
      range.selectNodeContents(node);
      box = range.getBoundingClientRect();
      break;
    }
    if (!node) return null;
  }
  return { x: box.left + box.width / 2, y: box.top + box.height / 2 };
})()`);
const move = async (selector, unmarked = false) => {
  const point = await location(selector, unmarked);
  if (!point) throw new TransitionFailure(`probe target absent: ${selector}${unmarked ? " (unmarked text)" : ""}`);
  await send("Input.dispatchMouseEvent", { type: "mouseMoved", ...point }, session);
  return point;
};
const click = async (selector, unmarked = false) => {
  const point = await move(selector, unmarked);
  await send("Input.dispatchMouseEvent", { type: "mousePressed", ...point, button: "left", clickCount: 1 }, session);
  await send("Input.dispatchMouseEvent", { type: "mouseReleased", ...point, button: "left", clickCount: 1 }, session);
};
const tap = async (selector) => {
  const point = await location(selector);
  if (!point) throw new TransitionFailure(`tap target absent: ${selector}`);
  // A touch-only device has no cursor apart from the tap point: Chrome reruns hover at the CDP mouse
  // position after a scroll, so a cursor parked elsewhere fires mouseout from the tapped mark (pick cleared).
  await send("Input.dispatchMouseEvent", { type: "mouseMoved", ...point }, session);
  await send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [{ ...point, id: 0, radiusX: 1, radiusY: 1, force: 1 }] }, session);
  await send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] }, session);
};
const load = async (path) => {
  const loaded = event("Page.loadEventFired", session);
  const navigation = await send("Page.navigate", { url: pathToFileURL(path).href }, session);
  if (navigation.errorText) throw new Error(`${path}: ${navigation.errorText}`);
  await timed(loaded, `${path}: load`);
};

async function readingPage(path, name, mobile) {
  await load(path);
  const combined = name === "combined", checkbox = combined ? true : null;
  const initial = await snap(), label = `${name}/${mobile ? "touch" : "desktop"}`;
  require(JSON.stringify(initial.buttons.map((b) => b.w)) === JSON.stringify(["usually", "ideally", "usually"]), label, "fixture must have reading buttons usually, ideally, usually");
  require(JSON.stringify(initial.spans.map((s) => s.w)) === JSON.stringify(["", "usually", "ideally"]), label, "reserved line must have hint, usually, ideally in occurrence order");
  require(initial.live === "polite" && initial.scripts === 1, label, "one live reading line and one script required");
  if (initial.buttons.length !== 3 || initial.spans.length !== 3) return;
  require(initial.spans[0].text === "Select a word with a solid underline to see how the compiler reads it.", label, "exact hint copy");
  require(initial.spans[1].text === "In this guideline, the compiler reads usually as a qualifier: How often.", label, "exact usually copy");
  require(initial.spans[2].text === "In this guideline, the compiler reads ideally as a qualifier: Best case.", label, "exact ideally copy");
  const box = initial.box;
  const first = 'button.rw[data-w="usually"]', other = 'button.rw[data-w="ideally"]', duplicate = 'button.rw[data-w="usually"]:last-of-type';
  const nested = combined ? `${first} mark` : first;
  await state(`${label}/initial`, "", "", box, checkbox);
  await move(nested);
  await state(`${label}/hover has no reading action`, "", "", box, checkbox);
  await click(nested);
  await state(`${label}/pointer nested mark`, "usually", combined ? "t0" : "", box, checkbox);
  await click(nested);
  await state(`${label}/second activation`, "", "", box, checkbox);
  await click(first, true);
  await state(`${label}/unmarked characters`, "usually", "", box, checkbox);
  await click(other);
  await state(`${label}/switch lemma`, "ideally", combined ? "t1" : "", box, checkbox);
  await click(duplicate, true);
  await state(`${label}/duplicate lemma button`, "usually", "", box, checkbox);
  await click(first, true);
  await state(`${label}/same lemma clears all buttons`, "", "", box, checkbox);

  await load(path);
  for (const [index, lemma] of ["usually", "ideally", "usually"].entries()) {
    let focused;
    for (let attempts = 0; attempts < 30; attempts += 1) {
      await tab(); focused = await snap();
      if (focused.active >= 0) break;
    }
    require(focused.active === index, `${label}/Tab ${index}`, `expected reading button ${index}, got ${focused.active}`);
    require(focused.outline !== "none" && focused.outlineWidth >= 3, `${label}/Tab ${index}`, "reading button lacks visible keyboard outline");
    if (index === 1) await space(); else await enter();
    await state(`${label}/Tab + ${index === 1 ? "Space" : "Enter"} ${index}`, lemma, "", box, checkbox);
  }
  await enter();
  await state(`${label}/keyboard second activation`, "", "", box, checkbox);
  await space();
  await state(`${label}/keyboard reselect`, "usually", "", box, checkbox);
  await click("#reviewer");
  await escape();
  await state(`${label}/Escape from another control`, "", "", box, checkbox);

  if (combined) {
    await click(nested);
    await state(`${label}/pick before disable`, "usually", "t0", box, true);
    await click("input.hl-toggle");
    await state(`${label}/disable highlights preserves reading`, "usually", "", box, false);
    await click(nested);
    await state(`${label}/reading clears while highlighting off`, "", "", box, false);
    await click(nested);
    await state(`${label}/reading selects while highlighting off`, "usually", "", box, false);
    await click(other);
    await state(`${label}/reading switches while highlighting off`, "ideally", "", box, false);
    await click("input.hl-toggle");
    await state(`${label}/enable highlights preserves reading`, "ideally", "", box, true);
    await click(nested);
    await state(`${label}/both actions resume`, "usually", "t0", box, true);
    await evaluate('document.getElementById("reviewer").focus()');
    await escape();
    await state(`${label}/Escape from another control preserves highlight`, "", "t0", box, true);
    await move("#reviewer");
    await state(`${label}/mouseout clears highlight only`, "", "", box, true);
    await click(nested);
    await state(`${label}/reading selected before pointer leaves`, "usually", "t0", box, true);
    await move("#reviewer");
    await state(`${label}/pointer leaves but reading stays`, "usually", "", box, true);
    await escape();
    await state(`${label}/Escape after mouseout`, "", "", box, true);
  }
  if (mobile) {
    await load(path);
    await tap(nested);
    await state(`${label}/tap`, "usually", combined ? "t0" : "", box, checkbox);
    await tap(nested);
    await state(`${label}/second tap`, "", "", box, checkbox);
  }
}

async function highlightPage(path, mobile) {
  await load(path);
  const label = `highlight-only/${mobile ? "touch" : "desktop"}`, initial = await snap(), box = initial.box;
  require(initial.buttons.length === 0 && initial.spans.length === 0 && initial.scripts === 1, label, "highlight-only fixture shape");
  require(initial.picked.length === 4, label, "two aligned groups on both prose blocks");
  if (initial.picked.length !== 4) return;
  await state(`${label}/initial`, "", "", box, true, false);
  await click("mark.t0");
  await state(`${label}/pick`, "", "t0", box, true, false);
  await click("mark.t0");
  await state(`${label}/second click`, "", "", box, true, false);
  await click("mark.t1");
  await state(`${label}/other group`, "", "t1", box, true, false);
  await move("#reviewer");
  await state(`${label}/mouseout`, "", "", box, true, false);
  await click("input.hl-toggle");
  await click("mark.t0");
  await state(`${label}/off ignores mark`, "", "", box, false, false);
  await click("input.hl-toggle");
  if (mobile) {
    await tap("mark.t0");
    await state(`${label}/tap`, "", "t0", box, true, false);
    await tap("mark.t0");
    await state(`${label}/second tap`, "", "", box, true, false);
  }
}

try {
  const { targetId } = await send("Target.createTarget", { url: "about:blank" });
  ({ sessionId: session } = await send("Target.attachToTarget", { targetId, flatten: true }));
  await send("Page.enable", {}, session);
  for (const mobile of [false, true]) {
    await send("Emulation.setDeviceMetricsOverride", { width: mobile ? 390 : 1280, height: 844, deviceScaleFactor: mobile ? 3 : 1, mobile, screenWidth: mobile ? 390 : 1280, screenHeight: 844 }, session);
    await send("Emulation.setTouchEmulationEnabled", { enabled: mobile, maxTouchPoints: 1 }, session);
    for (const [name, [path]] of selected) {
      try {
        if (name === "highlight-only") await highlightPage(path, mobile);
        else await readingPage(path, name, mobile);
      } catch (error) {
        if (!(error instanceof TransitionFailure)) throw error;
        failures.push(`${name}/${mobile ? "touch" : "desktop"}: ${error.message}`);
      }
    }
  }
} catch (error) { quit(error.message); }
await close();
for (const failure of failures) console.log(`bad: ${failure}`);
console.log(`ui-reading: ${transitions} transitions, ${failures.length} failures`);
process.exit(failures.length === 0 ? 0 : 1);
