use crate::client::{NetboxClient, NetboxError};
use crate::tools::{CommonListParams, PaginationParams, QueryBuilder, resolve_device_id_or};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Devices
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DevicesListParams {
    #[schemars(description = "Filter by exact device name (multi-value)")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by name contains, case-insensitive (partial match)")]
    pub name_ic: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by device role slug")]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Filter by status (e.g. active, planned)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by rack ID")]
    pub rack_id: Option<i32>,
    #[schemars(description = "Filter by cluster ID")]
    pub cluster_id: Option<i32>,
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn devices_list(
    client: &NetboxClient,
    p: DevicesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .opt("name__ic", p.name_ic)
        .many("site", p.site)
        .many("role", p.role)
        .many("status", p.status)
        .many("tenant", p.tenant)
        .opt("rack_id", p.rack_id)
        .opt("cluster_id", p.cluster_id)
        .many("tag", p.tag);
    qb.run_common(client, "/api/dcim/devices/", p.common).await
}

// --------------------------------------------------------------------------
// Sites
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SitesListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by region slug")]
    pub region: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn sites_list(client: &NetboxClient, p: SitesListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("status", p.status)
        .many("region", p.region)
        .many("tag", p.tag);
    qb.run_common(client, "/api/dcim/sites/", p.common).await
}

// --------------------------------------------------------------------------
// Racks
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RacksListParams {
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by location slug")]
    pub location: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn racks_list(client: &NetboxClient, p: RacksListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("site", p.site)
        .many("location", p.location)
        .many("status", p.status)
        .many("tag", p.tag);
    qb.run_common(client, "/api/dcim/racks/", p.common).await
}

// --------------------------------------------------------------------------
// Interfaces
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct InterfacesListParams {
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by interface name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by interface type")]
    pub r#type: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Return only management interfaces when true")]
    pub mgmt_only: Option<bool>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn interfaces_list(
    client: &NetboxClient,
    p: InterfacesListParams,
) -> Result<Value, NetboxError> {
    let device_id = resolve_device_id_or(client, p.device, p.device_id).await?;
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .many("name", p.name)
        .many("type", p.r#type)
        .many("tag", p.tag)
        .opt("mgmt_only", p.mgmt_only);
    qb.run_common(client, "/api/dcim/interfaces/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Cables
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CablesListParams {
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn cables_list(client: &NetboxClient, p: CablesListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("site", p.site)
        .many("status", p.status)
        .many("tag", p.tag);
    qb.run_common(client, "/api/dcim/cables/", p.common).await
}

// --------------------------------------------------------------------------
// Regions
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RegionsListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Filter by parent region slug")]
    pub parent: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn regions_list(
    client: &NetboxClient,
    p: RegionsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("slug", p.slug)
        .many("parent", p.parent);
    qb.run_common(client, "/api/dcim/regions/", p.common).await
}

// --------------------------------------------------------------------------
// Locations
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LocationsListParams {
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by parent location slug")]
    pub parent: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn locations_list(
    client: &NetboxClient,
    p: LocationsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("site", p.site)
        .many("parent", p.parent)
        .many("status", p.status)
        .many("tenant", p.tenant)
        .many("tag", p.tag);
    qb.run_common(client, "/api/dcim/locations/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Manufacturers
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ManufacturersListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn manufacturers_list(
    client: &NetboxClient,
    p: ManufacturersListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("slug", p.slug);
    qb.run_common(client, "/api/dcim/manufacturers/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Device types
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DeviceTypesListParams {
    #[schemars(description = "Filter by manufacturer slug")]
    pub manufacturer: Option<Vec<String>>,
    #[schemars(description = "Filter by model")]
    pub model: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn device_types_list(
    client: &NetboxClient,
    p: DeviceTypesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("manufacturer", p.manufacturer)
        .many("model", p.model)
        .many("tag", p.tag);
    qb.run_common(client, "/api/dcim/device-types/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Device roles
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DeviceRolesListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Filter to roles eligible for virtual machines")]
    pub vm_role: Option<bool>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn device_roles_list(
    client: &NetboxClient,
    p: DeviceRolesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("vm_role", p.vm_role);
    qb.run_common(client, "/api/dcim/device-roles/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Platforms
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PlatformsListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by manufacturer slug")]
    pub manufacturer: Option<Vec<String>>,
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn platforms_list(
    client: &NetboxClient,
    p: PlatformsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("manufacturer", p.manufacturer)
        .many("tag", p.tag);
    qb.run_common(client, "/api/dcim/platforms/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Power panels
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PowerPanelsListParams {
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn power_panels_list(
    client: &NetboxClient,
    p: PowerPanelsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new().many("site", p.site);
    qb.run_common(client, "/api/dcim/power-panels/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Power feeds
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PowerFeedsListParams {
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by type (primary or redundant)")]
    pub r#type: Option<String>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn power_feeds_list(
    client: &NetboxClient,
    p: PowerFeedsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("site", p.site)
        .many("status", p.status)
        .opt("type", p.r#type);
    qb.run_common(client, "/api/dcim/power-feeds/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Virtual chassis
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VirtualChassisListParams {
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn virtual_chassis_list(
    client: &NetboxClient,
    p: VirtualChassisListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("site", p.site)
        .many("tenant", p.tenant);
    qb.run_common(client, "/api/dcim/virtual-chassis/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Inventory items
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct InventoryItemsListParams {
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by manufacturer slug")]
    pub manufacturer: Option<Vec<String>>,
    #[schemars(description = "Filter to discovered items only")]
    pub discovered: Option<bool>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn inventory_items_list(
    client: &NetboxClient,
    p: InventoryItemsListParams,
) -> Result<Value, NetboxError> {
    let device_id = resolve_device_id_or(client, p.device, p.device_id).await?;
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .many("manufacturer", p.manufacturer)
        .opt("discovered", p.discovered);
    qb.run_common(client, "/api/dcim/inventory-items/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Cable terminations
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CableTerminationsListParams {
    #[schemars(description = "Filter by cable ID")]
    pub cable_id: Option<i32>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationParams,
}

pub async fn cable_terminations_list(
    client: &NetboxClient,
    p: CableTerminationsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("cable_id", p.cable_id)
        .opt("ordering", p.ordering);
    qb.run(client, "/api/dcim/cable-terminations/", p.pagination)
        .await
}

// --------------------------------------------------------------------------
// Console ports
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ConsolePortsListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn console_ports_list(
    client: &NetboxClient,
    p: ConsolePortsListParams,
) -> Result<Value, NetboxError> {
    let device_id = resolve_device_id_or(client, p.device, p.device_id).await?;
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .many("name", p.name)
        .many("site", p.site);
    qb.run_common(client, "/api/dcim/console-ports/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Console server ports
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ConsoleServerPortsListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn console_server_ports_list(
    client: &NetboxClient,
    p: ConsoleServerPortsListParams,
) -> Result<Value, NetboxError> {
    let device_id = resolve_device_id_or(client, p.device, p.device_id).await?;
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .many("name", p.name)
        .many("site", p.site);
    qb.run_common(client, "/api/dcim/console-server-ports/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Device bays
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DeviceBaysListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn device_bays_list(
    client: &NetboxClient,
    p: DeviceBaysListParams,
) -> Result<Value, NetboxError> {
    let device_id = resolve_device_id_or(client, p.device, p.device_id).await?;
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .many("name", p.name)
        .many("site", p.site);
    qb.run_common(client, "/api/dcim/device-bays/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Front ports
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FrontPortsListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn front_ports_list(
    client: &NetboxClient,
    p: FrontPortsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new().many("name", p.name);
    qb.run_common(client, "/api/dcim/front-ports/", p.common)
        .await
}

// --------------------------------------------------------------------------
// MAC addresses
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MacAddressesListParams {
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn mac_addresses_list(
    client: &NetboxClient,
    p: MacAddressesListParams,
) -> Result<Value, NetboxError> {
    let device_id = resolve_device_id_or(client, p.device, p.device_id).await?;
    let qb = QueryBuilder::new().opt("device_id", device_id);
    qb.run_common(client, "/api/dcim/mac-addresses/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Modules
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModulesListParams {
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn modules_list(
    client: &NetboxClient,
    p: ModulesListParams,
) -> Result<Value, NetboxError> {
    let device_id = resolve_device_id_or(client, p.device, p.device_id).await?;
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .many("site", p.site)
        .many("status", p.status);
    qb.run_common(client, "/api/dcim/modules/", p.common).await
}

// --------------------------------------------------------------------------
// Module bays
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModuleBaysListParams {
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn module_bays_list(
    client: &NetboxClient,
    p: ModuleBaysListParams,
) -> Result<Value, NetboxError> {
    let device_id = resolve_device_id_or(client, p.device, p.device_id).await?;
    let qb = QueryBuilder::new().opt("device_id", device_id);
    qb.run_common(client, "/api/dcim/module-bays/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Module types
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModuleTypesListParams {
    #[schemars(description = "Filter by manufacturer slug")]
    pub manufacturer: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn module_types_list(
    client: &NetboxClient,
    p: ModuleTypesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new().many("manufacturer", p.manufacturer);
    qb.run_common(client, "/api/dcim/module-types/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Power outlets
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PowerOutletsListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn power_outlets_list(
    client: &NetboxClient,
    p: PowerOutletsListParams,
) -> Result<Value, NetboxError> {
    let device_id = resolve_device_id_or(client, p.device, p.device_id).await?;
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .many("name", p.name)
        .many("site", p.site);
    qb.run_common(client, "/api/dcim/power-outlets/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Power ports
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PowerPortsListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn power_ports_list(
    client: &NetboxClient,
    p: PowerPortsListParams,
) -> Result<Value, NetboxError> {
    let device_id = resolve_device_id_or(client, p.device, p.device_id).await?;
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .many("name", p.name)
        .many("site", p.site);
    qb.run_common(client, "/api/dcim/power-ports/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Rack reservations
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RackReservationsListParams {
    #[schemars(description = "Filter by rack ID")]
    pub rack_id: Option<i32>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn rack_reservations_list(
    client: &NetboxClient,
    p: RackReservationsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("rack_id", p.rack_id)
        .many("site", p.site)
        .many("tenant", p.tenant);
    qb.run_common(client, "/api/dcim/rack-reservations/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Rack roles
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RackRolesListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn rack_roles_list(
    client: &NetboxClient,
    p: RackRolesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("slug", p.slug);
    qb.run_common(client, "/api/dcim/rack-roles/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Rack types
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RackTypesListParams {
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Filter by manufacturer slug")]
    pub manufacturer: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn rack_types_list(
    client: &NetboxClient,
    p: RackTypesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("slug", p.slug)
        .many("manufacturer", p.manufacturer);
    qb.run_common(client, "/api/dcim/rack-types/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Rear ports
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RearPortsListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn rear_ports_list(
    client: &NetboxClient,
    p: RearPortsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new().many("name", p.name);
    qb.run_common(client, "/api/dcim/rear-ports/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Site groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SiteGroupsListParams {
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn site_groups_list(
    client: &NetboxClient,
    p: SiteGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .many("name", p.name)
        .many("slug", p.slug);
    qb.run_common(client, "/api/dcim/site-groups/", p.common)
        .await
}

// --------------------------------------------------------------------------
// Virtual device contexts
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VirtualDeviceContextsListParams {
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[serde(flatten)]
    pub common: CommonListParams,
}

pub async fn virtual_device_contexts_list(
    client: &NetboxClient,
    p: VirtualDeviceContextsListParams,
) -> Result<Value, NetboxError> {
    let device_id = resolve_device_id_or(client, p.device, p.device_id).await?;
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .many("tenant", p.tenant);
    qb.run_common(client, "/api/dcim/virtual-device-contexts/", p.common)
        .await
}
