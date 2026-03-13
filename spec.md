# NetBox MCP Server — Specification

## Overview

`netbox-mcp` is a Model Context Protocol (MCP) server written in Go that exposes NetBox data to Claude and other MCP-compatible clients. It allows an AI assistant to query network infrastructure data — devices, IP addresses, sites, prefixes, circuits, VMs, and more — directly from a NetBox instance.

The server communicates over stdio using the MCP protocol and is intended to be run as a local subprocess by a Claude client (e.g., Claude Desktop, Claude Code).

---

## Libraries

| Library | Version | Purpose |
|---|---|---|
| `github.com/modelcontextprotocol/go-sdk` | latest | MCP server framework |
| `github.com/netbox-community/go-netbox/v4` | latest | NetBox REST API client (generated from OpenAPI) |

Source checkouts:
- go-sdk: `/Users/kirtis/source/repos/go-sdk`
- go-netbox: `/Users/kirtis/source/repos/go-netbox`

---

## Module

```
module github.com/3kirt/netbox-mcp

go 1.24
```

---

## Project Structure

```
netbox-mcp/
├── cmd/
│   └── netbox-mcp/
│       └── main.go          # Entry point: load config, build server, run on stdio
├── internal/
│   ├── config/
│   │   └── config.go        # Config loading (env vars + JSON file)
│   ├── server/
│   │   └── server.go        # MCP server construction and tool registration
│   └── tools/
│       ├── circuits.go      # Circuits tools
│       ├── dcim.go          # DCIM tools
│       ├── ipam.go          # IPAM tools
│       ├── tenancy.go       # Tenancy tools
│       └── virtualization.go # Virtualization tools
├── investigation.md
├── spec.md
├── style.md
├── Makefile
├── go.mod
└── go.sum
```

---

## Configuration

Configuration follows the same approach as `netbox_cli`: a JSON file with environment variable override for the token.

**Config file** (default: `~/.netbox_mcp.json`):
```json
{
  "url": "https://netbox.example.com",
  "token": "optional-fallback-token"
}
```

**Resolution order:**
1. `NETBOX_URL` environment variable overrides the `url` field (optional — URL may also come from file only)
2. `NETBOX_TOKEN` environment variable (first choice for token)
3. `token` field in config file (fallback)
4. Error at startup if URL or token is missing

**Config struct:**
```go
// Config holds the NetBox connection settings.
type Config struct {
    URL   string
    Token string
}
```

**Flags:**
- `--config FILE` — override the default config file path

---

## Server Construction

```go
// in cmd/netbox-mcp/main.go

func main() {
    cfg, err := config.Load(configPath)
    if err != nil {
        log.Fatal(err)
    }

    client := netbox.NewAPIClientFor(cfg.URL, cfg.Token)

    s := mcp.NewServer(&mcp.Implementation{
        Name:    "netbox-mcp",
        Version: "0.1.0",
    }, nil)

    server.Register(s, client)

    if err := s.Run(context.Background(), mcp.NewStdioTransport()); err != nil {
        log.Fatal(err)
    }
}
```

Tool registration lives in `internal/server/server.go`:
```go
// Register adds all NetBox tools to the MCP server.
func Register(s *mcp.Server, client *netbox.APIClient) {
    tools.RegisterCircuits(s, client)
    tools.RegisterDCIM(s, client)
    tools.RegisterIPAM(s, client)
    tools.RegisterTenancy(s, client)
    tools.RegisterVirtualization(s, client)
}
```

---

## Tool Design Principles

- **Read-only**: all tools are list/get operations. No create, update, or delete.
- **Tool naming**: `netbox_{area}_{resource}_{action}` — e.g., `netbox_dcim_devices_list`, `netbox_ipam_ip_addresses_get`.
- **Inputs**: tool inputs use Go structs that map naturally to NetBox query parameters. Use `omitempty`-style optional fields (pointers or zero values skipped).
- **Outputs**: return JSON-marshalled NetBox response objects as `mcp.TextContent`. On error, set `IsError: true` with a descriptive message.
- **Pagination**: list tools accept `limit` (default 50, max 1000) and `offset` parameters.
- **Handler pattern**: use the typed `mcp.AddTool[In, Out]` form where practical, falling back to the raw handler only when output types are complex.

### Tool handler pattern

```go
type ListDevicesInput struct {
    Site   string `json:"site,omitempty"`
    Rack   string `json:"rack,omitempty"`
    Role   string `json:"role,omitempty"`
    Status string `json:"status,omitempty"`
    Limit  int32  `json:"limit,omitempty"`
    Offset int32  `json:"offset,omitempty"`
}

mcp.AddTool(s, &mcp.Tool{
    Name:        "netbox_dcim_devices_list",
    Description: "List devices in NetBox, optionally filtered by site, rack, role, or status.",
}, func(ctx context.Context, req *mcp.CallToolRequest, in ListDevicesInput) (*mcp.CallToolResult, any, error) {
    r := client.DcimAPI.DcimDevicesList(ctx)
    if in.Site != "" {
        r = r.Site([]string{in.Site})
    }
    // ... apply other filters
    if in.Limit > 0 {
        r = r.Limit(in.Limit)
    }
    resp, _, err := r.Execute()
    if err != nil {
        return &mcp.CallToolResult{IsError: true, Content: []mcp.Content{
            mcp.NewTextContent(fmt.Sprintf("netbox error: %v", err)),
        }}, nil, nil
    }
    data, err := json.MarshalIndent(resp, "", "  ")
    if err != nil {
        return nil, nil, fmt.Errorf("marshalling response: %w", err)
    }
    return &mcp.CallToolResult{Content: []mcp.Content{
        mcp.NewTextContent(string(data)),
    }}, nil, nil
})
```

---

## Tools

### DCIM (`internal/tools/dcim.go`)

| Tool name | Description | Key filters |
|---|---|---|
| `netbox_dcim_devices_list` | List devices | site, rack, role, status, limit, offset |
| `netbox_dcim_devices_get` | Get a device by ID | id |
| `netbox_dcim_sites_list` | List sites | name, status, region, limit, offset |
| `netbox_dcim_sites_get` | Get a site by ID | id |
| `netbox_dcim_racks_list` | List racks | site, location, status, limit, offset |
| `netbox_dcim_racks_get` | Get a rack by ID | id |
| `netbox_dcim_interfaces_list` | List device interfaces | device_id, name, type, limit, offset |
| `netbox_dcim_cables_list` | List cables | site, status, limit, offset |

### IPAM (`internal/tools/ipam.go`)

| Tool name | Description | Key filters |
|---|---|---|
| `netbox_ipam_ip_addresses_list` | List IP addresses | address, vrf, status, tenant, device, limit, offset |
| `netbox_ipam_ip_addresses_get` | Get an IP address by ID | id |
| `netbox_ipam_prefixes_list` | List prefixes | prefix, vrf, status, site, tenant, limit, offset |
| `netbox_ipam_prefixes_get` | Get a prefix by ID | id |
| `netbox_ipam_vrfs_list` | List VRFs | name, rd, tenant, limit, offset |
| `netbox_ipam_vrfs_get` | Get a VRF by ID | id |
| `netbox_ipam_vlans_list` | List VLANs | vid, name, site, group, status, limit, offset |
| `netbox_ipam_vlans_get` | Get a VLAN by ID | id |

### Circuits (`internal/tools/circuits.go`)

| Tool name | Description | Key filters |
|---|---|---|
| `netbox_circuits_circuits_list` | List circuits | provider, status, type, site, tenant, limit, offset |
| `netbox_circuits_circuits_get` | Get a circuit by ID | id |
| `netbox_circuits_providers_list` | List circuit providers | name, limit, offset |

### Tenancy (`internal/tools/tenancy.go`)

| Tool name | Description | Key filters |
|---|---|---|
| `netbox_tenancy_tenants_list` | List tenants | name, group, limit, offset |
| `netbox_tenancy_tenants_get` | Get a tenant by ID | id |

### Virtualization (`internal/tools/virtualization.go`)

| Tool name | Description | Key filters |
|---|---|---|
| `netbox_virtualization_vms_list` | List virtual machines | cluster, site, status, role, tenant, limit, offset |
| `netbox_virtualization_vms_get` | Get a VM by ID | id |
| `netbox_virtualization_clusters_list` | List clusters | name, type, site, limit, offset |

---

## Error Handling

- NetBox API errors are returned as tool-level errors (`IsError: true`), not Go errors, so Claude receives useful feedback rather than a server crash.
- Internal marshalling errors are returned as Go errors (the MCP framework converts these to error responses automatically).
- Startup errors (config missing, invalid URL, connection failure) use `log.Fatal`.

---

## Makefile Targets

```makefile
build:
	go build ./cmd/netbox-mcp

lint:
	golangci-lint run

test:
	go test ./...

install:
	go install ./cmd/netbox-mcp
```

---

## Progress

### Phase 1 — Foundation

- [x] Initialise Go module (`go.mod`)
- [x] Implement `internal/config` package
- [x] Write `cmd/netbox-mcp/main.go` entry point
- [x] Implement `internal/server/server.go` registration scaffold
- [x] Add `Makefile`

### Phase 2 — Core Tools

- [x] Implement DCIM tools (`internal/tools/dcim.go`)
- [x] Implement IPAM tools (`internal/tools/ipam.go`)

### Phase 3 — Additional Tools

- [ ] Implement Circuits tools (`internal/tools/circuits.go`)
- [ ] Implement Tenancy tools (`internal/tools/tenancy.go`)
- [ ] Implement Virtualization tools (`internal/tools/virtualization.go`)

### Phase 4 — Polish

- [ ] Add linter config (`.golangci.yml`)
- [ ] Write tests for config loading
- [ ] Write README with installation and Claude Desktop configuration example
- [ ] Test against a live NetBox instance
