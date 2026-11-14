# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

### Added

- **GPG-encrypted token storage (pass-compatible)** — set `"pass_entry"` in the config instead of a plaintext `"token"`. The token is GPG-encrypted in a `pass`-compatible store (`~/.password-store` / `$PASSWORD_STORE_DIR`, encrypted to the nearest `.gpg-id`); only `gpg` is required, and `pass show` reads the entry too. Modeled on gitlab-mcp v0.39.0/v0.39.1.
- **Unattended operation with a locked key** — the server decrypts the token on first use and never prompts (`--pinentry-mode error`, 10s timeout), so it starts while the key is locked. Tool calls fail with an actionable "locked, ask the user to run `netbox-mcp unlock`, then retry" error until gpg-agent has the passphrase cached, then succeed without a restart; concurrent first calls run gpg once. Locked/no-key states come from gpg's numeric status codes, never its localized messages.
- **`netbox-mcp login` / `logout` / `unlock` / `status` subcommands** — `login` verifies the token against NetBox (`/api/authentication-check/`, falling back to `/api/status/` on older releases), encrypts it, updates the config at 0600, and removes any plaintext token. Each NetBox instance gets its own entry (`netbox-mcp/<host>`), so logging in to a second instance never overwrites the first one's token, and `--gpg-id` is written to `.gpg-id` only after encryption to that key succeeds. `status` reports the token source and whether the key is unlocked by asking gpg-agent over its Assuan socket, without decrypting anything.
- **Rotated tokens are picked up without a restart** — after NetBox rejects the token (401, or the 403 NetBox uses for an invalid token) the server re-reads the pass entry on the next call. If that re-read fails (e.g. the agent cache has expired), it keeps reporting NetBox's real auth error rather than a misleading "locked".
- **Intel Mac binaries** — releases now include `netbox-mcp-darwin-amd64` alongside linux amd64/arm64 and darwin arm64, and every binary ships with a build-provenance attestation (`gh attestation verify`).
- `"gpg"`, `"password_store_dir"` and `"gnupg_home"` config keys — the gpg binary, store root and keyring, for MCP clients that launch servers with a minimal `PATH` or without shell-only `PASSWORD_STORE_DIR` / `GNUPGHOME` exports. `login` records the store and keyring it used.

### Changed

- **License identifier is now `GPL-3.0-or-later`** — Cargo.toml used the deprecated SPDX id `GPL-3.0`, which cargo-deny flags; the README says the same.
- A config file that sets both `"token"` and `"pass_entry"` is rejected rather than one silently shadowing the other.
- An unexpanded `${NAME}` / `${NAME:-…}` value in an env var or config value now counts as unset, like a blank one, with a warning naming the variable. Claude Code passes a bare `${NETBOX_TOKEN}` through literally when the variable is unset, which would otherwise shadow a `pass_entry` with a bogus token.
- `--config` is accepted after a subcommand as well as before it, and `--version` is available.
- **TLS uses rustls' ring provider instead of aws-lc-rs** — drops `aws-lc-sys` (and with it cmake and a C toolchain for aws-lc, plus quinn and rand) from the build; `deny.toml` bans it from coming back.
- **Toolchain pinned to Rust 1.100.0** (from 1.99.0), with the Docker builder image to match. The minimum supported Rust version is now declared: `rust-version = "1.89"` (measured; 1.88 fails on `uuid`).
- **Linux release binaries are built on ubuntu-22.04** (glibc 2.35 floor, down from the `ubuntu-latest` glibc), and arm64 builds natively instead of through a cross linker.
- Release notes now come from the matching CHANGELOG section instead of being auto-generated.

### Internals

- **Launch-path tests** — `tests/launch_path.rs` spawns the built binary over MCP stdio with a pass-entry config, a fake `gpg`, and wiremock, and checks that NetBox receives the pass token whether `NETBOX_TOKEN` is unset, empty, or a literal placeholder, and that a locked key still starts the server and returns the unlock message. A unit test lints the README's MCP config examples so every `${VAR}` has a `:-` default.
- `make lint` now runs a `secrets-scan` for NetBox v2 (`nbt_…`) tokens in tracked and untracked files.
- All requests go through one `NetboxClient::send()` choke point, so the lazy-auth rules apply to every verb.
- **CI/CD harmonized with gitlab-mcp and proxmox-mcp** — CI only calls `make` targets (`lint`, `test`, `test-real-gpg`, `deny`, `msrv`, `smoke`, `docker-smoke`); `make check` (`fmt-check lint test deny release smoke`) is the pre-release gate. `test-live` is renamed `live-test`.
  - `ci.yml`: least-privilege permissions, PR-only cancellation of superseded runs, tests on Linux **and macOS**, a real-GnuPG job, cargo-deny, MSRV (outside PRs), and a Docker image build that also checks the builder matches `rust-toolchain.toml`.
  - `scheduled.yml` (weekly): cargo-deny, MSRV, lint+test on current stable and beta, real GnuPG on both OSes, and the **live suite against the netbox-docker stack** (`ENGINE=docker test/netbox-docker/up.sh`). A failure opens or comments on a `scheduled-failure` issue. The weekly cron moved here from `ci.yml`.
  - `release.yml`: a verify job (tag = Cargo.toml version, tagged commit on main, non-empty CHANGELOG section), then the full CI suite, then a 4-target build where each binary is smoke-tested before upload (`scripts/smoke.py`: `--version` plus an MCP initialize + `tools/list` returning all 178 tools), a Docker check, checksums, and provenance attestation. Only the publish job can write.
  - Every action is pinned by commit SHA; Dependabot bumps actions and cargo weekly, grouped.
- **cargo-deny replaces cargo-audit** (`deny.toml`): advisories, banned crates, crates.io-only sources, and an explicit license allow-list.
- **Real-GnuPG tests** (`make test-real-gpg`) pin the status codes the classifier relies on: an unprotected key decrypts, a locked key is `Locked` (code 85), a foreign-key ciphertext is `NoSecretKey` (code 17).
- `LIVE_REQUIRED=1` makes missing live-test credentials a failure instead of a silent skip.
- Fixed an intermittent 10s stall in the gpg timeout unit test on macOS (the hung fake `gpg` now `exec`s its sleep, so the kill ends it).
- `scripts/changelog-section.sh` extracts a version's notes; `test/netbox-docker/up.sh` runs under docker as well as podman. The CHANGELOG drops its `---` separators.

### Documentation

- **Setup guide for GPG-encrypted tokens** — the README walks through creating an encryption key, configuring gpg-agent (cache lifetimes, and when a graphical pinentry is needed), storing the token, and keeping `NETBOX_TOKEN` out of the MCP client config so it doesn't override the encrypted token. It also adds a shared `.mcp.json` example using `${VAR:-}` defaults.
- **README no longer calls the server read-only** — it described "169 read-only tools" that "make no changes", which predated the write tools. It now lists all 178 tools, including create/update/delete for VMs, IP addresses, and tags, and explains that a read-only token keeps the server read-only.
- `docs/testing.md` describes the new launch-path layer and the fake-gpg unit tests; `CLAUDE.md` documents the credentials module, lazy pass auth, the subcommands, and the `${VAR}` rule.

## [0.13.1] - 2026-11-09

### Fixed

- **Security: RUSTSEC-2026-0462 cleared** — `rmcp` 3.9.0 fixes unbounded response-body buffering in its Streamable HTTP client (rated 6.5, medium). netbox-mcp doesn't enable that client, so released binaries were not exposed, but `cargo audit` flagged the lockfile and failed the weekly CI audit. The `rmcp` floor is raised to `3.9` so the vulnerable version can't be resolved again; routine in-range patch updates (`cc`, `itoa`, `smallvec`, `syn`, `uuid`) ride along.

## [0.13.0] - 2026-11-02

### Fixed

- **Security: 12 RustSec advisories cleared** — an in-range `cargo update` (~100 crates) pulls `quinn-proto` 0.11.19 (RUSTSEC-2026-0338..0342, two rated high), `rustls` 0.23.46 (RUSTSEC-2026-0285/0346/0348/0349), `rustls-webpki` 0.103.16 (RUSTSEC-2026-0382), `webpki-root-certs` 1.0.11 (RUSTSEC-2026-0383), and `h2` 0.4.20 (RUSTSEC-2026-0258), plus the `tokio` 1.54.1 unsoundness warning (RUSTSEC-2026-0386) and a yanked `chacha20`. `cargo audit` is clean.
- **Security: HTTPS check could be bypassed** — `enforce_https` prefix-matched `http://localhost` / `http://127.0.0.1`, so look-alike hosts (`http://localhost.evil.com`) and userinfo tricks (`http://127.0.0.1@evil.com/`) passed as local and would have received the token over plain HTTP. The URL is now parsed and judged by its real host: plain HTTP is allowed only to `localhost` or a loopback IP (now including `[::1]`), and scheme matching is case-insensitive.
- **`tools/list` was unparseable for rmcp clients** — the 0.12.0 cache hint used `ttlMs: u64::MAX`, but rmcp's own client deserializes `ttlMs` as `i64` and rejected the whole response (JavaScript clients also lose precision above 2^53). The hint is now one day, which also lets a persisted catalog pick up tools from an upgraded binary; a new test round-trips it through rmcp's parser.
- **Docker image failed to start** — the floating `rust:slim` builder moved to a newer Debian than the `bookworm-slim` runtime, producing a binary that died with `GLIBC_2.38 not found`. The builder is pinned to `rust:1.99.0-slim-bookworm`.

### Changed

- **rmcp 3.1 → 3.8** — the floor is raised to 3.8 (`ServerConfig` needs 3.4+). The deprecated `ServerInfo` alias is replaced by `ServerConfig`, and `list_tools`/`list_prompts` return `std::future::ready(...)` instead of an `async fn` with no `.await`.
- **Non-blocking stdio on Linux** — the server runs over rmcp's `stdio_pipes()` transport, which drives FIFO stdin/stdout through the IO reactor on Linux (as in the Docker image) and behaves exactly like `stdio()` elsewhere.
- **Compact JSON output** — tool results are serialized without pretty-printing; the reader is an LLM, and indentation was pure token cost on every response.
- **Explicit `--config` must exist** — a `--config` path that doesn't exist is now an error instead of being silently ignored (which surfaced later as a misleading "URL not set"). A missing *default* `~/.netbox_mcp.json` is still fine, and `try_exists` now reports I/O errors such as a permission-denied parent directory.
- **Empty env vars fall back to the config file** — a set-but-empty `NETBOX_URL` / `NETBOX_TOKEN` (as MCP clients pass when expanding an unexported `${NETBOX_TOKEN}`) now counts as unset.

### Robustness

- **Client cancellation is honored** — `call_tool` races every tool call against the client's `notifications/cancelled`. rmcp only signals cancellation and never aborts the handler, so a cancelled `fetch_all` walk previously kept paging through NetBox; the call is now dropped immediately along with its in-flight request. rmcp 3.6+ clients cancel abandoned calls automatically.
- **Token kept out of debug output** — the `Authorization` header value is marked sensitive, `Config` has a redacting `Debug` impl, and `RawConfig` no longer derives `Debug`.

### Internals

- **Pinned toolchain and stricter CI** — `rust-toolchain.toml` pins Rust 1.99.0, so a new stable clippy can't break CI or the release gate unannounced. CI's lint job now runs `make lint` (all targets including tests, both feature sets) instead of skipping test code, a new job runs `cargo audit`, and a weekly scheduled run surfaces new advisories without a push. The release workflow uses the pinned toolchain.
- **Rust 1.99 lints** — fixed clippy's new `assert_is_empty` in the unit and live tests, and every env-mutating `unsafe` block in the config tests now carries its own `SAFETY` comment. Lint, the offline suite (108), and the live suite (79, against a seeded NetBox) all pass.

## [0.12.0] - 2026-08-06

### Changed

- **rmcp 2.1 → 3.1 (major)** — bumped to the 3.x line of the MCP Rust SDK. netbox-mcp relies on the `#[tool_handler]`/`#[prompt_handler]` macros rather than hand-rolled `ServerHandler::call_tool`/`read_resource` overrides, so the 3.x change to MRTR-aware `CallToolResponse`/`ReadResourceResponse` return types was fully absorbed by the macro layer — no source changes were required beyond the version bump. `cargo audit` is clean on 3.1.1.

### Added

- **Real SEP-2549 cache hints on `tools/list` and `prompts/list`** — the tool and prompt catalogs are fixed for the life of the process, but the `#[tool_handler]`/`#[prompt_handler]` macro defaults only ever emit `ttl_ms: 0` (spec-compliant but useless, since it never lets a capable client skip a refetch). `list_tools` and `list_prompts` are now hand-written to hint a real TTL (`u64::MAX`) with `Public` scope. `#[prompt_handler]` was dropped in favor of a manual `get_prompt`/`list_prompts` implementation, since it unconditionally overwrites any hand-written override of those two methods; `#[tool_handler]` has no such issue and continues to generate `call_tool`/`get_tool`.

### Internals

- **Dependency refresh** — moved several in-range lockfile entries to their latest compatible versions, including `anyhow` 1.0.103 → 1.0.104, `clap` 4.6.1 → 4.6.6, `schemars` 1.2.1 → 1.2.2, `serde`/`serde_json`, `thiserror` 2.0.18 → 2.0.19, and `tokio` 1.52.3 → 1.53.1. No source changes; lint, the offline suite (101), and the live suite (79, against a seeded NetBox) all pass.

## [0.11.0] - 2026-07-04

### Changed

- **rmcp 1.7 → 2.1 (major)** — bumped to the 2.x line of the MCP Rust SDK. Two API changes reach this crate: `model::Content` was renamed to `ContentBlock` (updated in `json_result`, `tool_error`, and the `delegate_delete!` macro), and the prompt-message role enum was folded into the generic `model::Role`, so the prompt shims now pass `Role::User` instead of `PromptMessageRole::User`. The crate uses neither progress notifications nor the (2.x-deprecated) MCP logging feature, so no other breaking changes applied — a clean direct jump. `cargo audit` is clean on 2.1.0.
- **Clippy now runs at pedantic strictness** — the `pedantic`, `nursery`, and `cargo` lint groups are enabled in `Cargo.toml` under `[lints.clippy]` (with a curated, commented allow-list), so every `cargo clippy` invocation — CI, rust-analyzer, and the release gate — enforces them with no extra flags. Added package `keywords`/`categories` metadata to satisfy `cargo_common_metadata`, a `make check` gate, and documented the setup in `CLAUDE.md`.

### Testing

- **Fail-closed guard tests for tool annotations (+2 tests)** — all 178 tool shims already carry MCP behavior annotations (`readOnlyHint`, `destructiveHint`, `idempotentHint`); two new tests keep it that way. `every_tool_carries_behavior_annotations` panics if any tool ships without annotations (fail closed for newly added tools) and asserts no tool is both read-only and destructive; `annotation_profiles_match_operation` spot-checks the read/create/update/delete profiles against the IPAM IP-address CRUD tools.

### Internals

- **Pedantic-lint cleanup** — fixed the resulting warnings, mostly mechanical (`use_self`, `map_unwrap_or`, `uninlined_format_args`, `semicolon_if_nothing_returned`, redundant closures, `missing_const_for_fn`, manual `let…else`), plus one justified local `allow` on `tool_error`'s uniform `Result` shape. No behavior change; lint, the offline suite (98), and the live suite (79, against a seeded NetBox) all pass.

## [0.10.0] - 2026-06-25

### Fixed

- **Security: remote memory exhaustion in `quinn-proto`** — refreshed `Cargo.lock` to pull `quinn-proto` 0.11.15, resolving [RUSTSEC-2026-0185](https://rustsec.org/advisories/RUSTSEC-2026-0185) (high, CVSS 7.5; unbounded out-of-order stream reassembly), which reached the tree transitively via `reqwest → quinn`. `cargo audit` is now clean.

### Internals

- **Dependency refresh** — moved ~38 in-range lockfile entries to their latest compatible versions, including `rmcp` 1.7.0 → 1.8.0, `anyhow` 1.0.102 → 1.0.103, `rustls` 0.23.40 → 0.23.41, and `hyper`, `chrono`, `regex`, and the `wasm-bindgen` family. No source changes; lint, the offline suite (96), and the live suite (79, against a seeded NetBox) all pass.

## [0.9.0] - 2026-06-12

### Changed

- **Tool descriptions standardized and corrected** — all 84 list tools now use a uniform `List X. Filters: …` description that enumerates their actual filter parameters (previously three competing styles). The pass corrected nine descriptions whose filter lists had drifted from the params structs: journal entries advertised non-existent `assigned_object_type`/`assigned_object_id` (actual: `created_by`, `kind`), tags claimed `slug` (actual: `name`, `color`), ASNs claimed `site` (actual: `asn`, `rir`, `tenant`), custom fields and export templates said `object_type` (actual: `content_types`), VPN tunnels listed `group` (actual: `name`, `encapsulation`, `tag`), and devices, VMs, and IP addresses were missing real filters (`cluster_id`, `platform`, `vrf_id`/`dns_name`).

### Fixed

- **`lookup_host` error truncation** — the two `netbox_lookup_host` error paths formatted failures with plain `Display`, bypassing the 300-byte truncation every other tool applies to NetBox API error bodies. They now route through `to_tool_message()`, so a large 4xx response can no longer flood the client's context.
- **`make test-live` silently skipped the live suite** — the config env-var tests `remove_var` `NETBOX_URL`/`NETBOX_TOKEN` mid-run; in the unfiltered `--test-threads=1` run they executed before the live module, so every live test returned early through `skip_unless_live!` while still reporting `ok`. The target is now scoped to `live::` (offline tests are already covered by `make test`), and a credentialed run executes all 79 live tests.

### Internals

- **Paging invariant built in one place** — extracted `inject_paging()` (plus a `response_count()` reader); both `clean_page_response` and `paginate`'s `fetch_all` branch now construct the `{has_more, next_offset}` contract through it instead of duplicating the field surgery. Also dropped a dead re-collect in `paginate`.
- **`delegate_list!` collapsed into `delegate_write!`** — the two macro bodies were identical except for the failure-message verb; list shims now expand through the generic macro with the verb fixed to "listing".
- **Shared `sent_body()` test helper** — replaces the six copies of the received-requests/find-method/parse-body block in the virtualization, IPAM, and extras write tests; `vms_list` now uses the existing `VMS_PATH` constant instead of repeating the path literal.

## [0.8.0] - 2026-06-09

### Added

- **Write support — Extras tags** — added `netbox_extras_tags_create`, `_update`, and `_delete`, following the established write pattern. Create requires `name` and `slug`; optional `color` (6-digit hex), `description`, `object_types` (content-type labels like `["dcim.device"]`), and `weight`. Delete carries the MCP `destructive_hint` (removing a tag detaches it from every tagged object).

### Testing

- **Live coverage — tag write lifecycle (+1 test)** — added a self-cleaning `tag_create_update_delete_lifecycle` to `live/extras.rs` that creates a run-unique tag, recolors it via PATCH, confirms persistence through the get path, then deletes it and asserts the follow-up get 404s. Leaves the seed clean.

## [0.7.0] - 2026-06-08

### Added

- **Write support — Virtualization VMs (first mutating tools)** — added `netbox_virtualization_vms_create`, `_update`, and `_delete`, backed by new `post()`/`patch()`/`delete()` verbs on the client. This is the initial read/write test case; other objects remain read-only. Writes are always registered (no read-only gate) — a NetBox token without write permission simply receives `403`, surfaced as a tool error. Foreign keys are passed as numeric IDs (resolve names via the `*_list` tools); create/update return the slimmed object, delete returns a confirmation. Delete carries the MCP `destructive_hint`.
- **Write support — IPAM IP addresses** — added `netbox_ipam_ip_addresses_create`, `_update`, and `_delete`, following the VM write pattern. Create requires `address` (CIDR); optional `status`, `role`, `vrf`, `tenant`, `dns_name`, `description`, `comments`, `nat_inside`, `tags`, plus `assigned_object_type`/`assigned_object_id` to attach the IP to an interface.

### Internals

- **`BodyBuilder` for write request bodies** — write payloads are now assembled with a fluent `BodyBuilder` (`.req` for required fields, `.opt` to include a field only when `Some`), the write-side counterpart to `QueryBuilder`. Replaces the earlier per-domain `insert_opt` helper, so VM and IP-address writes build their bodies the same way and PATCH stays a partial update.
- **`CommonListParams` extraction + shared test client** — the near-universal `q` + `ordering` + pagination fields are now flattened from a single `CommonListParams` (driven by `qb.run_common(...)`) instead of being re-declared on every list-params struct; endpoints without a `q` filter keep their fields inline and call `qb.run(...)`. Unit tests share one `test_support::mock_client` helper rather than redefining it per module.

### Testing

- **Live coverage — IP-address write lifecycle (+1 test)** — added a self-cleaning `ip_address_create_update_delete_lifecycle` to `live/ipam.rs` that creates a throwaway TEST-NET-3 (`192.0.2.0/24`) address, flips its status via PATCH, confirms persistence through the get path, then deletes it and asserts the follow-up get 404s. Leaves the seed clean.
- **Live coverage — VM write lifecycle (+1 test)** — added a self-cleaning `vm_create_update_delete_lifecycle` to `live/virtualization.rs` that creates a VM in a seeded cluster, flips its status via PATCH, confirms the change persisted through the get path, then deletes it and asserts the follow-up get 404s. It leaves the instance as it found it, so the seed stays clean. Requires a write-enabled token (a read-only token makes `vm_create` 403). The live-suite module doc no longer claims the server is read-only.
- **Live coverage — Core, Users, pagination, and error paths (+12 tests)** — added `live/core.rs` (object changes: list, get, type filter, `diff_only`), `live/users.rs` (users, tokens), `live/pagination.rs` (first-page math, walking every page, `fetch_all` against the real `list_all()` path), and `live/errors.rs` (404 on an unknown id, 400 on an invalid filter value). These need no seed changes — the changelog, admin user, and minted token already exist.
- **Live coverage — Circuits, VPN, Wireless (+19 tests, 76 total)** — extended `scripts/seed_data.py` to populate all three domains (circuit providers/types/circuits/terminations, VPN tunnel group/tunnels/terminations/IKE+IPSec policies/L2VPN, wireless LAN group/LANs/link) and added `live/circuits.rs`, `live/vpn.rs`, `live/wireless.rs`. Terminations use the NetBox 4.x generic `scope` (`termination_type`/`_id`); the wireless link joins two `ieee802.11ac` interfaces. Seed remains idempotent (termination/link creates use filter-first `get_or_create`).

## [0.6.0] - 2026-06-07

### Testing

- **Live integration test layer** — added a feature-gated suite (`live-tests`) under `src/tools/live/` that runs the real domain functions against a seeded NetBox, applies the same `slim_value` transform as the rmcp boundary, and asserts filter behavior plus the universal invariants. Covers DCIM (devices, interfaces, sites, regions, racks, manufacturers, device types, device roles, platforms, locations), IPAM (IP addresses, prefixes, VRFs, services, aggregates, ASNs, RIRs, VLANs, VLAN groups), Virtualization (VMs, clusters, cluster types, VM interfaces), Tenancy (tenants, contacts, contact roles), Extras (tags), and `lookup_host` — 45 tests. Skips cleanly without `NETBOX_URL`/`NETBOX_TOKEN`. Run with `make test-live`; see `docs/testing.md`.
- **Self-contained test NetBox stack** — `test/netbox-docker/` provides a trimmed, Podman-friendly NetBox deployment that boots, mints a v1 API token, and seeds itself (`scripts/seed_data.py`) as part of `up`. One-shot `netbox-token` and `netbox-seed` compose services make `./up.sh` produce a ready-to-test instance with no manual steps. Vendored from netbox-community/netbox-docker under Apache-2.0 (see `test/netbox-docker/LICENSE` and `NOTICE`).
- **Harness invariant mirrors `slim` exactly** — `assert_clean` now flags an un-slimmed choice field only when `label` is a string (matching `slim_value`'s rule) and sources its stripped-key list from `slim::STRIP_KEYS`, so the test invariant can't drift from what the slimmer actually removes.

### Fixed

- **Seed script for NetBox 4.x scope fields** — `scripts/seed_data.py` set the deprecated `site` field on prefixes and clusters, which NetBox 4.x ignores in favor of the generic `scope` (`scope_type`/`scope_id`). Objects had no site association, so `site=` filters returned nothing. Now sets `scope_type: dcim.site` / `scope_id`, which also restores VM-by-site filtering (a VM's site derives from its cluster's scope).

## [0.5.0] - 2026-06-07

### Internals

- **`make lint` now covers test code** — changed `cargo clippy` invocation to `--all-targets` so test modules are linted alongside production code. Fixed 14 `needless_borrows_for_generic_args` violations this surfaced in the test suite.
- **`QueryBuilder::run()`** — new method that issues the paginated GET directly, collapsing the identical 8-line `paginate(...)` tail that every `*_list` function repeated into a single call. Net −608 lines across all domain modules.
- **`resolve_named_id` shared core** — `resolve_device_id` and `resolve_vm_id` now delegate to a single private helper, eliminating the duplicated count/ambiguous/not-found logic that existed in both resolver pairs.
- **`lookup_host` extract closure** — replaced the two copy-pasted `match` arms (total/results extraction) with a shared `extract` closure.

## [0.4.0] - 2026-05-28

### Changed

- **TLS root store** — `reqwest` is now built with the `rustls` feature only, dropping the bundled `webpki-roots` Mozilla CA list. Certificate verification is delegated to `rustls-platform-verifier`, which uses the operating system's native certificate store. The OS-managed roots stay current with system updates and respect enterprise PKI configurations, eliminating the maintenance window where a stale bundled root list could shadow a fresh OS update.

### Internals

- **rmcp 1.5 → 1.7** — upgraded the MCP server crate to its current minor release. No behavioural changes in the tool surface; the bump pulls in upstream protocol fixes and macro improvements.
- **CI: Helm chart job removed** from the release workflow, completing the deploy/ cleanup that began in v0.3.0.

## [0.2.0] - 2026-05-16

### Added

- **`netbox_lookup_host` meta-tool** — searches `dcim/devices/` and `virtualization/virtual-machines/` in parallel by name (case-insensitive partial match) and returns a merged result with `total_matches` and `has_more`. Eliminates the need to know which endpoint to query first.
- **`diff_only` mode on `object_changes_list`** — when `diff_only: true`, replaces `prechange_data`/`postchange_data` with only the keys that differ between the two snapshots. Create and delete records (where one side is null) are left untouched. Significantly reduces response size for update-heavy audit logs.
- **`name__ic` filter on `devices_list`** — case-insensitive partial match (e.g. `"web01"` matches `"web01.example.com"`). The existing `name` filter remains for exact matches.
- **`cluster_id` filter on `devices_list`** — enumerate physical nodes belonging to a cluster.
- **IP address filters** — `ip_addresses_list` gains `device`, `device_id`, `virtual_machine`, `virtual_machine_id`, `dns_name`, and `parent` (prefix containment). Enables device-scoped and subnet-scoped IP lookups without multi-hop queries.
- **`mgmt_only` filter on `interfaces_list`** — pass `mgmt_only: true` to return only management interfaces for a device.
- **Response slimming** — `slim_value()` now strips additional noise from every response:
  - `local_context_data` (duplicates resolved `config_context`)
  - `primary_ip` alias (duplicates `primary_ip4` / `primary_ip6`)
  - `display_url` (web UI deep-links)
  - `_depth` (tree-rendering hint)
  - `label` from choice-field objects (`{"value": "active", "label": "Active"}` → `{"value": "active"}`)
  - Embedded tag objects collapsed to `{id, name, slug}`, dropping `color`, `weight`, `tagged_items`, etc.

### Robustness

- HTTP connect timeout (10 s) and total request timeout (30 s) added to `NetboxClient`.
- `list_all()` capped at 200 pages — returns `PageLimitExceeded` rather than looping indefinitely on large datasets.
- `resolve_device_id` / `resolve_vm_id` return a typed `Ambiguous` error (with match count) when more than one record shares a name, instead of silently returning the first.
- NetBox API error bodies truncated to 300 characters before forwarding to the client, preventing large filter-echo responses from flooding the assistant's context.
- Typed error variants — `NotFound`, `Ambiguous`, and `PageLimitExceeded` replace the former catch-all generic error string.
- `enforce_https()` rewritten to require `https://` explicitly; `http://localhost` and `http://127.0.0.1` are the only plain-HTTP exceptions.
- Negative `offset` values clamped to 0 in `finalize_params` before the outbound NetBox request is made.
- Empty or whitespace-only `name` argument to `netbox_lookup_host` is rejected immediately with a clear error.
- `require_bearer` 401 response now includes `WWW-Authenticate: Bearer` header.
- `NetboxClient::new()` returns `anyhow::Result` instead of panicking on a token containing non-ASCII characters.

### Internals

- `PaginationParams` struct flattened into every `*ListParams` via `#[serde(flatten)]`, removing ~655 lines of duplicated `limit`/`offset`/`fetch_all` field declarations.
- `resolve_device_id_or` / `resolve_vm_id_or` helpers replace 13 repeated name-to-ID resolution patterns across `dcim.rs` and `virtualization.rs`.
- `slim_value` and related constants extracted from `tools/mod.rs` into `tools/slim.rs`.
- `finalize_params` and `clean_page_response` split out of `paginate()` to be unit-testable without an HTTP client.
- 82 tests (up from 34) including wiremock-based pipeline tests covering the full `client.list()` → `paginate()` → `slim_value()` chain, `apply_change_diff` branches, `to_tool_message` UTF-8 boundary safety, `enforce_https` edge cases, `resolve_device_id`/`resolve_vm_id` ambiguous/not-found paths, and the `list_all` MAX_PAGES guard.

## [0.1.2] - 2026-05-14

### Fixed

- Removed invalid `within` IP address filter from `ip_addresses_list` (NetBox does not support this parameter; `parent` is the correct containment filter).
- `netbox_lookup_host` now reports the true NetBox total counts in `total_matches` and correctly sets `has_more` when results are truncated.
- `list_all` response now always includes `has_more: false` and `next_offset` equal to the total count, matching the shape of regular paginated responses.
- `next_offset` is always present in paginated responses (previously absent on the last page).

## [0.1.1] - 2026-05-13

### Added

- `fetch_all` parameter on all list tools — set `fetch_all: true` to retrieve all matching results automatically across as many pages as needed.
- Per-session HTTP Bearer token authentication — the token supplied in the `Authorization: Bearer` header is forwarded to NetBox; no server-side token is configured or compared.
- `tag` filter on all list tools.
- Device-name and VM-name resolution on interface and related list tools — supply a name string directly instead of looking up the numeric ID first.
- `http://localhost` and `http://127.0.0.1` bypasses for the HTTPS enforcement check (for local development).

### Changed

- All 84 list endpoint handler bodies collapsed to a single `delegate_list!` macro call; all 84 get handlers collapsed to `delegate_get!`. `QueryBuilder` and `paginate()` shared across every domain module.
- VRF filter on IPAM tools corrected: parameter is the route distinguisher (`rd`, e.g. `65000:100`), not the VRF name.

### Fixed

- Env-mutating config tests serialized via a module-level `Mutex` to prevent `NETBOX_URL`/`NETBOX_TOKEN` leakage across parallel test threads.

## [0.1.0] - 2026-05-13

Complete rewrite of netbox-mcp in Rust, replacing the previous Go implementation.

### Changed

- **Language**: Go → Rust (single binary, no runtime dependencies).
- **Transports**: stdio (token at startup) and HTTP (per-session Bearer token via `initialize()`), both driven by the `rmcp` crate.
- **Response format**: all list responses return `{ count, has_more, next_offset, results }` — no raw `next`/`previous` NetBox URLs exposed to the client.
- **Config**: `~/.netbox_mcp.json` with `NETBOX_URL` / `NETBOX_TOKEN` env var overrides; HTTP (non-localhost) URLs rejected.
- All tools from the Go era re-implemented across ten domain modules: `dcim`, `ipam`, `virtualization`, `circuits`, `vpn`, `wireless`, `tenancy`, `extras`, `core`, `users`.

## [0.0.11] - 2026-03-16

### Added

- Helm chart published to GHCR (`oci://ghcr.io/3kirt/charts`) on every release tag.
- Helm values for toggling ingress, metrics service, and Prometheus `ServiceMonitor`.

## [0.0.10] - 2026-03-15

### Added

- Prometheus `ServiceMonitor` for Prometheus Operator environments.
- Cluster-internal metrics `Service` exposing the `/metrics` endpoint.
- Documentation for health endpoints, structured logging, and graceful shutdown behavior.

## [0.0.9] - 2026-03-15

### Fixed

- NetBox response body now explicitly closed in the token verifier, resolving a potential resource leak flagged by the linter.

## [0.0.8] - 2026-03-15

### Added

- Dockerfile and `docker-build` / `docker-run` Makefile targets.
- HTTP transport with Bearer-token authentication for remote MCP deployments.
- Example Kubernetes manifests for remote MCP deployment.

## [0.0.7] - 2026-03-15

### Added

- MCP prompts for common NetBox workflows (site inventory, device report, prefix utilization, tenant summary).
- 46 additional tools covering gaps across all domains.

### Fixed

- Server test updated to account for all registered tools.

## [0.0.6] - 2026-03-15

### Changed

- ~62 individual GET handlers consolidated into a shared `addGetTool` helper, removing significant repetition.

## [0.0.5] - 2026-03-13

### Added

- `extras`, `vpn`, `wireless`, `core`, and `users` domain modules.
- `q` (free-text search) and `ordering` filters added to all list tools.
- Additional resources in existing modules (Phase C/D gap-fill).

### Fixed

- Two code-quality issues identified in review.

## [0.0.4] - 2026-03-13

### Added

- GET tools for interfaces and cables.

## [0.0.3] - 2026-03-13

### Added

- Security hardening: HTTPS enforcement, config file permission check, pagination limit cap.
- Pinned GitHub Actions versions.

## [0.0.2] - 2026-03-13

### Added

- GitHub Actions CI workflow and GoReleaser-based release workflow.

## [0.0.1] - 2026-03-12

Initial release (Go implementation).

### Added

- MCP server with stdio transport.
- DCIM and IPAM list/get tools covering devices, sites, racks, interfaces, cables, prefixes, IP addresses, VLANs, and VRFs.
- `circuits` and `tenancy` domain modules.
- `~/.netbox_mcp.json` configuration with `NETBOX_URL` / `NETBOX_TOKEN` env var overrides.
- Unit tests for config loading, helpers, and tool registration.
