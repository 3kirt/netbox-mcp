package tools

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterUsers adds user-related tools to s.
func RegisterUsers(s *mcp.Server, client *netbox.APIClient) {
	addUsersUsersList(s, client)
	addUsersUsersGet(s, client)
	addUsersGroupsList(s, client)
	addUsersGroupsGet(s, client)
	addUsersTokensList(s, client)
	addUsersTokensGet(s, client)
}

func addUsersUsersList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Username []string `json:"username,omitempty" jsonschema:"Username(s) to filter by"`
		IsActive *bool    `json:"is_active,omitempty" jsonschema:"Filter by active status"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_users_users_list",
		Description: "List users in NetBox, optionally filtered by username or active status.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.UsersAPI.UsersUsersList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Username) > 0 {
			r = r.Username(in.Username)
		}
		if in.IsActive != nil {
			r = r.IsActive(*in.IsActive)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing users: %v", err))
		}
		return jsonResult(resp)
	})
}

func addUsersUsersGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_users_users_get", "Get a single user by their NetBox ID.", "user",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.UsersAPI.UsersUsersRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addUsersGroupsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Group name(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_users_groups_list",
		Description: "List user groups in NetBox, optionally filtered by name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.UsersAPI.UsersGroupsList(ctx)
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
			return toolError(fmt.Sprintf("listing user groups: %v", err))
		}
		return jsonResult(resp)
	})
}

func addUsersGroupsGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_users_groups_get", "Get a single user group by its NetBox ID.", "user group",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.UsersAPI.UsersGroupsRetrieve(ctx, id).Execute()
			return r, err
		})
}

func addUsersTokensList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string `json:"q,omitempty"         jsonschema:"Free-text search"`
		Ordering string `json:"ordering,omitempty"  jsonschema:"Field to order results by (prefix with - for descending)"`
		UserID   int32  `json:"user_id,omitempty"   jsonschema:"User ID to filter tokens by"`
		Limit    int32  `json:"limit,omitempty"     jsonschema:"Maximum number of results (default 50)"`
		Offset   int32  `json:"offset,omitempty"    jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_users_tokens_list",
		Description: "List API tokens in NetBox, optionally filtered by user ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.UsersAPI.UsersTokensList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if in.UserID != 0 {
			r = r.UserId([]int32{in.UserID})
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing tokens: %v", err))
		}
		return jsonResult(resp)
	})
}

func addUsersTokensGet(s *mcp.Server, client *netbox.APIClient) {
	addGetTool(s, "netbox_users_tokens_get", "Get a single API token by its NetBox ID.", "token",
		func(ctx context.Context, id int32) (any, error) {
			r, _, err := client.UsersAPI.UsersTokensRetrieve(ctx, id).Execute()
			return r, err
		})
}
