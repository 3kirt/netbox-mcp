package tools

import (
	"encoding/json"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

func TestJsonResult_validValue(t *testing.T) {
	result, _, err := jsonResult(map[string]string{"key": "value"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if result == nil || result.IsError {
		t.Fatal("expected a non-error result")
	}
	if len(result.Content) != 1 {
		t.Fatalf("expected 1 content item, got %d", len(result.Content))
	}
	text, ok := result.Content[0].(*mcp.TextContent)
	if !ok {
		t.Fatalf("expected *mcp.TextContent, got %T", result.Content[0])
	}
	var got map[string]string
	if err := json.Unmarshal([]byte(text.Text), &got); err != nil {
		t.Fatalf("content is not valid JSON: %v\ncontent: %s", err, text.Text)
	}
	if got["key"] != "value" {
		t.Errorf("got %v", got)
	}
}

func TestJsonResult_unmarshalableValueReturnsError(t *testing.T) {
	_, _, err := jsonResult(make(chan int))
	if err == nil {
		t.Fatal("expected error for unmarshalable value, got nil")
	}
}

func TestToolError(t *testing.T) {
	result, _, err := toolError("something went wrong")
	if err != nil {
		t.Fatalf("unexpected Go error: %v", err)
	}
	if !result.IsError {
		t.Fatal("expected IsError to be true")
	}
	if len(result.Content) != 1 {
		t.Fatalf("expected 1 content item, got %d", len(result.Content))
	}
	text, ok := result.Content[0].(*mcp.TextContent)
	if !ok {
		t.Fatalf("expected *mcp.TextContent, got %T", result.Content[0])
	}
	if text.Text != "something went wrong" {
		t.Errorf("got %q", text.Text)
	}
}

func TestPtrOf(t *testing.T) {
	s := "hello"
	p := ptrOf(s)
	if p == &s {
		t.Error("ptrOf should return a new pointer, not address of the original")
	}
	if *p != s {
		t.Errorf("got %q, want %q", *p, s)
	}
}

func TestPtrSlice(t *testing.T) {
	input := []string{"a", "b", "c"}
	got := ptrSlice(input)
	if len(got) != len(input) {
		t.Fatalf("len = %d, want %d", len(got), len(input))
	}
	for i, p := range got {
		if *p != input[i] {
			t.Errorf("[%d] = %q, want %q", i, *p, input[i])
		}
	}
}

func TestPtrSlice_empty(t *testing.T) {
	got := ptrSlice([]int{})
	if len(got) != 0 {
		t.Errorf("len = %d, want 0", len(got))
	}
}

func TestClampLimit(t *testing.T) {
	tests := []struct {
		in   int32
		want int32
	}{
		{0, 50},
		{-1, 50},
		{-100, 50},
		{1, 1},
		{50, 50},
		{999, 999},
		{1000, 1000},
		{1001, 1000},
		{1 << 30, 1000},
	}
	for _, tc := range tests {
		if got := clampLimit(tc.in); got != tc.want {
			t.Errorf("clampLimit(%d) = %d, want %d", tc.in, got, tc.want)
		}
	}
}
