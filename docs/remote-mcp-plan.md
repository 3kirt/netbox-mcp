# Remote MCP Implementation Plan

This document describes the changes needed to add an HTTP transport to
netbox-mcp, enabling it to be used as a remote MCP server from Claude.ai
(web, iOS, Android) without a locally installed binary.

---

## Background

The MCP go-sdk (`github.com/modelcontextprotocol/go-sdk`) already ships
everything needed:

- **`mcp.StreamableHTTPHandler`** — a full `http.Handler` implementing the
  [Streamable HTTP transport](https://modelcontextprotocol.io/specification/2025-06-18/basic/transports#streamable-http)
  (spec 2025-06-18), with session management, SSE streaming, and reconnection support.
- **`auth.RequireBearerToken`** — HTTP middleware that extracts and validates
  an `Authorization: Bearer <token>` header, stores the result in the request
  context, and rejects unauthenticated requests with 401.
- **`auth.TokenVerifier`** — the callback signature we implement to validate
  tokens against NetBox.

No new dependencies are needed. The SDK is already at v1.4.0 in go.mod.

---

## Auth strategy

NetBox does not ship a built-in OAuth server, so a full OAuth flow (as
required to appear in Claude's Settings → Connectors) is deferred to a later
phase. For the initial implementation, authentication uses a **bearer token**
equal to the user's NetBox API token:

```
Authorization: Bearer <netbox-api-token>
```

Claude Code supports this natively via `--header` when registering an MCP
server. Tooling that requires OAuth (Claude.ai web/mobile connectors) will
need the later OAuth phase.

The NETBOX_URL remains a server-side configuration value — the deployed
server points at a specific NetBox instance. Users provide only their token.

---

## Changes required

### 1. `cmd/netbox-mcp/main.go`

Add a `--listen` flag. When it is set the server runs in HTTP mode; when it
is absent the existing stdio mode is used unchanged.

```go
var listenAddr string
flag.StringVar(&listenAddr, "listen", "", "address to listen on for HTTP mode (e.g. :8080)")
flag.Parse()

if listenAddr != "" {
    runHTTP(listenAddr, url)   // url from cfg.ResolveURL()
} else {
    runStdio(url, token)       // existing flow
}
```

In HTTP mode the NetBox token is **not** loaded from config or environment —
it comes from each request's `Authorization` header. `NETBOX_URL` is still
required.

### 2. `internal/config/config.go`

Make `ResolveToken` return `("", nil)` when neither env var nor config field
is set (rather than an error), so the caller can decide whether a missing
token is fatal based on the transport mode.

```go
func (c *Config) ResolveToken() (string, error) {
    if v := os.Getenv("NETBOX_TOKEN"); v != "" {
        return v, nil
    }
    return c.Token, nil   // empty string is not an error
}
```

`main.go` becomes responsible for enforcing that a token is present in stdio
mode and absent in HTTP mode.

### 3. New file: `internal/server/http.go`

Contains the HTTP server setup. Key responsibilities:

**Token verifier**

Validates a bearer token by making a lightweight authenticated request to
NetBox. A 2xx response means the token is valid; any other response (401,
403, etc.) means it is not.

```go
func makeTokenVerifier(netboxURL string) auth.TokenVerifier {
    return func(ctx context.Context, token string, _ *http.Request) (*auth.TokenInfo, error) {
        client := netbox.NewAPIClientFor(netboxURL, token)
        // Use any cheap, authenticated endpoint; /api/users/tokens/ is suitable.
        _, _, err := client.UsersAPI.UsersTokensList(ctx).Limit(1).Execute()
        if err != nil {
            return nil, auth.ErrInvalidToken
        }
        return &auth.TokenInfo{}, nil
    }
}
```

**Server factory**

`StreamableHTTPHandler` calls its `getServer` function once per new session.
The factory extracts the validated token from context (placed there by the
bearer-token middleware) and constructs a fresh `mcp.Server` + NetBox client
for that session.

```go
func makeGetServer(netboxURL string) func(*http.Request) *mcp.Server {
    return func(r *http.Request) *mcp.Server {
        tokenInfo := auth.TokenInfoFromContext(r.Context())
        if tokenInfo == nil {
            return nil   // rejected before reaching here by middleware
        }
        token := tokenInfo.Extra["netbox_token"].(string)
        client := netbox.NewAPIClientFor(netboxURL, token)
        s := mcp.NewServer(&mcp.Implementation{Name: "netbox-mcp", Version: version}, nil)
        server.Register(s, client)
        return s
    }
}
```

To pass the raw token through to `getServer`, the `TokenVerifier` must store
it in `TokenInfo.Extra`:

```go
return &auth.TokenInfo{Extra: map[string]any{"netbox_token": token}}, nil
```

**HTTP server wiring**

```go
func RunHTTP(listenAddr, netboxURL, version string) error {
    verifier := makeTokenVerifier(netboxURL)
    authMiddleware := auth.RequireBearerToken(verifier, nil)

    mcpHandler := mcp.NewStreamableHTTPHandler(
        makeGetServer(netboxURL, version),
        &mcp.StreamableHTTPOptions{
            SessionTimeout: 30 * time.Minute,
        },
    )

    mux := http.NewServeMux()
    mux.Handle("/mcp", authMiddleware(mcpHandler))

    log.Printf("netbox-mcp listening on %s", listenAddr)
    return http.ListenAndServe(listenAddr, mux)
}
```

### 4. `internal/server/server.go`

No changes needed. `Register(s, client)` already accepts any client.

### 5. `internal/tools/` (all tool files)

No changes needed.

### 6. `Makefile`

No changes needed for the initial implementation.

---

## Token validation caching (optional optimisation)

Validating the bearer token against NetBox on every new session adds one
round-trip. For a server with many simultaneous users this could be improved
by caching valid tokens with a short TTL (e.g. 5 minutes):

```go
type cachedVerifier struct {
    inner auth.TokenVerifier
    mu    sync.Mutex
    cache map[string]cachedEntry
}
```

This is an optimisation, not required for correctness, and can be added
incrementally.

---

## Registering the server with Claude Code (HTTP mode)

Once deployed:

```sh
claude mcp add --transport http \
  --header "Authorization: Bearer your-netbox-token" \
  netbox https://your-host/mcp
```

---

## TLS

The existing `ResolveURL` already enforces HTTPS for the NetBox URL. The
HTTP listener itself does not enforce TLS — that is expected to be handled
by a reverse proxy (nginx, Caddy, etc.) or a platform like Fly.io or
Railway. The server should document that it must not be exposed without TLS
in production.

---

## OAuth (future phase)

To appear in Claude.ai's Settings → Connectors, the server needs a full
OAuth 2.0 Authorization Code flow. NetBox does not provide this natively.
Options for a future phase:

1. **Token provisioning proxy**: Implement a minimal OAuth server that
   presents a login form, calls `/api/users/tokens/provision/` on NetBox
   with the user's credentials, and issues a short-lived JWT containing the
   NetBox token. The MCP server validates the JWT as the bearer token.

2. **NetBox OAuth plugin**: If the target NetBox instance has an OAuth plugin
   installed, configure the MCP server to validate tokens against its
   introspection endpoint.

3. **External identity provider**: Use an IdP (Keycloak, Okta) with NetBox
   LDAP/SSO integration, delegating the OAuth flow to the IdP.

Option 1 is the most self-contained and requires no changes to the NetBox
instance.

---

## Implementation order

1. Add `--listen` flag and HTTP listener with **no auth** (useful for local
   testing behind a firewall).
2. Add bearer-token middleware and `TokenVerifier`.
3. Update README and docs.
4. (Future) Add token validation caching.
5. (Future) Implement OAuth phase.

---

## Files changed summary

| File | Change |
|---|---|
| `cmd/netbox-mcp/main.go` | Add `--listen` flag; branch on stdio vs HTTP mode |
| `internal/config/config.go` | Make empty token non-fatal |
| `internal/server/http.go` | **New** — HTTP server, token verifier, server factory |
| `README.md` | Update Remote MCP section; add `--header` registration example |
| `docs/remote-mcp-plan.md` | This document |

Everything else stays unchanged.
