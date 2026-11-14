# Testing

This project tests at three layers that verify **different risks**. None
replaces the others; together they cover the whole surface.

| Layer | Lives in | Hits network? | Verifies |
|---|---|---|---|
| **Unit tests** | `#[cfg(test)] mod tests` in `src/` (config, client, credentials, commands, tools) | No (wiremock, a fake `gpg`, or pure logic) | Our code against *our* assumptions — slimming, pagination math, request construction, config parsing, token handling |
| **Launch-path tests** | `tests/launch_path.rs` | No (wiremock + a fake `gpg`) | The *built binary* as an MCP client launches it — environment and config file handling that unit tests can't see |
| **Live integration tests** | `src/tools/live/` (feature-gated) | Yes — a real, seeded NetBox | Fidelity to the *actual* NetBox API — filter param names, response shapes, which fields really need stripping |

## Why these layers

A unit test mocks NetBox, so **the mock is our assumption**. It can prove "given
params X we build request Y and transform response Z correctly," which is
excellent regression protection — but it can never prove NetBox actually accepts
request Y or returns response Z. That fidelity check only a live call can make.

The corollary matters for planning: increasing unit coverage drives down
*regression* risk but has a hard ceiling on *contract* risk. You can reach 100%
unit coverage and still be calling NetBox wrong.

A concrete example only the live layer can catch: `slim_value` strips
`local_context_data`, and the unit tests prove it strips that key — but only
from a payload *we* constructed. The live VM tests prove NetBox genuinely
returns `local_context_data` on virtual machines and that our strip fires
against the real shape. A wiremock test structurally cannot establish that.

## Layer 1 — Unit tests

Run by default with `cargo test --all`. Three flavors:

**Pure-logic tests** — no HTTP. `src/tools/mod.rs` covers `slim_value()`,
`clean_page_response()`, and `clamp_limit`/`PaginationParams`; `src/config.rs`
covers config loading, the env override, HTTPS enforcement, token-source
resolution, unexpanded `${VAR}` placeholders, and the atomic config rewrite.
It also lints every MCP config example in the README, so each `${VAR}` has a
`:-` default.

**Fake-gpg tests** — `src/credentials/` and the pass-auth tests in
`src/client.rs` use `credentials::test_support::fake_gpg()`: one shared fake
`gpg` script, written once (so no test executes a file another thread is still
writing — the ETXTBSY race). In decrypt mode it *sources* the entry file, so
each test's "ciphertext" is a shell snippet that emits the canned
`[GNUPG:]` status lines it needs: success, locked, no secret key, a hang (for
the timeout), or garbage. The locked and no-key lines were captured from real
GnuPG 2.4. `src/credentials/agent.rs` tests the Assuan exchange against an
in-memory duplex. Tests that read or change process environment variables
(`NETBOX_*`, `HOME`, …) hold `test_support::lock_env()` for their whole body.

**Wiremock pipeline tests** — stand up an in-process mock server and fire a
request through the full `paginate()` path, asserting the request we send and
how the response is slimmed. See the `pipeline_*` tests in `src/tools/mod.rs`.

## Layer 2 — Launch-path tests

[`tests/launch_path.rs`](../tests/launch_path.rs) runs with `cargo test --all`
(it needs no NetBox). It spawns the **built binary**, speaks MCP to it over
stdio, and configures it only through its environment and a config file — the
way an MCP client launches it. A fake `gpg` and a wiremock NetBox complete the
picture.

Unit tests call functions directly, so they never see the environment a client
builds. gitlab-mcp v0.39.0 shipped a bug that lived exactly there: for
`"GITLAB_TOKEN": "${GITLAB_TOKEN}"` with the variable unset, Claude Code passes
the placeholder through literally, and the server used it as the token instead
of the configured `pass_entry`. These tests assert NetBox receives the
pass-entry token whether `NETBOX_TOKEN` is unset, empty, or a literal
`${NETBOX_TOKEN}`, and that a locked key still lets the server start and
returns the "run `netbox-mcp unlock`" message.

Add a test here when a bug could live between the client's config and the
server: environment variables, argument handling, startup, or anything else a
direct function call skips.

## Layer 3 — Live integration tests

A suite that runs the real domain functions against a live NetBox seeded by
[`scripts/seed_data.py`](../scripts/seed_data.py), then applies the same
`slim_value` transform the rmcp boundary applies — so assertions run on exactly
what an MCP client receives. The suite lives under
[`src/tools/live/`](../src/tools/live/), one module per domain plus a shared
`harness`.

### The read-only adaptation

A write-capable server (e.g. gitlab-mcp) self-seeds and self-cleans by creating
and deleting resources through its own write tools. **netbox-mcp is read-only**,
so it can't. Instead the deterministic fixture is `scripts/seed_data.py`, and
the tests assert **behavior, not exact counts**:

- *Filter correctness* — `site=lon-dc` returns only London devices, `parent=
  10.0.1.0/24` returns only addresses inside that prefix, `mgmt_only=true`
  returns only management interfaces.
- *Universal invariants* — every response is clean (no nulls, no stripped keys,
  no un-slimmed choice fields) and every list response satisfies the pagination
  contract.

Because nothing asserts "exactly 12 devices," the suite survives edits to the
seed script. An empty result where rows are expected fails with a pointer to
re-run the seed, since that is the usual cause.

### How it's wired

- **Cargo feature `live-tests`** (`Cargo.toml`) gates compilation. Default
  `cargo test` never builds or runs these, so the everyday loop stays fast and
  offline.
- **Located inside the `tools` module** (`#[cfg(all(test, feature =
  "live-tests"))] mod live;` in `src/tools/mod.rs`), *not* in a top-level
  `tests/` directory. An external `tests/` crate can only see the public API,
  but the live tests need the private `slim` module, the `pub(crate)`
  `PaginationParams`, and the private `netbox_lookup_host` method to reproduce
  the server's exact output. A child module of `tools` gets that access without
  widening the crate's public surface.
- **One module per domain, plus a shared `harness`.** `live/harness.rs` holds
  `LiveEnv`/`live_env`, the `skip_unless_live!` macro, the param/slim helpers,
  and the cross-domain invariants (`assert_clean`, `assert_page_shape`,
  `assert_nonempty`). Each domain file (`live/dcim.rs`, `live/ipam.rs`, …) does
  `use super::harness::*` and holds that domain's tests.

### Design properties

- **Tests the server's real output path.** Helpers run the domain function *and*
  apply `slim_value` — the same transform `json_result` applies at the rmcp
  boundary — so a stripped/collapsed field is asserted exactly as a client sees
  it.
- **Invariants-as-code.** `assert_clean` and `assert_page_shape` encode the
  universal invariants from [`testing-protocol.md`](testing-protocol.md) as
  reusable assertions instead of prose a human eyeballs.
- **Skips without credentials.** `skip_unless_live!` returns early (printing a
  notice) when `NETBOX_URL`/`NETBOX_TOKEN` are absent, so the feature is safe to
  enable in CI without secrets — supply credentials in a dedicated job to
  actually exercise it.

### Running the live tests

The quickest path is the bundled test stack in
[`test/netbox-docker/`](../test/netbox-docker/), which boots NetBox under Podman
and seeds it automatically (see its [README](../test/netbox-docker/README.md)):

```sh
cd test/netbox-docker && ./up.sh          # boot + mint token + seed
# then, from the repo root:
NETBOX_URL=http://localhost:8000 \
NETBOX_TOKEN=0123456789abcdef0123456789abcdef01234567 \
  make live-test
```

Against any other seeded instance, run the suite directly:

```sh
NETBOX_URL=https://netbox.example.com \
NETBOX_TOKEN=<token> \
  cargo test --locked --features live-tests live:: -- --test-threads=1
```

- Seed the target first with `scripts/seed_data.py` (the test stack does this for
  you); see [`testing-protocol.md`](testing-protocol.md) for the seed reference.
- netbox-mcp sends `Authorization: Token <token>`. NetBox 4.5+ accepts both v1
  tokens and v2 `nbt_…` tokens under that scheme (verified on 4.6). The test
  stack mints a fixed v1 token explicitly because the built-in superuser
  bootstrap only creates v2 tokens.
- The URL must be HTTPS or localhost — the client rejects other plain-HTTP URLs
  to avoid token exposure.
- `--test-threads=1` keeps output and rate-limiting predictable; the tests are
  read-only and independent, so parallel runs are also safe.
- `make live-test` wraps the same command. Set `LIVE_REQUIRED=1` to make missing
  credentials a failure rather than a skip; the scheduled CI live job and the
  release process do, so a misconfigured run can't pass with zero tests.
- In CI, `.github/workflows/scheduled.yml` brings the same stack up with
  `ENGINE=docker test/netbox-docker/up.sh` every week and runs the suite.

## Coverage

The live suite is grown domain-by-domain. Covered today:

- **DCIM** — devices, interfaces, sites, regions, racks, manufacturers, device
  types, device roles, platforms, locations
- **IPAM** — IP addresses, prefixes, VRFs, services, aggregates, ASNs, RIRs,
  VLANs, VLAN groups
- **Virtualization** — VMs, clusters, cluster types, VM interfaces
- **Tenancy** — tenants, contacts, contact roles
- **Extras** — tags
- **Core** — object changes (list, get, type filter, `diff_only`)
- **Users** — users, tokens
- **Circuits** — providers, provider accounts/networks, circuit types, circuits,
  terminations
- **VPN** — tunnel groups, tunnels, tunnel terminations, IKE/IPSec policies,
  L2VPNs
- **Wireless** — LAN groups, wireless LANs, links
- the **`lookup_host`** meta-tool
- **cross-cutting** — pagination (first page, walking every page, `fetch_all`)
  and error paths (404 on unknown id, 400 on an invalid filter value)

Not yet automated and unseeded: the DCIM hardware-detail endpoints (cables,
ports, modules, inventory, …) and Extras beyond tags. Core `jobs` /
`data_sources` need a background job / configured data source.

The manual exploratory protocol in [`testing-protocol.md`](testing-protocol.md)
remains the seed-data reference and the checklist for areas the live suite has
not reached yet.

## Command reference

```sh
make test            # unit + launch-path tests (live excluded)
make test-real-gpg   # credentials tests against the real gpg (REAL_GPG=1)
make live-test       # live suite (needs a seeded instance; LIVE_REQUIRED=1 to fail on skip)
make lint            # clippy over every target and feature set, incl. the live suite
make check           # full offline gate: fmt-check lint test deny release smoke
```

## Adding a new domain to the live layer

1. Add `src/tools/live/<domain>.rs` and declare it in `src/tools/live/mod.rs`.
2. `use super::harness::*` for the shared bits, then write one test per filter /
   get path, asserting behavior (filter honored) plus the universal invariants
   via `assert_page_shape` / `assert_clean`.
3. Prefer behavioral assertions over exact counts so the suite survives seed
   drift; use `assert_nonempty` to turn an unseeded instance into a clear
   failure rather than a silent pass.
