package tools

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterWireless adds wireless-related tools to s.
func RegisterWireless(s *mcp.Server, client *netbox.APIClient) {
	addWirelessLANsList(s, client)
	addWirelessLANsGet(s, client)
	addWirelessLANGroupsList(s, client)
	addWirelessLANGroupsGet(s, client)
	addWirelessLinksList(s, client)
	addWirelessLinksGet(s, client)
}

func addWirelessLANsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		SSID     []string `json:"ssid,omitempty"     jsonschema:"SSID(s) to filter by"`
		Group    []string `json:"group,omitempty"    jsonschema:"Wireless LAN group name or slug to filter by"`
		Status   []string `json:"status,omitempty"   jsonschema:"Wireless LAN status(es) to filter by"`
		Tenant   []string `json:"tenant,omitempty"   jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_wireless_lans_list",
		Description: "List wireless LANs in NetBox, optionally filtered by SSID, group, status, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.WirelessAPI.WirelessWirelessLansList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.SSID) > 0 {
			r = r.Ssid(in.SSID)
		}
		if len(in.Group) > 0 {
			r = r.Group(in.Group)
		}
		if len(in.Status) > 0 {
			r = r.Status(in.Status)
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
			return toolError(fmt.Sprintf("listing wireless LANs: %v", err))
		}
		return jsonResult(resp)
	})
}

func addWirelessLANsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_wireless_lans_get", "Get a single wireless LAN by its NetBox ID.", "wireless LAN",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.WirelessAPI.WirelessWirelessLansRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addWirelessLANGroupsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Wireless LAN group name(s) to filter by"`
		Parent   []string `json:"parent,omitempty"   jsonschema:"Parent group name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_wireless_lan_groups_list",
		Description: "List wireless LAN groups in NetBox, optionally filtered by name or parent.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.WirelessAPI.WirelessWirelessLanGroupsList(ctx)
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
			return toolError(fmt.Sprintf("listing wireless LAN groups: %v", err))
		}
		return jsonResult(resp)
	})
}

func addWirelessLANGroupsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_wireless_lan_groups_get", "Get a single wireless LAN group by its NetBox ID.", "wireless LAN group",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.WirelessAPI.WirelessWirelessLanGroupsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addWirelessLinksList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Status   []string `json:"status,omitempty"   jsonschema:"Wireless link status(es) to filter by"`
		Tenant   []string `json:"tenant,omitempty"   jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_wireless_links_list",
		Description: "List wireless links in NetBox, optionally filtered by status or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.WirelessAPI.WirelessWirelessLinksList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Status) > 0 {
			r = r.Status(in.Status)
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
			return toolError(fmt.Sprintf("listing wireless links: %v", err))
		}
		return jsonResult(resp)
	})
}

func addWirelessLinksGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_wireless_links_get", "Get a single wireless link by its NetBox ID.", "wireless link",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.WirelessAPI.WirelessWirelessLinksRetrieve(ctx, id).Execute()
			return r, err
		})
}
