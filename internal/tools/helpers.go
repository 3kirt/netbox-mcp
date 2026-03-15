package tools

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// jsonResult marshals v to indented JSON and returns it as a successful tool result.
func jsonResult(v any) (*mcp.CallToolResult, any, error) {
	b, err := json.MarshalIndent(v, "", "  ")
	if err != nil {
		return nil, nil, fmt.Errorf("marshalling response: %w", err)
	}
	return &mcp.CallToolResult{
		Content: []mcp.Content{&mcp.TextContent{Text: string(b)}},
	}, nil, nil
}

// toolError returns a tool-level error result so that Claude receives useful
// feedback rather than a server crash when NetBox returns an error.
func toolError(msg string) (*mcp.CallToolResult, any, error) {
	return &mcp.CallToolResult{
		IsError: true,
		Content: []mcp.Content{&mcp.TextContent{Text: msg}},
	}, nil, nil
}

// ptrOf returns a pointer to a copy of v. It is used when a NetBox API filter
// requires a slice of pointers (e.g. []*string).
func ptrOf[T any](v T) *T { return &v }

// ptrSlice converts a slice of values to a slice of pointers. It is used when
// a NetBox API filter requires []*string rather than []string (nullable FK fields).
func ptrSlice[T any](vs []T) []*T {
	ps := make([]*T, len(vs))
	for i, v := range vs {
		ps[i] = ptrOf(v)
	}
	return ps
}

const (
	defaultLimit = int32(50)
	maxLimit     = int32(1000)
)

// clampLimit returns a sensible page size: the default (50) when limit is ≤ 0,
// the caller's value when within bounds, and a hard cap (1000) otherwise.
func clampLimit(limit int32) int32 {
	if limit <= 0 {
		return defaultLimit
	}
	if limit > maxLimit {
		return maxLimit
	}
	return limit
}

// addGetTool registers a "get by ID" tool that retrieves a single NetBox object.
// retrieve receives the request context and object ID and must return the object
// (or an error); the *http.Response from the API call is discarded.
func addGetTool(
	s *mcp.Server,
	name, description, itemName string,
	retrieve func(ctx context.Context, id int32) (any, error),
) {
	type input struct {
		ID int32 `json:"id" jsonschema:"NetBox ID to retrieve"`
	}
	mcp.AddTool(s, &mcp.Tool{Name: name, Description: description},
		func(ctx context.Context, _ *mcp.CallToolRequest, in input) (*mcp.CallToolResult, any, error) {
			if in.ID == 0 {
				return toolError("id is required")
			}
			resp, err := retrieve(ctx, in.ID)
			if err != nil {
				return toolError(fmt.Sprintf("getting %s %d: %v", itemName, in.ID, err))
			}
			return jsonResult(resp)
		})
}
