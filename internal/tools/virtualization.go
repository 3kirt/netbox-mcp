package tools

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterVirtualization adds virtualization tools to s.
func RegisterVirtualization(s *mcp.Server, client *netbox.APIClient) {
	addVirtualizationVMsList(s, client)
	addVirtualizationVMsGet(s, client)
	addVirtualizationClustersList(s, client)
	addVirtualizationClustersGet(s, client)
	addVirtualizationClusterGroupsList(s, client)
	addVirtualizationClusterGroupsGet(s, client)
	addVirtualizationClusterTypesList(s, client)
	addVirtualizationClusterTypesGet(s, client)
	addVirtualizationInterfacesList(s, client)
	addVirtualizationInterfacesGet(s, client)
	addVirtualizationVirtualDisksList(s, client)
	addVirtualizationVirtualDisksGet(s, client)
}

func addVirtualizationVMsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Cluster  string `json:"cluster,omitempty" jsonschema:"Cluster name to filter by"`
		Site     string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Status   string `json:"status,omitempty" jsonschema:"VM status (active, offline, staged, failed, decommissioning)"`
		Role     string `json:"role,omitempty" jsonschema:"Device role name or slug to filter by"`
		Tenant   string `json:"tenant,omitempty" jsonschema:"Tenant name or slug to filter by"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_vms_list",
		Description: "List virtual machines in NetBox, optionally filtered by cluster, site, status, role, or tenant.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VirtualizationAPI.VirtualizationVirtualMachinesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.Cluster != "" {
			r = r.Cluster([]string{in.Cluster})
		}
		if in.Site != "" {
			r = r.Site([]string{in.Site})
		}
		if in.Status != "" {
			r = r.Status([]string{in.Status})
		}
		if in.Role != "" {
			r = r.Role([]string{in.Role})
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
			return toolError(fmt.Sprintf("listing virtual machines: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVirtualizationVMsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the virtual machine to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_vms_get",
		Description: "Get a single virtual machine by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.VirtualizationAPI.VirtualizationVirtualMachinesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting virtual machine %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addVirtualizationClustersList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     string `json:"name,omitempty" jsonschema:"Cluster name to filter by"`
		Type     string `json:"type,omitempty" jsonschema:"Cluster type name or slug to filter by"`
		Site     string `json:"site,omitempty" jsonschema:"Site name or slug to filter by"`
		Limit    int32  `json:"limit,omitempty" jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty" jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_clusters_list",
		Description: "List virtualization clusters in NetBox, optionally filtered by name, type, or site.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VirtualizationAPI.VirtualizationClustersList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.Name != "" {
			r = r.Name([]string{in.Name})
		}
		if in.Type != "" {
			r = r.Type_([]string{in.Type})
		}
		if in.Site != "" {
			r = r.Site([]string{in.Site})
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing clusters: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVirtualizationClustersGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the cluster to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_clusters_get",
		Description: "Get a single virtualization cluster by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.VirtualizationAPI.VirtualizationClustersRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting cluster %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addVirtualizationClusterGroupsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Cluster group name(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_cluster_groups_list",
		Description: "List cluster groups in NetBox, optionally filtered by name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VirtualizationAPI.VirtualizationClusterGroupsList(ctx)
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
			return toolError(fmt.Sprintf("listing cluster groups: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVirtualizationClusterGroupsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the cluster group to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_cluster_groups_get",
		Description: "Get a single cluster group by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.VirtualizationAPI.VirtualizationClusterGroupsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting cluster group %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addVirtualizationClusterTypesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Cluster type name(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_cluster_types_list",
		Description: "List cluster types in NetBox, optionally filtered by name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VirtualizationAPI.VirtualizationClusterTypesList(ctx)
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
			return toolError(fmt.Sprintf("listing cluster types: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVirtualizationClusterTypesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the cluster type to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_cluster_types_get",
		Description: "Get a single cluster type by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.VirtualizationAPI.VirtualizationClusterTypesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting cluster type %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addVirtualizationInterfacesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q                string   `json:"q,omitempty"                  jsonschema:"Free-text search"`
		Ordering         string   `json:"ordering,omitempty"           jsonschema:"Field to order results by (prefix with - for descending)"`
		VirtualMachineID int32    `json:"virtual_machine_id,omitempty" jsonschema:"Virtual machine ID to filter by"`
		Name             []string `json:"name,omitempty"               jsonschema:"Interface name(s) to filter by"`
		Enabled          *bool    `json:"enabled,omitempty"            jsonschema:"Filter by enabled status"`
		Limit            int32    `json:"limit,omitempty"              jsonschema:"Maximum number of results (default 50)"`
		Offset           int32    `json:"offset,omitempty"             jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_interfaces_list",
		Description: "List VM interfaces in NetBox, optionally filtered by virtual machine, name, or enabled status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VirtualizationAPI.VirtualizationInterfacesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.VirtualMachineID != 0 {
			r = r.VirtualMachineId([]int32{in.VirtualMachineID})
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if in.Enabled != nil {
			r = r.Enabled(*in.Enabled)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing VM interfaces: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVirtualizationInterfacesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the VM interface to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_interfaces_get",
		Description: "Get a single VM interface by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.VirtualizationAPI.VirtualizationInterfacesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting VM interface %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addVirtualizationVirtualDisksList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q                string   `json:"q,omitempty"                  jsonschema:"Free-text search"`
		Ordering         string   `json:"ordering,omitempty"           jsonschema:"Field to order results by (prefix with - for descending)"`
		VirtualMachineID int32    `json:"virtual_machine_id,omitempty" jsonschema:"Virtual machine ID to filter by"`
		Name             []string `json:"name,omitempty"               jsonschema:"Virtual disk name(s) to filter by"`
		Limit            int32    `json:"limit,omitempty"              jsonschema:"Maximum number of results (default 50)"`
		Offset           int32    `json:"offset,omitempty"             jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_virtual_disks_list",
		Description: "List virtual disks in NetBox, optionally filtered by virtual machine or name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.VirtualizationAPI.VirtualizationVirtualDisksList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.VirtualMachineID != 0 {
			r = r.VirtualMachineId([]int32{in.VirtualMachineID})
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
			return toolError(fmt.Sprintf("listing virtual disks: %v", err))
		}
		return jsonResult(resp)
	})
}

func addVirtualizationVirtualDisksGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the virtual disk to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_virtualization_virtual_disks_get",
		Description: "Get a single virtual disk by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.VirtualizationAPI.VirtualizationVirtualDisksRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting virtual disk %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}
