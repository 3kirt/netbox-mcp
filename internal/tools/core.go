package tools

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterCore adds core-related tools to s.
func RegisterCore(s *mcp.Server, client *netbox.APIClient) {
	addCoreDataSourcesList(s, client)
	addCoreDataSourcesGet(s, client)
	addCoreJobsList(s, client)
	addCoreJobsGet(s, client)
	addCoreObjectChangesList(s, client)
	addCoreObjectChangesGet(s, client)
}

func addCoreDataSourcesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Data source name(s) to filter by"`
		Status   []string `json:"status,omitempty"   jsonschema:"Data source status(es) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_core_data_sources_list",
		Description: "List data sources in NetBox, optionally filtered by name or status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.CoreAPI.CoreDataSourcesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
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
			return toolError(fmt.Sprintf("listing data sources: %v", err))
		}
		return jsonResult(resp)
	})
}

func addCoreDataSourcesGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_core_data_sources_get", "Get a single data source by its NetBox ID.", "data source",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.CoreAPI.CoreDataSourcesRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addCoreJobsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Status   []string `json:"status,omitempty"   jsonschema:"Job status(es) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_core_jobs_list",
		Description: "List background jobs in NetBox, optionally filtered by status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.CoreAPI.CoreJobsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
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
			return toolError(fmt.Sprintf("listing jobs: %v", err))
		}
		return jsonResult(resp)
	})
}

func addCoreJobsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_core_jobs_get", "Get a single background job by its NetBox ID.", "job",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.CoreAPI.CoreJobsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addCoreObjectChangesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		User     []string `json:"user,omitempty"     jsonschema:"Username(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_core_object_changes_list",
		Description: "List object changes (audit log) in NetBox, optionally filtered by user.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.CoreAPI.CoreObjectChangesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.User) > 0 {
			r = r.User(in.User)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing object changes: %v", err))
		}
		return jsonResult(resp)
	})
}

func addCoreObjectChangesGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_core_object_changes_get", "Get a single object change record by its NetBox ID.", "object change",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.CoreAPI.CoreObjectChangesRetrieve(ctx, id).Execute()
			return r, err
		})
}
