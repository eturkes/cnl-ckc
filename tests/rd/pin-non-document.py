#!/usr/bin/env python3
"""RD P3: python3 -I tests/rd/pin-non-document.py <base-ckc> <repo-root>.

Base executable = 065d814b; stdout → tests/rd/non-document.sha256.
Only the randomized sign-in URL normalizes; HTTP pins hash complete bodies.
"""

import hashlib
import os
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import time

BASE = "065d814b1acc43992bfbb36f4b356da411d7b61e"


def main():
    if len(sys.argv) != 3:
        raise SystemExit("usage: pin-non-document.py <base-ckc> <repo-root>")
    binary, root = (Path(p).resolve(strict=True) for p in sys.argv[1:])
    tree = root / "tests/ui/green/basic/tree"
    session = "a" * 64
    pins = []
    for name, route in [("session-403", "/"), ("session-303", "/session/" + session)]:
        out = subprocess.run(
            [str(binary), "ui", "request", "GET", route, str(tree), "--session", session],
            capture_output=True,
            check=True,
        )
        assert not out.stderr, out.stderr
        head, body = out.stdout.split(b"\n\n", 1)
        assert int(head.splitlines()[0].split()[1]) == int(name[-3:]), head
        pins.append((name, hashlib.sha256(body).hexdigest()))
    with tempfile.TemporaryDirectory(prefix="rd-pins-", dir=root / "tests/rd") as tmp:
        tmp = Path(tmp)
        runtime = tmp / "run"
        runtime.mkdir(mode=0o700)
        with socket.socket() as sock:
            sock.bind(("127.0.0.1", 0))
            port = sock.getsockname()[1]
        env = {
            k: v
            for k, v in os.environ.items()
            if k not in {"DISPLAY", "WAYLAND_DISPLAY", "WSL_DISTRO_NAME"}
        }
        env.update(BROWSER="/bin/true", XDG_RUNTIME_DIR=str(runtime), TMPDIR=str(tmp))
        with (tmp / "stdout").open("wb") as stdout, (tmp / "stderr").open("wb") as stderr:
            child = subprocess.Popen(
                [str(binary), "ui", "serve", str(port), str(tree)],
                cwd=tmp,
                env=env,
                stdin=subprocess.DEVNULL,
                stdout=stdout,
                stderr=stderr,
            )
            try:
                deadline = time.monotonic() + 20
                page = runtime / f"ckc-ui-{port}.html"
                while not page.exists() or not page.read_bytes().endswith(b"</html>\n"):
                    assert child.poll() is None, (tmp / "stderr").read_text()
                    assert time.monotonic() < deadline, "sign-in timeout"
                    time.sleep(0.02)
                body = page.read_bytes()
                address = body.split(b'content="0; url=', 1)[1].split(b'"', 1)[0]
                assert address.startswith(f"http://127.0.0.1:{port}/session/".encode())
                key = address.rsplit(b"/", 1)[1]
                assert len(key) == 64 and all(c in b"0123456789abcdef" for c in key)
                normalized = body.replace(address, b"@SIGNIN_URL@")
                pins.append(("signin", hashlib.sha256(normalized).hexdigest()))
            finally:
                child.terminate()
                child.wait(timeout=10)
        with socket.socket() as sock:
            assert sock.connect_ex(("127.0.0.1", port)) != 0, "server cleanup"
        case = "green/unicode-digit-region"
        render = tmp / "render"
        out = subprocess.run(
            [str(binary), "ui", "render", str(render), str(root / "tests/ui" / case / "tree")],
            capture_output=True,
            check=True,
        )
        assert not out.stderr, out.stderr
        pages = sorted(p for p in render.rglob("*.html") if p.parent.name != "doc")
        assert len(pages) == 3, "unfrozen non-document census"
        for page in pages:
            name = f"{case}/golden/{page.relative_to(render).as_posix()}"
            pins.append((name, hashlib.sha256(page.read_bytes()).hexdigest()))
    print(f"# RD P3 byte identity; base {BASE}.")
    print("# Sign-in URL alone normalizes to @SIGNIN_URL@; HTTP pins hash bodies.")
    for name, digest in pins:
        print(f"{digest}\t{name}")


if __name__ == "__main__":
    main()
