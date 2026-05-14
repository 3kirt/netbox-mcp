#!/usr/bin/env python3
"""Populate a local NetBox instance with realistic dummy data for testing."""

import os
import sys

import pynetbox

NETBOX_URL = os.environ.get("NETBOX_URL", "http://localhost:8000")
NETBOX_TOKEN = os.environ.get("NETBOX_TOKEN")

if not NETBOX_TOKEN:
    sys.exit("NETBOX_TOKEN environment variable is required")

nb = pynetbox.api(NETBOX_URL, token=NETBOX_TOKEN)


def create(resource, payload, label=None):
    name = label or payload.get("name") or payload.get("prefix") or payload.get("address")
    try:
        obj = resource.create(payload)
        print(f"  + {name}")
        return obj
    except pynetbox.RequestError as e:
        if "already exists" in str(e) or "must make a unique set" in str(e):
            print(f"  ~ {name} (exists)")
            results = resource.filter(**{k: v for k, v in payload.items() if k in ("name", "slug", "prefix", "address")})
            return next(iter(results), None)
        raise


def main():
    print("=== Sites & Regions ===")
    na = create(nb.dcim.regions, {"name": "North America", "slug": "north-america"})
    eu = create(nb.dcim.regions, {"name": "Europe", "slug": "europe"})

    nyc = create(nb.dcim.sites, {"name": "New York DC", "slug": "nyc-dc", "status": "active", "region": na.id, "facility": "Equinix NY4", "physical_address": "755 Secaucus Rd, Secaucus, NJ"})
    lon = create(nb.dcim.sites, {"name": "London DC", "slug": "lon-dc", "status": "active", "region": eu.id, "facility": "Telehouse North"})
    fra = create(nb.dcim.sites, {"name": "Frankfurt DC", "slug": "fra-dc", "status": "active", "region": eu.id, "facility": "Interxion FRA1"})

    print("\n=== Locations & Racks ===")
    nyc_floor1 = create(nb.dcim.locations, {"name": "Floor 1", "slug": "nyc-floor-1", "site": nyc.id})
    lon_floor1 = create(nb.dcim.locations, {"name": "Floor 1", "slug": "lon-floor-1", "site": lon.id})

    racks = [
        create(nb.dcim.racks, {"name": "NYC-A01", "site": nyc.id, "location": nyc_floor1.id, "u_height": 42, "status": "active"}),
        create(nb.dcim.racks, {"name": "NYC-A02", "site": nyc.id, "location": nyc_floor1.id, "u_height": 42, "status": "active"}),
        create(nb.dcim.racks, {"name": "LON-A01", "site": lon.id, "location": lon_floor1.id, "u_height": 48, "status": "active"}),
        create(nb.dcim.racks, {"name": "FRA-A01", "site": fra.id, "u_height": 42, "status": "active"}),
    ]

    print("\n=== Manufacturers & Device Types ===")
    cisco = create(nb.dcim.manufacturers, {"name": "Cisco", "slug": "cisco"})
    juniper = create(nb.dcim.manufacturers, {"name": "Juniper", "slug": "juniper"})
    dell = create(nb.dcim.manufacturers, {"name": "Dell", "slug": "dell"})

    nexus = create(nb.dcim.device_types, {"manufacturer": cisco.id, "model": "Nexus 9300", "slug": "nexus-9300", "u_height": 1})
    asr = create(nb.dcim.device_types, {"manufacturer": cisco.id, "model": "ASR 1001-X", "slug": "asr-1001-x", "u_height": 1})
    qfx = create(nb.dcim.device_types, {"manufacturer": juniper.id, "model": "QFX5100", "slug": "qfx5100", "u_height": 1})
    mx = create(nb.dcim.device_types, {"manufacturer": juniper.id, "model": "MX204", "slug": "mx204", "u_height": 1})
    r750 = create(nb.dcim.device_types, {"manufacturer": dell.id, "model": "PowerEdge R750", "slug": "poweredge-r750", "u_height": 2})

    print("\n=== Device Roles ===")
    role_spine = create(nb.dcim.device_roles, {"name": "Spine Switch", "slug": "spine-switch", "color": "aa1409"})
    role_leaf = create(nb.dcim.device_roles, {"name": "Leaf Switch", "slug": "leaf-switch", "color": "f44336"})
    role_router = create(nb.dcim.device_roles, {"name": "Core Router", "slug": "core-router", "color": "ff9800"})
    role_server = create(nb.dcim.device_roles, {"name": "Server", "slug": "server", "color": "2196f3"})

    print("\n=== Devices ===")
    devices = [
        create(nb.dcim.devices, {"name": "nyc-spine-01", "device_type": nexus.id, "role": role_spine.id, "site": nyc.id, "rack": racks[0].id, "position": 40, "face": "front", "status": "active"}),
        create(nb.dcim.devices, {"name": "nyc-spine-02", "device_type": nexus.id, "role": role_spine.id, "site": nyc.id, "rack": racks[0].id, "position": 38, "face": "front", "status": "active"}),
        create(nb.dcim.devices, {"name": "nyc-leaf-01", "device_type": qfx.id, "role": role_leaf.id, "site": nyc.id, "rack": racks[0].id, "position": 36, "face": "front", "status": "active"}),
        create(nb.dcim.devices, {"name": "nyc-leaf-02", "device_type": qfx.id, "role": role_leaf.id, "site": nyc.id, "rack": racks[1].id, "position": 40, "face": "front", "status": "active"}),
        create(nb.dcim.devices, {"name": "nyc-router-01", "device_type": asr.id, "role": role_router.id, "site": nyc.id, "rack": racks[0].id, "position": 42, "face": "front", "status": "active"}),
        create(nb.dcim.devices, {"name": "nyc-server-01", "device_type": r750.id, "role": role_server.id, "site": nyc.id, "rack": racks[1].id, "position": 1, "face": "front", "status": "active"}),
        create(nb.dcim.devices, {"name": "nyc-server-02", "device_type": r750.id, "role": role_server.id, "site": nyc.id, "rack": racks[1].id, "position": 3, "face": "front", "status": "active"}),
        create(nb.dcim.devices, {"name": "lon-spine-01", "device_type": nexus.id, "role": role_spine.id, "site": lon.id, "rack": racks[2].id, "position": 46, "face": "front", "status": "active"}),
        create(nb.dcim.devices, {"name": "lon-leaf-01", "device_type": qfx.id, "role": role_leaf.id, "site": lon.id, "rack": racks[2].id, "position": 44, "face": "front", "status": "active"}),
        create(nb.dcim.devices, {"name": "lon-router-01", "device_type": mx.id, "role": role_router.id, "site": lon.id, "rack": racks[2].id, "position": 48, "face": "front", "status": "active"}),
        create(nb.dcim.devices, {"name": "fra-spine-01", "device_type": nexus.id, "role": role_spine.id, "site": fra.id, "rack": racks[3].id, "position": 40, "face": "front", "status": "active"}),
        create(nb.dcim.devices, {"name": "fra-server-01", "device_type": r750.id, "role": role_server.id, "site": fra.id, "rack": racks[3].id, "position": 1, "face": "front", "status": "active"}),
    ]

    print("\n=== RIR & Aggregates ===")
    arin = create(nb.ipam.rirs, {"name": "ARIN", "slug": "arin", "is_private": False})
    rfc1918 = create(nb.ipam.rirs, {"name": "RFC 1918", "slug": "rfc1918", "is_private": True})

    create(nb.ipam.aggregates, {"prefix": "10.0.0.0/8", "rir": rfc1918.id, "description": "Private RFC1918"})
    create(nb.ipam.aggregates, {"prefix": "172.16.0.0/12", "rir": rfc1918.id, "description": "Private RFC1918"})
    create(nb.ipam.aggregates, {"prefix": "192.168.0.0/16", "rir": rfc1918.id, "description": "Private RFC1918"})

    print("\n=== VRFs ===")
    vrf_global = create(nb.ipam.vrfs, {"name": "Global", "rd": "65000:0"})
    vrf_mgmt = create(nb.ipam.vrfs, {"name": "Management", "rd": "65000:100"})

    print("\n=== Prefixes ===")
    create(nb.ipam.prefixes, {"prefix": "10.0.0.0/8", "status": "container", "description": "Global private space"})
    create(nb.ipam.prefixes, {"prefix": "10.0.0.0/16", "site": nyc.id, "status": "active", "description": "NYC DC block", "vrf": vrf_global.id})
    create(nb.ipam.prefixes, {"prefix": "10.1.0.0/16", "site": lon.id, "status": "active", "description": "LON DC block", "vrf": vrf_global.id})
    create(nb.ipam.prefixes, {"prefix": "10.2.0.0/16", "site": fra.id, "status": "active", "description": "FRA DC block", "vrf": vrf_global.id})
    create(nb.ipam.prefixes, {"prefix": "10.0.0.0/24", "site": nyc.id, "status": "active", "description": "NYC management", "vrf": vrf_mgmt.id})
    create(nb.ipam.prefixes, {"prefix": "10.0.1.0/24", "site": nyc.id, "status": "active", "description": "NYC servers", "vrf": vrf_global.id})
    create(nb.ipam.prefixes, {"prefix": "10.0.10.0/24", "site": nyc.id, "status": "active", "description": "NYC spine loopbacks", "vrf": vrf_global.id})
    create(nb.ipam.prefixes, {"prefix": "10.1.0.0/24", "site": lon.id, "status": "active", "description": "LON management", "vrf": vrf_mgmt.id})
    create(nb.ipam.prefixes, {"prefix": "10.2.0.0/24", "site": fra.id, "status": "active", "description": "FRA management", "vrf": vrf_mgmt.id})

    print("\n=== IP Addresses ===")
    create(nb.ipam.ip_addresses, {"address": "10.0.0.1/24", "status": "active", "description": "nyc-spine-01 mgmt", "vrf": vrf_mgmt.id})
    create(nb.ipam.ip_addresses, {"address": "10.0.0.2/24", "status": "active", "description": "nyc-spine-02 mgmt", "vrf": vrf_mgmt.id})
    create(nb.ipam.ip_addresses, {"address": "10.0.0.3/24", "status": "active", "description": "nyc-leaf-01 mgmt", "vrf": vrf_mgmt.id})
    create(nb.ipam.ip_addresses, {"address": "10.0.0.4/24", "status": "active", "description": "nyc-leaf-02 mgmt", "vrf": vrf_mgmt.id})
    create(nb.ipam.ip_addresses, {"address": "10.0.0.5/24", "status": "active", "description": "nyc-router-01 mgmt", "vrf": vrf_mgmt.id})
    create(nb.ipam.ip_addresses, {"address": "10.0.1.10/24", "status": "active", "description": "nyc-server-01", "vrf": vrf_global.id})
    create(nb.ipam.ip_addresses, {"address": "10.0.1.11/24", "status": "active", "description": "nyc-server-02", "vrf": vrf_global.id})
    create(nb.ipam.ip_addresses, {"address": "10.0.10.1/32", "status": "active", "description": "nyc-spine-01 lo0", "vrf": vrf_global.id})
    create(nb.ipam.ip_addresses, {"address": "10.0.10.2/32", "status": "active", "description": "nyc-spine-02 lo0", "vrf": vrf_global.id})
    create(nb.ipam.ip_addresses, {"address": "10.1.0.1/24", "status": "active", "description": "lon-spine-01 mgmt", "vrf": vrf_mgmt.id})
    create(nb.ipam.ip_addresses, {"address": "10.2.0.1/24", "status": "active", "description": "fra-spine-01 mgmt", "vrf": vrf_mgmt.id})

    print("\n=== VLANs ===")
    vlan_group = create(nb.ipam.vlan_groups, {"name": "NYC VLANs", "slug": "nyc-vlans", "scope_type": "dcim.site", "scope_id": nyc.id})
    create(nb.ipam.vlans, {"vid": 10, "name": "Management", "site": nyc.id, "status": "active"})
    create(nb.ipam.vlans, {"vid": 20, "name": "Servers", "site": nyc.id, "status": "active"})
    create(nb.ipam.vlans, {"vid": 30, "name": "Storage", "site": nyc.id, "status": "active"})
    create(nb.ipam.vlans, {"vid": 100, "name": "Transit", "site": nyc.id, "status": "active"})
    create(nb.ipam.vlans, {"vid": 10, "name": "Management", "site": lon.id, "status": "active"})
    create(nb.ipam.vlans, {"vid": 20, "name": "Servers", "site": lon.id, "status": "active"})

    print("\n=== ASNs ===")
    create(nb.ipam.asns, {"asn": 65000, "rir": arin.id, "description": "Primary AS"})
    create(nb.ipam.asns, {"asn": 65001, "rir": arin.id, "description": "NYC DC AS"})
    create(nb.ipam.asns, {"asn": 65002, "rir": arin.id, "description": "LON DC AS"})

    print("\n=== Tenants ===")
    infra = create(nb.tenancy.tenants, {"name": "Infrastructure", "slug": "infrastructure"})
    app_team = create(nb.tenancy.tenants, {"name": "App Team", "slug": "app-team"})
    create(nb.tenancy.tenants, {"name": "Security Team", "slug": "security-team"})

    print("\n=== Contacts ===")
    noc = create(nb.tenancy.contact_roles, {"name": "NOC", "slug": "noc"})
    owner = create(nb.tenancy.contact_roles, {"name": "Owner", "slug": "owner"})
    create(nb.tenancy.contacts, {"name": "Alice Smith", "email": "alice@example.com", "phone": "+1-212-555-0101"})
    create(nb.tenancy.contacts, {"name": "Bob Jones", "email": "bob@example.com", "phone": "+44-20-5550-0102"})

    print("\n=== Cables ===")
    # Cables need interfaces — skip if devices have no interfaces pre-created by device type template
    # Leaving this for manual wiring or a follow-up script.

    print("\n=== Done! ===")
    print(f"\nNetBox UI: {NETBOX_URL}")


if __name__ == "__main__":
    main()
