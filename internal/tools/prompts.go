package tools

import (
	"context"
	"fmt"

	"github.com/modelcontextprotocol/go-sdk/mcp"
)

// RegisterPrompts registers MCP prompt templates on s.
func RegisterPrompts(s *mcp.Server) {
	s.AddPrompt(&mcp.Prompt{
		Name:        "site-inventory",
		Description: "Compile a full infrastructure inventory for a NetBox site",
		Arguments: []*mcp.PromptArgument{
			{
				Name:        "site",
				Description: "Site name or slug",
				Required:    true,
			},
		},
	}, siteInventoryPrompt)

	s.AddPrompt(&mcp.Prompt{
		Name:        "device-report",
		Description: "Generate a detailed report for a specific NetBox device",
		Arguments: []*mcp.PromptArgument{
			{
				Name:        "device",
				Description: "Device name",
				Required:    true,
			},
		},
	}, deviceReportPrompt)

	s.AddPrompt(&mcp.Prompt{
		Name:        "prefix-utilization",
		Description: "Analyze IP address utilization for a prefix",
		Arguments: []*mcp.PromptArgument{
			{
				Name:        "prefix",
				Description: "IP prefix to analyze (e.g. 10.0.0.0/24)",
				Required:    true,
			},
		},
	}, prefixUtilizationPrompt)

	s.AddPrompt(&mcp.Prompt{
		Name:        "tenant-summary",
		Description: "Summarize all NetBox resources assigned to a tenant",
		Arguments: []*mcp.PromptArgument{
			{
				Name:        "tenant",
				Description: "Tenant name or slug",
				Required:    true,
			},
		},
	}, tenantSummaryPrompt)
}

func siteInventoryPrompt(_ context.Context, req *mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	site := req.Params.Arguments["site"]
	return &mcp.GetPromptResult{
		Description: fmt.Sprintf("Infrastructure inventory for site %q", site),
		Messages: []*mcp.PromptMessage{
			{
				Role: "user",
				Content: &mcp.TextContent{
					Text: fmt.Sprintf(
						"Please compile a complete infrastructure inventory for NetBox site %q. "+
							"Use the available NetBox tools to gather:\n"+
							"- All devices (with role, platform, and status)\n"+
							"- All IP prefixes assigned to the site\n"+
							"- All circuits terminating at the site\n"+
							"- All virtual machines in clusters at the site\n\n"+
							"Present the results as a structured report grouped by category.",
						site,
					),
				},
			},
		},
	}, nil
}

func deviceReportPrompt(_ context.Context, req *mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	device := req.Params.Arguments["device"]
	return &mcp.GetPromptResult{
		Description: fmt.Sprintf("Detailed report for device %q", device),
		Messages: []*mcp.PromptMessage{
			{
				Role: "user",
				Content: &mcp.TextContent{
					Text: fmt.Sprintf(
						"Please generate a detailed report for NetBox device %q. "+
							"Use the available NetBox tools to gather:\n"+
							"- Device details (role, device type, platform, status, site, rack)\n"+
							"- All interfaces and their assigned IP addresses\n"+
							"- Cable connections to other devices\n"+
							"- Any config context or local context data\n\n"+
							"Present the results in a structured format suitable for documentation.",
						device,
					),
				},
			},
		},
	}, nil
}

func prefixUtilizationPrompt(_ context.Context, req *mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	prefix := req.Params.Arguments["prefix"]
	return &mcp.GetPromptResult{
		Description: fmt.Sprintf("IP utilization analysis for prefix %q", prefix),
		Messages: []*mcp.PromptMessage{
			{
				Role: "user",
				Content: &mcp.TextContent{
					Text: fmt.Sprintf(
						"Please analyze IP address utilization for the prefix %q in NetBox. "+
							"Use the available NetBox tools to:\n"+
							"- Look up the prefix details (VRF, tenant, site, role, status)\n"+
							"- List all IP addresses allocated within it\n"+
							"- Identify child prefixes if any\n"+
							"- Calculate utilization percentage and highlight gaps\n\n"+
							"Summarize findings and flag any concerns such as near-exhaustion.",
						prefix,
					),
				},
			},
		},
	}, nil
}

func tenantSummaryPrompt(_ context.Context, req *mcp.GetPromptRequest) (*mcp.GetPromptResult, error) {
	tenant := req.Params.Arguments["tenant"]
	return &mcp.GetPromptResult{
		Description: fmt.Sprintf("Resource summary for tenant %q", tenant),
		Messages: []*mcp.PromptMessage{
			{
				Role: "user",
				Content: &mcp.TextContent{
					Text: fmt.Sprintf(
						"Please summarize all NetBox resources assigned to tenant %q. "+
							"Use the available NetBox tools to gather:\n"+
							"- Devices owned by the tenant\n"+
							"- IP prefixes and addresses assigned to the tenant\n"+
							"- Circuits associated with the tenant\n"+
							"- Virtual machines assigned to the tenant\n\n"+
							"Present a concise summary with counts and key details per category.",
						tenant,
					),
				},
			},
		},
	}, nil
}
