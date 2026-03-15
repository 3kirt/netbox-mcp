package tools

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"
)

// RegisterExtras adds extras-related tools to s.
func RegisterExtras(s *mcp.Server, client *netbox.APIClient) {
	addExtrasTagsList(s, client)
	addExtrasTagsGet(s, client)
	addExtrasConfigContextsList(s, client)
	addExtrasConfigContextsGet(s, client)
	addExtrasJournalEntriesList(s, client)
	addExtrasJournalEntriesGet(s, client)
	addExtrasCustomFieldsList(s, client)
	addExtrasCustomFieldsGet(s, client)
	addExtrasExportTemplatesList(s, client)
	addExtrasExportTemplatesGet(s, client)
	addExtrasWebhooksList(s, client)
	addExtrasWebhooksGet(s, client)
}

func addExtrasTagsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Tag name(s) to filter by"`
		Slug     []string `json:"slug,omitempty"     jsonschema:"Tag slug(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_tags_list",
		Description: "List tags in NetBox, optionally filtered by name or slug.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.ExtrasAPI.ExtrasTagsList(ctx)
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
			return toolError(fmt.Sprintf("listing tags: %v", err))
		}
		return jsonResult(resp)
	})
}

func addExtrasTagsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the tag to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_tags_get",
		Description: "Get a single tag by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.ExtrasAPI.ExtrasTagsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting tag %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addExtrasConfigContextsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Config context name(s) to filter by"`
		IsActive *bool    `json:"is_active,omitempty" jsonschema:"Filter by active status"`
		Site     []string `json:"site,omitempty"     jsonschema:"Site name or slug to filter by"`
		Role     []string `json:"role,omitempty"     jsonschema:"Device role name or slug to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_config_contexts_list",
		Description: "List config contexts in NetBox, optionally filtered by name, active status, site, or role.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.ExtrasAPI.ExtrasConfigContextsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if in.IsActive != nil {
			r = r.IsActive(*in.IsActive)
		}
		if len(in.Site) > 0 {
			r = r.Site(in.Site)
		}
		if len(in.Role) > 0 {
			r = r.DeviceRole(in.Role)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing config contexts: %v", err))
		}
		return jsonResult(resp)
	})
}

func addExtrasConfigContextsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the config context to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_config_contexts_get",
		Description: "Get a single config context by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.ExtrasAPI.ExtrasConfigContextsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting config context %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addExtrasJournalEntriesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q                  string   `json:"q,omitempty"                    jsonschema:"Free-text search"`
		Ordering           string   `json:"ordering,omitempty"             jsonschema:"Field to order results by (prefix with - for descending)"`
		AssignedObjectType []string `json:"assigned_object_type,omitempty" jsonschema:"Assigned object type(s) to filter by"`
		AssignedObjectID   int32    `json:"assigned_object_id,omitempty"   jsonschema:"Assigned object ID to filter by"`
		Kind               []string `json:"kind,omitempty"                 jsonschema:"Journal entry kind(s) to filter by"`
		CreatedBy          []string `json:"created_by,omitempty"           jsonschema:"Creator username(s) to filter by"`
		Limit              int32    `json:"limit,omitempty"                jsonschema:"Maximum number of results (default 50)"`
		Offset             int32    `json:"offset,omitempty"               jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_journal_entries_list",
		Description: "List journal entries in NetBox, optionally filtered by object type, object ID, kind, or creator.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.ExtrasAPI.ExtrasJournalEntriesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.AssignedObjectType) > 0 {
			r = r.AssignedObjectType(in.AssignedObjectType[0])
		}
		if in.AssignedObjectID != 0 {
			r = r.AssignedObjectId([]int32{in.AssignedObjectID})
		}
		if len(in.Kind) > 0 {
			r = r.Kind(in.Kind)
		}
		if len(in.CreatedBy) > 0 {
			r = r.CreatedBy(in.CreatedBy)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing journal entries: %v", err))
		}
		return jsonResult(resp)
	})
}

func addExtrasJournalEntriesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the journal entry to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_journal_entries_get",
		Description: "Get a single journal entry by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.ExtrasAPI.ExtrasJournalEntriesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting journal entry %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addExtrasCustomFieldsList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q          string   `json:"q,omitempty"           jsonschema:"Free-text search"`
		Ordering   string   `json:"ordering,omitempty"    jsonschema:"Field to order results by (prefix with - for descending)"`
		Name       []string `json:"name,omitempty"        jsonschema:"Custom field name(s) to filter by"`
		Type       []string `json:"type,omitempty"        jsonschema:"Custom field type(s) to filter by"`
		ObjectType string   `json:"object_type,omitempty" jsonschema:"Object type to filter by"`
		Limit      int32    `json:"limit,omitempty"       jsonschema:"Maximum number of results (default 50)"`
		Offset     int32    `json:"offset,omitempty"      jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_custom_fields_list",
		Description: "List custom fields in NetBox, optionally filtered by name, type, or object type.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.ExtrasAPI.ExtrasCustomFieldsList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if len(in.Type) > 0 {
			r = r.Type_(in.Type)
		}
		if in.ObjectType != "" {
			r = r.ObjectType(in.ObjectType)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing custom fields: %v", err))
		}
		return jsonResult(resp)
	})
}

func addExtrasCustomFieldsGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the custom field to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_custom_fields_get",
		Description: "Get a single custom field by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.ExtrasAPI.ExtrasCustomFieldsRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting custom field %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addExtrasExportTemplatesList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q          string   `json:"q,omitempty"           jsonschema:"Free-text search"`
		Ordering   string   `json:"ordering,omitempty"    jsonschema:"Field to order results by (prefix with - for descending)"`
		Name       []string `json:"name,omitempty"        jsonschema:"Export template name(s) to filter by"`
		ObjectType string   `json:"object_type,omitempty" jsonschema:"Object type to filter by"`
		Limit      int32    `json:"limit,omitempty"       jsonschema:"Maximum number of results (default 50)"`
		Offset     int32    `json:"offset,omitempty"      jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_export_templates_list",
		Description: "List export templates in NetBox, optionally filtered by name or object type.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.ExtrasAPI.ExtrasExportTemplatesList(ctx)
		if in.Q != "" {
			r = r.Q(in.Q)
		}
		if len(in.Name) > 0 {
			r = r.Name(in.Name)
		}
		if in.ObjectType != "" {
			r = r.ObjectType(in.ObjectType)
		}
		if in.Ordering != "" {
			r = r.Ordering(in.Ordering)
		}
		limit := clampLimit(in.Limit)
		resp, _, err := r.Limit(limit).Offset(in.Offset).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("listing export templates: %v", err))
		}
		return jsonResult(resp)
	})
}

func addExtrasExportTemplatesGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the export template to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_export_templates_get",
		Description: "Get a single export template by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.ExtrasAPI.ExtrasExportTemplatesRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting export template %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}

func addExtrasWebhooksList(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		Q        string   `json:"q,omitempty"        jsonschema:"Free-text search"`
		Ordering string   `json:"ordering,omitempty" jsonschema:"Field to order results by (prefix with - for descending)"`
		Name     []string `json:"name,omitempty"     jsonschema:"Webhook name(s) to filter by"`
		Limit    int32    `json:"limit,omitempty"    jsonschema:"Maximum number of results (default 50)"`
		Offset   int32    `json:"offset,omitempty"   jsonschema:"Pagination offset"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_webhooks_list",
		Description: "List webhooks in NetBox, optionally filtered by name.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		r := client.ExtrasAPI.ExtrasWebhooksList(ctx)
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
			return toolError(fmt.Sprintf("listing webhooks: %v", err))
		}
		return jsonResult(resp)
	})
}

func addExtrasWebhooksGet(s *mcp.Server, client *netbox.APIClient) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID of the webhook to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{
		Name:        "netbox_extras_webhooks_get",
		Description: "Get a single webhook by its NetBox ID.",
	}, func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
		resp, _, err := client.ExtrasAPI.ExtrasWebhooksRetrieve(ctx, in.ID).Execute()
		if err != nil {
			return toolError(fmt.Sprintf("getting webhook %d: %v", in.ID, err))
		}
		return jsonResult(resp)
	})
}
