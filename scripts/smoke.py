#!/usr/bin/env python3
"""Smoke-test a netbox-mcp binary: no network, no real credentials.

    python3 scripts/smoke.py [path/to/netbox-mcp]     (or: make smoke BIN=...)

1. `<bin> --version` reports the version in Cargo.toml (in a release, the
   verify job has already checked that equals the tag).
2. The stdio server, given a dummy URL and token, answers MCP initialize and
   lists exactly EXPECTED_TOOLS tools (following nextCursor). tools/list never
   touches NetBox, so a dummy config is a valid launch.

Run against every release artifact on a native runner, this proves the binary
starts, links, and exposes its full tool router.

Standard library only: python3 is on every CI runner.
"""

import json
import os
import re
import subprocess
import sys
import threading
from pathlib import Path

# Must change together with `tool_router_exposes_every_tool` in
# src/tools/mod.rs (EXPECTED_TOOLS there).
EXPECTED_TOOLS = 178

# Dummy credentials: the server refuses to start without a URL and token, but
# nothing here is sent anywhere. Must not match `make secrets-scan`.
DUMMY_ENV = {"NETBOX_URL": "http://localhost:1", "NETBOX_TOKEN": "dummy"}

STEP_TIMEOUT = 20  # seconds per JSON-RPC response

ROOT = Path(__file__).resolve().parent.parent


def fail(msg: str) -> None:
    print(f"smoke: FAIL: {msg}", file=sys.stderr)
    sys.exit(1)


def cargo_version() -> str:
    text = (ROOT / "Cargo.toml").read_text()
    m = re.search(r'^version\s*=\s*"([^"]+)"', text, re.MULTILINE)
    if not m:
        fail("no version in Cargo.toml")
    return m.group(1)


def isolated_env(home: str) -> dict:
    """A minimal environment, so the developer's own config can't leak in."""
    env = {"PATH": os.environ.get("PATH", "/usr/bin:/bin"), "HOME": home}
    env.update(DUMMY_ENV)
    return env


class Server:
    def __init__(self, binary: str, env: dict):
        self.proc = subprocess.Popen(
            [binary],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            env=env,
            text=True,
        )
        self.stderr: list[str] = []
        threading.Thread(target=self._drain_stderr, daemon=True).start()

    def _drain_stderr(self) -> None:
        for line in self.proc.stderr:
            self.stderr.append(line)

    def send(self, msg: dict) -> None:
        self.proc.stdin.write(json.dumps(msg) + "\n")
        self.proc.stdin.flush()

    def request(self, rid: int, method: str, params: dict) -> dict:
        self.send({"jsonrpc": "2.0", "id": rid, "method": method, "params": params})
        result: dict = {}

        def read() -> None:
            for line in self.proc.stdout:
                msg = json.loads(line)
                if msg.get("id") == rid:
                    result["msg"] = msg
                    return

        reader = threading.Thread(target=read, daemon=True)
        reader.start()
        reader.join(STEP_TIMEOUT)
        if "msg" not in result:
            tail = "".join(self.stderr[-20:])
            fail(f"no response to {method} within {STEP_TIMEOUT}s; server stderr:\n{tail}")
        msg = result["msg"]
        if "error" in msg:
            fail(f"{method} returned an error: {msg['error']}")
        return msg["result"]

    def close(self) -> None:
        self.proc.terminate()
        try:
            self.proc.wait(5)
        except subprocess.TimeoutExpired:
            self.proc.kill()


def main() -> None:
    binary = sys.argv[1] if len(sys.argv) > 1 else str(ROOT / "target/release/netbox-mcp")
    if not os.access(binary, os.X_OK):
        fail(f"{binary} is not an executable file")
    want_version = cargo_version()

    import tempfile

    with tempfile.TemporaryDirectory() as home:
        env = isolated_env(home)

        out = subprocess.run(
            [binary, "--version"], env=env, capture_output=True, text=True, timeout=STEP_TIMEOUT
        )
        if out.returncode != 0:
            fail(f"--version exited {out.returncode}: {out.stderr.strip()}")
        if want_version not in out.stdout:
            fail(f"--version printed {out.stdout.strip()!r}, expected {want_version}")
        print(f"smoke: --version ok ({out.stdout.strip()})")

        server = Server(binary, env)
        try:
            init = server.request(
                1,
                "initialize",
                {
                    "protocolVersion": "2025-06-18",
                    "capabilities": {},
                    "clientInfo": {"name": "smoke", "version": "0"},
                },
            )
            name = init.get("serverInfo", {}).get("name", "?")
            print(f"smoke: initialize ok (server {name})")
            server.send({"jsonrpc": "2.0", "method": "notifications/initialized"})

            tools: list = []
            cursor = None
            rid = 2
            while True:
                params = {"cursor": cursor} if cursor else {}
                page = server.request(rid, "tools/list", params)
                tools.extend(page.get("tools", []))
                cursor = page.get("nextCursor")
                rid += 1
                if not cursor:
                    break
                if rid > 100:
                    fail("tools/list pagination did not terminate")
        finally:
            server.close()

    names = [t["name"] for t in tools]
    if len(set(names)) != len(names):
        fail("tools/list returned duplicate tool names")
    if len(names) != EXPECTED_TOOLS:
        fail(f"tools/list returned {len(names)} tools, expected {EXPECTED_TOOLS}")
    print(f"smoke: tools/list ok ({len(names)} tools)")
    print("smoke: PASS")


if __name__ == "__main__":
    main()
