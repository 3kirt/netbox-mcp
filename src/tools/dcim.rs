use crate::client::{NetboxClient, NetboxError};
use crate::tools::{QueryBuilder, paginate, resolve_device_id};
use serde::Deserialize;
use serde_json::Value;

// --------------------------------------------------------------------------
// Devices
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct DevicesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
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
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by (prefix with - for descending)")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn devices_list(
    client: &NetboxClient,
    p: DevicesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .opt("name__ic", p.name_ic)
        .many("site", p.site)
        .many("role", p.role)
        .many("status", p.status)
        .many("tenant", p.tenant)
        .opt("rack_id", p.rack_id)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/devices/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn sites_list(client: &NetboxClient, p: SitesListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("status", p.status)
        .many("region", p.region)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/sites/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn racks_list(client: &NetboxClient, p: RacksListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("site", p.site)
        .many("location", p.location)
        .many("status", p.status)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/racks/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
}

// --------------------------------------------------------------------------
// Interfaces
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct InterfacesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
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
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn interfaces_list(
    client: &NetboxClient,
    p: InterfacesListParams,
) -> Result<Value, NetboxError> {
    let device_id = match p.device {
        Some(name) => Some(resolve_device_id(client, &name).await?),
        None => p.device_id,
    };
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .opt("q", p.q)
        .many("name", p.name)
        .many("type", p.r#type)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/interfaces/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn cables_list(client: &NetboxClient, p: CablesListParams) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("site", p.site)
        .many("status", p.status)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/cables/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn regions_list(
    client: &NetboxClient,
    p: RegionsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .many("parent", p.parent)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/regions/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn locations_list(
    client: &NetboxClient,
    p: LocationsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("site", p.site)
        .many("parent", p.parent)
        .many("status", p.status)
        .many("tenant", p.tenant)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/locations/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn manufacturers_list(
    client: &NetboxClient,
    p: ManufacturersListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/manufacturers/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn device_types_list(
    client: &NetboxClient,
    p: DeviceTypesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("manufacturer", p.manufacturer)
        .many("model", p.model)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/device-types/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn device_roles_list(
    client: &NetboxClient,
    p: DeviceRolesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("vm_role", p.vm_role)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/device-roles/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(description = "Filter by tag slug (multi-value)")]
    pub tag: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn platforms_list(
    client: &NetboxClient,
    p: PlatformsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("manufacturer", p.manufacturer)
        .many("tag", p.tag)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/platforms/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn power_panels_list(
    client: &NetboxClient,
    p: PowerPanelsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("site", p.site)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/power-panels/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn power_feeds_list(
    client: &NetboxClient,
    p: PowerFeedsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("site", p.site)
        .many("status", p.status)
        .opt("type", p.r#type)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/power-feeds/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn virtual_chassis_list(
    client: &NetboxClient,
    p: VirtualChassisListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("site", p.site)
        .many("tenant", p.tenant)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/virtual-chassis/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
}

// --------------------------------------------------------------------------
// Inventory items
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct InventoryItemsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
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
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn inventory_items_list(
    client: &NetboxClient,
    p: InventoryItemsListParams,
) -> Result<Value, NetboxError> {
    let device_id = match p.device {
        Some(name) => Some(resolve_device_id(client, &name).await?),
        None => p.device_id,
    };
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .opt("q", p.q)
        .many("manufacturer", p.manufacturer)
        .opt("discovered", p.discovered)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/inventory-items/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn cable_terminations_list(
    client: &NetboxClient,
    p: CableTerminationsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("cable_id", p.cable_id)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/cable-terminations/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn console_ports_list(
    client: &NetboxClient,
    p: ConsolePortsListParams,
) -> Result<Value, NetboxError> {
    let device_id = match p.device {
        Some(name) => Some(resolve_device_id(client, &name).await?),
        None => p.device_id,
    };
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .opt("q", p.q)
        .many("name", p.name)
        .many("site", p.site)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/console-ports/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn console_server_ports_list(
    client: &NetboxClient,
    p: ConsoleServerPortsListParams,
) -> Result<Value, NetboxError> {
    let device_id = match p.device {
        Some(name) => Some(resolve_device_id(client, &name).await?),
        None => p.device_id,
    };
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .opt("q", p.q)
        .many("name", p.name)
        .many("site", p.site)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/console-server-ports/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
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
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn device_bays_list(
    client: &NetboxClient,
    p: DeviceBaysListParams,
) -> Result<Value, NetboxError> {
    let device_id = match p.device {
        Some(name) => Some(resolve_device_id(client, &name).await?),
        None => p.device_id,
    };
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .opt("q", p.q)
        .many("name", p.name)
        .many("site", p.site)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/device-bays/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn front_ports_list(
    client: &NetboxClient,
    p: FrontPortsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/front-ports/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
}

// --------------------------------------------------------------------------
// MAC addresses
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct MacAddressesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn mac_addresses_list(
    client: &NetboxClient,
    p: MacAddressesListParams,
) -> Result<Value, NetboxError> {
    let device_id = match p.device {
        Some(name) => Some(resolve_device_id(client, &name).await?),
        None => p.device_id,
    };
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .opt("q", p.q)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/mac-addresses/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
}

// --------------------------------------------------------------------------
// Modules
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModulesListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
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
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn modules_list(
    client: &NetboxClient,
    p: ModulesListParams,
) -> Result<Value, NetboxError> {
    let device_id = match p.device {
        Some(name) => Some(resolve_device_id(client, &name).await?),
        None => p.device_id,
    };
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .opt("q", p.q)
        .many("site", p.site)
        .many("status", p.status)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/modules/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
}

// --------------------------------------------------------------------------
// Module bays
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct ModuleBaysListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn module_bays_list(
    client: &NetboxClient,
    p: ModuleBaysListParams,
) -> Result<Value, NetboxError> {
    let device_id = match p.device {
        Some(name) => Some(resolve_device_id(client, &name).await?),
        None => p.device_id,
    };
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .opt("q", p.q)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/module-bays/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn module_types_list(
    client: &NetboxClient,
    p: ModuleTypesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("manufacturer", p.manufacturer)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/module-types/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn power_outlets_list(
    client: &NetboxClient,
    p: PowerOutletsListParams,
) -> Result<Value, NetboxError> {
    let device_id = match p.device {
        Some(name) => Some(resolve_device_id(client, &name).await?),
        None => p.device_id,
    };
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .opt("q", p.q)
        .many("name", p.name)
        .many("site", p.site)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/power-outlets/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by site slug")]
    pub site: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn power_ports_list(
    client: &NetboxClient,
    p: PowerPortsListParams,
) -> Result<Value, NetboxError> {
    let device_id = match p.device {
        Some(name) => Some(resolve_device_id(client, &name).await?),
        None => p.device_id,
    };
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .opt("q", p.q)
        .many("name", p.name)
        .many("site", p.site)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/power-ports/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn rack_reservations_list(
    client: &NetboxClient,
    p: RackReservationsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .opt("rack_id", p.rack_id)
        .many("site", p.site)
        .many("tenant", p.tenant)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/rack-reservations/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn rack_roles_list(
    client: &NetboxClient,
    p: RackRolesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/rack-roles/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn rack_types_list(
    client: &NetboxClient,
    p: RackTypesListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("slug", p.slug)
        .many("manufacturer", p.manufacturer)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/rack-types/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn rear_ports_list(
    client: &NetboxClient,
    p: RearPortsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/rear-ports/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
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
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn site_groups_list(
    client: &NetboxClient,
    p: SiteGroupsListParams,
) -> Result<Value, NetboxError> {
    let qb = QueryBuilder::new()
        .opt("q", p.q)
        .many("name", p.name)
        .many("slug", p.slug)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/site-groups/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
}

// --------------------------------------------------------------------------
// Virtual device contexts
// --------------------------------------------------------------------------

#[derive(Debug, Deserialize, schemars::JsonSchema)]
pub struct VirtualDeviceContextsListParams {
    #[schemars(description = "Free-text search")]
    pub q: Option<String>,
    #[schemars(description = "Filter by device ID (prefer device for name-based lookup)")]
    pub device_id: Option<i32>,
    #[schemars(
        description = "Filter by device name — resolves to ID automatically (preferred over device_id)"
    )]
    pub device: Option<String>,
    #[schemars(description = "Filter by tenant slug")]
    pub tenant: Option<Vec<String>>,
    #[schemars(description = "Field to order results by")]
    pub ordering: Option<String>,
    #[schemars(
        description = "Maximum number of results (default 50, max 1000); ignored when fetch_all is true"
    )]
    pub limit: Option<i32>,
    #[schemars(description = "Pagination offset; ignored when fetch_all is true")]
    pub offset: Option<i32>,
    #[schemars(
        description = "Fetch all matching results automatically, ignoring limit and offset"
    )]
    pub fetch_all: Option<bool>,
}

pub async fn virtual_device_contexts_list(
    client: &NetboxClient,
    p: VirtualDeviceContextsListParams,
) -> Result<Value, NetboxError> {
    let device_id = match p.device {
        Some(name) => Some(resolve_device_id(client, &name).await?),
        None => p.device_id,
    };
    let qb = QueryBuilder::new()
        .opt("device_id", device_id)
        .opt("q", p.q)
        .many("tenant", p.tenant)
        .opt("ordering", p.ordering);
    paginate(
        client,
        "/api/dcim/virtual-device-contexts/",
        qb.into_params(),
        p.limit,
        p.offset,
        p.fetch_all,
    )
    .await
}
