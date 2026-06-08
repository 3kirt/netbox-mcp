//! Live DCIM coverage: devices (site/role/name filters, get-by-id),
//! interfaces (mgmt_only filter), sites/regions/racks.

use serde_json::json;

use super::harness::{
    assert_clean, assert_nonempty, assert_page_shape, first_id, params, results, skip_unless_live,
    slim, slim_get,
};
use crate::tools::dcim::{
    self, DeviceRolesListParams, DeviceTypesListParams, DevicesListParams, InterfacesListParams,
    LocationsListParams, ManufacturersListParams, PlatformsListParams, RacksListParams,
    RegionsListParams, SitesListParams,
};

#[tokio::test]
async fn devices_filter_by_site_returns_only_that_site() {
    let env = skip_unless_live!();
    let p: DevicesListParams = params(json!({ "site": ["lon-dc"] }));
    let resp = slim(dcim::devices_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "devices?site=lon-dc");
    assert_nonempty(&resp, "devices?site=lon-dc");
    for d in results(&resp) {
        assert_eq!(
            d["site"]["slug"], "lon-dc",
            "device not in filtered site: {d}"
        );
    }
}

#[tokio::test]
async fn devices_filter_by_role_returns_only_that_role() {
    let env = skip_unless_live!();
    let p: DevicesListParams = params(json!({ "role": ["server"] }));
    let resp = slim(dcim::devices_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "devices?role=server");
    assert_nonempty(&resp, "devices?role=server");
    for d in results(&resp) {
        assert_eq!(
            d["role"]["slug"], "server",
            "device not in filtered role: {d}"
        );
    }
}

#[tokio::test]
async fn devices_filter_by_name_is_exact() {
    let env = skip_unless_live!();
    let p: DevicesListParams = params(json!({ "name": ["nyc-spine-01"] }));
    let resp = slim(dcim::devices_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "devices?name=nyc-spine-01");
    assert_nonempty(&resp, "devices?name=nyc-spine-01");
    for d in results(&resp) {
        assert_eq!(d["name"], "nyc-spine-01");
    }
}

#[tokio::test]
async fn device_get_by_id_is_clean_and_drops_primary_ip_alias() {
    let env = skip_unless_live!();
    let list = slim(
        dcim::devices_list(&env.client, params(json!({ "name": ["nyc-spine-01"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&list, "devices?name=nyc-spine-01");
    let id = first_id(&list);

    let device = slim_get(&env.client, "/api/dcim/devices/", id).await;
    assert_clean(&device, "device.get");
    assert_eq!(device["id"], id);
    // The `primary_ip` shorthand must never survive slimming — assert_clean
    // covers it, but make the DCIM-specific expectation explicit.
    assert!(
        device.get("primary_ip").is_none(),
        "device.get leaked the primary_ip alias"
    );
}

#[tokio::test]
async fn interfaces_mgmt_only_filter_returns_only_mgmt_interfaces() {
    let env = skip_unless_live!();
    let p: InterfacesListParams = params(json!({ "device": "nyc-server-01", "mgmt_only": true }));
    let resp = slim(dcim::interfaces_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "interfaces?device=nyc-server-01&mgmt_only=true");
    assert_nonempty(&resp, "interfaces?device=nyc-server-01&mgmt_only=true");
    for i in results(&resp) {
        assert_eq!(
            i["mgmt_only"], true,
            "non-mgmt interface leaked through filter: {i}"
        );
    }
}

#[tokio::test]
async fn sites_filter_by_region_returns_rows() {
    let env = skip_unless_live!();
    let p: SitesListParams = params(json!({ "region": ["europe"] }));
    let resp = slim(dcim::sites_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "sites?region=europe");
    assert_nonempty(&resp, "sites?region=europe");
}

#[tokio::test]
async fn racks_filter_by_site_returns_only_that_site() {
    let env = skip_unless_live!();
    let p: RacksListParams = params(json!({ "site": ["lon-dc"] }));
    let resp = slim(dcim::racks_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "racks?site=lon-dc");
    assert_nonempty(&resp, "racks?site=lon-dc");
    for r in results(&resp) {
        assert_eq!(
            r["site"]["slug"], "lon-dc",
            "rack not in filtered site: {r}"
        );
    }
}

#[tokio::test]
async fn regions_list_is_clean() {
    let env = skip_unless_live!();
    let p: RegionsListParams = params(json!({}));
    let resp = slim(dcim::regions_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "regions");
    assert_nonempty(&resp, "regions");
}

#[tokio::test]
async fn manufacturers_filter_by_slug_is_exact() {
    let env = skip_unless_live!();
    let p: ManufacturersListParams = params(json!({ "slug": ["cisco"] }));
    let resp = slim(dcim::manufacturers_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "manufacturers?slug=cisco");
    assert_nonempty(&resp, "manufacturers?slug=cisco");
    for m in results(&resp) {
        assert_eq!(m["slug"], "cisco");
    }
}

#[tokio::test]
async fn device_types_filter_by_manufacturer_scopes_to_that_manufacturer() {
    let env = skip_unless_live!();
    let p: DeviceTypesListParams = params(json!({ "manufacturer": ["cisco"] }));
    let resp = slim(dcim::device_types_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "device-types?manufacturer=cisco");
    assert_nonempty(&resp, "device-types?manufacturer=cisco");
    for dt in results(&resp) {
        assert_eq!(
            dt["manufacturer"]["slug"], "cisco",
            "device type not from filtered manufacturer: {dt}"
        );
    }
}

#[tokio::test]
async fn device_roles_filter_by_slug_is_exact() {
    let env = skip_unless_live!();
    let p: DeviceRolesListParams = params(json!({ "slug": ["server"] }));
    let resp = slim(dcim::device_roles_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "device-roles?slug=server");
    assert_nonempty(&resp, "device-roles?slug=server");
    for r in results(&resp) {
        assert_eq!(r["slug"], "server");
    }
}

#[tokio::test]
async fn platforms_filter_by_manufacturer_scopes_to_that_manufacturer() {
    let env = skip_unless_live!();
    let p: PlatformsListParams = params(json!({ "manufacturer": ["juniper"] }));
    let resp = slim(dcim::platforms_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "platforms?manufacturer=juniper");
    assert_nonempty(&resp, "platforms?manufacturer=juniper");
    for p in results(&resp) {
        assert_eq!(
            p["manufacturer"]["slug"], "juniper",
            "platform not from filtered manufacturer: {p}"
        );
    }
}

#[tokio::test]
async fn locations_filter_by_site_returns_only_that_site() {
    let env = skip_unless_live!();
    let p: LocationsListParams = params(json!({ "site": ["nyc-dc"] }));
    let resp = slim(dcim::locations_list(&env.client, p).await.unwrap());

    assert_page_shape(&resp, "locations?site=nyc-dc");
    assert_nonempty(&resp, "locations?site=nyc-dc");
    for l in results(&resp) {
        assert_eq!(
            l["site"]["slug"], "nyc-dc",
            "location not in filtered site: {l}"
        );
    }
}

#[tokio::test]
async fn site_rack_interface_region_get_by_id_are_clean() {
    let env = skip_unless_live!();

    // Resolve one id of each from a list, then fetch and check the get path.
    let site_list = slim(
        dcim::sites_list(&env.client, params(json!({ "name": ["London DC"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&site_list, "sites?name=London DC");
    let site = slim_get(&env.client, "/api/dcim/sites/", first_id(&site_list)).await;
    assert_clean(&site, "site.get");

    let rack_list = slim(
        dcim::racks_list(&env.client, params(json!({ "site": ["lon-dc"] })))
            .await
            .unwrap(),
    );
    assert_nonempty(&rack_list, "racks?site=lon-dc");
    let rack = slim_get(&env.client, "/api/dcim/racks/", first_id(&rack_list)).await;
    assert_clean(&rack, "rack.get");

    let iface_list = slim(
        dcim::interfaces_list(&env.client, params(json!({ "device": "lon-spine-01" })))
            .await
            .unwrap(),
    );
    assert_nonempty(&iface_list, "interfaces?device=lon-spine-01");
    let iface = slim_get(&env.client, "/api/dcim/interfaces/", first_id(&iface_list)).await;
    assert_clean(&iface, "interface.get");

    let region_list = slim(
        dcim::regions_list(&env.client, params(json!({})))
            .await
            .unwrap(),
    );
    assert_nonempty(&region_list, "regions");
    let region = slim_get(&env.client, "/api/dcim/regions/", first_id(&region_list)).await;
    assert_clean(&region, "region.get");
}
