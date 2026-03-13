package tools

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterTenancy adds tenancy tools to s.
func RegisterTenancy(s *mcp.Server, client *netbox.APIClient) {
	addTenancyTenantsList(s, client)
	addTenancyTenantsGet(s, client)
}

func addTenancyTenantsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Name   string `json:"name,omitempty" jsonschema:"Tenant name to filter by"`
		Group  string `json:"group,omitempty" jsonschema:"Tenant group name or slug to filter by"`
		Limit  int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_tenancy_tenants_list",
		Description: "List tenants in NetBox, optionally filtered by name or group.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.TenancyAPI.TenancyTenantsList(ctx)
		if in.Name != "" {
			r = r.Name([]string{in.Name})
		}
		if in.Group != "" {
			r = r.Group([]string{in.Group})
		}
		limit := in.Limit
		if limit == 0 {
			limit = 50
		}
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing tenants: %v", err))
		}
		return jsonResult(resp)
	})
}

func addTenancyTenantsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the tenant to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_tenancy_tenants_get",
		Description: "Get a single tenant by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.TenancyAPI.TenancyTenantsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting tenant %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}
