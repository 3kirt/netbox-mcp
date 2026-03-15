// Command netbox-mcp is a Model Context Protocol server that exposes NetBox
// infrastructure data as MCP tools for use with Claude and other MCP clients.
// By default it communicates over stdio (local subprocess mode). Pass --listen
// to run an HTTP server instead.
package main

import (
	"context"
	"flag"
	"log/slog"
	"os"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"

	"github.com/3kirt/netbox-mcp/internal/config"
	"github.com/3kirt/netbox-mcp/internal/server"
)

var version = "dev"

func main() {
	slog.SetDefault(slog.New(slog.NewJSONHandler(os.Stderr, nil)))

	var configPath string
	var listenAddr string
	flag.StringVar(&configPath, "config", "", "path to JSON config file (default: ~/.netbox_mcp.json)")
	flag.StringVar(&listenAddr, "listen", "", "address to listen on for HTTP transport (e.g. :8080)")
	flag.Parse()

	cfg, err := config.Load(configPath)
	if err != nil {
		slog.Error("failed to load config", "error", err)
		os.Exit(1)
	}

	url, err := cfg.ResolveURL()
	if err != nil {
		slog.Error("failed to resolve NetBox URL", "error", err)
		os.Exit(1)
	}

	// HTTP mode: each session authenticates with its own bearer token.
	// No server-side NetBox token is needed.
	if listenAddr != "" {
		if err := server.RunHTTP(listenAddr, url, version); err != nil {
			slog.Error("HTTP server error", "error", err)
			os.Exit(1)
		}
		return
	}

	// Stdio mode: a single server-side token is required.
	token, err := cfg.ResolveToken()
	if err != nil {
		slog.Error("failed to resolve NetBox token", "error", err)
		os.Exit(1)
	}

	client := netbox.NewAPIClientFor(url, token)

	s := mcp.NewServer(&mcp.Implementation{
		Name:    "netbox-mcp",
		Version: version,
	}, nil)

	server.Register(s, client)

	if err := s.Run(context.Background(), &mcp.StdioTransport{}); err != nil {
		slog.Error("server error", "error", err)
	}
}
