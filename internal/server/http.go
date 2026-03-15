package server

import (
	"context"
	"fmt"
	"log"
	"net/http"
	"time"

	"github.com/modelcontextprotocol/go-sdk/auth"
	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// netboxTokenKey is the key under which the validated NetBox API token is
// stored in auth.TokenInfo.Extra, shared between makeTokenVerifier and
// makeGetServer.
const netboxTokenKey = "netbox_token"

// RunHTTP starts an HTTP MCP server on addr. Each session authenticates with
// its own NetBox API token supplied as an Authorization: Bearer header.
// netboxURL is the base URL of the NetBox instance used for token validation
// and API calls. The MCP endpoint is available at /mcp.
func RunHTTP(addr, netboxURL, version string) error {
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

	srv := &http.Server{
		Addr:    addr,
		Handler: mux,
		// ReadHeaderTimeout guards against slowloris attacks.
		// WriteTimeout is intentionally omitted: SSE streams used by the
		// streamable HTTP transport are long-lived and would be killed by it.
		ReadHeaderTimeout: 10 * time.Second,
		IdleTimeout:       120 * time.Second,
	}

	log.Printf("netbox-mcp HTTP server listening on %s/mcp", addr)
	return srv.ListenAndServe()
}

// makeTokenVerifier returns a TokenVerifier that validates a bearer token by
// calling the NetBox users/tokens endpoint. A 401 or 403 response means the
// token is invalid; any other response (including an empty list) means it is
// valid. Network or unexpected errors are logged server-side and result in a
// denied request to avoid leaking infrastructure details to callers.
func makeTokenVerifier(netboxURL string) auth.TokenVerifier {
	return func(ctx context.Context, token string, _ *http.Request) (*auth.TokenInfo, error) {
		client := netbox.NewAPIClientFor(netboxURL, token)
		_, resp, err := client.UsersAPI.UsersTokensList(ctx).Limit(1).Execute()
		if resp != nil {
			_ = resp.Body.Close()
		}
		if err != nil {
			if resp != nil && (resp.StatusCode == http.StatusUnauthorized || resp.StatusCode == http.StatusForbidden) {
				return nil, fmt.Errorf("%w", auth.ErrInvalidToken)
			}
			// Log the real error server-side; return a generic denial to the
			// caller so internal details are not exposed in the HTTP response.
			log.Printf("netbox-mcp: token validation request failed: %v", err)
			return nil, fmt.Errorf("%w", auth.ErrInvalidToken)
		}
		return &auth.TokenInfo{Extra: map[string]any{netboxTokenKey: token}}, nil
	}
}

// makeGetServer returns a function that creates a new per-session MCP server
// using the NetBox token stored in the request context by the bearer-auth
// middleware.
func makeGetServer(netboxURL, version string) func(*http.Request) *mcp.Server {
	return func(r *http.Request) *mcp.Server {
		tokenInfo := auth.TokenInfoFromContext(r.Context())
		if tokenInfo == nil {
			return nil
		}
		token, ok := tokenInfo.Extra[netboxTokenKey].(string)
		if !ok || token == "" {
			log.Printf("netbox-mcp: internal error: token missing from session context")
			return nil
		}
		client := netbox.NewAPIClientFor(netboxURL, token)
		s := mcp.NewServer(&mcp.Implementation{
			Name:    "netbox-mcp",
			Version: version,
		}, nil)
		Register(s, client)
		return s
	}
}
