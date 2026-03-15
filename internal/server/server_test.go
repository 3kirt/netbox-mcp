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
	"netbox_circuits_circuit_types_list",
	"netbox_circuits_circuit_types_get",
	"netbox_circuits_circuit_terminations_list",
	"netbox_circuits_circuit_terminations_get",
	"netbox_circuits_provider_accounts_list",
	"netbox_circuits_provider_accounts_get",
	"netbox_circuits_provider_networks_list",
	"netbox_circuits_provider_networks_get",
	// DCIM
	"netbox_dcim_devices_list",
	"netbox_dcim_devices_get",
	"netbox_dcim_sites_list",
	"netbox_dcim_sites_get",
	"netbox_dcim_racks_list",
	"netbox_dcim_racks_get",
	"netbox_dcim_interfaces_list",
	"netbox_dcim_interfaces_get",
	"netbox_dcim_cables_list",
	"netbox_dcim_cables_get",
	"netbox_dcim_regions_list",
	"netbox_dcim_regions_get",
	"netbox_dcim_locations_list",
	"netbox_dcim_locations_get",
	"netbox_dcim_manufacturers_list",
	"netbox_dcim_manufacturers_get",
	"netbox_dcim_device_types_list",
	"netbox_dcim_device_types_get",
	"netbox_dcim_device_roles_list",
	"netbox_dcim_device_roles_get",
	"netbox_dcim_platforms_list",
	"netbox_dcim_platforms_get",
	"netbox_dcim_power_panels_list",
	"netbox_dcim_power_panels_get",
	"netbox_dcim_power_feeds_list",
	"netbox_dcim_power_feeds_get",
	"netbox_dcim_virtual_chassis_list",
	"netbox_dcim_virtual_chassis_get",
	"netbox_dcim_inventory_items_list",
	"netbox_dcim_inventory_items_get",
	// IPAM
	"netbox_ipam_ip_addresses_list",
	"netbox_ipam_ip_addresses_get",
	"netbox_ipam_prefixes_list",
	"netbox_ipam_prefixes_get",
	"netbox_ipam_vrfs_list",
	"netbox_ipam_vrfs_get",
	"netbox_ipam_vlans_list",
	"netbox_ipam_vlans_get",
	"netbox_ipam_aggregates_list",
	"netbox_ipam_aggregates_get",
	"netbox_ipam_ip_ranges_list",
	"netbox_ipam_ip_ranges_get",
	"netbox_ipam_route_targets_list",
	"netbox_ipam_route_targets_get",
	"netbox_ipam_rirs_list",
	"netbox_ipam_rirs_get",
	"netbox_ipam_vlan_groups_list",
	"netbox_ipam_vlan_groups_get",
	"netbox_ipam_services_list",
	"netbox_ipam_services_get",
	// Tenancy
	"netbox_tenancy_tenants_list",
	"netbox_tenancy_tenants_get",
	"netbox_tenancy_tenant_groups_list",
	"netbox_tenancy_tenant_groups_get",
	"netbox_tenancy_contacts_list",
	"netbox_tenancy_contacts_get",
	"netbox_tenancy_contact_groups_list",
	"netbox_tenancy_contact_groups_get",
	"netbox_tenancy_contact_roles_list",
	"netbox_tenancy_contact_roles_get",
	// Virtualization
	"netbox_virtualization_vms_list",
	"netbox_virtualization_vms_get",
	"netbox_virtualization_clusters_list",
	"netbox_virtualization_clusters_get",
	"netbox_virtualization_cluster_groups_list",
	"netbox_virtualization_cluster_groups_get",
	"netbox_virtualization_cluster_types_list",
	"netbox_virtualization_cluster_types_get",
	"netbox_virtualization_interfaces_list",
	"netbox_virtualization_interfaces_get",
	"netbox_virtualization_virtual_disks_list",
	"netbox_virtualization_virtual_disks_get",
	// VPN
	"netbox_vpn_tunnels_list",
	"netbox_vpn_tunnels_get",
	"netbox_vpn_tunnel_groups_list",
	"netbox_vpn_tunnel_groups_get",
	"netbox_vpn_l2vpns_list",
	"netbox_vpn_l2vpns_get",
	"netbox_vpn_ike_policies_list",
	"netbox_vpn_ike_policies_get",
	"netbox_vpn_ipsec_policies_list",
	"netbox_vpn_ipsec_policies_get",
	// Wireless
	"netbox_wireless_lans_list",
	"netbox_wireless_lans_get",
	"netbox_wireless_lan_groups_list",
	"netbox_wireless_lan_groups_get",
	"netbox_wireless_links_list",
	"netbox_wireless_links_get",
	// Extras
	"netbox_extras_tags_list",
	"netbox_extras_tags_get",
	"netbox_extras_config_contexts_list",
	"netbox_extras_config_contexts_get",
	"netbox_extras_journal_entries_list",
	"netbox_extras_journal_entries_get",
	"netbox_extras_custom_fields_list",
	"netbox_extras_custom_fields_get",
	"netbox_extras_export_templates_list",
	"netbox_extras_export_templates_get",
	"netbox_extras_webhooks_list",
	"netbox_extras_webhooks_get",
	// Core
	"netbox_core_data_sources_list",
	"netbox_core_data_sources_get",
	"netbox_core_jobs_list",
	"netbox_core_jobs_get",
	"netbox_core_object_changes_list",
	"netbox_core_object_changes_get",
	// Users
	"netbox_users_users_list",
	"netbox_users_users_get",
	"netbox_users_groups_list",
	"netbox_users_groups_get",
	"netbox_users_tokens_list",
	"netbox_users_tokens_get",
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
