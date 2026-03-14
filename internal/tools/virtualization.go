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
