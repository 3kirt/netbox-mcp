// Package config handles loading the netbox-mcp JSON configuration file and
// resolving the NetBox URL and API token. The token is read from the
// NETBOX_TOKEN environment variable first, falling back to the config file.
// The URL is read from the NETBOX_URL environment variable first, falling
// back to the config file.
package config

import (
	"encoding/json"
	"errors"
	"fmt"
	"net/url"
	"os"
	"path/filepath"
	"runtime"
)

const defaultConfigFile = ".netbox_mcp.json"

// Config holds the values loaded from the JSON configuration file.
type Config struct {
	URL   string `json:"url"`
	Token string `json:"token"`
}

// Load reads and parses the JSON config file at path. If path is empty the
// default location (~/.netbox_mcp.json) is used. A missing file is not an
// error; an empty Config is returned instead.
func Load(path string) (*Config, error) {
	if path == "" {
		home, err := os.UserHomeDir()
		if err != nil {
			return nil, fmt.Errorf("could not determine home directory: %w", err)
		}
		path = filepath.Join(home, defaultConfigFile)
	}

	// Reject world-readable config files before loading credentials from them.
	if runtime.GOOS != "windows" {
		if info, err := os.Stat(path); err == nil && info.Mode().Perm()&0o004 != 0 {
			return nil, fmt.Errorf(
				"config file %s is world-readable (mode %04o): run 'chmod 600 %s' to protect your API token",
				path, info.Mode().Perm(), path,
			)
		}
	}

	data, err := os.ReadFile(path) //nolint:gosec // path is user-supplied config file location
	if errors.Is(err, os.ErrNotExist) {
		return &Config{}, nil
	}
	if err != nil {
		return nil, fmt.Errorf("reading config file %s: %w", path, err)
	}

	var cfg Config
	if err := json.Unmarshal(data, &cfg); err != nil {
		return nil, fmt.Errorf("parsing config file %s: %w", path, err)
	}

	return &cfg, nil
}

// ResolveURL returns the NetBox base URL, preferring the NETBOX_URL
// environment variable over the url field in the config file. An error is
// returned if no URL is found in either location, or if the URL does not use
// HTTPS (which would send the API token in plaintext).
func (c *Config) ResolveURL() (string, error) {
	rawURL := os.Getenv("NETBOX_URL")
	if rawURL == "" {
		rawURL = c.URL
	}
	if rawURL == "" {
		return "", errors.New(
			"no NetBox URL found: set the NETBOX_URL environment variable " +
				"or add a \"url\" key to your config file",
		)
	}
	u, err := url.Parse(rawURL)
	if err != nil {
		return "", fmt.Errorf("invalid NetBox URL %q: %w", rawURL, err)
	}
	if u.Scheme != "https" {
		return "", fmt.Errorf(
			"NetBox URL must use HTTPS (got scheme %q): the API token would be sent in plaintext",
			u.Scheme,
		)
	}
	return rawURL, nil
}

// ResolveToken returns the API token, preferring the NETBOX_TOKEN environment
// variable over the token field in the config file. An error is returned if
// no token is found in either location.
func (c *Config) ResolveToken() (string, error) {
	if token := os.Getenv("NETBOX_TOKEN"); token != "" {
		return token, nil
	}
	if c.Token != "" {
		return c.Token, nil
	}
	return "", errors.New(
		"no API token found: set the NETBOX_TOKEN environment variable " +
			"or add a \"token\" key to your config file",
	)
}
