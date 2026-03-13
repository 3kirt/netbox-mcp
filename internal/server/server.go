// Package server constructs the MCP server and registers all NetBox tools.
package server

import (
	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"

	"github.com/3kirt/netbox-mcp/internal/tools"
)

// Register adds all NetBox tools to s.
func Register(s *mcp.Server, client *netbox.APIClient) {
	tools.RegisterCircuits(s, client)
	tools.RegisterDCIM(s, client)
	tools.RegisterIPAM(s, client)
	tools.RegisterTenancy(s, client)
	tools.RegisterVirtualization(s, client)
}
