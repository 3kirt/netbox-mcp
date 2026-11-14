# netbox-mcp

A [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server that connects Claude and other MCP-compatible AI clients to a live [NetBox](https://netbox.dev) instance.

Ask questions like *"Which devices in the NYC site are currently in maintenance?"* or *"Show me all IPs assigned to web01 and their VRFs"* — the server translates them into real-time NetBox API queries and returns structured results.

- **Read plus limited write** — 178 tools: 169 read-only queries, plus create/update/delete for VMs, IP addresses, and tags. Writes are gated by your NetBox token's permissions, so a read-only token keeps the server read-only
- **stdio transport** — runs as a local subprocess managed by your MCP client
- **Token-efficient** — responses are slimmed before delivery, cutting typical NetBox payloads by 50–70%

---

## Table of contents

- [Requirements](#requirements)
- [Installation](#installation)
- [Configuration](#configuration)
  - [GPG-encrypted token (pass)](#gpg-encrypted-token-pass)
- [Client setup](#client-setup)
  - [Claude Desktop](#claude-desktop)
  - [Claude Code](#claude-code)
- [Available tools](#available-tools)
- [Development](#development)

---

## Requirements

- A running NetBox instance with a valid API token
- To build from source: Rust 1.89 or later (`rustup install stable`); the repo pins its development toolchain in `rust-toolchain.toml`
- To use a pre-built binary: nothing — grab it from the [releases page](https://github.com/3kirt/netbox-mcp/releases)
- Optional: GnuPG, to keep the token encrypted instead of in plaintext ([GPG-encrypted token](#gpg-encrypted-token-pass))

---

## Installation

### Pre-built binary (recommended)

Download the binary for your platform from the [releases page](https://github.com/3kirt/netbox-mcp/releases) and place it somewhere on your `$PATH`.

### From source

```sh
git clone https://github.com/3kirt/netbox-mcp
cd netbox-mcp
make install
```

Installs the `netbox-mcp` binary to `$CARGO_HOME/bin` (typically `~/.cargo/bin`).

---

## Configuration

netbox-mcp reads credentials from `~/.netbox_mcp.json`:

```json
{
  "url": "https://netbox.example.com",
  "token": "your-api-token"
}
```

Environment variables take precedence over the config file (a variable that is
set but empty counts as unset):

| Variable | Description |
|---|---|
| `NETBOX_URL` | NetBox base URL |
| `NETBOX_TOKEN` | NetBox API token |

A custom config file path can be specified with `--config`:

```sh
netbox-mcp --config /path/to/config.json
```

### Obtaining an API token

Tokens can be created in NetBox under **Admin → API Tokens**, or provisioned via the API:

```sh
curl -s -X POST https://netbox.example.com/api/users/tokens/provision/ \
  -H "Content-Type: application/json" \
  -d '{"username": "you", "password": "yourpassword"}' \
  | jq '.key'
```

netbox-mcp has a few write tools (VMs, IP addresses, tags), but it relies on the token's permissions to authorize them. If you only want reads, create a **read-only token** (deselect "Write enabled" in the token settings); write tools then fail with 403. If the server starts returning 403 errors, check whether the token has expired.

### GPG-encrypted token (pass)

*Linux (including WSL) or macOS.*

Instead of a plaintext `token`, netbox-mcp can keep the token GPG-encrypted in a
[`pass`](https://www.passwordstore.org/)-compatible password store
(`~/.password-store`, or `$PASSWORD_STORE_DIR`). Only `gpg` is required; the
`pass` CLI is optional, but `pass show` reads the entry if you have it.

#### 1. Set up GnuPG

Install GnuPG if you don't have it: your package manager on Linux, or Homebrew
or GPG Suite on macOS.

You need a GPG key that can encrypt. Run `gpg --list-secret-keys` and look for
a key whose subkey shows `[E]`. If you don't have one, create it:

```sh
gpg --quick-gen-key "Your Name <you@example.com>" default default 2y
```

Then configure gpg-agent in `~/.gnupg/gpg-agent.conf` (or under `$GNUPGHOME`
if you set it). The default cache lifetimes (10 minutes, 2 hours at most) are
short for unattended use. This caches an unlock for an 8-hour work day:

```
default-cache-ttl 28800
max-cache-ttl 28800
```

Apply changes with `gpgconf --kill gpg-agent`; the agent restarts on its next
use.

**Which pinentry.** `netbox-mcp unlock` prompts through gpg-agent's pinentry
(the server itself never prompts). In a normal terminal, a terminal pinentry
works as long as gpg knows the terminal: add `export GPG_TTY=$(tty)` to your
shell profile. To unlock from somewhere without a real terminal, such as an
agent's shell (Claude Code's `!` prefix), you need a graphical pinentry, set
with a `pinentry-program <path>` line in `gpg-agent.conf`:

| Platform | Graphical pinentry |
|---|---|
| Linux | `pinentry-gnome3` or `pinentry-qt` from your package manager |
| macOS | `pinentry-mac` (Homebrew); it can also save the passphrase in the Keychain |
| Linux under WSL | `pinentry-gtk2` or `pinentry-qt`, which display through WSLg on Windows 11; without WSLg, unlock from a WSL terminal instead |

#### 2. Store the token

```sh
netbox-mcp login --gpg-id <your-key-id>   # first time: no .gpg-id in the store yet
netbox-mcp login                          # later: reuses the store's .gpg-id
```

`login` prompts for the token (no echo; `--token-stdin` reads it from a pipe),
checks it against NetBox, encrypts it to `netbox-mcp/<host>.gpg`, and updates
the config file. It also removes any plaintext `token` from that file:

```json
{
  "url": "https://netbox.example.com",
  "pass_entry": "netbox-mcp/netbox.example.com",
  "password_store_dir": "/home/you/.password-store"
}
```

Because the plaintext token is removed, every netbox-mcp the config is used
with must support `pass_entry`; an older one can't start without a token.

#### 3. Leave `NETBOX_TOKEN` out of your MCP client config

`NETBOX_TOKEN` takes precedence over `pass_entry`. If your MCP client config
passes it (for example via `--env NETBOX_TOKEN=…` in `claude mcp add`), the
server uses that value and never touches the encrypted token. Pass only
`NETBOX_URL`, or nothing, since the config file has the URL. A
`"${NETBOX_TOKEN:-}"` placeholder is fine as long as the variable isn't
exported: it expands to an empty value, which counts as unset. (Without the
`:-`, Claude Code passes the placeholder through literally when the variable
is unset; netbox-mcp ignores that too, and logs a warning naming the
variable.)

Run `netbox-mcp status` to check which token source applies:

```
NetBox URL:   https://netbox.example.com
Token source: pass entry netbox-mcp/netbox.example.com (/home/you/.password-store/netbox-mcp/netbox.example.com.gpg)
Recipients:   ABCD1234EF567890
gpg-agent:    unlocked (passphrase cached in gpg-agent)
```

#### Day to day

The server decrypts the token on first use and never prompts (gpg runs with
`--pinentry-mode error`), so it starts even while your key is locked. Until
gpg-agent has your passphrase cached, tool calls fail with a message asking
you to unlock. Your agent relays that, and the next call after you unlock
succeeds without restarting the server. After that first decrypt the token
stays in memory for the life of the server process, so the cache only matters
when a server starts. If NetBox rejects the token, the server re-reads the
entry on the next call, so a token rotated with `netbox-mcp login` is picked
up without a restart.

```sh
netbox-mcp unlock   # caches the passphrase in gpg-agent (prompts if needed)
netbox-mcp status   # token source, recipients, locked/unlocked; never decrypts
netbox-mcp logout   # deletes the encrypted entry
```

#### Config keys

| Config key | Description |
|---|---|
| `pass_entry` | Store entry holding the token (mutually exclusive with `token`) |
| `gpg` | Full path to `gpg`, for MCP clients that launch servers with a minimal `PATH` (e.g. `/opt/homebrew/bin/gpg`); `gpgconf` is looked up next to it |
| `password_store_dir` | Store root; overrides `$PASSWORD_STORE_DIR` (written by `login`) |
| `gnupg_home` | GnuPG home passed to gpg as `--homedir`; overrides `$GNUPGHOME` (written by `login` when one is set) |

`login` records the store and keyring it used. That way a server started by a
GUI client, which doesn't see variables exported only in your shell, reads the
same ones. `~/` is expanded in these values.

---

## Client setup

### Claude Desktop

Add the following to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "netbox": {
      "command": "netbox-mcp",
      "env": {
        "NETBOX_URL": "https://netbox.example.com",
        "NETBOX_TOKEN": "your-api-token"
      }
    }
  }
}
```

Config file location:

| OS | Path |
|---|---|
| macOS | `~/Library/Application Support/Claude/claude_desktop_config.json` |
| Linux | `~/.config/Claude/claude_desktop_config.json` |
| Windows | `%APPDATA%\Claude\claude_desktop_config.json` |

### Claude Code

Register via the CLI:

```sh
claude mcp add --transport stdio \
  --env NETBOX_URL=https://netbox.example.com \
  --env NETBOX_TOKEN=your-api-token \
  netbox -- netbox-mcp
```

With a [GPG-encrypted token](#gpg-encrypted-token-pass), drop the
`--env NETBOX_TOKEN=…` line: an explicit token overrides the encrypted one.

To share the configuration with your team (writes to `.mcp.json`):

```sh
claude mcp add --transport stdio --scope project \
  --env NETBOX_URL=https://netbox.example.com \
  netbox -- netbox-mcp
```

Or commit a `.mcp.json` by hand. Claude Code expands `${VAR}` (and
`${VAR:-default}`) in `env` values from each user's environment, so the token
never lands in the file. Always give a `:-` default: with a bare `${VAR}` and
the variable unset, Claude Code passes the placeholder text through literally.

```json
{
  "mcpServers": {
    "netbox": {
      "command": "netbox-mcp",
      "args": [],
      "env": {
        "NETBOX_URL": "${NETBOX_URL:-https://netbox.example.com}",
        "NETBOX_TOKEN": "${NETBOX_TOKEN:-}"
      }
    }
  }
}
```

---

## Available tools

178 tools spanning ten NetBox API areas:

| Area | Coverage |
|---|---|
| **DCIM** | Sites, racks, devices, interfaces, cables, power, modules, inventory |
| **IPAM** | IP addresses, prefixes, VRFs, VLANs, aggregates, ranges, ASNs, FHRP |
| **Virtualization** | VMs, clusters, VM interfaces, virtual disks |
| **Circuits** | Circuits, providers, circuit types, terminations |
| **VPN** | Tunnels, L2VPNs, IKE/IPSec policies |
| **Wireless** | Wireless LANs, LAN groups, wireless links |
| **Tenancy** | Tenants, tenant groups, contacts, contact roles |
| **Extras** | Tags, config contexts, journal entries, custom fields, webhooks |
| **Core** | Data sources, background jobs, object change log |
| **Users** | Users, groups, API tokens |

Each area provides `list` (with filters) and `get` (by ID) tools. Virtual machines, IP addresses, and tags also have `create`, `update`, and `delete` tools. A meta-tool `netbox_lookup_host` searches both physical devices and VMs by name in a single call.

---

## Development

```sh
make build          # cargo build
make release        # cargo build --release
make test           # unit + launch-path tests (offline)
make test-real-gpg  # credentials tests against the real gpg on PATH
make live-test      # live integration tests against a seeded NetBox
make lint           # secrets scan + clippy (every feature set) + fmt --check
make deny           # cargo-deny: advisories, banned crates, sources, licenses
make msrv           # build + test on the declared rust-version
make smoke          # run the release binary: --version + MCP tools/list
make check          # full offline gate: fmt-check lint test deny release smoke
make clean          # remove build artifacts
make docker-build   # build Docker image (docker or podman)
```

### Testing

Two layers verify different risks — fast offline unit tests, and a feature-gated
live suite that checks fidelity to the real NetBox API. A self-contained,
auto-seeding NetBox stack for the live suite lives in
[`test/netbox-docker/`](test/netbox-docker/) (Podman on macOS):

```sh
cd test/netbox-docker && ./up.sh          # boot + seed a local NetBox
NETBOX_URL=http://localhost:8000 \
NETBOX_TOKEN=0123456789abcdef0123456789abcdef01234567 make live-test
```

See [`docs/testing.md`](docs/testing.md) for the full rationale and how to add
coverage.

---

## License

[GPL-3.0-or-later](LICENSE)
