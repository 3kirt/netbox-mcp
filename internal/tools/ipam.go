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
	addIPAMAggregatesList(s, client)
	addIPAMAggregatesGet(s, client)
	addIPAMIPRangesList(s, client)
	addIPAMIPRangesGet(s, client)
	addIPAMRouteTargetsList(s, client)
	addIPAMRouteTargetsGet(s, client)
	addIPAMRIRsList(s, client)
	addIPAMRIRsGet(s, client)
	addIPAMVLANGroupsList(s, client)
	addIPAMVLANGroupsGet(s, client)
	addIPAMServicesList(s, client)
	addIPAMServicesGet(s, client)
}

func addIPAMIPAddressesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Address  string `json:"address,omitempty" jsonschema:"IP address to filter by (e.g. 192.0.2.1/24)"`
		VRF      string `json:"vrf,omitempty" jsonschema:"VRF name to filter by"`
		Status   string `json:"status,omitempty" jsonschema:"IP address status (active, reserved, deprecated, dhcp, slaac)"`
		Tenant   string `json:"tenant,omitempty" jsonschema:"Tenant name or slug to filter by"`
		Device   string `json:"device,omitempty" jsonschema:"Device name to filter by"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_ip_addresses_list",
		Description: "List IP addresses in NetBox, optionally filtered by address, VRF, status, tenant, or device.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamIpAddressesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
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
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
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
		Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Prefix   string `json:"prefix,omitempty" jsonschema:"Prefix to filter by (e.g. 192.0.2.0/24)"`
		VRF      string `json:"vrf,omitempty" jsonschema:"VRF name to filter by"`
		Status   string `json:"status,omitempty" jsonschema:"Prefix status (active, container, reserved, deprecated)"`
		Site     string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Tenant   string `json:"tenant,omitempty" jsonschema:"Tenant name or slug to filter by"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_prefixes_list",
		Description: "List IP prefixes in NetBox, optionally filtered by prefix, VRF, status, site, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamPrefixesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
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
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
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
		Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     string `json:"name,omitempty" jsonschema:"VRF name to filter by"`
		RD       string `json:"rd,omitempty" jsonschema:"Route distinguisher to filter by"`
		Tenant   string `json:"tenant,omitempty" jsonschema:"Tenant name or slug to filter by"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_vrfs_list",
		Description: "List VRFs in NetBox, optionally filtered by name, route distinguisher, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamVrfsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.Name != "" {
			r = r.Name([]string{in.Name})
		}
		if in.RD != "" {
			r = r.Rd([]string{in.RD})
		}
		if in.Tenant != "" {
			r = r.Tenant([]string{in.Tenant})
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
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
		Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		VID      int32  `json:"vid,omitempty" jsonschema:"VLAN ID number to filter by (1-4094)"`
		Name     string `json:"name,omitempty" jsonschema:"VLAN name to filter by"`
		Site     string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Group    string `json:"group,omitempty" jsonschema:"VLAN group name or slug to filter by"`
		Status   string `json:"status,omitempty" jsonschema:"VLAN status (active, reserved, deprecated)"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_vlans_list",
		Description: "List VLANs in NetBox, optionally filtered by VID, name, site, group, or status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamVlansList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
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
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
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

func addIPAMAggregatesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Family   float32  `json:"family,omitempty"   jsonschema:"IP family to filter by (4 or 6)"`
		RIR      []string `json:"rir,omitempty"      jsonschema:"RIR name or slug to filter by"`
		Tenant   []string `json:"tenant,omitempty"   jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_aggregates_list",
		Description: "List IP aggregates in NetBox, optionally filtered by IP family, RIR, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamAggregatesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.Family != 0 {
			r = r.Family(in.Family)
		}
		if len(in.RIR) > 0 {
			r = r.Rir(in.RIR)
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
			return toolError(fmt.Sprintf("listing aggregates: %v", err))
		}
		return jsonResult(resp)
	})
}

func addIPAMAggregatesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the aggregate to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_aggregates_get",
		Description: "Get a single IP aggregate by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.IpamAPI.IpamAggregatesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting aggregate %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addIPAMIPRangesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		VRF      string   `json:"vrf,omitempty"      jsonschema:"VRF name to filter by (use empty string for global)"`
		Status   []string `json:"status,omitempty"   jsonschema:"IP range status (active, reserved, deprecated)"`
		Tenant   []string `json:"tenant,omitempty"   jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_ip_ranges_list",
		Description: "List IP ranges in NetBox, optionally filtered by VRF, status, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamIpRangesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.VRF != "" {
			r = r.Vrf([]*string{ptrOf(in.VRF)})
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
			return toolError(fmt.Sprintf("listing IP ranges: %v", err))
		}
		return jsonResult(resp)
	})
}

func addIPAMIPRangesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the IP range to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_ip_ranges_get",
		Description: "Get a single IP range by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.IpamAPI.IpamIpRangesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting IP range %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addIPAMRouteTargetsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Route target name(s) to filter by"`
		Tenant   []string `json:"tenant,omitempty"   jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_route_targets_list",
		Description: "List route targets in NetBox, optionally filtered by name or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamRouteTargetsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
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
			return toolError(fmt.Sprintf("listing route targets: %v", err))
		}
		return jsonResult(resp)
	})
}

func addIPAMRouteTargetsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the route target to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_route_targets_get",
		Description: "Get a single route target by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.IpamAPI.IpamRouteTargetsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting route target %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addIPAMRIRsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"RIR name(s) to filter by"`
		Slug     []string `json:"slug,omitempty"     jsonschema:"RIR slug(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_rirs_list",
		Description: "List RIRs (Regional Internet Registries) in NetBox, optionally filtered by name or slug.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamRirsList(ctx)
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
			return toolError(fmt.Sprintf("listing RIRs: %v", err))
		}
		return jsonResult(resp)
	})
}

func addIPAMRIRsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the RIR to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_rirs_get",
		Description: "Get a single RIR by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.IpamAPI.IpamRirsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting RIR %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addIPAMVLANGroupsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"VLAN group name(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_vlan_groups_list",
		Description: "List VLAN groups in NetBox, optionally filtered by name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamVlanGroupsList(ctx)
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
			return toolError(fmt.Sprintf("listing VLAN groups: %v", err))
		}
		return jsonResult(resp)
	})
}

func addIPAMVLANGroupsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the VLAN group to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_vlan_groups_get",
		Description: "Get a single VLAN group by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.IpamAPI.IpamVlanGroupsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting VLAN group %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addIPAMServicesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q                string `json:"q,omitempty"                 jsonschema:"Free-text search"`
		Ordering         string `json:"ordering,omitempty"          jsonschema:"Field to order results by (prefix with - for descending)"`
		DeviceID         int32  `json:"device_id,omitempty"         jsonschema:"Device ID to filter by"`
		VirtualMachineID int32  `json:"virtual_machine_id,omitempty" jsonschema:"Virtual machine ID to filter by"`
		Protocol         string `json:"protocol,omitempty"          jsonschema:"Protocol to filter by (tcp, udp, sctp)"`
		Limit            int32  `json:"limit,omitempty"             jsonschema:"Maximum number of results (default 50)"`
		Offset           int32  `json:"offset,omitempty"            jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_services_list",
		Description: "List services in NetBox, optionally filtered by device, virtual machine, or protocol.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.IpamAPI.IpamServicesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
		}
		if in.VirtualMachineID != 0 {
			r = r.VirtualMachineId([]int32{in.VirtualMachineID})
		}
		if in.Protocol != "" {
			r = r.Protocol(netbox.IpamServiceTemplatesListProtocolParameter(in.Protocol))
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing services: %v", err))
		}
		return jsonResult(resp)
	})
}

func addIPAMServicesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the service to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_ipam_services_get",
		Description: "Get a single service by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.IpamAPI.IpamServicesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting service %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}
