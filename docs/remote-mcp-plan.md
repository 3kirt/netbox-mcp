# Remote MCP Architecture

This document describes the HTTP transport implementation that enables
netbox-mcp to run as a remote MCP server (i.e. without a locally installed
binary), accessible from Claude.ai web or Claude Code via HTTP.

---

## Overview

The server supports two transports, selected at startup:

- **stdio** (default) — the binary is run as a subprocess by the MCP client.
  `NETBOX_URL` and `NETBOX_TOKEN` are required at startup.
- **HTTP** — the server listens on a TCP port. `NETBOX_URL` is required at
  startup; the NetBox API token is supplied per-request via a bearer token
  header. Start with `--listen :8080`.

---

## Auth strategy

The bearer token passed in each HTTP request is the caller's NetBox API token:

```
Authorization: Bearer <netbox-api-token>
```

Claude Code supports this natively via `--header` when registering an MCP
server. Each MCP session gets its own `NetboxClient` instance initialised with
the token extracted from that request, so concurrent users are fully isolated.

`NETBOX_URL` is a server-side configuration value — the deployed server is
bound to a specific NetBox instance. Callers supply only their token.

---

## Implementation

### `src/main.rs`

Parses `--listen` (optional). When present, runs in HTTP mode via
`src/server/http.rs`; when absent, runs in stdio mode.

In HTTP mode `NETBOX_TOKEN` is not loaded from config or env — it comes from
each request's `Authorization` header. `NETBOX_URL` is still required.

### `src/server/http.rs`

Handles the HTTP server. Key responsibilities:

**Token extraction**

The `rmcp` streamable-HTTP handler calls a `get_service` closure once per new
session. The closure reads the `Authorization` header, strips the `Bearer `
prefix, and constructs a `NetboxClient` for that token. Requests with no or
invalid `Authorization` header return `401`.

**Health endpoints**

- `GET /healthz` — liveness probe; always `200 {"status":"ok","version":"…"}`.
- `GET /readyz` — readiness probe; `200` when the NetBox hostname resolves,
  `503` otherwise. Used by Kubernetes to gate traffic.

**Axum routing**

```
GET  /healthz  → liveness handler
GET  /readyz   → readiness handler
POST /mcp      → rmcp StreamableHttpServer (authenticated)
```

### `src/config.rs`

`Config::resolve_token()` returns an error when the token is absent in stdio
mode. HTTP mode does not call `resolve_token()`.

### Tool modules (`src/tools/`)

Unchanged. All tool functions accept a `&NetboxClient` reference, so they work
identically in both transports.

---

## Registering with Claude Code (HTTP mode)

Once the server is deployed:

```sh
claude mcp add --transport http \
  --header "Authorization: Bearer your-netbox-token" \
  netbox https://your-host/mcp
```

---

## TLS

The server does not terminate TLS. Deploy behind a reverse proxy (nginx,
Caddy, Traefik) or a platform that provides TLS (Fly.io, Railway, etc.). The
`NETBOX_URL` enforces HTTPS to prevent the NetBox token from being sent in
plaintext to NetBox.

---

## Token validation caching (optional optimisation)

The current implementation validates the bearer token on every new session by
attempting a lightweight NetBox API call. For high-concurrency deployments a
short-TTL in-memory cache could reduce round-trips. This is not implemented
yet.

---

## OAuth (future phase)

To appear in Claude.ai's Settings → Connectors, the server needs a full OAuth
2.0 Authorization Code flow. NetBox does not provide this natively. Options:

1. **Token provisioning proxy** — implement a minimal OAuth server that
   presents a login form, calls `/api/users/tokens/provision/` on NetBox,
   and issues a short-lived JWT containing the NetBox token. The MCP server
   validates the JWT as the bearer token.

2. **NetBox OAuth plugin** — if the target NetBox instance has an OAuth plugin
   installed, validate tokens against its introspection endpoint.

3. **External identity provider** — delegate the OAuth flow to an IdP
   (Keycloak, Okta) with NetBox LDAP/SSO integration.
