// Command netbox-mcp is a Model Context Protocol server that exposes NetBox
// infrastructure data as MCP tools for use with Claude and other MCP clients.
// By default it communicates over stdio (local subprocess mode). Pass --listen
// to run an HTTP server instead.
package main

import (
	"context"
	"flag"
	"log"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"

	"github.com/3kirt/netbox-mcp/internal/config"
	"github.com/3kirt/netbox-mcp/internal/server"
)

var version = "dev"

func main() {
	var configPath string
	var listenAddr string
	flag.StringVar(&configPath, "config", "", "path to JSON config file (default: ~/.netbox_mcp.json)")
	flag.StringVar(&listenAddr, "listen", "", "address to listen on for HTTP transport (e.g. :8080)")
	flag.Parse()

	cfg, err := config.Load(configPath)
	if err != nil {
		log.Fatal(err)
	}

	url, err := cfg.ResolveURL()
	if err != nil {
		log.Fatal(err)
	}

	// HTTP mode: each session authenticates with its own bearer token.
	// No server-side NetBox token is needed.
	if listenAddr != "" {
		if err := server.RunHTTP(listenAddr, url, version); err != nil {
			log.Fatal(err)
		}
		return
	}

	// Stdio mode: a single server-side token is required.
	token, err := cfg.ResolveToken()
	if err != nil {
		log.Fatal(err)
	}

	client := netbox.NewAPIClientFor(url, token)

	s := mcp.NewServer(&mcp.Implementation{
		Name:    "netbox-mcp",
		Version: version,
	}, nil)

	server.Register(s, client)

	if err := s.Run(context.Background(), &mcp.StdioTransport{}); err != nil {
		log.Printf("server error: %v", err)
	}
}
