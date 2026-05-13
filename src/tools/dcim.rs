use crate::client::{NetboxClient, NetboxError};
use crate::tools::clamp_limit;
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Shared "get by ID" params (reused across all domains via dcim::GetByIdParams)
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct GetByIdParams {
    #[schemars(description = "NetBox ID of the object to retrieve")]
    pub id: i32,
}

// --------------------------------------------------------------------------
// Devices
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DevicesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by device role slug")]
    pub role: Option<Vec<String>>,
    #[schemars(description = "Filter by status (e.g. active, planned)")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by rack ID")]
    pub rack_id: Option<i32>,
    #[schemars(description = "Field to order results by (prefix with - for descending)")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn devices_list(
    client: &NetboxClient,
    p: DevicesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.role.unwrap_or_default() {
        params.push(("role", v));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    if let Some(v) = p.rack_id {
        params.push(("rack_id", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/devices/", &params).await
}

// --------------------------------------------------------------------------
// Sites
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SitesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by region slug")]
    pub region: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn sites_list(client: &NetboxClient, p: SitesListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    for v in p.region.unwrap_or_default() {
        params.push(("region", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/sites/", &params).await
}

// --------------------------------------------------------------------------
// Racks
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RacksListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by location slug")]
    pub location: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn racks_list(client: &NetboxClient, p: RacksListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.location.unwrap_or_default() {
        params.push(("location", v));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/racks/", &params).await
}

// --------------------------------------------------------------------------
// Interfaces
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct InterfacesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by interface name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by interface type")]
    pub r#type: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn interfaces_list(
    client: &NetboxClient,
    p: InterfacesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.r#type.unwrap_or_default() {
        params.push(("type", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/interfaces/", &params).await
}

// --------------------------------------------------------------------------
// Cables
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct CablesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn cables_list(client: &NetboxClient, p: CablesListParams) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/cables/", &params).await
}

// --------------------------------------------------------------------------
// Regions
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RegionsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Filter by parent region slug")]
    pub parent: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn regions_list(
    client: &NetboxClient,
    p: RegionsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.slug.unwrap_or_default() {
        params.push(("slug", v));
    }
    for v in p.parent.unwrap_or_default() {
        params.push(("parent", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/regions/", &params).await
}

// --------------------------------------------------------------------------
// Locations
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct LocationsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by parent location slug")]
    pub parent: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn locations_list(
    client: &NetboxClient,
    p: LocationsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.parent.unwrap_or_default() {
        params.push(("parent", v));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/locations/", &params).await
}

// --------------------------------------------------------------------------
// Manufacturers
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ManufacturersListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn manufacturers_list(
    client: &NetboxClient,
    p: ManufacturersListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.slug.unwrap_or_default() {
        params.push(("slug", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/manufacturers/", &params).await
}

// --------------------------------------------------------------------------
// Device types
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DeviceTypesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by manufacturer slug")]
    pub manufacturer: Option<Vec<String>>,
    #[schemars(description = "Filter by model")]
    pub model: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn device_types_list(
    client: &NetboxClient,
    p: DeviceTypesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.manufacturer.unwrap_or_default() {
        params.push(("manufacturer", v));
    }
    for v in p.model.unwrap_or_default() {
        params.push(("model", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/device-types/", &params).await
}

// --------------------------------------------------------------------------
// Device roles
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DeviceRolesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Filter to roles eligible for virtual machines")]
    pub vm_role: Option<bool>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn device_roles_list(
    client: &NetboxClient,
    p: DeviceRolesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.slug.unwrap_or_default() {
        params.push(("slug", v));
    }
    if let Some(v) = p.vm_role {
        params.push(("vm_role", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/device-roles/", &params).await
}

// --------------------------------------------------------------------------
// Platforms
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PlatformsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by manufacturer slug")]
    pub manufacturer: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn platforms_list(
    client: &NetboxClient,
    p: PlatformsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.manufacturer.unwrap_or_default() {
        params.push(("manufacturer", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/platforms/", &params).await
}

// --------------------------------------------------------------------------
// Power panels
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PowerPanelsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn power_panels_list(
    client: &NetboxClient,
    p: PowerPanelsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/power-panels/", &params).await
}

// --------------------------------------------------------------------------
// Power feeds
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PowerFeedsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Filter by type (primary or redundant)")]
    pub r#type: Option<String>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn power_feeds_list(
    client: &NetboxClient,
    p: PowerFeedsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    if let Some(v) = p.r#type {
        params.push(("type", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/power-feeds/", &params).await
}

// --------------------------------------------------------------------------
// Virtual chassis
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VirtualChassisListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn virtual_chassis_list(
    client: &NetboxClient,
    p: VirtualChassisListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/virtual-chassis/", &params).await
}

// --------------------------------------------------------------------------
// Inventory items
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct InventoryItemsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by manufacturer slug")]
    pub manufacturer: Option<Vec<String>>,
    #[schemars(description = "Filter to discovered items only")]
    pub discovered: Option<bool>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn inventory_items_list(
    client: &NetboxClient,
    p: InventoryItemsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    for v in p.manufacturer.unwrap_or_default() {
        params.push(("manufacturer", v));
    }
    if let Some(v) = p.discovered {
        params.push(("discovered", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/inventory-items/", &params).await
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
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn cable_terminations_list(
    client: &NetboxClient,
    p: CableTerminationsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(v) = p.cable_id {
        params.push(("cable_id", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/cable-terminations/", &params).await
}

// --------------------------------------------------------------------------
// Console ports
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ConsolePortsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn console_ports_list(
    client: &NetboxClient,
    p: ConsolePortsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/console-ports/", &params).await
}

// --------------------------------------------------------------------------
// Console server ports
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ConsoleServerPortsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn console_server_ports_list(
    client: &NetboxClient,
    p: ConsoleServerPortsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client
        .list("/api/dcim/console-server-ports/", &params)
        .await
}

// --------------------------------------------------------------------------
// Device bays
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DeviceBaysListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn device_bays_list(
    client: &NetboxClient,
    p: DeviceBaysListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/device-bays/", &params).await
}

// --------------------------------------------------------------------------
// Front ports
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct FrontPortsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn front_ports_list(
    client: &NetboxClient,
    p: FrontPortsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/front-ports/", &params).await
}

// --------------------------------------------------------------------------
// MAC addresses
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MacAddressesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn mac_addresses_list(
    client: &NetboxClient,
    p: MacAddressesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/mac-addresses/", &params).await
}

// --------------------------------------------------------------------------
// Modules
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModulesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by status")]
    pub status: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn modules_list(
    client: &NetboxClient,
    p: ModulesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.status.unwrap_or_default() {
        params.push(("status", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/modules/", &params).await
}

// --------------------------------------------------------------------------
// Module bays
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModuleBaysListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn module_bays_list(
    client: &NetboxClient,
    p: ModuleBaysListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/module-bays/", &params).await
}

// --------------------------------------------------------------------------
// Module types
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModuleTypesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by manufacturer slug")]
    pub manufacturer: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn module_types_list(
    client: &NetboxClient,
    p: ModuleTypesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.manufacturer.unwrap_or_default() {
        params.push(("manufacturer", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/module-types/", &params).await
}

// --------------------------------------------------------------------------
// Power outlets
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PowerOutletsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn power_outlets_list(
    client: &NetboxClient,
    p: PowerOutletsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/power-outlets/", &params).await
}

// --------------------------------------------------------------------------
// Power ports
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct PowerPortsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn power_ports_list(
    client: &NetboxClient,
    p: PowerPortsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/power-ports/", &params).await
}

// --------------------------------------------------------------------------
// Rack reservations
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RackReservationsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by rack ID")]
    pub rack_id: Option<i32>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn rack_reservations_list(
    client: &NetboxClient,
    p: RackReservationsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.rack_id {
        params.push(("rack_id", v.to_string()));
    }
    for v in p.site.unwrap_or_default() {
        params.push(("site", v));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/rack-reservations/", &params).await
}

// --------------------------------------------------------------------------
// Rack roles
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RackRolesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn rack_roles_list(
    client: &NetboxClient,
    p: RackRolesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.slug.unwrap_or_default() {
        params.push(("slug", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/rack-roles/", &params).await
}

// --------------------------------------------------------------------------
// Rack types
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RackTypesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Filter by manufacturer slug")]
    pub manufacturer: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn rack_types_list(
    client: &NetboxClient,
    p: RackTypesListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.slug.unwrap_or_default() {
        params.push(("slug", v));
    }
    for v in p.manufacturer.unwrap_or_default() {
        params.push(("manufacturer", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/rack-types/", &params).await
}

// --------------------------------------------------------------------------
// Rear ports
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct RearPortsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn rear_ports_list(
    client: &NetboxClient,
    p: RearPortsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/rear-ports/", &params).await
}

// --------------------------------------------------------------------------
// Site groups
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct SiteGroupsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by name")]
    pub name: Option<Vec<String>>,
    #[schemars(description = "Filter by slug")]
    pub slug: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn site_groups_list(
    client: &NetboxClient,
    p: SiteGroupsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    for v in p.name.unwrap_or_default() {
        params.push(("name", v));
    }
    for v in p.slug.unwrap_or_default() {
        params.push(("slug", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client.list("/api/dcim/site-groups/", &params).await
}

// --------------------------------------------------------------------------
// Virtual device contexts
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VirtualDeviceContextsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by device ID")]
    pub device_id: Option<i32>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(description = "Maximum number of results (default 50, max 1000)")]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset")]
    pub offset: Option<i32>,
}

pub async fn virtual_device_contexts_list(
    client: &NetboxClient,
    p: VirtualDeviceContextsListParams,
) -> Result<Value, NetboxError> {
    let mut params: Vec<(&str, String)> = vec![];
    if let Some(q) = p.q {
        params.push(("q", q));
    }
    if let Some(v) = p.device_id {
        params.push(("device_id", v.to_string()));
    }
    for v in p.tenant.unwrap_or_default() {
        params.push(("tenant", v));
    }
    if let Some(v) = p.ordering {
        params.push(("ordering", v));
    }
    params.push(("limit", clamp_limit(p.limit).to_string()));
    params.push(("offset", p.offset.unwrap_or(0).to_string()));
    client
        .list("/api/dcim/virtual-device-contexts/", &params)
        .await
}
