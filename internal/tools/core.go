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
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the data source to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_core_data_sources_get",
		Description: "Get a single data source by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.CoreAPI.CoreDataSourcesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting data source %d: %v", in.ID, err))
		}
		return jsonResult(resp)
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
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the job to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_core_jobs_get",
		Description: "Get a single background job by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.CoreAPI.CoreJobsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting job %d: %v", in.ID, err))
		}
		return jsonResult(resp)
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
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the object change to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_core_object_changes_get",
		Description: "Get a single object change record by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		if in.ID == 0 {
			return toolError("id is required")
		}
		resp, _, err := client.CoreAPI.CoreObjectChangesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting object change %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}
