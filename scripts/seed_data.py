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


_DUPE_PHRASES = (
    "already exists",
    "must make a unique set",
    "Constraint",
    "is violated",
    "must be unique",
    "is already occupied",   # rack position conflict when device name already exists
    "already covered by",    # aggregate overlap when aggregate already exists
    "Duplicate prefix",      # prefix duplicate in global table
)


def get_or_create(resource, lookup, payload, label=None):
    """Filter first; only POST if nothing found. For resources with no unique constraint."""
    name = label or payload.get("name") or payload.get("prefix") or payload.get("address")
    existing = list(resource.filter(**lookup))
    if existing:
        print(f"  ~ {name} (exists)")
        return existing[0]
    obj = resource.create(payload)
    print(f"  + {name}")
    return obj


def create(resource, payload, label=None, filter_keys=None):
    name = (
        label
        or payload.get("name")
        or payload.get("model")
        or payload.get("prefix")
        or payload.get("address")
    )
    try:
        obj = resource.create(payload)
        print(f"  + {name}")
        return obj
    except pynetbox.RequestError as e:
        if any(p in str(e) for p in _DUPE_PHRASES):
            print(f"  ~ {name} (exists)")
            keys = filter_keys or ("name", "slug", "prefix", "address", "model")
            results = resource.filter(
                **{k: v for k, v in payload.items() if k in keys and v is not None}
            )
            return next(iter(results), None)
        raise


def mkiface(device, name, itype="virtual", mgmt_only=False):
    label = f"{device.name}/{name}"
    try:
        obj = nb.dcim.interfaces.create(
            {"device": device.id, "name": name, "type": itype, "mgmt_only": mgmt_only}
        )
        print(f"  + {label}")
        return obj
    except pynetbox.RequestError as e:
        if "already exists" in str(e) or "must make a unique set" in str(e):
            print(f"  ~ {label} (exists)")
            return next(iter(nb.dcim.interfaces.filter(device_id=device.id, name=name)), None)
        raise


def mkvm_iface(vm, name):
    label = f"{vm.name}/{name}"
    try:
        obj = nb.virtualization.interfaces.create(
            {"virtual_machine": vm.id, "name": name, "type": "virtual"}
        )
        print(f"  + {label}")
        return obj
    except pynetbox.RequestError as e:
        if "already exists" in str(e) or "must make a unique set" in str(e):
            print(f"  ~ {label} (exists)")
            return next(
                iter(nb.virtualization.interfaces.filter(virtual_machine_id=vm.id, name=name)),
                None,
            )
        raise


def wire_ip(address, vrf, device, iface_name, description="", role=None, make_primary=False):
    """Create an IP address, assign it to a device interface, and optionally set it as primary."""
    iface = next(iter(nb.dcim.interfaces.filter(device_id=device.id, name=iface_name)), None)
    if not iface:
        print(f"  ! WARNING: interface {device.name}/{iface_name} not found, skipping {address}")
        return None

    existing = list(nb.ipam.ip_addresses.filter(address=address, vrf_id=vrf.id))
    if existing:
        ip = existing[0]
        print(f"  ~ {address}")
    else:
        payload = {
            "address": address,
            "status": "active",
            "vrf": vrf.id,
            "description": description,
            "assigned_object_type": "dcim.interface",
            "assigned_object_id": iface.id,
        }
        if role:
            payload["role"] = role
        ip = nb.ipam.ip_addresses.create(payload)
        print(f"  + {address}")

    if ip.assigned_object_id != iface.id:
        ip.assigned_object_type = "dcim.interface"
        ip.assigned_object_id = iface.id
        ip.save()

    if make_primary:
        dev = nb.dcim.devices.get(device.id)
        if not dev.primary_ip4 or dev.primary_ip4.id != ip.id:
            dev.primary_ip4 = ip.id
            dev.save()
            print(f"    → primary IP set on {device.name}")

    return ip


def wire_vm_ip(address, vrf, vm, iface, description=""):
    """Create an IP address, assign it to a VM interface, and set it as primary."""
    existing = list(nb.ipam.ip_addresses.filter(address=address, vrf_id=vrf.id))
    if existing:
        ip = existing[0]
        print(f"  ~ {address}")
    else:
        ip = nb.ipam.ip_addresses.create(
            {
                "address": address,
                "status": "active",
                "vrf": vrf.id,
                "description": description,
                "assigned_object_type": "virtualization.vminterface",
                "assigned_object_id": iface.id,
            }
        )
        print(f"  + {address}")

    if ip.assigned_object_id != iface.id:
        try:
            ip.assigned_object_type = "virtualization.vminterface"
            ip.assigned_object_id = iface.id
            ip.save()
        except pynetbox.RequestError as e:
            if "Cannot reassign" in str(e):
                print(f"    ! {ip.address} already primary elsewhere, skipping reassign")
            else:
                raise

    vm_obj = nb.virtualization.virtual_machines.get(vm.id)
    if not vm_obj.primary_ip4 or vm_obj.primary_ip4.id != ip.id:
        try:
            vm_obj.primary_ip4 = ip.id
            vm_obj.save()
            print(f"    → primary IP set on {vm.name}")
        except pynetbox.RequestError as e:
            print(f"    ! could not set primary IP on {vm.name}: {e}")

    return ip


def main():
    # ------------------------------------------------------------------
    print("=== Sites & Regions ===")
    na = create(nb.dcim.regions, {"name": "North America", "slug": "north-america"})
    eu = create(nb.dcim.regions, {"name": "Europe", "slug": "europe"})

    nyc = create(nb.dcim.sites, {
        "name": "New York DC", "slug": "nyc-dc", "status": "active",
        "region": na.id, "facility": "Equinix NY4",
        "physical_address": "755 Secaucus Rd, Secaucus, NJ",
    })
    lon = create(nb.dcim.sites, {
        "name": "London DC", "slug": "lon-dc", "status": "active",
        "region": eu.id, "facility": "Telehouse North",
    })
    fra = create(nb.dcim.sites, {
        "name": "Frankfurt DC", "slug": "fra-dc", "status": "active",
        "region": eu.id, "facility": "Interxion FRA1",
    })

    # ------------------------------------------------------------------
    print("\n=== Locations & Racks ===")
    nyc_floor1 = create(nb.dcim.locations, {"name": "Floor 1", "slug": "nyc-floor-1", "site": nyc.id})
    lon_floor1 = create(nb.dcim.locations, {"name": "Floor 1", "slug": "lon-floor-1", "site": lon.id})
    fra_floor1 = create(nb.dcim.locations, {"name": "Floor 1", "slug": "fra-floor-1", "site": fra.id})

    rack_nyc_a01 = create(nb.dcim.racks, {"name": "NYC-A01", "site": nyc.id, "location": nyc_floor1.id, "u_height": 42, "status": "active"})
    rack_nyc_a02 = create(nb.dcim.racks, {"name": "NYC-A02", "site": nyc.id, "location": nyc_floor1.id, "u_height": 42, "status": "active"})
    rack_lon_a01 = create(nb.dcim.racks, {"name": "LON-A01", "site": lon.id, "location": lon_floor1.id, "u_height": 48, "status": "active"})
    rack_fra_a01 = create(nb.dcim.racks, {"name": "FRA-A01", "site": fra.id, "location": fra_floor1.id, "u_height": 42, "status": "active"})

    # ------------------------------------------------------------------
    print("\n=== Manufacturers & Device Types ===")
    cisco   = create(nb.dcim.manufacturers, {"name": "Cisco",   "slug": "cisco"})
    juniper = create(nb.dcim.manufacturers, {"name": "Juniper", "slug": "juniper"})
    dell    = create(nb.dcim.manufacturers, {"name": "Dell",    "slug": "dell"})

    nexus = create(nb.dcim.device_types, {"manufacturer": cisco.id,   "model": "Nexus 9300",   "slug": "nexus-9300",    "u_height": 1})
    asr   = create(nb.dcim.device_types, {"manufacturer": cisco.id,   "model": "ASR 1001-X",   "slug": "asr-1001-x",   "u_height": 1})
    qfx   = create(nb.dcim.device_types, {"manufacturer": juniper.id, "model": "QFX5100",      "slug": "qfx5100",       "u_height": 1})
    mx    = create(nb.dcim.device_types, {"manufacturer": juniper.id, "model": "MX204",        "slug": "mx204",         "u_height": 1})
    r750  = create(nb.dcim.device_types, {"manufacturer": dell.id,    "model": "PowerEdge R750","slug": "poweredge-r750","u_height": 2})

    # ------------------------------------------------------------------
    print("\n=== Platforms ===")
    nxos   = create(nb.dcim.platforms, {"name": "Cisco NX-OS",   "slug": "nxos",        "manufacturer": cisco.id})
    iosxe  = create(nb.dcim.platforms, {"name": "Cisco IOS-XE",  "slug": "iosxe",       "manufacturer": cisco.id})
    junos  = create(nb.dcim.platforms, {"name": "Juniper Junos", "slug": "junos",       "manufacturer": juniper.id})
    ubuntu = create(nb.dcim.platforms, {"name": "Ubuntu 22.04",  "slug": "ubuntu-2204"})

    # ------------------------------------------------------------------
    print("\n=== Device Roles ===")
    role_spine  = create(nb.dcim.device_roles, {"name": "Spine Switch", "slug": "spine-switch", "color": "aa1409"})
    role_leaf   = create(nb.dcim.device_roles, {"name": "Leaf Switch",  "slug": "leaf-switch",  "color": "f44336"})
    role_router = create(nb.dcim.device_roles, {"name": "Core Router",  "slug": "core-router",  "color": "ff9800"})
    role_server = create(nb.dcim.device_roles, {"name": "Server",       "slug": "server",       "color": "2196f3"})

    # ------------------------------------------------------------------
    print("\n=== Tags ===")
    tag_prod     = create(nb.extras.tags, {"name": "production",  "slug": "production",  "color": "4caf50"})
    tag_mgmt     = create(nb.extras.tags, {"name": "management",  "slug": "management",  "color": "2196f3"})
    tag_backbone = create(nb.extras.tags, {"name": "backbone",    "slug": "backbone",    "color": "9c27b0"})

    # ------------------------------------------------------------------
    print("\n=== Tenants ===")
    infra    = create(nb.tenancy.tenants, {"name": "Infrastructure", "slug": "infrastructure"})
    app_team = create(nb.tenancy.tenants, {"name": "App Team",       "slug": "app-team"})
    sec_team = create(nb.tenancy.tenants, {"name": "Security Team",  "slug": "security-team"})

    # ------------------------------------------------------------------
    print("\n=== Contacts ===")
    noc_role   = create(nb.tenancy.contact_roles, {"name": "NOC",   "slug": "noc"})
    owner_role = create(nb.tenancy.contact_roles, {"name": "Owner", "slug": "owner"})
    get_or_create(nb.tenancy.contacts, {"name": "Alice Smith"}, {"name": "Alice Smith", "email": "alice@example.com", "phone": "+1-212-555-0101"})
    get_or_create(nb.tenancy.contacts, {"name": "Bob Jones"},   {"name": "Bob Jones",   "email": "bob@example.com",   "phone": "+44-20-5550-0102"})
    get_or_create(nb.tenancy.contacts, {"name": "Carla Reyes"}, {"name": "Carla Reyes", "email": "carla@example.com", "phone": "+49-69-5550-0103"})

    # ------------------------------------------------------------------
    print("\n=== Devices ===")
    device_defs = [
        # (name, device_type, role, site, rack, position, platform, tenant)
        ("nyc-spine-01",  nexus, role_spine,  nyc, rack_nyc_a01, 40, nxos,  infra),
        ("nyc-spine-02",  nexus, role_spine,  nyc, rack_nyc_a01, 38, nxos,  infra),
        ("nyc-leaf-01",   qfx,   role_leaf,   nyc, rack_nyc_a01, 36, junos, infra),
        ("nyc-leaf-02",   qfx,   role_leaf,   nyc, rack_nyc_a02, 40, junos, infra),
        ("nyc-router-01", asr,   role_router, nyc, rack_nyc_a01, 42, iosxe, infra),
        ("nyc-server-01", r750,  role_server, nyc, rack_nyc_a02,  1, ubuntu,infra),
        ("nyc-server-02", r750,  role_server, nyc, rack_nyc_a02,  3, ubuntu,infra),
        ("lon-spine-01",  nexus, role_spine,  lon, rack_lon_a01, 46, nxos,  infra),
        ("lon-leaf-01",   qfx,   role_leaf,   lon, rack_lon_a01, 44, junos, infra),
        ("lon-router-01", mx,    role_router, lon, rack_lon_a01, 48, junos, infra),
        ("fra-spine-01",  nexus, role_spine,  fra, rack_fra_a01, 40, nxos,  infra),
        ("fra-server-01", r750,  role_server, fra, rack_fra_a01,  1, ubuntu,infra),
    ]
    d = {}
    for name, dtype, role, site, rack, pos, platform, tenant in device_defs:
        d[name] = create(nb.dcim.devices, {
            "name": name, "device_type": dtype.id, "role": role.id,
            "site": site.id, "rack": rack.id, "position": pos, "face": "front",
            "status": "active", "platform": platform.id, "tenant": tenant.id,
        })

    # ------------------------------------------------------------------
    print("\n=== RIR & Aggregates ===")
    arin    = create(nb.ipam.rirs, {"name": "ARIN",     "slug": "arin",    "is_private": False})
    rfc1918 = create(nb.ipam.rirs, {"name": "RFC 1918", "slug": "rfc1918", "is_private": True})

    create(nb.ipam.aggregates, {"prefix": "10.0.0.0/8",    "rir": rfc1918.id, "description": "Private RFC1918"})
    create(nb.ipam.aggregates, {"prefix": "172.16.0.0/12", "rir": rfc1918.id, "description": "Private RFC1918"})
    create(nb.ipam.aggregates, {"prefix": "192.168.0.0/16","rir": rfc1918.id, "description": "Private RFC1918"})

    # ------------------------------------------------------------------
    print("\n=== VRFs ===")
    vrf_global = create(nb.ipam.vrfs, {"name": "Global",     "rd": "65000:0"})
    vrf_mgmt   = create(nb.ipam.vrfs, {"name": "Management", "rd": "65000:100"})

    # ------------------------------------------------------------------
    print("\n=== Prefixes ===")
    prefix_defs = [
        # container
        ("10.0.0.0/8",    None, None,       "container", "Global private space"),
        # site blocks
        ("10.0.0.0/16",   nyc,  vrf_global, "active", "NYC DC block"),
        ("10.1.0.0/16",   lon,  vrf_global, "active", "LON DC block"),
        ("10.2.0.0/16",   fra,  vrf_global, "active", "FRA DC block"),
        # NYC subnets
        ("10.0.0.0/24",   nyc,  vrf_mgmt,   "active", "NYC management"),
        ("10.0.1.0/24",   nyc,  vrf_global, "active", "NYC servers"),
        ("10.0.2.0/24",   nyc,  vrf_global, "active", "NYC VMs"),
        ("10.0.10.0/24",  nyc,  vrf_global, "active", "NYC loopbacks"),
        # LON subnets
        ("10.1.0.0/24",   lon,  vrf_mgmt,   "active", "LON management"),
        ("10.1.1.0/24",   lon,  vrf_global, "active", "LON servers"),
        ("10.1.10.0/24",  lon,  vrf_global, "active", "LON loopbacks"),
        # FRA subnets
        ("10.2.0.0/24",   fra,  vrf_mgmt,   "active", "FRA management"),
        ("10.2.1.0/24",   fra,  vrf_global, "active", "FRA servers"),
        ("10.2.10.0/24",  fra,  vrf_global, "active", "FRA loopbacks"),
    ]
    for prefix, site, vrf, status, desc in prefix_defs:
        payload = {"prefix": prefix, "status": status, "description": desc}
        if site:
            payload["site"] = site.id
        if vrf:
            payload["vrf"] = vrf.id
        create(nb.ipam.prefixes, payload)

    # ------------------------------------------------------------------
    print("\n=== VLANs ===")
    create(nb.ipam.vlan_groups, {"name": "NYC VLANs", "slug": "nyc-vlans", "scope_type": "dcim.site", "scope_id": nyc.id})
    vlan_defs = [
        (10,  "Management", nyc), (20, "Servers", nyc), (30, "Storage", nyc), (100, "Transit", nyc),
        (10,  "Management", lon), (20, "Servers", lon), (30, "Storage", lon),
        (10,  "Management", fra), (20, "Servers", fra),
    ]
    for vid, name, site in vlan_defs:
        get_or_create(
            nb.ipam.vlans,
            {"vid": vid, "site_id": site.id},
            {"vid": vid, "name": name, "site": site.id, "status": "active"},
            label=f"{site.slug}/vlan{vid}",
        )

    # ------------------------------------------------------------------
    print("\n=== ASNs ===")
    create(nb.ipam.asns, {"asn": 65000, "rir": arin.id, "description": "Primary AS"})
    create(nb.ipam.asns, {"asn": 65001, "rir": arin.id, "description": "NYC DC AS"})
    create(nb.ipam.asns, {"asn": 65002, "rir": arin.id, "description": "LON DC AS"})
    create(nb.ipam.asns, {"asn": 65003, "rir": arin.id, "description": "FRA DC AS"})

    # ------------------------------------------------------------------
    print("\n=== Device Interfaces ===")

    # Spine switches (NX-OS): mgmt0, Loopback0, fabric uplinks
    for dev_name in ("nyc-spine-01", "nyc-spine-02", "fra-spine-01", "lon-spine-01"):
        dev = d[dev_name]
        mkiface(dev, "mgmt0",     "1000base-t",      mgmt_only=True)
        mkiface(dev, "Loopback0", "virtual")
        for i in range(1, 5):
            mkiface(dev, f"Ethernet1/{i}", "25gbase-x-sfp28")

    # Leaf switches (Junos): em0, lo0, xe uplinks
    for dev_name in ("nyc-leaf-01", "nyc-leaf-02", "lon-leaf-01"):
        dev = d[dev_name]
        mkiface(dev, "em0",  "1000base-t", mgmt_only=True)
        mkiface(dev, "lo0",  "virtual")
        for i in range(0, 4):
            mkiface(dev, f"xe-0/0/{i}", "10gbase-x-sfpp")

    # Routers
    dev = d["nyc-router-01"]  # ASR (IOS-XE)
    mkiface(dev, "GigabitEthernet0/0/0", "1000base-t", mgmt_only=True)
    mkiface(dev, "Loopback0",            "virtual")
    mkiface(dev, "GigabitEthernet0/1/0", "1000base-t")
    mkiface(dev, "GigabitEthernet0/1/1", "1000base-t")

    dev = d["lon-router-01"]  # MX204 (Junos)
    mkiface(dev, "em0",      "1000base-t", mgmt_only=True)
    mkiface(dev, "lo0",      "virtual")
    mkiface(dev, "xe-0/0/0", "10gbase-x-sfpp")
    mkiface(dev, "xe-0/0/1", "10gbase-x-sfpp")

    # Servers: iDRAC (mgmt), bond0 (data), eth0/eth1 (members)
    for dev_name in ("nyc-server-01", "nyc-server-02", "fra-server-01"):
        dev = d[dev_name]
        mkiface(dev, "idrac", "1000base-t",      mgmt_only=True)
        mkiface(dev, "bond0", "lag")
        mkiface(dev, "eth0",  "25gbase-x-sfp28")
        mkiface(dev, "eth1",  "25gbase-x-sfp28")

    # ------------------------------------------------------------------
    print("\n=== IP Addresses & Interface Assignments ===")

    # NYC spine
    wire_ip("10.0.0.1/24",   vrf_mgmt,   d["nyc-spine-01"], "mgmt0",     "nyc-spine-01 mgmt",   make_primary=True)
    wire_ip("10.0.10.1/32",  vrf_global, d["nyc-spine-01"], "Loopback0", "nyc-spine-01 lo0",    role="loopback")
    wire_ip("10.0.0.2/24",   vrf_mgmt,   d["nyc-spine-02"], "mgmt0",     "nyc-spine-02 mgmt",   make_primary=True)
    wire_ip("10.0.10.2/32",  vrf_global, d["nyc-spine-02"], "Loopback0", "nyc-spine-02 lo0",    role="loopback")

    # NYC leaf
    wire_ip("10.0.0.3/24",   vrf_mgmt,   d["nyc-leaf-01"],  "em0",  "nyc-leaf-01 mgmt",    make_primary=True)
    wire_ip("10.0.10.3/32",  vrf_global, d["nyc-leaf-01"],  "lo0",  "nyc-leaf-01 lo0",     role="loopback")
    wire_ip("10.0.0.4/24",   vrf_mgmt,   d["nyc-leaf-02"],  "em0",  "nyc-leaf-02 mgmt",    make_primary=True)
    wire_ip("10.0.10.4/32",  vrf_global, d["nyc-leaf-02"],  "lo0",  "nyc-leaf-02 lo0",     role="loopback")

    # NYC router
    wire_ip("10.0.0.5/24",   vrf_mgmt,   d["nyc-router-01"], "GigabitEthernet0/0/0", "nyc-router-01 mgmt", make_primary=True)
    wire_ip("10.0.10.5/32",  vrf_global, d["nyc-router-01"], "Loopback0",            "nyc-router-01 lo0",  role="loopback")

    # NYC servers
    wire_ip("10.0.0.6/24",   vrf_mgmt,   d["nyc-server-01"], "idrac", "nyc-server-01 idrac", make_primary=True)
    wire_ip("10.0.1.10/24",  vrf_global, d["nyc-server-01"], "bond0", "nyc-server-01 data")
    wire_ip("10.0.0.7/24",   vrf_mgmt,   d["nyc-server-02"], "idrac", "nyc-server-02 idrac", make_primary=True)
    wire_ip("10.0.1.11/24",  vrf_global, d["nyc-server-02"], "bond0", "nyc-server-02 data")

    # LON spine
    wire_ip("10.1.0.1/24",   vrf_mgmt,   d["lon-spine-01"], "mgmt0",     "lon-spine-01 mgmt",   make_primary=True)
    wire_ip("10.1.10.1/32",  vrf_global, d["lon-spine-01"], "Loopback0", "lon-spine-01 lo0",    role="loopback")

    # LON leaf
    wire_ip("10.1.0.2/24",   vrf_mgmt,   d["lon-leaf-01"],  "em0",  "lon-leaf-01 mgmt",    make_primary=True)
    wire_ip("10.1.10.2/32",  vrf_global, d["lon-leaf-01"],  "lo0",  "lon-leaf-01 lo0",     role="loopback")

    # LON router
    wire_ip("10.1.0.3/24",   vrf_mgmt,   d["lon-router-01"], "em0", "lon-router-01 mgmt",  make_primary=True)
    wire_ip("10.1.10.3/32",  vrf_global, d["lon-router-01"], "lo0", "lon-router-01 lo0",   role="loopback")

    # FRA spine
    wire_ip("10.2.0.1/24",   vrf_mgmt,   d["fra-spine-01"], "mgmt0",     "fra-spine-01 mgmt",   make_primary=True)
    wire_ip("10.2.10.1/32",  vrf_global, d["fra-spine-01"], "Loopback0", "fra-spine-01 lo0",    role="loopback")

    # FRA server
    wire_ip("10.2.0.2/24",   vrf_mgmt,   d["fra-server-01"], "idrac", "fra-server-01 idrac", make_primary=True)
    wire_ip("10.2.1.10/24",  vrf_global, d["fra-server-01"], "bond0", "fra-server-01 data")

    # ------------------------------------------------------------------
    print("\n=== Cluster Types & Clusters ===")
    vsphere = create(nb.virtualization.cluster_types, {"name": "VMware vSphere", "slug": "vsphere"})

    nyc_cluster = get_or_create(
        nb.virtualization.clusters,
        {"name": "NYC-PROD"},
        {"name": "NYC-PROD", "type": vsphere.id, "site": nyc.id, "tenant": infra.id, "status": "active"},
    )
    lon_cluster = get_or_create(
        nb.virtualization.clusters,
        {"name": "LON-PROD"},
        {"name": "LON-PROD", "type": vsphere.id, "site": lon.id, "tenant": infra.id, "status": "active"},
    )

    # ------------------------------------------------------------------
    print("\n=== Virtual Machines ===")
    # local_context_data holds VM-specific overrides — this is what slim_value() strips in responses.
    vm_defs = [
        # (name, cluster, vcpus, memory_mb, disk_gb, platform, tenant, local_context_data)
        ("web-prod-01",   nyc_cluster, 4,  8192,  100, ubuntu, app_team, {"env": "production", "tier": "web",        "deploy_key": "prod-web-01"}),
        ("web-prod-02",   nyc_cluster, 4,  8192,  100, ubuntu, app_team, {"env": "production", "tier": "web",        "deploy_key": "prod-web-02"}),
        ("db-prod-01",    nyc_cluster, 8,  32768, 500, ubuntu, app_team, {"env": "production", "tier": "db",         "backup_schedule": "daily", "replica_lag_warn_s": 30}),
        ("cache-prod-01", nyc_cluster, 2,  4096,   50, ubuntu, app_team, {"env": "production", "tier": "cache",      "eviction": "lru", "maxmemory_mb": 3072}),
        ("mon-prod-01",   nyc_cluster, 2,  4096,  200, ubuntu, infra,    {"env": "production", "tier": "monitoring", "retention_days": 90}),
        ("web-lon-01",    lon_cluster, 4,  8192,  100, ubuntu, app_team, {"env": "production", "tier": "web",        "region": "eu-west", "deploy_key": "prod-web-lon-01"}),
        ("web-lon-02",    lon_cluster, 4,  8192,  100, ubuntu, app_team, {"env": "production", "tier": "web",        "region": "eu-west", "deploy_key": "prod-web-lon-02"}),
    ]
    vms = {}
    for name, cluster, vcpus, memory, disk, platform, tenant, local_ctx in vm_defs:
        vms[name] = get_or_create(
            nb.virtualization.virtual_machines,
            {"name": name, "cluster_id": cluster.id},
            {
                "name": name, "cluster": cluster.id, "status": "active",
                "vcpus": vcpus, "memory": memory, "disk": disk,
                "platform": platform.id, "tenant": tenant.id,
                "local_context_data": local_ctx,
            },
        )

    # ------------------------------------------------------------------
    print("\n=== VM Interfaces & IPs ===")
    vm_ip_map = {
        "web-prod-01":   ("10.0.2.10/24", vrf_global),
        "web-prod-02":   ("10.0.2.11/24", vrf_global),
        "db-prod-01":    ("10.0.2.20/24", vrf_global),
        "cache-prod-01": ("10.0.2.30/24", vrf_global),
        "mon-prod-01":   ("10.0.2.40/24", vrf_global),
        "web-lon-01":    ("10.1.1.10/24", vrf_global),
        "web-lon-02":    ("10.1.1.11/24", vrf_global),
    }
    for vm_name, (address, vrf) in vm_ip_map.items():
        vm = vms[vm_name]
        iface = mkvm_iface(vm, "eth0")
        wire_vm_ip(address, vrf, vm, iface, description=f"{vm_name} eth0")

    # ------------------------------------------------------------------
    print("\n=== Services ===")
    # NetBox 4.x uses generic parent_object_type/parent_object_id for services.
    svc_defs = [
        # (name, parent_type, parent_obj, protocol, ports)
        ("ssh",        "dcim.device",                    d["nyc-server-01"],    "tcp", [22]),
        ("ssh",        "dcim.device",                    d["nyc-server-02"],    "tcp", [22]),
        ("https",      "virtualization.virtualmachine",  vms["web-prod-01"],    "tcp", [443]),
        ("http",       "virtualization.virtualmachine",  vms["web-prod-01"],    "tcp", [80]),
        ("https",      "virtualization.virtualmachine",  vms["web-prod-02"],    "tcp", [443]),
        ("http",       "virtualization.virtualmachine",  vms["web-prod-02"],    "tcp", [80]),
        ("postgresql", "virtualization.virtualmachine",  vms["db-prod-01"],     "tcp", [5432]),
        ("redis",      "virtualization.virtualmachine",  vms["cache-prod-01"],  "tcp", [6379]),
        ("prometheus", "virtualization.virtualmachine",  vms["mon-prod-01"],    "tcp", [9090]),
        ("https",      "virtualization.virtualmachine",  vms["web-lon-01"],     "tcp", [443]),
        ("https",      "virtualization.virtualmachine",  vms["web-lon-02"],     "tcp", [443]),
    ]
    for svc_name, parent_type, parent_obj, protocol, ports in svc_defs:
        label = f"{parent_obj.name}/{svc_name}"
        get_or_create(
            nb.ipam.services,
            {"name": svc_name, "parent_object_type": parent_type, "parent_object_id": parent_obj.id},
            {"name": svc_name, "protocol": protocol, "ports": ports,
             "parent_object_type": parent_type, "parent_object_id": parent_obj.id},
            label=label,
        )

    # ------------------------------------------------------------------
    print("\n=== Done! ===")
    print(f"\nNetBox UI: {NETBOX_URL}")
    print(
        "\nSummary of what was created:"
        "\n  • 3 sites (NYC/LON/FRA) with racks, floors, platforms"
        "\n  • 12 devices with interfaces and primary IPs assigned"
        "\n  • 2 VMware clusters with 7 VMs (local_context_data set on each)"
        "\n  • 7 VM interfaces with IPs and primary IPs assigned"
        "\n  • 14 prefixes across 2 VRFs (global + management)"
        "\n  • Services on servers and VMs"
    )


if __name__ == "__main__":
    main()
