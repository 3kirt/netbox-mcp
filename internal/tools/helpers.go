package tools

import (
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
