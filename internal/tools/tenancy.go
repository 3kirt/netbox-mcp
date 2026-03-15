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
	addTenancyTenantGroupsList(s, client)
	addTenancyTenantGroupsGet(s, client)
	addTenancyContactsList(s, client)
	addTenancyContactsGet(s, client)
	addTenancyContactGroupsList(s, client)
	addTenancyContactGroupsGet(s, client)
	addTenancyContactRolesList(s, client)
	addTenancyContactRolesGet(s, client)
}

func addTenancyTenantsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty" jsonschema:"Tenant name to filter by"`
		Group    []string `json:"group,omitempty" jsonschema:"Tenant group name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_tenancy_tenants_list",
		Description: "List tenants in NetBox, optionally filtered by name or group.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.TenancyAPI.TenancyTenantsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if len(in.Group) > 0 {
			r = r.Group(in.Group)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
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
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.TenancyAPI.TenancyTenantsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting tenant %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addTenancyTenantGroupsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Tenant group name(s) to filter by"`
		Parent   []string `json:"parent,omitempty"   jsonschema:"Parent group name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_tenancy_tenant_groups_list",
		Description: "List tenant groups in NetBox, optionally filtered by name or parent.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.TenancyAPI.TenancyTenantGroupsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if len(in.Parent) > 0 {
			r = r.Parent(in.Parent)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing tenant groups: %v", err))
		}
		return jsonResult(resp)
	})
}

func addTenancyTenantGroupsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the tenant group to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_tenancy_tenant_groups_get",
		Description: "Get a single tenant group by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.TenancyAPI.TenancyTenantGroupsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting tenant group %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addTenancyContactsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Contact name(s) to filter by"`
		Group    []string `json:"group,omitempty"    jsonschema:"Contact group name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_tenancy_contacts_list",
		Description: "List contacts in NetBox, optionally filtered by name or group.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.TenancyAPI.TenancyContactsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if len(in.Group) > 0 {
			r = r.Group(in.Group)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing contacts: %v", err))
		}
		return jsonResult(resp)
	})
}

func addTenancyContactsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the contact to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_tenancy_contacts_get",
		Description: "Get a single contact by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.TenancyAPI.TenancyContactsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting contact %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addTenancyContactGroupsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Contact group name(s) to filter by"`
		Parent   []string `json:"parent,omitempty"   jsonschema:"Parent group name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_tenancy_contact_groups_list",
		Description: "List contact groups in NetBox, optionally filtered by name or parent.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.TenancyAPI.TenancyContactGroupsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if len(in.Parent) > 0 {
			r = r.Parent(in.Parent)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing contact groups: %v", err))
		}
		return jsonResult(resp)
	})
}

func addTenancyContactGroupsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the contact group to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_tenancy_contact_groups_get",
		Description: "Get a single contact group by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.TenancyAPI.TenancyContactGroupsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting contact group %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addTenancyContactRolesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Contact role name(s) to filter by"`
		Slug     []string `json:"slug,omitempty"     jsonschema:"Contact role slug(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_tenancy_contact_roles_list",
		Description: "List contact roles in NetBox, optionally filtered by name or slug.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.TenancyAPI.TenancyContactRolesList(ctx)
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
			return toolError(fmt.Sprintf("listing contact roles: %v", err))
		}
		return jsonResult(resp)
	})
}

func addTenancyContactRolesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the contact role to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_tenancy_contact_roles_get",
		Description: "Get a single contact role by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.TenancyAPI.TenancyContactRolesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting contact role %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}
