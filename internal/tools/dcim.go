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
	addDCIMCableTerminationsList(s, client)
	addDCIMCableTerminationsGet(s, client)
	addDCIMConsolePortsList(s, client)
	addDCIMConsolePortsGet(s, client)
	addDCIMConsoleServerPortsList(s, client)
	addDCIMConsoleServerPortsGet(s, client)
	addDCIMDeviceBaysList(s, client)
	addDCIMDeviceBaysGet(s, client)
	addDCIMFrontPortsList(s, client)
	addDCIMFrontPortsGet(s, client)
	addDCIMMacAddressesList(s, client)
	addDCIMMacAddressesGet(s, client)
	addDCIMModulesList(s, client)
	addDCIMModulesGet(s, client)
	addDCIMModuleBaysList(s, client)
	addDCIMModuleBaysGet(s, client)
	addDCIMModuleTypesList(s, client)
	addDCIMModuleTypesGet(s, client)
	addDCIMPowerOutletsList(s, client)
	addDCIMPowerOutletsGet(s, client)
	addDCIMPowerPortsList(s, client)
	addDCIMPowerPortsGet(s, client)
	addDCIMRackReservationsList(s, client)
	addDCIMRackReservationsGet(s, client)
	addDCIMRackRolesList(s, client)
	addDCIMRackRolesGet(s, client)
	addDCIMRackTypesList(s, client)
	addDCIMRackTypesGet(s, client)
	addDCIMRearPortsList(s, client)
	addDCIMRearPortsGet(s, client)
	addDCIMSiteGroupsList(s, client)
	addDCIMSiteGroupsGet(s, client)
	addDCIMVirtualDeviceContextsList(s, client)
	addDCIMVirtualDeviceContextsGet(s, client)
}

func addDCIMDevicesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Site     []string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Role     []string `json:"role,omitempty" jsonschema:"Device role name or slug to filter by"`
		Status   []string `json:"status,omitempty" jsonschema:"Device status (active, planned, staged, failed, inventory, decommissioning)"`
		RackID   int32    `json:"rack_id,omitempty" jsonschema:"Rack ID to filter by"`
		Limit    int32    `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_devices_list",
		Description: "List devices in NetBox, optionally filtered by site, role, status, or rack.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimDevicesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Site) > 0 {
			r = r.Site(in.Site)
		}
		if len(in.Role) > 0 {
			r = r.Role(in.Role)
		}
		if len(in.Status) > 0 {
			r = r.Status(in.Status)
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
	addGetTool(s, "netbox_dcim_devices_get", "Get a single device by its NetBox ID.", "device",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimDevicesRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMSitesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty" jsonschema:"Site name to filter by"`
		Status   []string `json:"status,omitempty" jsonschema:"Site status (active, planned, retired, staging)"`
		Region   []string `json:"region,omitempty" jsonschema:"Region name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_sites_list",
		Description: "List sites in NetBox, optionally filtered by name, status, or region.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimSitesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if len(in.Status) > 0 {
			r = r.Status(in.Status)
		}
		if len(in.Region) > 0 {
			r = r.Region(in.Region)
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
	addGetTool(s, "netbox_dcim_sites_get", "Get a single site by its NetBox ID.", "site",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimSitesRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMRacksList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Site     []string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Location []string `json:"location,omitempty" jsonschema:"Location name or slug to filter by"`
		Status   []string `json:"status,omitempty" jsonschema:"Rack status (active, planned, reserved, available, deprecated)"`
		Limit    int32    `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_racks_list",
		Description: "List racks in NetBox, optionally filtered by site, location, or status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimRacksList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Site) > 0 {
			r = r.Site(in.Site)
		}
		if len(in.Location) > 0 {
			r = r.Location(in.Location)
		}
		if len(in.Status) > 0 {
			r = r.Status(in.Status)
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
	addGetTool(s, "netbox_dcim_racks_get", "Get a single rack by its NetBox ID.", "rack",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimRacksRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMInterfacesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		DeviceID int32    `json:"device_id,omitempty" jsonschema:"Device ID to filter by"`
		Name     []string `json:"name,omitempty" jsonschema:"Interface name to filter by"`
		Type     []string `json:"type,omitempty" jsonschema:"Interface type to filter by (e.g. 1000base-t, 10gbase-x-sfpp)"`
		Limit    int32    `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty" jsonschema:"Pagination offset"`
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
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if len(in.Type) > 0 {
			r = r.Type_(in.Type)
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
	addGetTool(s, "netbox_dcim_interfaces_get", "Get a single device interface by its NetBox ID.", "interface",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimInterfacesRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMCablesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Site     []string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Status   []string `json:"status,omitempty" jsonschema:"Cable status (connected, planned, decommissioning)"`
		Limit    int32    `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_cables_list",
		Description: "List cables in NetBox, optionally filtered by site or status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimCablesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Site) > 0 {
			r = r.Site(in.Site)
		}
		if len(in.Status) > 0 {
			r = r.Status(in.Status)
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
	addGetTool(s, "netbox_dcim_cables_get", "Get a single cable by its NetBox ID.", "cable",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimCablesRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_dcim_regions_get", "Get a single region by its NetBox ID.", "region",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimRegionsRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_dcim_locations_get", "Get a single location by its NetBox ID.", "location",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimLocationsRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_dcim_manufacturers_get", "Get a single manufacturer by its NetBox ID.", "manufacturer",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimManufacturersRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_dcim_device_types_get", "Get a single device type by its NetBox ID.", "device type",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimDeviceTypesRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_dcim_device_roles_get", "Get a single device role by its NetBox ID.", "device role",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimDeviceRolesRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_dcim_platforms_get", "Get a single platform by its NetBox ID.", "platform",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimPlatformsRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_dcim_power_panels_get", "Get a single power panel by its NetBox ID.", "power panel",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimPowerPanelsRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_dcim_power_feeds_get", "Get a single power feed by its NetBox ID.", "power feed",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimPowerFeedsRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_dcim_virtual_chassis_get", "Get a single virtual chassis by its NetBox ID.", "virtual chassis",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimVirtualChassisRetrieve(ctx, id).Execute()
			return r, err
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
	addGetTool(s, "netbox_dcim_inventory_items_get", "Get a single inventory item by its NetBox ID.", "inventory item",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimInventoryItemsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMCableTerminationsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		CableID  int32  `json:"cable_id,omitempty" jsonschema:"Cable ID to filter by"`
		Limit    int32  `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_cable_terminations_list",
		Description: "List cable terminations in NetBox, optionally filtered by cable.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimCableTerminationsList(ctx)
		if in.CableID != 0 {
			r = r.Cable(in.CableID)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing cable terminations: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMCableTerminationsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_cable_terminations_get", "Get a single cable termination by its NetBox ID.", "cable termination",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimCableTerminationsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMConsolePortsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Console port name(s) to filter by"`
		DeviceID int32    `json:"device_id,omitempty" jsonschema:"Device ID to filter by"`
		Site     []string `json:"site,omitempty"     jsonschema:"Site name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_console_ports_list",
		Description: "List console ports in NetBox, optionally filtered by name, device, or site.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimConsolePortsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
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
			return toolError(fmt.Sprintf("listing console ports: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMConsolePortsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_console_ports_get", "Get a single console port by its NetBox ID.", "console port",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimConsolePortsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMConsoleServerPortsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Console server port name(s) to filter by"`
		DeviceID int32    `json:"device_id,omitempty" jsonschema:"Device ID to filter by"`
		Site     []string `json:"site,omitempty"     jsonschema:"Site name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_console_server_ports_list",
		Description: "List console server ports in NetBox, optionally filtered by name, device, or site.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimConsoleServerPortsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
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
			return toolError(fmt.Sprintf("listing console server ports: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMConsoleServerPortsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_console_server_ports_get", "Get a single console server port by its NetBox ID.", "console server port",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimConsoleServerPortsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMDeviceBaysList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Device bay name(s) to filter by"`
		DeviceID int32    `json:"device_id,omitempty" jsonschema:"Device ID to filter by"`
		Site     []string `json:"site,omitempty"     jsonschema:"Site name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_device_bays_list",
		Description: "List device bays in NetBox, optionally filtered by name, device, or site.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimDeviceBaysList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
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
			return toolError(fmt.Sprintf("listing device bays: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMDeviceBaysGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_device_bays_get", "Get a single device bay by its NetBox ID.", "device bay",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimDeviceBaysRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMFrontPortsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Front port name(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_front_ports_list",
		Description: "List front ports in NetBox, optionally filtered by name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimFrontPortsList(ctx)
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
			return toolError(fmt.Sprintf("listing front ports: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMFrontPortsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_front_ports_get", "Get a single front port by its NetBox ID.", "front port",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimFrontPortsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMMacAddressesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string `json:"q,omitempty"         jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty"  jsonschema:"Field to order results by (prefix with - for descending)"`
		DeviceID int32  `json:"device_id,omitempty" jsonschema:"Device ID to filter by"`
		Limit    int32  `json:"limit,omitempty"     jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty"    jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_mac_addresses_list",
		Description: "List MAC addresses in NetBox, optionally filtered by device.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimMacAddressesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing MAC addresses: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMMacAddressesGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_mac_addresses_get", "Get a single MAC address by its NetBox ID.", "MAC address",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimMacAddressesRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMModulesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		DeviceID int32    `json:"device_id,omitempty" jsonschema:"Device ID to filter by"`
		Site     []string `json:"site,omitempty"     jsonschema:"Site name or slug to filter by"`
		Status   []string `json:"status,omitempty"   jsonschema:"Module status to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_modules_list",
		Description: "List modules in NetBox, optionally filtered by device, site, or status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimModulesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
		}
		if len(in.Site) > 0 {
			r = r.Site(in.Site)
		}
		if len(in.Status) > 0 {
			r = r.Status(in.Status)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing modules: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMModulesGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_modules_get", "Get a single module by its NetBox ID.", "module",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimModulesRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMModuleBaysList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string `json:"q,omitempty"         jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty"  jsonschema:"Field to order results by (prefix with - for descending)"`
		DeviceID int32  `json:"device_id,omitempty" jsonschema:"Device ID to filter by"`
		Limit    int32  `json:"limit,omitempty"     jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty"    jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_module_bays_list",
		Description: "List module bays in NetBox, optionally filtered by device.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimModuleBaysList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing module bays: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMModuleBaysGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_module_bays_get", "Get a single module bay by its NetBox ID.", "module bay",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimModuleBaysRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMModuleTypesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q            string   `json:"q,omitempty"            jsonschema:"Free-text search"`
		Ordering     string   `json:"ordering,omitempty"     jsonschema:"Field to order results by (prefix with - for descending)"`
		Manufacturer []string `json:"manufacturer,omitempty" jsonschema:"Manufacturer name or slug to filter by"`
		Limit        int32    `json:"limit,omitempty"        jsonschema:"Maximum number of results (default 50)"`
		Offset       int32    `json:"offset,omitempty"       jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_module_types_list",
		Description: "List module types in NetBox, optionally filtered by manufacturer.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimModuleTypesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
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
			return toolError(fmt.Sprintf("listing module types: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMModuleTypesGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_module_types_get", "Get a single module type by its NetBox ID.", "module type",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimModuleTypesRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMPowerOutletsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"         jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty"  jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"      jsonschema:"Power outlet name(s) to filter by"`
		DeviceID int32    `json:"device_id,omitempty" jsonschema:"Device ID to filter by"`
		Site     []string `json:"site,omitempty"      jsonschema:"Site name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"     jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"    jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_power_outlets_list",
		Description: "List power outlets in NetBox, optionally filtered by name, device, or site.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimPowerOutletsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
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
			return toolError(fmt.Sprintf("listing power outlets: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMPowerOutletsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_power_outlets_get", "Get a single power outlet by its NetBox ID.", "power outlet",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimPowerOutletsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMPowerPortsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"         jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty"  jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"      jsonschema:"Power port name(s) to filter by"`
		DeviceID int32    `json:"device_id,omitempty" jsonschema:"Device ID to filter by"`
		Site     []string `json:"site,omitempty"      jsonschema:"Site name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"     jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"    jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_power_ports_list",
		Description: "List power ports in NetBox, optionally filtered by name, device, or site.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimPowerPortsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
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
			return toolError(fmt.Sprintf("listing power ports: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMPowerPortsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_power_ports_get", "Get a single power port by its NetBox ID.", "power port",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimPowerPortsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMRackReservationsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		RackID   int32    `json:"rack_id,omitempty"  jsonschema:"Rack ID to filter by"`
		Site     []string `json:"site,omitempty"     jsonschema:"Site name or slug to filter by"`
		Tenant   []string `json:"tenant,omitempty"   jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_rack_reservations_list",
		Description: "List rack reservations in NetBox, optionally filtered by rack, site, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimRackReservationsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.RackID != 0 {
			r = r.RackId([]int32{in.RackID})
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
			return toolError(fmt.Sprintf("listing rack reservations: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMRackReservationsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_rack_reservations_get", "Get a single rack reservation by its NetBox ID.", "rack reservation",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimRackReservationsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMRackRolesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Rack role name(s) to filter by"`
		Slug     []string `json:"slug,omitempty"     jsonschema:"Rack role slug(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_rack_roles_list",
		Description: "List rack roles in NetBox, optionally filtered by name or slug.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimRackRolesList(ctx)
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
			return toolError(fmt.Sprintf("listing rack roles: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMRackRolesGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_rack_roles_get", "Get a single rack role by its NetBox ID.", "rack role",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimRackRolesRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMRackTypesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q            string   `json:"q,omitempty"            jsonschema:"Free-text search"`
		Ordering     string   `json:"ordering,omitempty"     jsonschema:"Field to order results by (prefix with - for descending)"`
		Slug         []string `json:"slug,omitempty"         jsonschema:"Rack type slug(s) to filter by"`
		Manufacturer []string `json:"manufacturer,omitempty" jsonschema:"Manufacturer name or slug to filter by"`
		Limit        int32    `json:"limit,omitempty"        jsonschema:"Maximum number of results (default 50)"`
		Offset       int32    `json:"offset,omitempty"       jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_rack_types_list",
		Description: "List rack types in NetBox, optionally filtered by slug or manufacturer.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimRackTypesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Slug) > 0 {
			r = r.Slug(in.Slug)
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
			return toolError(fmt.Sprintf("listing rack types: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMRackTypesGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_rack_types_get", "Get a single rack type by its NetBox ID.", "rack type",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimRackTypesRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMRearPortsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Rear port name(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_rear_ports_list",
		Description: "List rear ports in NetBox, optionally filtered by name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimRearPortsList(ctx)
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
			return toolError(fmt.Sprintf("listing rear ports: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMRearPortsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_rear_ports_get", "Get a single rear port by its NetBox ID.", "rear port",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimRearPortsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMSiteGroupsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Site group name(s) to filter by"`
		Slug     []string `json:"slug,omitempty"     jsonschema:"Site group slug(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_site_groups_list",
		Description: "List site groups in NetBox, optionally filtered by name or slug.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimSiteGroupsList(ctx)
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
			return toolError(fmt.Sprintf("listing site groups: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMSiteGroupsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_site_groups_get", "Get a single site group by its NetBox ID.", "site group",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimSiteGroupsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addDCIMVirtualDeviceContextsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"         jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty"  jsonschema:"Field to order results by (prefix with - for descending)"`
		DeviceID int32    `json:"device_id,omitempty" jsonschema:"Device ID to filter by"`
		Tenant   []string `json:"tenant,omitempty"    jsonschema:"Tenant name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"     jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"    jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_virtual_device_contexts_list",
		Description: "List virtual device contexts in NetBox, optionally filtered by device or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimVirtualDeviceContextsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
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
			return toolError(fmt.Sprintf("listing virtual device contexts: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMVirtualDeviceContextsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_dcim_virtual_device_contexts_get", "Get a single virtual device context by its NetBox ID.", "virtual device context",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.DcimAPI.DcimVirtualDeviceContextsRetrieve(ctx, id).Execute()
			return r, err
		})
}
