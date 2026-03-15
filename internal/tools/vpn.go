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
	addVPNTunnelTerminationsList(s, client)
	addVPNTunnelTerminationsGet(s, client)
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
	addGetTool(s, "netbox_vpn_tunnels_get", "Get a single VPN tunnel by its NetBox ID.", "VPN tunnel",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.VpnAPI.VpnTunnelsRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_vpn_tunnel_groups_get", "Get a single VPN tunnel group by its NetBox ID.", "VPN tunnel group",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.VpnAPI.VpnTunnelGroupsRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_vpn_l2vpns_get", "Get a single L2VPN by its NetBox ID.", "L2VPN",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.VpnAPI.VpnL2vpnsRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_vpn_ike_policies_get", "Get a single IKE policy by its NetBox ID.", "IKE policy",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.VpnAPI.VpnIkePoliciesRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_vpn_ipsec_policies_get", "Get a single IPSec policy by its NetBox ID.", "IPSec policy",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.VpnAPI.VpnIpsecPoliciesRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addVPNTunnelTerminationsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"         jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty"  jsonschema:"Field to order results by (prefix with - for descending)"`
		TunnelID int32    `json:"tunnel_id,omitempty" jsonschema:"Tunnel ID to filter by"`
		Role     []string `json:"role,omitempty"      jsonschema:"Termination role to filter by (peer, hub, spoke)"`
		Limit    int32    `json:"limit,omitempty"     jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"    jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_vpn_tunnel_terminations_list",
		Description: "List VPN tunnel terminations in NetBox, optionally filtered by tunnel or role.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VpnAPI.VpnTunnelTerminationsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.TunnelID != 0 {
			r = r.TunnelId([]int32{in.TunnelID})
		}
		if len(in.Role) > 0 {
			r = r.Role(in.Role)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing VPN tunnel terminations: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVPNTunnelTerminationsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_vpn_tunnel_terminations_get", "Get a single VPN tunnel termination by its NetBox ID.", "VPN tunnel termination",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.VpnAPI.VpnTunnelTerminationsRetrieve(ctx, id).Execute()
			return r, err
		})
}
