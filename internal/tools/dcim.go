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
	addDCIMCablesList(s, client)
}

func addDCIMDevicesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Site   string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Role   string `json:"role,omitempty" jsonschema:"Device role name or slug to filter by"`
		Status string `json:"status,omitempty" jsonschema:"Device status (active, planned, staged, failed, inventory, decommissioning)"`
		RackID int32  `json:"rack_id,omitempty" jsonschema:"Rack ID to filter by"`
		Limit  int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_devices_list",
		Description: "List devices in NetBox, optionally filtered by site, role, status, or rack.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimDevicesList(ctx)
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
		Name   string `json:"name,omitempty" jsonschema:"Site name to filter by"`
		Status string `json:"status,omitempty" jsonschema:"Site status (active, planned, retired, staging)"`
		Region string `json:"region,omitempty" jsonschema:"Region name or slug to filter by"`
		Limit  int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_sites_list",
		Description: "List sites in NetBox, optionally filtered by name, status, or region.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimSitesList(ctx)
		if in.Name != "" {
			r = r.Name([]string{in.Name})
		}
		if in.Status != "" {
			r = r.Status([]string{in.Status})
		}
		if in.Region != "" {
			r = r.Region([]string{in.Region})
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
		if in.Site != "" {
			r = r.Site([]string{in.Site})
		}
		if in.Location != "" {
			r = r.Location([]string{in.Location})
		}
		if in.Status != "" {
			r = r.Status([]string{in.Status})
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
		if in.DeviceID != 0 {
			r = r.DeviceId([]int32{in.DeviceID})
		}
		if in.Name != "" {
			r = r.Name([]string{in.Name})
		}
		if in.Type != "" {
			r = r.Type_([]string{in.Type})
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing interfaces: %v", err))
		}
		return jsonResult(resp)
	})
}

func addDCIMCablesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Site   string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Status string `json:"status,omitempty" jsonschema:"Cable status (connected, planned, decommissioning)"`
		Limit  int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_dcim_cables_list",
		Description: "List cables in NetBox, optionally filtered by site or status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.DcimAPI.DcimCablesList(ctx)
		if in.Site != "" {
			r = r.Site([]string{in.Site})
		}
		if in.Status != "" {
			r = r.Status([]string{in.Status})
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing cables: %v", err))
		}
		return jsonResult(resp)
	})
}
