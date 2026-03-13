package tools

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterIPAM adds IPAM tools to s.
func RegisterIPAM(s *mcp.Server, client *netbox.APIClient) {
	addIPAMIPAddressesList(s, client)
	addIPAMIPAddressesGet(s, client)
	addIPAMPrefixesList(s, client)
	addIPAMPrefixesGet(s, client)
	addIPAMVRFsList(s, client)
	addIPAMVRFsGet(s, client)
	addIPAMVLANsList(s, client)
	addIPAMVLANsGet(s, client)
}

func addIPAMIPAddressesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Address string `json:"address,omitempty" jsonschema:"IP address to filter by (e.g. 192.0.2.1/24)"`
		VRF     string `json:"vrf,omitempty" jsonschema:"VRF name to filter by"`
		Status  string `json:"status,omitempty" jsonschema:"IP address status (active, reserved, deprecated, dhcp, slaac)"`
		Tenant  string `json:"tenant,omitempty" jsonschema:"Tenant name or slug to filter by"`
		Device  string `json:"device,omitempty" jsonschema:"Device name to filter by"`
		Limit   int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset  int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_ip_addresses_list",
		Description: "List IP addresses in NetBox, optionally filtered by address, VRF, status, tenant, or device.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamIpAddressesList(ctx)
		if in.Address != "" {
			r = r.Address([]string{in.Address})
		}
		if in.VRF != "" {
			r = r.Vrf([]*string{ptrOf(in.VRF)})
		}
		if in.Status != "" {
			r = r.Status([]string{in.Status})
		}
		if in.Tenant != "" {
			r = r.Tenant([]string{in.Tenant})
		}
		if in.Device != "" {
			r = r.Device([]string{in.Device})
		}
		limit := in.Limit
		if limit == 0 {
			limit = 50
		}
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing IP addresses: %v", err))
		}
		return jsonResult(resp)
	})
}

func addIPAMIPAddressesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the IP address to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_ip_addresses_get",
		Description: "Get a single IP address by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.IpamAPI.IpamIpAddressesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting IP address %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addIPAMPrefixesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Prefix string `json:"prefix,omitempty" jsonschema:"Prefix to filter by (e.g. 192.0.2.0/24)"`
		VRF    string `json:"vrf,omitempty" jsonschema:"VRF name to filter by"`
		Status string `json:"status,omitempty" jsonschema:"Prefix status (active, container, reserved, deprecated)"`
		Site   string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Tenant string `json:"tenant,omitempty" jsonschema:"Tenant name or slug to filter by"`
		Limit  int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_prefixes_list",
		Description: "List IP prefixes in NetBox, optionally filtered by prefix, VRF, status, site, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamPrefixesList(ctx)
		if in.Prefix != "" {
			r = r.Prefix([]string{in.Prefix})
		}
		if in.VRF != "" {
			r = r.Vrf([]*string{ptrOf(in.VRF)})
		}
		if in.Status != "" {
			r = r.Status([]string{in.Status})
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
			return toolError(fmt.Sprintf("listing prefixes: %v", err))
		}
		return jsonResult(resp)
	})
}

func addIPAMPrefixesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the prefix to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_prefixes_get",
		Description: "Get a single IP prefix by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.IpamAPI.IpamPrefixesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting prefix %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addIPAMVRFsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Name   string `json:"name,omitempty" jsonschema:"VRF name to filter by"`
		RD     string `json:"rd,omitempty" jsonschema:"Route distinguisher to filter by"`
		Tenant string `json:"tenant,omitempty" jsonschema:"Tenant name or slug to filter by"`
		Limit  int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_vrfs_list",
		Description: "List VRFs in NetBox, optionally filtered by name, route distinguisher, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamVrfsList(ctx)
		if in.Name != "" {
			r = r.Name([]string{in.Name})
		}
		if in.RD != "" {
			r = r.Rd([]string{in.RD})
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
			return toolError(fmt.Sprintf("listing VRFs: %v", err))
		}
		return jsonResult(resp)
	})
}

func addIPAMVRFsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the VRF to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_vrfs_get",
		Description: "Get a single VRF by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.IpamAPI.IpamVrfsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting VRF %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addIPAMVLANsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		VID    int32  `json:"vid,omitempty" jsonschema:"VLAN ID number to filter by (1-4094)"`
		Name   string `json:"name,omitempty" jsonschema:"VLAN name to filter by"`
		Site   string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Group  string `json:"group,omitempty" jsonschema:"VLAN group name or slug to filter by"`
		Status string `json:"status,omitempty" jsonschema:"VLAN status (active, reserved, deprecated)"`
		Limit  int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_vlans_list",
		Description: "List VLANs in NetBox, optionally filtered by VID, name, site, group, or status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamVlansList(ctx)
		if in.VID != 0 {
			r = r.Vid([]int32{in.VID})
		}
		if in.Name != "" {
			r = r.Name([]string{in.Name})
		}
		if in.Site != "" {
			r = r.Site([]string{in.Site})
		}
		if in.Group != "" {
			r = r.Group([]string{in.Group})
		}
		if in.Status != "" {
			r = r.Status([]string{in.Status})
		}
		limit := in.Limit
		if limit == 0 {
			limit = 50
		}
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing VLANs: %v", err))
		}
		return jsonResult(resp)
	})
}

func addIPAMVLANsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the VLAN to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_vlans_get",
		Description: "Get a single VLAN by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.IpamAPI.IpamVlansRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting VLAN %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}
