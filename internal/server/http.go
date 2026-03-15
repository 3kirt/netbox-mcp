package server

import (
	"context"
	"fmt"
	"log/slog"
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
		Handler: requestLogger(mux),
		// ReadHeaderTimeout guards against slowloris attacks.
		// WriteTimeout is intentionally omitted: SSE streams used by the
		// streamable HTTP transport are long-lived and would be killed by it.
		ReadHeaderTimeout: 10 * time.Second,
		IdleTimeout:       120 * time.Second,
	}

	slog.Info("netbox-mcp HTTP server listening", "addr", addr+"/mcp")
	return srv.ListenAndServe()
}

// statusRecorder wraps http.ResponseWriter to capture the response status code
// for logging. It forwards Flush so that SSE streams work through the middleware.
type statusRecorder struct {
	http.ResponseWriter
	status int
}

func (r *statusRecorder) WriteHeader(status int) {
	r.status = status
	r.ResponseWriter.WriteHeader(status)
}

func (r *statusRecorder) Flush() {
	if f, ok := r.ResponseWriter.(http.Flusher); ok {
		f.Flush()
	}
}

// requestLogger is an HTTP middleware that logs each request as a structured
// log line once the handler returns. For short-lived requests this records
// outcome and latency; for long-lived SSE streams it records session duration.
func requestLogger(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		start := time.Now()
		rec := &statusRecorder{ResponseWriter: w, status: http.StatusOK}
		next.ServeHTTP(rec, r)
		slog.Info("request", //nolint:gosec // G706: values are JSON-encoded by slog.NewJSONHandler; no injection risk
			"method", r.Method,
			"path", r.URL.Path,
			"status", rec.status,
			"duration_ms", time.Since(start).Milliseconds(),
			"remote_addr", r.RemoteAddr,
		)
	})
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
			slog.Error("token validation request failed", "error", err)
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
			slog.Error("token missing from session context")
			return nil
		}
		client := netbox.NewAPIClientFor(netboxURL, token)
		s := mcp.NewServer(&mcp.Implementation{
			Name:    "netbox-mcp",
			Version: version,
		}, nil)
		Register(s, client)
		slog.Info("session created", "remote_addr", r.RemoteAddr) //nolint:gosec // G706: r.RemoteAddr is a validated host:port from net; JSON-encoded by handler
		return s
	}
}
