# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
make build          # cargo build --locked (debug)
make release        # cargo build --release --locked (TARGET=<triple> to cross-build)
make test           # cargo test --all --locked (unit + tests/launch_path.rs)
make test-real-gpg  # #[ignore]d credentials tests against the real gpg (REAL_GPG=1)
make live-test      # live suite against a seeded NetBox (LIVE_REQUIRED=1: fail instead of skip)
make lint           # secrets-scan + clippy --all-targets -D warnings (every feature set) + fmt --check
make deny           # cargo-deny: advisories, bans (aws-lc), sources, licenses (deny.toml)
make msrv           # cargo +<rust-version> test --all --locked
make smoke          # scripts/smoke.py on BIN (default target/release/netbox-mcp)
make check          # full offline gate: fmt-check lint test deny release smoke
make docker-smoke   # build the image, check its builder = rust-toolchain.toml, run --version
make install        # cargo install --locked --path . (installs to ~/.cargo/bin)
make clean          # remove build artifacts

# Run a single test
cargo test <test_name>               # e.g. cargo test parses_valid_json
cargo test -p netbox-mcp <test_name> # same, scoped to the crate

# Docker (docker if present, else podman; override with ENGINE=)
make docker-build
```

Formatting and lint must be clean before every commit. Run `cargo fmt` to fix formatting; `cargo clippy --all-targets -- -D warnings` to check for warnings treated as errors (the `--all-targets` flag lints test code too).

Clippy runs at pedantic strictness: the `pedantic`, `nursery`, and `cargo` lint groups are enabled in `Cargo.toml` under `[lints.clippy]` (with a curated set of per-lint `allow`s documented there), so *every* `cargo clippy` invocation — CI, rust-analyzer, and the release gate — enforces them with no extra flags. `make lint` runs `secrets-scan` (fails on a NetBox v2 `nbt_<key>.<secret>` token in any tracked or untracked file — test fixtures must not use that prefix), then clippy over all targets and both feature sets; `make check` runs the full offline gate (`fmt-check lint test deny release smoke`). It never includes the live suite.

**The Makefile is the single source of truth for CI.** Every workflow job calls a `make` target, so add or change a check in the Makefile, not in YAML. The workflows (all actions pinned by commit SHA, bumped by Dependabot):
- `.github/workflows/ci.yml` — push/PR to main, dispatch, and `workflow_call` (release reuses it): lint, test on ubuntu + macOS, real-gpg, deny, msrv (not on PRs), docker-smoke.
- `.github/workflows/scheduled.yml` — weekly: deny, msrv, lint+test on stable and beta, real-gpg on both OSes, and the live suite against `ENGINE=docker test/netbox-docker/up.sh` with `LIVE_REQUIRED=1`. A failure opens or comments on a `scheduled-failure` issue.
- `.github/workflows/release.yml` — on a `v*` tag: verify (tag = Cargo.toml version, commit on main, non-empty CHANGELOG section via `scripts/changelog-section.sh`), the whole CI suite, a 4-target build (linux amd64/arm64 on ubuntu-22.04 for a glibc 2.35 floor, darwin arm64/amd64) with `make smoke` on each artifact, docker-smoke, then checksums, provenance attestation, and a release body from the CHANGELOG.

**Toolchain and MSRV.** `rust-toolchain.toml` pins an exact Rust version (bump it deliberately, together with the Dockerfile's `rust:<ver>-slim-bookworm` builder — `make docker-smoke` fails if they differ, and fix any new lints in the same change). `rust-version` in Cargo.toml is the measured MSRV (1.89: 1.88 fails on `uuid`); `make msrv` checks it. TLS is rustls with the **ring** provider, installed in `client::http_client()` (reqwest uses `rustls-no-provider` and panics without one); `deny.toml` bans aws-lc-sys/aws-lc-rs.

**`scripts/smoke.py`** asserts the binary's `--version` and that MCP `tools/list` returns exactly `EXPECTED_TOOLS` (178). Adding or removing a tool means updating that constant **and** `EXPECTED_TOOLS` in `src/tools/mod.rs` (`tool_router_exposes_every_tool`) together.

## Architecture

The codebase is a single Rust binary:

```
src/
  main.rs         — CLI (clap): subcommand dispatch (no subcommand = serve stdio), startup logging
  commands.rs     — login / logout / unlock / status subcommands (GPG-encrypted token management)
  config.rs       — Config loading: ~/.netbox_mcp.json + env var override; TokenSource; ${VAR} placeholder handling
  credentials/
    mod.rs        — PassStore / PassEntry / PassError: pass-compatible store, gpg invocation, status-fd classification
    agent.rs      — minimal Assuan client for gpg-agent (KEYINFO), used only by `status` (Unix)
  client.rs       — Thin reqwest wrapper: list(), get(), list_all() (reads); post(), patch(), delete() (writes); lazy pass-token auth
  tools/
    mod.rs        — NetboxMcpServer struct, paginate(), clean_page_response(), all tool shims, unit + integration tests
    slim.rs       — slim_value(), STRIP_KEYS, TAG_KEEP_KEYS (response slimming logic)
    dcim.rs       — DCIM domain functions + param structs
    ipam.rs       — IPAM domain functions + param structs
    virtualization.rs / circuits.rs / vpn.rs / wireless.rs
    tenancy.rs / extras.rs / core.rs / users.rs
```

**Transport:** stdio only. The binary runs as a subprocess managed by the MCP client; `main.rs` resolves the token *source* at startup (`Config::resolve_token()` → `TokenSource::{Env, File, Pass}`) and builds the `NetboxClient` from it, which `NetboxMcpServer::new(client)` stores. A static token (`NETBOX_TOKEN` or the config's plaintext `token`) goes into the client's default headers. A `pass_entry` token is decrypted lazily on the first request (see "Lazy pass auth" below), so the server starts even while the GPG key is locked. Don't add startup-time token checks that would undo that.

**Tool registration flow:**
The `#[tool_router]` macro on `impl NetboxMcpServer` in `tools/mod.rs` generates the MCP tool registry. Each shim is a thin `#[tool]`-annotated async method that calls a domain function (e.g. `dcim::devices_list`) via a `delegate_*!` macro: `delegate_list!`/`delegate_get!` for reads, `delegate_write!` (create/update) and `delegate_delete!` for writes.

**Adding a new tool:**
1. Add a `*Params` struct and a domain function to the relevant `tools/<domain>.rs`.
2. Add the `#[tool]` shim to `tools/mod.rs` using the matching `delegate_*!` macro.
3. No routing table to update — the `#[tool_router]` macro handles registration.

**Write tools (create/update/delete):**
A limited set of mutating tools exists (virtualization VMs are the first — see `tools/virtualization.rs`). Writes are always registered; there is **no read-only gate** — safety relies on the NetBox API token's permissions (a read-only token simply gets `403`). Conventions: foreign-key fields are taken as numeric NetBox IDs (callers use the `*_list` tools to resolve names first); bodies are built with `BodyBuilder` (`.req` for required fields, `.opt` for optional ones — unset options are omitted, so PATCH stays a partial update; the write-side counterpart to `QueryBuilder`); create/update return the object through `json_result` (so writes are slimmed like reads), delete returns a text confirmation. Write shims carry `read_only_hint = false` and, for delete, `destructive_hint = true`.

**Pagination pattern:**
All list tools share `paginate()` in `tools/mod.rs`. It calls `client.list()` (single page) or `client.list_all()` (all pages) based on `fetch_all`. Responses go through `clean_page_response()` which strips `next`/`previous` URLs and injects `{ has_more, next_offset }`. Default page size is 50; max is 1000.

**Response slimming:**
`slim_value()` is applied to every response before it is sent to the client. It:
- Strips `null`-valued fields from every object.
- Strips keys in `STRIP_KEYS`: `local_context_data`, `primary_ip`, `display_url`, `_depth`.
- Strips `label` from NetBox choice-field objects `{"value": …, "label": …}` — `label` is always just a human-readable capitalisation of `value`.
- Collapses embedded `tags` arrays to `{id, name, slug}` only, dropping `color`, `weight`, `tagged_items`, etc. (the top-level tags-list endpoint is unaffected — it uses `results`, not `tags`).

This cuts typical NetBox payloads by 50–70%.

**Config resolution order:** `NETBOX_URL`/`NETBOX_TOKEN` env vars → `~/.netbox_mcp.json` (`url`, plus either a plaintext `token` or a `pass_entry` — setting both is an error) → error. HTTP (non-loopback) URLs are rejected to prevent token exposure. Every env var and config value goes through one "is this set?" rule (`non_blank` / `config::env_var`): blank **and** an unexpanded `${NAME}` / `${NAME:-…}` placeholder count as unset (with a warning naming the variable), because Claude Code passes a bare `${NETBOX_TOKEN}` through literally when the variable is unset. Every MCP config example in the README must use `"${VAR:-}"`; a unit test lints them.

**`src/credentials/`** — GPG-encrypted token storage in a `pass`-compatible store (config `password_store_dir`, else `$PASSWORD_STORE_DIR`, else `~/.password-store`; GnuPG home likewise from config `gnupg_home`, else `$GNUPGHOME`, passed to gpg/gpgconf as `--homedir`; `gpg` config key for a full binary path, with `gpgconf` looked up next to it. `login` records the store and keyring, because GUI-launched MCP clients don't inherit shell-only exports). Layout: `<entry>.gpg` files plus `.gpg-id` recipient lists, nearest-ancestor lookup like `pass`. Only the `gpg` binary is required, not `pass`. `PassEntry::read_token()` decrypts with `--batch --pinentry-mode error --status-fd 2` under a 10s timeout, so **it never prompts**: a passphrase gpg-agent hasn't cached fails immediately as `PassError::Locked`, whose message tells the MCP client to ask the user to run `netbox-mcp unlock` and retry. Outcomes are classified from the **numeric gpg-error codes** in `[GNUPG:] ERROR` status lines (low 16 bits: 85 `NO_PIN_ENTRY` / 99 `CANCELED` → locked, 17 `NO_SECKEY`), never from gpg's localized human text, and success requires `DECRYPTION_OKAY` as well as exit 0 (verified against real GnuPG 2.4 and 2.5). `agent.rs` (Unix only) is a minimal Assuan client for gpg-agent's socket (`KEYINFO <keygrip>` → cached / protection / disk-vs-card), used **only** by `netbox-mcp status`. It is deliberately not a gate on reads, because "not cached" ≠ "can't decrypt" (passphrase-less and smartcard keys report uncached yet decrypt fine). Tests use `credentials::test_support::fake_gpg()`: one shared fake `gpg` script, written once to avoid ETXTBSY races, that *sources* the entry file in decrypt mode, so each test's "ciphertext" is a shell snippet emitting the canned status lines it needs.

**Lazy pass auth in `client.rs`** — `NetboxClient::with_pass_entry()` builds a client whose `Authorization: Token …` header is added per request by `send()` (the single choke point every verb goes through) from a `PassToken`: decrypted on first use under a mutex (concurrent requests run gpg once), cached in memory on success, **never cached on failure** (so the first call after the user unlocks succeeds with no restart). A 401 **or 403** (NetBox answers an invalid/revoked token with 403 `"Invalid v1 token"`, not 401) marks the token stale (only if it is still the current one), and the next request re-reads the entry to pick up a token rotated via `login`. If that re-read fails, e.g. because the agent cache has expired since, the old token is reused so the caller sees NetBox's real auth error rather than a misleading "locked". Decrypt failures surface as `NetboxError::Credentials`. Static-token clients (`NetboxClient::new`) are unchanged. NetBox 4.5+ v2 `nbt_…` tokens work under the `Token` scheme, so the header format is the same for both token versions.

**`src/commands.rs`** — the `login` / `logout` / `unlock` / `status` subcommands (`--config` is global, so it works before or after the subcommand). `login` checks local preconditions (`.gpg-id` or `--gpg-id`) **before** prompting for the token (no-echo via `rpassword`, or `--token-stdin`). It then verifies the token with `GET /api/authentication-check/` (falling back to `/api/status/` on a 404 from an older NetBox) and encrypts it, and writes `--gpg-id` to the store's `.gpg-id` only *after* encryption to that key has succeeded, so a typo can't leave a blocking `.gpg-id` behind. It reuses the config's `pass_entry` only when the target URL matches the config's own `url` (otherwise `netbox-mcp/<host>`, so a second instance never overwrites the first one's token). Finally it atomically rewrites the config at 0600, setting `url`, `pass_entry` and `password_store_dir` (plus `gnupg_home` when one is in effect) and removing any plaintext `token`. `logout` deletes the `.gpg` entry but leaves `pass_entry` in the config. `unlock`/`status` use the config's `pass_entry` directly (`Config::configured_pass_entry`), not `resolve_token()`, so a `NETBOX_TOKEN` exported in the shell can't hide the entry a server started without it is trying to decrypt. User-facing output goes to stderr.

**`QueryBuilder`** is the canonical way to build filter params: `.opt(key, Option<T>)` for scalar filters, `.many(key, Option<Vec<String>>)` for multi-value filters. Never push to the params vec directly. List-params structs flatten `CommonListParams` (the near-universal `q` + `ordering` + pagination) instead of re-declaring those fields, and finish with `qb.run_common(client, path, p.common)`. The handful of endpoints without a `q` filter (e.g. cable terminations, FHRP group assignments) keep `ordering` + `PaginationParams` inline and call `qb.run(...)` instead.

## Testing

Two layers verify different risks; see `docs/testing.md` for the full rationale.

**Unit tests** (default `cargo test --all`, offline):
- `src/config.rs` — config loading, env override, HTTPS enforcement, `${VAR}` placeholder handling, token-source resolution, atomic config rewrite, and a lint of the README's MCP config examples.
- `src/credentials/` — store layout, gpg status classification, timeouts, Assuan exchange (via the shared fake `gpg`).
- `src/client.rs` — lazy pass auth: locked → unlocked without restart, rotation after a rejection, the real 403 surviving a failed re-read, single decrypt under concurrency.
- `src/tools/mod.rs` — `slim_value()`, `clean_page_response()`, `PaginationParams`, and wiremock-based pipeline integration tests that fire requests through the full `paginate()` path against a mock server.

**Live integration tests** (`src/tools/live/`, gated behind the `live-tests` cargo feature) run the real domain functions against a seeded NetBox, apply the same `slim_value` transform as the rmcp boundary, and assert behavior (filters honored) plus the universal invariants — the contract fidelity unit tests structurally cannot reach. They're shape-only (no exact counts) so they survive seed drift, and `skip_unless_live!` returns early when `NETBOX_URL`/`NETBOX_TOKEN` are absent — unless `LIVE_REQUIRED=1`, which turns that into a failure (the scheduled CI job and the release process set it, so a misconfigured run can't pass with zero tests). Run with `make live-test` against a seeded instance — the self-contained, auto-seeding stack in `test/netbox-docker/` (`./up.sh`) is the easiest way to get one.

**Real-gpg tests** (`credentials::tests::real_gpg`, `#[ignore]`d; `make test-real-gpg` sets `REAL_GPG=1`) run against the real `gpg` on PATH with throwaway keyrings under a short `/tmp` path (macOS caps gpg-agent socket paths at 104 bytes): an unprotected key decrypts, a locked key is `Locked` (code 85), a foreign-key ciphertext is `NoSecretKey` (code 17). They catch GnuPG status-code drift that the fake `gpg` can't.

**Launch-path tests** (`tests/launch_path.rs`, part of `cargo test --all`) spawn the *built binary* over MCP stdio, configured only through its environment and a pass-entry config, with a fake `gpg` and wiremock. They check NetBox receives the pass token whether `NETBOX_TOKEN` is unset, empty, or a literal `${NETBOX_TOKEN}` placeholder, and that a locked key still starts the server and returns the unlock message. This is the only layer that sees the environment an MCP client builds; add to it when a bug could live between the client config and the server.

`docs/testing-protocol.md` is the seed-data reference and the manual checklist for areas the live suite has not reached yet.

## Response invariants

Every list response must have `{ count, has_more, next_offset, results }`. `has_more` is `true` iff `next_offset < count`. `next_offset` is always present (equals `count` when `has_more` is `false`). No `null` values, no `local_context_data`, no `primary_ip` alias, no `display_url`, no `_depth`, no bare `label` on choice fields, no `next`/`previous` URL fields.
