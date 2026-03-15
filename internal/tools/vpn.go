package tools

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterVPN adds VPN-related tools to s.
func RegisterVPN(s *mcp.Server, client *netbox.APIClient) {
	addVPNTunnelsList(s, client)
	addVPNTunnelsGet(s, client)
	addVPNTunnelGroupsList(s, client)
	addVPNTunnelGroupsGet(s, client)
	addVPNL2VPNsList(s, client)
	addVPNL2VPNsGet(s, client)
	addVPNIKEPoliciesList(s, client)
	addVPNIKEPoliciesGet(s, client)
	addVPNIPSecPoliciesList(s, client)
	addVPNIPSecPoliciesGet(s, client)
}

func addVPNTunnelsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Status   []string `json:"status,omitempty"   jsonschema:"Tunnel status(es) to filter by"`
		Group    []string `json:"group,omitempty"    jsonschema:"Tunnel group name or slug to filter by"`
		Tenant   []string `json:"tenant,omitempty"   jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_vpn_tunnels_list",
		Description: "List VPN tunnels in NetBox, optionally filtered by status, group, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VpnAPI.VpnTunnelsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Status) > 0 {
			r = r.Status(in.Status)
		}
		if len(in.Group) > 0 {
			r = r.Group(in.Group)
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
			return toolError(fmt.Sprintf("listing VPN tunnels: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVPNTunnelsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the VPN tunnel to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_vpn_tunnels_get",
		Description: "Get a single VPN tunnel by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.VpnAPI.VpnTunnelsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting VPN tunnel %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addVPNTunnelGroupsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Tunnel group name(s) to filter by"`
		Slug     []string `json:"slug,omitempty"     jsonschema:"Tunnel group slug(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_vpn_tunnel_groups_list",
		Description: "List VPN tunnel groups in NetBox, optionally filtered by name or slug.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VpnAPI.VpnTunnelGroupsList(ctx)
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
			return toolError(fmt.Sprintf("listing VPN tunnel groups: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVPNTunnelGroupsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the VPN tunnel group to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_vpn_tunnel_groups_get",
		Description: "Get a single VPN tunnel group by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.VpnAPI.VpnTunnelGroupsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting VPN tunnel group %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addVPNL2VPNsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Type     []string `json:"type,omitempty"     jsonschema:"L2VPN type(s) to filter by"`
		Tenant   []string `json:"tenant,omitempty"   jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_vpn_l2vpns_list",
		Description: "List L2VPNs in NetBox, optionally filtered by type or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VpnAPI.VpnL2vpnsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Type) > 0 {
			r = r.Type_(in.Type)
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
			return toolError(fmt.Sprintf("listing L2VPNs: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVPNL2VPNsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the L2VPN to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_vpn_l2vpns_get",
		Description: "Get a single L2VPN by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.VpnAPI.VpnL2vpnsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting L2VPN %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addVPNIKEPoliciesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"IKE policy name(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_vpn_ike_policies_list",
		Description: "List IKE policies in NetBox, optionally filtered by name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VpnAPI.VpnIkePoliciesList(ctx)
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
			return toolError(fmt.Sprintf("listing IKE policies: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVPNIKEPoliciesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the IKE policy to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_vpn_ike_policies_get",
		Description: "Get a single IKE policy by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.VpnAPI.VpnIkePoliciesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting IKE policy %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addVPNIPSecPoliciesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"IPSec policy name(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_vpn_ipsec_policies_list",
		Description: "List IPSec policies in NetBox, optionally filtered by name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VpnAPI.VpnIpsecPoliciesList(ctx)
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
			return toolError(fmt.Sprintf("listing IPSec policies: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVPNIPSecPoliciesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the IPSec policy to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_vpn_ipsec_policies_get",
		Description: "Get a single IPSec policy by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.VpnAPI.VpnIpsecPoliciesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting IPSec policy %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}
