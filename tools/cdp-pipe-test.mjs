// Red inputs for tools/cdp-pipe.mjs, run by `just mobile` before the probes: a mock
// browser answers the first request with a malformed frame (invalid JSON, JSON null,
// an unknown id) — each must reach `fail` with the profile removed — or splits a
// two-byte character across two writes, which must arrive whole.
// node cdp-pipe-test.mjs → rc 0 | 1 (failed cases listed)
import { existsSync, mkdtempSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { open } from "./cdp-pipe.mjs";

const dir = mkdtempSync(join(tmpdir(), "cdp-pipe-test-"));
const mock = join(dir, "mock.mjs");
writeFileSync(mock, `import fs from "node:fs";
const out = fs.createWriteStream(null, { fd: 4 });
let input = "";
fs.createReadStream(null, { fd: 3, encoding: "utf8" }).on("data", (text) => {
  input += text;
  const end = input.indexOf("\\0");
  if (end < 0) return;
  const { id } = JSON.parse(input.slice(0, end));
  input = input.slice(end + 1);
  const answer = {
    "invalid-json": () => out.write("{\\0"),
    "null-frame": () => out.write("null\\0"),
    "unknown-id": () => out.write(JSON.stringify({ id: id + 7, result: {} }) + "\\0"),
    "split-utf8": () => {
      const bytes = Buffer.from(JSON.stringify({ id, result: { v: "é" } }) + "\\0");
      const cut = bytes.indexOf(0xc3) + 1;
      out.write(bytes.subarray(0, cut));
      setTimeout(() => out.write(bytes.subarray(cut)), 50);
    },
  }[process.env.CDP_CASE];
  answer();
});
`);
const browser = join(dir, "browser");
writeFileSync(browser, `#!/bin/sh\nexec ${JSON.stringify(process.execPath)} ${JSON.stringify(mock)} "$@"\n`, { mode: 0o755 });

const profiles = (prefix) => readdirSync(tmpdir()).filter((n) => n.startsWith(prefix));
const bad = [];
for (const name of ["invalid-json", "null-frame", "unknown-id", "split-utf8"]) {
  process.env.CDP_CASE = name;
  const prefix = `cdp-pipe-${name}-${process.pid}-`;
  let failed = null;
  const pipe = open(browser, prefix, (why) => (failed = why));
  // A malformed answer leaves the request pending: `quit` ends the probe (rc 2) instead.
  const sent = pipe.send("Test.probe").catch((e) => ({ error: e.message }));
  if (name === "split-utf8") {
    const answer = await sent;
    await pipe.close();
    if (failed !== null || answer?.v !== "é") bad.push(`${name}: got ${JSON.stringify(answer)}, fail ${failed}`);
  } else {
    for (let t = 0; failed === null && t < 100; t++) await new Promise((ok) => setTimeout(ok, 20));
    if (failed === null || !failed.startsWith("browser")) bad.push(`${name}: fail not reached (${failed})`);
  }
  if (profiles(prefix).length) bad.push(`${name}: profile left behind`);
}
rmSync(dir, { recursive: true, force: true });
for (const b of bad) console.log(`bad: ${b}`);
console.log(`cdp-pipe: 4 cases, ${bad.length} failures`);
process.exit(bad.length ? 1 : 0);
