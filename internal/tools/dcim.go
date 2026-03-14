// Package tools implements the NetBox MCP tool handlers.
package tools

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterDCIM adds DCIM tools to s.
func RegisterDCIM(s *mcp.Server, client *netbox.APIClient) {
	addDCIMDevicesList(s, client)
	addDCIMDevicesGet(s, client)
	addDCIMSitesList(s, client)
	addDCIMSitesGet(s, client)
	addDCIMRacksList(s, client)
	addDCIMRacksGet(s, client)
	addDCIMInterfacesList(s, client)
	addDCIMInterfacesGet(s, client)
	addDCIMCablesList(s, client)
	addDCIMCablesGet(s, client)
	addDCIMRegionsList(s, client)
	addDCIMRegionsGet(s, client)
	addDCIMLocationsList(s, client)
	addDCIMLocationsGet(s, client)
	addDCIMManufacturersList(s, client)
	addDCIMManufacturersGet(s, client)
	addDCIMDeviceTypesList(s, client)
	addDCIMDeviceTypesGet(s, client)
	addDCIMDeviceRolesList(s, client)
	addDCIMDeviceRolesGet(s, client)
	addDCIMPlatformsList(s, client)
	addDCIMPlatformsGet(s, client)
	addDCIMPowerPanelsList(s, client)
	addDCIMPowerPanelsGet(s, client)
	addDCIMPowerFeedsList(s, client)
	addDCIMPowerFeedsGet(s, client)
	addDCIMVirtualChassisList(s, client)
	addDCIMVirtualChassisGet(s, client)
	addDCIMInventoryItemsList(s, client)
	addDCIMInventoryItemsGet(s, client)
}

func addDCIMDevicesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Site     string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Role     string `json:"role,omitempty" jsonschema:"Device role name or slug to filter by"`
		Status   string `json:"status,omitempty" jsonschema:"Device status (active, planned, staged, failed, inventory, decommissioning)"`
		RackID   int32  `json:"rack_id,omitempty" jsonschema:"Rack ID to filter by"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_devices_list",
		Description: "List devices in NetBox, optionally filtered by site, role, status, or rack.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimDevicesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.Site != "" {
			r = r.Site([]string{in.Site})
		}
		if in.Role != "" {
			r = r.Role([]string{in.Role})
		}
		if in.Status != "" {
			r = r.Status([]string{in.Status})
		}
		if in.RackID != 0 {
			r = r.RackId([]int32{in.RackID})
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing devices: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMDevicesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the device to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_devices_get",
		Description: "Get a single device by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimDevicesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting device %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMSitesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     string `json:"name,omitempty" jsonschema:"Site name to filter by"`
		Status   string `json:"status,omitempty" jsonschema:"Site status (active, planned, retired, staging)"`
		Region   string `json:"region,omitempty" jsonschema:"Region name or slug to filter by"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_sites_list",
		Description: "List sites in NetBox, optionally filtered by name, status, or region.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimSitesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.Name != "" {
			r = r.Name([]string{in.Name})
		}
		if in.Status != "" {
			r = r.Status([]string{in.Status})
		}
		if in.Region != "" {
			r = r.Region([]string{in.Region})
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing sites: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMSitesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the site to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_sites_get",
		Description: "Get a single site by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimSitesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting site %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMRacksList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Site     string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Location string `json:"location,omitempty" jsonschema:"Location name or slug to filter by"`
		Status   string `json:"status,omitempty" jsonschema:"Rack status (active, planned, reserved, available, deprecated)"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_racks_list",
		Description: "List racks in NetBox, optionally filtered by site, location, or status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimRacksList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.Site != "" {
			r = r.Site([]string{in.Site})
		}
		if in.Location != "" {
			r = r.Location([]string{in.Location})
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
			return toolError(fmt.Sprintf("listing racks: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMRacksGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the rack to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_racks_get",
		Description: "Get a single rack by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimRacksRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting rack %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMInterfacesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		DeviceID int32  `json:"device_id,omitempty" jsonschema:"Device ID to filter by"`
		Name     string `json:"name,omitempty" jsonschema:"Interface name to filter by"`
		Type     string `json:"type,omitempty" jsonschema:"Interface type to filter by (e.g. 1000base-t, 10gbase-x-sfpp)"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_interfaces_list",
		Description: "List device interfaces in NetBox, optionally filtered by device, name, or type.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimInterfacesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
		}
		if in.Name != "" {
			r = r.Name([]string{in.Name})
		}
		if in.Type != "" {
			r = r.Type_([]string{in.Type})
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing interfaces: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMInterfacesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the interface to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_interfaces_get",
		Description: "Get a single device interface by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimInterfacesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting interface %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMCablesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Site     string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Status   string `json:"status,omitempty" jsonschema:"Cable status (connected, planned, decommissioning)"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_cables_list",
		Description: "List cables in NetBox, optionally filtered by site or status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimCablesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.Site != "" {
			r = r.Site([]string{in.Site})
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
			return toolError(fmt.Sprintf("listing cables: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMCablesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the cable to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_cables_get",
		Description: "Get a single cable by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimCablesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting cable %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMRegionsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Region name(s) to filter by"`
		Slug     []string `json:"slug,omitempty"     jsonschema:"Region slug(s) to filter by"`
		Parent   []string `json:"parent,omitempty"   jsonschema:"Parent region name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_regions_list",
		Description: "List regions in NetBox, optionally filtered by name, slug, or parent.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimRegionsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if len(in.Slug) > 0 {
			r = r.Slug(in.Slug)
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
			return toolError(fmt.Sprintf("listing regions: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMRegionsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the region to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_regions_get",
		Description: "Get a single region by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimRegionsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting region %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMLocationsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Site     []string `json:"site,omitempty"     jsonschema:"Site name or slug to filter by"`
		Parent   []string `json:"parent,omitempty"   jsonschema:"Parent location name or slug to filter by"`
		Status   []string `json:"status,omitempty"   jsonschema:"Location status to filter by"`
		Tenant   []string `json:"tenant,omitempty"   jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_locations_list",
		Description: "List locations in NetBox, optionally filtered by site, parent, status, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimLocationsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Site) > 0 {
			r = r.Site(in.Site)
		}
		if len(in.Parent) > 0 {
			r = r.Parent(in.Parent)
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
			return toolError(fmt.Sprintf("listing locations: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMLocationsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the location to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_locations_get",
		Description: "Get a single location by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimLocationsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting location %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMManufacturersList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Manufacturer name(s) to filter by"`
		Slug     []string `json:"slug,omitempty"     jsonschema:"Manufacturer slug(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_manufacturers_list",
		Description: "List manufacturers in NetBox, optionally filtered by name or slug.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimManufacturersList(ctx)
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
			return toolError(fmt.Sprintf("listing manufacturers: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMManufacturersGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the manufacturer to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_manufacturers_get",
		Description: "Get a single manufacturer by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimManufacturersRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting manufacturer %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMDeviceTypesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q            string   `json:"q,omitempty"            jsonschema:"Free-text search"`
		Ordering     string   `json:"ordering,omitempty"     jsonschema:"Field to order results by (prefix with - for descending)"`
		Manufacturer []string `json:"manufacturer,omitempty" jsonschema:"Manufacturer name or slug to filter by"`
		Model        []string `json:"model,omitempty"        jsonschema:"Device type model name(s) to filter by"`
		Limit        int32    `json:"limit,omitempty"        jsonschema:"Maximum number of results (default 50)"`
		Offset       int32    `json:"offset,omitempty"       jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_device_types_list",
		Description: "List device types in NetBox, optionally filtered by manufacturer or model.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimDeviceTypesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Manufacturer) > 0 {
			r = r.Manufacturer(in.Manufacturer)
		}
		if len(in.Model) > 0 {
			r = r.Model(in.Model)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing device types: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMDeviceTypesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the device type to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_device_types_get",
		Description: "Get a single device type by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimDeviceTypesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting device type %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMDeviceRolesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Device role name(s) to filter by"`
		Slug     []string `json:"slug,omitempty"     jsonschema:"Device role slug(s) to filter by"`
		VMRole   *bool    `json:"vm_role,omitempty"  jsonschema:"Filter to roles usable on virtual machines"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_device_roles_list",
		Description: "List device roles in NetBox, optionally filtered by name, slug, or VM role flag.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimDeviceRolesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if len(in.Slug) > 0 {
			r = r.Slug(in.Slug)
		}
		if in.VMRole != nil {
			r = r.VmRole(*in.VMRole)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing device roles: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMDeviceRolesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the device role to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_device_roles_get",
		Description: "Get a single device role by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimDeviceRolesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting device role %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMPlatformsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q            string   `json:"q,omitempty"            jsonschema:"Free-text search"`
		Ordering     string   `json:"ordering,omitempty"     jsonschema:"Field to order results by (prefix with - for descending)"`
		Name         []string `json:"name,omitempty"         jsonschema:"Platform name(s) to filter by"`
		Manufacturer []string `json:"manufacturer,omitempty" jsonschema:"Manufacturer name or slug to filter by"`
		Limit        int32    `json:"limit,omitempty"        jsonschema:"Maximum number of results (default 50)"`
		Offset       int32    `json:"offset,omitempty"       jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_platforms_list",
		Description: "List platforms in NetBox, optionally filtered by name or manufacturer.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimPlatformsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if len(in.Manufacturer) > 0 {
			r = r.Manufacturer(in.Manufacturer)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing platforms: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMPlatformsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the platform to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_platforms_get",
		Description: "Get a single platform by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimPlatformsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting platform %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMPowerPanelsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Site     []string `json:"site,omitempty"     jsonschema:"Site name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_power_panels_list",
		Description: "List power panels in NetBox, optionally filtered by site.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimPowerPanelsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
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
			return toolError(fmt.Sprintf("listing power panels: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMPowerPanelsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the power panel to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_power_panels_get",
		Description: "Get a single power panel by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimPowerPanelsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting power panel %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMPowerFeedsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Site     []string `json:"site,omitempty"     jsonschema:"Site name or slug to filter by"`
		Status   []string `json:"status,omitempty"   jsonschema:"Power feed status (active, planned, failed)"`
		Type     string   `json:"type,omitempty"     jsonschema:"Power feed type (primary, redundant)"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_power_feeds_list",
		Description: "List power feeds in NetBox, optionally filtered by site, status, or type.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimPowerFeedsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Site) > 0 {
			r = r.Site(in.Site)
		}
		if len(in.Status) > 0 {
			r = r.Status(in.Status)
		}
		if in.Type != "" {
			r = r.Type_(netbox.DcimPowerFeedsListTypeParameter(in.Type))
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing power feeds: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMPowerFeedsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the power feed to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_power_feeds_get",
		Description: "Get a single power feed by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimPowerFeedsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting power feed %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMVirtualChassisList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Site     []string `json:"site,omitempty"     jsonschema:"Site name or slug to filter by"`
		Tenant   []string `json:"tenant,omitempty"   jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_virtual_chassis_list",
		Description: "List virtual chassis in NetBox, optionally filtered by site or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimVirtualChassisList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
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
			return toolError(fmt.Sprintf("listing virtual chassis: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMVirtualChassisGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the virtual chassis to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_virtual_chassis_get",
		Description: "Get a single virtual chassis by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimVirtualChassisRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting virtual chassis %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addDCIMInventoryItemsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q            string   `json:"q,omitempty"            jsonschema:"Free-text search"`
		Ordering     string   `json:"ordering,omitempty"     jsonschema:"Field to order results by (prefix with - for descending)"`
		DeviceID     int32    `json:"device_id,omitempty"    jsonschema:"Device ID to filter by"`
		Manufacturer []string `json:"manufacturer,omitempty" jsonschema:"Manufacturer name or slug to filter by"`
		Discovered   *bool    `json:"discovered,omitempty"   jsonschema:"Filter to discovered inventory items"`
		Limit        int32    `json:"limit,omitempty"        jsonschema:"Maximum number of results (default 50)"`
		Offset       int32    `json:"offset,omitempty"       jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_inventory_items_list",
		Description: "List inventory items in NetBox, optionally filtered by device, manufacturer, or discovered status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimInventoryItemsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
		}
		if len(in.Manufacturer) > 0 {
			r = r.Manufacturer(in.Manufacturer)
		}
		if in.Discovered != nil {
			r = r.Discovered(*in.Discovered)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing inventory items: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMInventoryItemsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the inventory item to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_inventory_items_get",
		Description: "Get a single inventory item by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.DcimAPI.DcimInventoryItemsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting inventory item %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}
