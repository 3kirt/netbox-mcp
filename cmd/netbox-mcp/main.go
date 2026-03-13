// Command netbox-mcp is a Model Context Protocol server that exposes NetBox
// infrastructure data as MCP tools for use with Claude and other MCP clients.
// It communicates over stdio and is intended to be run as a local subprocess.
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
	flag.StringVar(&configPath, "config", "", "path to JSON config file (default: ~/.netbox_mcp.json)")
	flag.Parse()

	cfg, err := config.Load(configPath)
	if err != nil {
		log.Fatal(err)
	}

	url, err := cfg.ResolveURL()
	if err != nil {
		log.Fatal(err)
	}

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
