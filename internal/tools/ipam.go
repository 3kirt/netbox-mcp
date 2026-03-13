package tools

import (
	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterIPAM adds IPAM tools to s.
func RegisterIPAM(s *mcp.Server, client *netbox.APIClient) {
	_ = client
}
