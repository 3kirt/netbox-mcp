package tools

import (
	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterDCIM adds DCIM tools to s.
func RegisterDCIM(s *mcp.Server, client *netbox.APIClient) {
	_ = client
}
