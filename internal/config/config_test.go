package config_test

import (
	"os"
	"path/filepath"
	"runtime"
	"testing"

	"github.com/3kirt/netbox-mcp/internal/config"
)

func TestLoad_missingFileReturnsEmpty(t *testing.T) {
	cfg, err := config.Load(filepath.Join(t.TempDir(), "does-not-exist.json"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if cfg.URL != "" || cfg.Token != "" {
		t.Fatalf("expected empty config, got %+v", cfg)
	}
}

func TestLoad_validFile(t *testing.T) {
	f := writeFile(t, `{"url":"https://netbox.example.com","token":"abc123"}`)
	cfg, err := config.Load(f)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if cfg.URL != "https://netbox.example.com" {
		t.Errorf("URL: got %q, want %q", cfg.URL, "https://netbox.example.com")
	}
	if cfg.Token != "abc123" {
		t.Errorf("Token: got %q, want %q", cfg.Token, "abc123")
	}
}

func TestLoad_malformedJSON(t *testing.T) {
	f := writeFile(t, `{not valid json}`)
	_, err := config.Load(f)
	if err == nil {
		t.Fatal("expected error for malformed JSON, got nil")
	}
}

func TestLoad_worldReadableFileReturnsError(t *testing.T) {
	if runtime.GOOS == "windows" {
		t.Skip("file permission checks not enforced on Windows")
	}
	f := filepath.Join(t.TempDir(), "config.json")
	//nolint:gosec // intentionally world-readable: this test verifies that Load rejects such files
	if err := os.WriteFile(f, []byte(`{"url":"https://netbox.example.com","token":"abc"}`), 0o644); err != nil {
		t.Fatalf("writing temp file: %v", err)
	}
	_, err := config.Load(f)
	if err == nil {
		t.Fatal("expected error for world-readable config file, got nil")
	}
}

func TestResolveURL_fromFile(t *testing.T) {
	cfg := &config.Config{URL: "https://netbox.example.com"}
	got, err := cfg.ResolveURL()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got != "https://netbox.example.com" {
		t.Errorf("got %q", got)
	}
}

func TestResolveURL_envOverridesFile(t *testing.T) {
	t.Setenv("NETBOX_URL", "https://override.example.com")
	cfg := &config.Config{URL: "https://netbox.example.com"}
	got, err := cfg.ResolveURL()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got != "https://override.example.com" {
		t.Errorf("got %q", got)
	}
}

func TestResolveURL_missingReturnsError(t *testing.T) {
	t.Setenv("NETBOX_URL", "")
	cfg := &config.Config{}
	_, err := cfg.ResolveURL()
	if err == nil {
		t.Fatal("expected error when URL is absent, got nil")
	}
}

func TestResolveURL_httpReturnsError(t *testing.T) {
	t.Setenv("NETBOX_URL", "")
	cfg := &config.Config{URL: "http://netbox.example.com"}
	_, err := cfg.ResolveURL()
	if err == nil {
		t.Fatal("expected error for HTTP URL, got nil")
	}
}

func TestResolveToken_fromFile(t *testing.T) {
	cfg := &config.Config{Token: "file-token"}
	got, err := cfg.ResolveToken()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got != "file-token" {
		t.Errorf("got %q", got)
	}
}

func TestResolveToken_envOverridesFile(t *testing.T) {
	t.Setenv("NETBOX_TOKEN", "env-token")
	cfg := &config.Config{Token: "file-token"}
	got, err := cfg.ResolveToken()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if got != "env-token" {
		t.Errorf("got %q", got)
	}
}

func TestResolveToken_missingReturnsError(t *testing.T) {
	t.Setenv("NETBOX_TOKEN", "")
	cfg := &config.Config{}
	_, err := cfg.ResolveToken()
	if err == nil {
		t.Fatal("expected error when token is absent, got nil")
	}
}

// writeFile writes content to a temp file and returns its path.
func writeFile(t *testing.T, content string) string {
	t.Helper()
	f := filepath.Join(t.TempDir(), "config.json")
	if err := os.WriteFile(f, []byte(content), 0o600); err != nil {
		t.Fatalf("writing temp file: %v", err)
	}
	return f
}
