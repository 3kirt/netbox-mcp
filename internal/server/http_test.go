package server

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/auth"
)

// --- healthzHandler ---

func TestHealthzHandler_returnsOKWithVersion(t *testing.T) {
	handler := healthzHandler("v1.2.3")
	req := httptest.NewRequest(http.MethodGet, "/healthz", nil)
	rec := httptest.NewRecorder()
	handler.ServeHTTP(rec, req)

	if rec.Code != http.StatusOK {
		t.Errorf("status = %d, want 200", rec.Code)
	}
	if ct := rec.Header().Get("Content-Type"); ct != "application/json" {
		t.Errorf("Content-Type = %q, want application/json", ct)
	}
	var body struct {
		Status  string `json:"status"`
		Version string `json:"version"`
	}
	if err := json.Unmarshal(rec.Body.Bytes(), &body); err != nil {
		t.Fatalf("body is not valid JSON: %v\nbody: %s", err, rec.Body)
	}
	if body.Status != "ok" {
		t.Errorf("status = %q, want ok", body.Status)
	}
	if body.Version != "v1.2.3" {
		t.Errorf("version = %q, want v1.2.3", body.Version)
	}
}

// --- readyzHandler ---

func TestReadyzHandler_resolvableHostname(t *testing.T) {
	handler := readyzHandler("localhost")
	req := httptest.NewRequest(http.MethodGet, "/readyz", nil)
	rec := httptest.NewRecorder()
	handler.ServeHTTP(rec, req)

	if rec.Code != http.StatusOK {
		t.Errorf("status = %d, want 200", rec.Code)
	}
}

func TestReadyzHandler_unresolvableHostname(t *testing.T) {
	handler := readyzHandler("this.hostname.definitely.does.not.exist.invalid")
	req := httptest.NewRequest(http.MethodGet, "/readyz", nil)
	rec := httptest.NewRecorder()
	handler.ServeHTTP(rec, req)

	if rec.Code != http.StatusServiceUnavailable {
		t.Errorf("status = %d, want 503", rec.Code)
	}
	var body struct {
		Status string `json:"status"`
		Error  string `json:"error"`
	}
	if err := json.Unmarshal(rec.Body.Bytes(), &body); err != nil {
		t.Fatalf("body is not valid JSON: %v\nbody: %s", err, rec.Body)
	}
	if body.Status != "error" {
		t.Errorf("status = %q, want error", body.Status)
	}
	if body.Error == "" {
		t.Error("error field should be non-empty on DNS failure")
	}
}

// --- statusRecorder ---

func TestStatusRecorder_capturesWriteHeader(t *testing.T) {
	inner := httptest.NewRecorder()
	rec := &statusRecorder{ResponseWriter: inner, status: http.StatusOK}

	rec.WriteHeader(http.StatusUnauthorized)

	if rec.status != http.StatusUnauthorized {
		t.Errorf("rec.status = %d, want %d", rec.status, http.StatusUnauthorized)
	}
	if inner.Code != http.StatusUnauthorized {
		t.Errorf("inner.Code = %d, want %d", inner.Code, http.StatusUnauthorized)
	}
}

// --- requestLogger ---

func TestRequestLogger_callsNextHandlerAndReturnsStatus(t *testing.T) {
	inner := http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusNotFound)
	})
	req := httptest.NewRequest(http.MethodGet, "/mcp", nil)
	rec := httptest.NewRecorder()
	requestLogger(inner).ServeHTTP(rec, req)

	if rec.Code != http.StatusNotFound {
		t.Errorf("status = %d, want 404", rec.Code)
	}
}

func TestRequestLogger_skipsStatusRecorderForProbes(t *testing.T) {
	// For probe paths the inner handler receives the plain ResponseWriter (not
	// a statusRecorder). We verify this indirectly by checking that the inner
	// handler is invoked and its response is passed through unchanged.
	for _, path := range []string{"/healthz", "/readyz"} {
		t.Run(path, func(t *testing.T) {
			called := false
			inner := http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
				called = true
				w.WriteHeader(http.StatusOK)
			})
			req := httptest.NewRequest(http.MethodGet, path, nil)
			rec := httptest.NewRecorder()
			requestLogger(inner).ServeHTTP(rec, req)

			if !called {
				t.Error("inner handler was not called")
			}
			if rec.Code != http.StatusOK {
				t.Errorf("status = %d, want 200", rec.Code)
			}
		})
	}
}

// --- makeTokenVerifier ---

// fakeNetBox starts a test HTTP server that responds to any request with the
// given status code and (for 2xx) an empty paginated list body.
func fakeNetBox(t *testing.T, statusCode int) *httptest.Server {
	t.Helper()
	return httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, _ *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(statusCode)
		if statusCode >= 200 && statusCode < 300 {
			_, _ = fmt.Fprint(w, `{"count":0,"next":null,"previous":null,"results":[]}`)
		}
	}))
}

func TestMakeTokenVerifier_validToken(t *testing.T) {
	srv := fakeNetBox(t, http.StatusOK)
	defer srv.Close()

	verifier := makeTokenVerifier(srv.URL)
	info, err := verifier(context.Background(), "valid-token", nil)
	if err != nil {
		t.Fatalf("want nil error, got %v", err)
	}
	if info == nil {
		t.Fatal("want non-nil TokenInfo")
	}
	if got, ok := info.Extra[netboxTokenKey].(string); !ok || got != "valid-token" {
		t.Errorf("token in Extra = %v, want %q", info.Extra[netboxTokenKey], "valid-token")
	}
}

func TestMakeTokenVerifier_unauthorized(t *testing.T) {
	srv := fakeNetBox(t, http.StatusUnauthorized)
	defer srv.Close()

	verifier := makeTokenVerifier(srv.URL)
	_, err := verifier(context.Background(), "bad-token", nil)
	if !errors.Is(err, auth.ErrInvalidToken) {
		t.Errorf("want ErrInvalidToken, got %v", err)
	}
}

func TestMakeTokenVerifier_forbidden(t *testing.T) {
	srv := fakeNetBox(t, http.StatusForbidden)
	defer srv.Close()

	verifier := makeTokenVerifier(srv.URL)
	_, err := verifier(context.Background(), "bad-token", nil)
	if !errors.Is(err, auth.ErrInvalidToken) {
		t.Errorf("want ErrInvalidToken, got %v", err)
	}
}

func TestMakeTokenVerifier_networkError(t *testing.T) {
	// Start and immediately close a server so we have an address that refuses connections.
	srv := httptest.NewServer(http.HandlerFunc(func(http.ResponseWriter, *http.Request) {}))
	addr := srv.URL
	srv.Close()

	verifier := makeTokenVerifier(addr)
	_, err := verifier(context.Background(), "any-token", nil)
	if !errors.Is(err, auth.ErrInvalidToken) {
		t.Errorf("want ErrInvalidToken on connection refused, got %v", err)
	}
}
