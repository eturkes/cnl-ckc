// One DevTools-pipe client for the reviewer-page probes (n22 mobile, UR P6 reading):
// the headless browser runs over fds 3 + 4 (NUL-delimited CDP JSON). The pipe decodes
// as one UTF-8 stream (a character split across chunks stays whole), and every
// malformed frame, unknown response id or browser failure goes to `quit`, which
// removes the profile before the caller's rc 2 path (`fail`).
import { spawn } from "node:child_process";
import { mkdtempSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";

// Chunks (strings) → frames: everything before each NUL.
export const frames = (onFrame) => {
  let buffer = "";
  return (text) => {
    buffer += text;
    for (let end; (end = buffer.indexOf("\0")) >= 0; ) {
      const frame = buffer.slice(0, end);
      buffer = buffer.slice(end + 1);
      onFrame(frame);
    }
  };
};

export const open = (browser, prefix, fail) => {
  const profile = mkdtempSync(join(tmpdir(), prefix));
  // --hide-scrollbars = Playwright's headless default: a classic scrollbar would narrow
  // clientWidth. --no-sandbox: hosts that restrict unprivileged user namespaces (the
  // Ubuntu CI runners) cannot start the sandbox; the inputs are this repo's own pages.
  const chrome = spawn(
    browser,
    ["--headless", "--no-sandbox", "--hide-scrollbars", "--no-first-run", "--no-startup-window",
      "--remote-debugging-pipe", `--user-data-dir=${profile}`],
    { stdio: ["ignore", "ignore", "pipe", "pipe", "pipe"] },
  );
  let stderr = "", stopped = false, next = 0;
  const pending = new Map(), waiters = [];
  const quit = (why) => {
    if (stopped) return;
    stopped = true;
    chrome.kill("SIGKILL");
    rmSync(profile, { recursive: true, force: true });
    fail(`${why}${stderr ? `\n${stderr.trimEnd()}` : ""}`);
  };
  chrome.stderr.setEncoding("utf8");
  chrome.stderr.on("data", (text) => (stderr += text));
  for (const x of [chrome, chrome.stdio[3], chrome.stdio[4]]) x.on("error", (e) => quit(`browser: ${e.message}`));
  chrome.on("exit", (code, signal) => quit(`browser exited (${signal ?? code})`));
  chrome.stdio[4].setEncoding("utf8");
  chrome.stdio[4].on("data", frames((frame) => {
    if (stopped) return;
    let m;
    try {
      m = JSON.parse(frame);
    } catch (e) {
      return quit(`browser: ${e.message}`);
    }
    if (m === null || typeof m !== "object") return quit(`browser: malformed frame ${frame.slice(0, 80)}`);
    if (m.id !== undefined) {
      const request = pending.get(m.id);
      if (!request) return quit(`browser: unexpected CDP id ${m.id}`);
      pending.delete(m.id);
      const [ok, ko, method] = request;
      m.error ? ko(new Error(`${method}: ${m.error.message}`)) : ok(m.result);
    } else {
      const k = waiters.findIndex((w) => w.method === m.method && w.session === m.sessionId);
      if (k >= 0) waiters.splice(k, 1)[0].ok(m.params);
    }
  }));
  const timed = (p, what) =>
    Promise.race([p, new Promise((_, ko) => setTimeout(() => ko(new Error(`${what}: timeout`)), 60000).unref())]);
  const send = (method, params = {}, sessionId) =>
    timed(new Promise((ok, ko) => {
      const id = ++next;
      pending.set(id, [ok, ko, method]);
      chrome.stdio[3].write(JSON.stringify({ id, method, params, ...(sessionId && { sessionId }) }) + "\0");
    }), method);
  const event = (method, session) => new Promise((ok) => waiters.push({ method, session, ok }));
  // Normal shutdown: Browser.close, wait for the exit (SIGKILL after 60 s), remove the profile.
  const close = async () => {
    for (const x of [chrome, chrome.stdio[3], chrome.stdio[4]]) x.removeAllListeners("error").on("error", () => {});
    chrome.removeAllListeners("exit");
    const exited = new Promise((ok) => chrome.once("exit", ok));
    await send("Browser.close").catch(() => {});
    await timed(exited, "browser close").catch(() => chrome.kill("SIGKILL"));
    stopped = true;
    rmSync(profile, { recursive: true, force: true });
  };
  return { send, event, timed, quit, close };
};
