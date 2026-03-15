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
	addCircuitsCircuitTypesList(s, client)
	addCircuitsCircuitTypesGet(s, client)
	addCircuitsCircuitTerminationsList(s, client)
	addCircuitsCircuitTerminationsGet(s, client)
	addCircuitsProviderAccountsList(s, client)
	addCircuitsProviderAccountsGet(s, client)
	addCircuitsProviderNetworksList(s, client)
	addCircuitsProviderNetworksGet(s, client)
}

func addCircuitsCircuitsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Provider []string `json:"provider,omitempty" jsonschema:"Provider name or slug to filter by"`
		Status   []string `json:"status,omitempty" jsonschema:"Circuit status (active, planned, provisioning, offline, deprovisioning, decommissioned)"`
		Type     []string `json:"type,omitempty" jsonschema:"Circuit type name or slug to filter by"`
		Site     []string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Tenant   []string `json:"tenant,omitempty" jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_circuits_circuits_list",
		Description: "List circuits in NetBox, optionally filtered by provider, status, type, site, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.CircuitsAPI.CircuitsCircuitsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Provider) > 0 {
			r = r.Provider(in.Provider)
		}
		if len(in.Status) > 0 {
			r = r.Status(in.Status)
		}
		if len(in.Type) > 0 {
			r = r.Type_(in.Type)
		}
		if len(in.Site) > 0 {
			r = r.Site(in.Site)
		}
		if len(in.Tenant) > 0 {
			r = r.Tenant(in.Tenant)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing circuits: %v", err))
		}
		return jsonResult(resp)
	})
}

func addCircuitsCircuitsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_circuits_circuits_get", "Get a single circuit by its NetBox ID.", "circuit",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.CircuitsAPI.CircuitsCircuitsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addCircuitsProvidersList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty" jsonschema:"Provider name to filter by"`
		Limit    int32    `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_circuits_providers_list",
		Description: "List circuit providers in NetBox, optionally filtered by name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.CircuitsAPI.CircuitsProvidersList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing providers: %v", err))
		}
		return jsonResult(resp)
	})
}

func addCircuitsProvidersGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_circuits_providers_get", "Get a single circuit provider by its NetBox ID.", "provider",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.CircuitsAPI.CircuitsProvidersRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addCircuitsCircuitTypesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Circuit type name(s) to filter by"`
		Slug     []string `json:"slug,omitempty"     jsonschema:"Circuit type slug(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_circuits_circuit_types_list",
		Description: "List circuit types in NetBox, optionally filtered by name or slug.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.CircuitsAPI.CircuitsCircuitTypesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if len(in.Slug) > 0 {
			r = r.Slug(in.Slug)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing circuit types: %v", err))
		}
		return jsonResult(resp)
	})
}

func addCircuitsCircuitTypesGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_circuits_circuit_types_get", "Get a single circuit type by its NetBox ID.", "circuit type",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.CircuitsAPI.CircuitsCircuitTypesRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addCircuitsCircuitTerminationsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q         string   `json:"q,omitempty"          jsonschema:"Free-text search"`
		Ordering  string   `json:"ordering,omitempty"   jsonschema:"Field to order results by (prefix with - for descending)"`
		CircuitID int32    `json:"circuit_id,omitempty" jsonschema:"Circuit ID to filter by"`
		Site      []string `json:"site,omitempty"       jsonschema:"Site name or slug to filter by"`
		Limit     int32    `json:"limit,omitempty"      jsonschema:"Maximum number of results (default 50)"`
		Offset    int32    `json:"offset,omitempty"     jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_circuits_circuit_terminations_list",
		Description: "List circuit terminations in NetBox, optionally filtered by circuit or site.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.CircuitsAPI.CircuitsCircuitTerminationsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.CircuitID != 0 {
			r = r.CircuitId([]int32{in.CircuitID})
		}
		if len(in.Site) > 0 {
			r = r.Site(in.Site)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing circuit terminations: %v", err))
		}
		return jsonResult(resp)
	})
}

func addCircuitsCircuitTerminationsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_circuits_circuit_terminations_get", "Get a single circuit termination by its NetBox ID.", "circuit termination",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.CircuitsAPI.CircuitsCircuitTerminationsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addCircuitsProviderAccountsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Provider []string `json:"provider,omitempty" jsonschema:"Provider name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_circuits_provider_accounts_list",
		Description: "List provider accounts in NetBox, optionally filtered by provider.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.CircuitsAPI.CircuitsProviderAccountsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Provider) > 0 {
			r = r.Provider(in.Provider)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing provider accounts: %v", err))
		}
		return jsonResult(resp)
	})
}

func addCircuitsProviderAccountsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_circuits_provider_accounts_get", "Get a single provider account by its NetBox ID.", "provider account",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.CircuitsAPI.CircuitsProviderAccountsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addCircuitsProviderNetworksList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Provider []string `json:"provider,omitempty" jsonschema:"Provider name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_circuits_provider_networks_list",
		Description: "List provider networks in NetBox, optionally filtered by provider.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.CircuitsAPI.CircuitsProviderNetworksList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Provider) > 0 {
			r = r.Provider(in.Provider)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing provider networks: %v", err))
		}
		return jsonResult(resp)
	})
}

func addCircuitsProviderNetworksGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_circuits_provider_networks_get", "Get a single provider network by its NetBox ID.", "provider network",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.CircuitsAPI.CircuitsProviderNetworksRetrieve(ctx, id).Execute()
			return r, err
		})
}
