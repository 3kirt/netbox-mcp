package tools

import (
	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterTenancy adds tenancy tools to s.
func RegisterTenancy(s *mcp.Server, client *netbox.APIClient) {
	_ = client
}
