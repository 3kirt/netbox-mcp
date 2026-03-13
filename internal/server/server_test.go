package server_test

import (
	"context"
	"slices"
	"testing"

	"github.com/modelcontextprotocol/go-sdk/mcp"
	netbox "github.com/netbox-community/go-netbox/v4"

	"github.com/3kirt/netbox-mcp/internal/server"
)

// wantTools is the complete list of tools that Register is expected to add.
var wantTools = []string{
	// Circuits
	"netbox_circuits_circuits_list",
	"netbox_circuits_circuits_get",
	"netbox_circuits_providers_list",
	"netbox_circuits_providers_get",
	// DCIM
	"netbox_dcim_devices_list",
	"netbox_dcim_devices_get",
	"netbox_dcim_sites_list",
	"netbox_dcim_sites_get",
	"netbox_dcim_racks_list",
	"netbox_dcim_racks_get",
	"netbox_dcim_interfaces_list",
	"netbox_dcim_cables_list",
	// IPAM
	"netbox_ipam_ip_addresses_list",
	"netbox_ipam_ip_addresses_get",
	"netbox_ipam_prefixes_list",
	"netbox_ipam_prefixes_get",
	"netbox_ipam_vrfs_list",
	"netbox_ipam_vrfs_get",
	"netbox_ipam_vlans_list",
	"netbox_ipam_vlans_get",
	// Tenancy
	"netbox_tenancy_tenants_list",
	"netbox_tenancy_tenants_get",
	// Virtualization
	"netbox_virtualization_vms_list",
	"netbox_virtualization_vms_get",
	"netbox_virtualization_clusters_list",
	"netbox_virtualization_clusters_get",
}

func TestRegister_allToolsPresent(t *testing.T) {
	ctx := context.Background()

	// Build a real MCP server with a stub NetBox client (no calls are made).
	s := mcp.NewServer(&mcp.Implementation{Name: "test", Version: "0.0.0"}, nil)
	client := netbox.NewAPIClientFor("http://netbox.invalid", "test-token")
	server.Register(s, client)

	// Connect via in-memory transports so we can issue a ListTools RPC.
	ct, st := mcp.NewInMemoryTransports()

	ss, err := s.Connect(ctx, st, nil)
	if err != nil {
		t.Fatalf("server connect: %v", err)
	}
	t.Cleanup(func() {
		if err := ss.Close(); err != nil {
			t.Errorf("closing server session: %v", err)
		}
	})

	c := mcp.NewClient(&mcp.Implementation{Name: "test-client", Version: "0.0.0"}, nil)
	cs, err := c.Connect(ctx, ct, nil)
	if err != nil {
		t.Fatalf("client connect: %v", err)
	}
	t.Cleanup(func() {
		if err := cs.Close(); err != nil {
			t.Errorf("closing client session: %v", err)
		}
	})

	// Collect all tool names via the paginated iterator.
	var got []string
	for tool, err := range cs.Tools(ctx, nil) {
		if err != nil {
			t.Fatalf("listing tools: %v", err)
		}
		got = append(got, tool.Name)
	}

	// Check every expected tool is present.
	for _, name := range wantTools {
		if !slices.Contains(got, name) {
			t.Errorf("tool %q not registered", name)
		}
	}

	// Check no unexpected tools are present.
	for _, name := range got {
		if !slices.Contains(wantTools, name) {
			t.Errorf("unexpected tool registered: %q", name)
		}
	}

	if len(got) != len(wantTools) {
		t.Errorf("tool count: got %d, want %d", len(got), len(wantTools))
	}
}
