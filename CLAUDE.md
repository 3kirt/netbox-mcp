# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
make build       # cargo build --release
make test        # cargo test --all
make lint        # cargo clippy --all-targets -- -D warnings && cargo fmt --check
make install     # cargo install --path . (installs to ~/.cargo/bin)
make clean       # remove build artifacts

# Run a single test
cargo test <test_name>               # e.g. cargo test parses_valid_json
cargo test -p netbox-mcp <test_name> # same, scoped to the crate

# Docker
NETBOX_URL=https://netbox.example.com make docker-build
```

Formatting and lint must be clean before every commit. Run `cargo fmt` to fix formatting; `cargo clippy --all-targets -- -D warnings` to check for warnings treated as errors (the `--all-targets` flag lints test code too).

## Architecture

The codebase is a single Rust binary with four modules:

```
src/
  main.rs         — CLI (clap), startup logging
  config.rs       — Config loading: ~/.netbox_mcp.json + env var override
  client.rs       — Thin reqwest wrapper: list(), get(), list_all() (reads); post(), patch(), delete() (writes)
  tools/
    mod.rs        — NetboxMcpServer struct, paginate(), clean_page_response(), all tool shims, unit + integration tests
    slim.rs       — slim_value(), STRIP_KEYS, TAG_KEEP_KEYS (response slimming logic)
    dcim.rs       — DCIM domain functions + param structs
    ipam.rs       — IPAM domain functions + param structs
    virtualization.rs / circuits.rs / vpn.rs / wireless.rs
    tenancy.rs / extras.rs / core.rs / users.rs
```

**Transport:** stdio only. The binary runs as a subprocess managed by the MCP client; credentials are read from config at startup via `NetboxMcpServer::new(url, token)`.

**Tool registration flow:**
The `#[tool_router]` macro on `impl NetboxMcpServer` in `tools/mod.rs` generates the MCP tool registry. Each shim is a thin `#[tool]`-annotated async method that calls a domain function (e.g. `dcim::devices_list`) via a `delegate_*!` macro: `delegate_list!`/`delegate_get!` for reads, `delegate_write!` (create/update) and `delegate_delete!` for writes.

**Adding a new tool:**
1. Add a `*Params` struct and a domain function to the relevant `tools/<domain>.rs`.
2. Add the `#[tool]` shim to `tools/mod.rs` using the matching `delegate_*!` macro.
3. No routing table to update — the `#[tool_router]` macro handles registration.

**Write tools (create/update/delete):**
A limited set of mutating tools exists (virtualization VMs are the first — see `tools/virtualization.rs`). Writes are always registered; there is **no read-only gate** — safety relies on the NetBox API token's permissions (a read-only token simply gets `403`). Conventions: foreign-key fields are taken as numeric NetBox IDs (callers use the `*_list` tools to resolve names first); bodies are built field-by-field with `insert_opt` so unset options are omitted (PATCH stays a partial update); create/update return the object through `json_result` (so writes are slimmed like reads), delete returns a text confirmation. Write shims carry `read_only_hint = false` and, for delete, `destructive_hint = true`.

**Pagination pattern:**
All list tools share `paginate()` in `tools/mod.rs`. It calls `client.list()` (single page) or `client.list_all()` (all pages) based on `fetch_all`. Responses go through `clean_page_response()` which strips `next`/`previous` URLs and injects `{ has_more, next_offset }`. Default page size is 50; max is 1000.

**Response slimming:**
`slim_value()` is applied to every response before it is sent to the client. It:
- Strips `null`-valued fields from every object.
- Strips keys in `STRIP_KEYS`: `local_context_data`, `primary_ip`, `display_url`, `_depth`.
- Strips `label` from NetBox choice-field objects `{"value": …, "label": …}` — `label` is always just a human-readable capitalisation of `value`.
- Collapses embedded `tags` arrays to `{id, name, slug}` only, dropping `color`, `weight`, `tagged_items`, etc. (the top-level tags-list endpoint is unaffected — it uses `results`, not `tags`).

This cuts typical NetBox payloads by 50–70%.

**Config resolution order:** `NETBOX_URL`/`NETBOX_TOKEN` env vars → `~/.netbox_mcp.json` → error. HTTP (non-localhost) URLs are rejected to prevent token exposure.

**`QueryBuilder`** is the canonical way to build filter params: `.opt(key, Option<T>)` for scalar filters, `.many(key, Option<Vec<String>>)` for multi-value filters. Never push to the params vec directly.

## Testing

Two layers verify different risks; see `docs/testing.md` for the full rationale.

**Unit tests** (default `cargo test --all`, offline):
- `src/config.rs` — config loading, env override, HTTPS enforcement.
- `src/tools/mod.rs` — `slim_value()`, `clean_page_response()`, `PaginationParams`, and wiremock-based pipeline integration tests that fire requests through the full `paginate()` path against a mock server.

**Live integration tests** (`src/tools/live/`, gated behind the `live-tests` cargo feature) run the real domain functions against a seeded NetBox, apply the same `slim_value` transform as the rmcp boundary, and assert behavior (filters honored) plus the universal invariants — the contract fidelity unit tests structurally cannot reach. They're shape-only (no exact counts) so they survive seed drift, and `skip_unless_live!` returns early when `NETBOX_URL`/`NETBOX_TOKEN` are absent. Run with `make test-live` against a seeded instance — the self-contained, auto-seeding stack in `test/netbox-docker/` (`./up.sh`) is the easiest way to get one.

`docs/testing-protocol.md` is the seed-data reference and the manual checklist for areas the live suite has not reached yet.

## Response invariants

Every list response must have `{ count, has_more, next_offset, results }`. `has_more` is `true` iff `next_offset < count`. `next_offset` is always present (equals `count` when `has_more` is `false`). No `null` values, no `local_context_data`, no `primary_ip` alias, no `display_url`, no `_depth`, no bare `label` on choice fields, no `next`/`previous` URL fields.
