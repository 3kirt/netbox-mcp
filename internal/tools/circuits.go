package tools

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterCircuits adds circuit-related tools to s.
func RegisterCircuits(s *mcp.Server, client *netbox.APIClient) {
	addCircuitsCircuitsList(s, client)
	addCircuitsCircuitsGet(s, client)
	addCircuitsProvidersList(s, client)
	addCircuitsProvidersGet(s, client)
}

func addCircuitsCircuitsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Provider string `json:"provider,omitempty" jsonschema:"Provider name or slug to filter by"`
		Status   string `json:"status,omitempty" jsonschema:"Circuit status (active, planned, provisioning, offline, deprovisioning, decommissioned)"`
		Type     string `json:"type,omitempty" jsonschema:"Circuit type name or slug to filter by"`
		Site     string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Tenant   string `json:"tenant,omitempty" jsonschema:"Tenant name or slug to filter by"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_circuits_circuits_list",
		Description: "List circuits in NetBox, optionally filtered by provider, status, type, site, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.CircuitsAPI.CircuitsCircuitsList(ctx)
		if in.Provider != "" {
			r = r.Provider([]string{in.Provider})
		}
		if in.Status != "" {
			r = r.Status([]string{in.Status})
		}
		if in.Type != "" {
			r = r.Type_([]string{in.Type})
		}
		if in.Site != "" {
			r = r.Site([]string{in.Site})
		}
		if in.Tenant != "" {
			r = r.Tenant([]string{in.Tenant})
		}
		limit := in.Limit
		if limit == 0 {
			limit = 50
		}
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing circuits: %v", err))
		}
		return jsonResult(resp)
	})
}

func addCircuitsCircuitsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the circuit to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_circuits_circuits_get",
		Description: "Get a single circuit by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.CircuitsAPI.CircuitsCircuitsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting circuit %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addCircuitsProvidersList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Name   string `json:"name,omitempty" jsonschema:"Provider name to filter by"`
		Limit  int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_circuits_providers_list",
		Description: "List circuit providers in NetBox, optionally filtered by name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.CircuitsAPI.CircuitsProvidersList(ctx)
		if in.Name != "" {
			r = r.Name([]string{in.Name})
		}
		limit := in.Limit
		if limit == 0 {
			limit = 50
		}
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing providers: %v", err))
		}
		return jsonResult(resp)
	})
}

func addCircuitsProvidersGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the provider to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_circuits_providers_get",
		Description: "Get a single circuit provider by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.CircuitsAPI.CircuitsProvidersRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting provider %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}
