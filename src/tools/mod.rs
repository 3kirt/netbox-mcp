use rmcp::{
    ErrorData as McpError, RoleServer, ServerHandler,
    handler::server::{
        router::{prompt::PromptRouter, tool::ToolRouter},
        wrapper::Parameters,
    },
    model::*,
    prompt, prompt_handler, prompt_router,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use serde_json::Value;
use std::sync::{Arc, OnceLock};

use crate::client::{NetboxClient, NetboxError};

pub mod circuits;
pub mod core;
pub mod dcim;
pub mod extras;
pub mod ipam;
pub mod tenancy;
pub mod users;
pub mod virtualization;
pub mod vpn;
pub mod wireless;

// --------------------------------------------------------------------------
// Shared helpers
// --------------------------------------------------------------------------

const DEFAULT_LIMIT: i32 = 50;
const MAX_LIMIT: i32 = 1000;

pub fn clamp_limit(limit: Option<i32>) -> i32 {
    match limit {
        None | Some(0) => DEFAULT_LIMIT,
        Some(n) if n < 0 => DEFAULT_LIMIT,
        Some(n) if n > MAX_LIMIT => MAX_LIMIT,
        Some(n) => n,
    }
}

pub fn json_result(v: Value) -> Result<CallToolResult, McpError> {
    let v = slim_value(v);
    let text = serde_json::to_string_pretty(&v)
        .map_err(|e| McpError::internal_error(format!("marshalling response: {e}"), None))?;
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

pub fn tool_error(msg: &str) -> Result<CallToolResult, McpError> {
    Ok(CallToolResult::error(vec![Content::text(msg)]))
}

/// Fields unconditionally removed from every object in a NetBox response.
const STRIP_KEYS: &[&str] = &[
    // Raw per-object config context; config_context is the resolved value the AI needs.
    "local_context_data",
    // Convenience alias for primary_ip4/primary_ip6; always duplicates one of them.
    "primary_ip",
];

/// Recursively remove null-valued fields and noise keys from a JSON value.
/// Cuts typical NetBox response sizes by 50–70%.
fn slim_value(v: Value) -> Value {
    match v {
        Value::Object(map) => {
            let map: serde_json::Map<_, _> = map
                .into_iter()
                .filter(|(k, v)| !v.is_null() && !STRIP_KEYS.contains(&k.as_str()))
                .map(|(k, v)| (k, slim_value(v)))
                .collect();
            Value::Object(map)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(slim_value).collect()),
        other => other,
    }
}

/// Resolve a device name to its NetBox numeric ID.
pub async fn resolve_device_id(client: &NetboxClient, name: &str) -> Result<i32, NetboxError> {
    let resp = client
        .list("/api/dcim/devices/", &[("name", name.to_string())])
        .await?;
    resp["results"][0]["id"]
        .as_i64()
        .map(|id| id as i32)
        .ok_or_else(|| NetboxError::Generic(format!("device '{name}' not found")))
}

/// Resolve a virtual machine name to its NetBox numeric ID.
pub async fn resolve_vm_id(client: &NetboxClient, name: &str) -> Result<i32, NetboxError> {
    let resp = client
        .list(
            "/api/virtualization/virtual-machines/",
            &[("name", name.to_string())],
        )
        .await?;
    resp["results"][0]["id"]
        .as_i64()
        .map(|id| id as i32)
        .ok_or_else(|| NetboxError::Generic(format!("virtual machine '{name}' not found")))
}

/// Shared "get by ID" parameter used by every `*_get` shim.
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetByIdParams {
    #[schemars(description = "NetBox ID of the object to retrieve")]
    pub id: i32,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct LookupHostParams {
    #[schemars(
        description = "Hostname or FQDN to search for. Case-insensitive, partial match — 'web01' matches 'web01.example.com'."
    )]
    pub name: String,
}

// --------------------------------------------------------------------------
// Query construction
//
// Each domain `*_list` function builds a flat (key, value) parameter vector
// and hands it to `paginate`. QueryBuilder is a small fluent helper that
// removes the repetitive `if let Some / for v in unwrap_or_default` noise.
// --------------------------------------------------------------------------

pub struct QueryBuilder {
    params: Vec<(&'static str, String)>,
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self { params: vec![] }
    }

    /// Append `(key, v.to_string())` if `v` is Some.
    pub fn opt<T: ToString>(mut self, key: &'static str, v: Option<T>) -> Self {
        if let Some(v) = v {
            self.params.push((key, v.to_string()));
        }
        self
    }

    /// Append `(key, x)` for every element of `v` (an empty / None list adds nothing).
    pub fn many(mut self, key: &'static str, v: Option<Vec<String>>) -> Self {
        for x in v.unwrap_or_default() {
            self.params.push((key, x));
        }
        self
    }

    pub fn into_params(self) -> Vec<(&'static str, String)> {
        self.params
    }
}

/// Append `limit` and `offset` to a parameter vector unless `fetch_all` is
/// requested (in which case the client iterates internally). Split out from
/// `paginate` so the branching is unit-testable without an HTTP client.
fn finalize_params(
    mut params: Vec<(&'static str, String)>,
    limit: Option<i32>,
    offset: Option<i32>,
    fetch_all: Option<bool>,
) -> Vec<(&'static str, String)> {
    if !fetch_all.unwrap_or(false) {
        params.push(("limit", clamp_limit(limit).to_string()));
        params.push(("offset", offset.unwrap_or(0).to_string()));
    }
    params
}

/// Replace raw NetBox pagination URLs with structured paging hints.
///
/// Strips `next`/`previous` (internal hostnames, unusable by the AI) and
/// adds `has_more: bool` plus `next_offset: u64` (only when `has_more` is
/// true) so the caller can page by incrementing `offset`.
fn clean_page_response(mut resp: Value, offset: u64, limit: u64) -> Value {
    if let Some(obj) = resp.as_object_mut() {
        let count = obj.get("count").and_then(|v| v.as_u64()).unwrap_or(0);
        let next_offset = offset + limit;
        let has_more = next_offset < count;
        obj.remove("next");
        obj.remove("previous");
        obj.insert("has_more".to_string(), serde_json::json!(has_more));
        obj.insert(
            "next_offset".to_string(),
            serde_json::json!(if has_more { next_offset } else { count }),
        );
    }
    resp
}

/// Issue a paginated GET. With `fetch_all=Some(true)` this calls `list_all`
/// and ignores `limit`/`offset`; otherwise it honors them with default
/// clamping.
pub async fn paginate(
    client: &NetboxClient,
    path: &str,
    params: Vec<(&'static str, String)>,
    limit: Option<i32>,
    offset: Option<i32>,
    fetch_all: Option<bool>,
) -> Result<Value, NetboxError> {
    let want_all = fetch_all.unwrap_or(false);
    let finalized = finalize_params(params, limit, offset, fetch_all);
    let p: Vec<(&str, String)> = finalized.into_iter().collect();
    if want_all {
        let mut resp = client.list_all(path, &p).await?;
        if let Some(obj) = resp.as_object_mut() {
            let count = obj.get("count").and_then(|v| v.as_u64()).unwrap_or(0);
            obj.remove("next");
            obj.remove("previous");
            obj.insert("has_more".to_string(), serde_json::json!(false));
            obj.insert("next_offset".to_string(), serde_json::json!(count));
        }
        Ok(resp)
    } else {
        let effective_limit = clamp_limit(limit) as u64;
        let effective_offset = offset.unwrap_or(0).max(0) as u64;
        let resp = client.list(path, &p).await?;
        Ok(clean_page_response(resp, effective_offset, effective_limit))
    }
}

// --------------------------------------------------------------------------
// Shim body macros
//
// Every `*_list` shim fetches the client, calls a domain function, and
// converts the Result into an MCP response. Every `*_get` shim does the
// same with `client.get(path, id)`. Bodies expand to a single macro call.
// --------------------------------------------------------------------------

macro_rules! delegate_list {
    ($self:expr, $domain_fn:path, $p:expr, $noun:literal) => {{
        let client = $self.get_client()?;
        match $domain_fn(client, $p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing {}: {}", $noun, e)),
        }
    }};
}

macro_rules! delegate_get {
    ($self:expr, $path:literal, $id:expr, $noun:literal) => {{
        let client = $self.get_client()?;
        match client.get($path, $id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting {} {}: {}", $noun, $id, e)),
        }
    }};
}

// --------------------------------------------------------------------------
// Server struct
// --------------------------------------------------------------------------

/// The MCP server — holds a NetBox client and the generated tool/prompt routers.
#[derive(Clone)]
pub struct NetboxMcpServer {
    /// Shared NetBox client. In HTTP mode this is populated in `initialize()`
    /// once the per-session bearer token has been extracted from request headers.
    client: Arc<OnceLock<NetboxClient>>,
    base_url: String,
    #[allow(dead_code)]
    tool_router: ToolRouter<NetboxMcpServer>,
    #[allow(dead_code)]
    prompt_router: PromptRouter<NetboxMcpServer>,
}

impl NetboxMcpServer {
    /// Create a server with an already-known token (stdio mode).
    pub fn new_stdio(base_url: String, token: String) -> Self {
        let cell = OnceLock::new();
        let _ = cell.set(NetboxClient::new(base_url.clone(), token));
        Self {
            client: Arc::new(cell),
            base_url,
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }

    /// Create a server without a token (HTTP mode — token injected in `initialize()`).
    pub fn new_http(base_url: String) -> Self {
        Self {
            client: Arc::new(OnceLock::new()),
            base_url,
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }

    /// Borrow the initialized client or return a tool error.
    fn get_client(&self) -> Result<&NetboxClient, McpError> {
        self.client
            .get()
            .ok_or_else(|| McpError::internal_error("NetBox client not initialized", None))
    }
}

// --------------------------------------------------------------------------
// Tool shims — one per tool, delegating to domain functions
// --------------------------------------------------------------------------

#[tool_router]
impl NetboxMcpServer {
    // DCIM — devices
    #[tool(
        description = "List devices. Filters: name, site, role, status, tenant, rack_id, tag. Use fetch_all=true to retrieve all results beyond the default 50-item limit."
    )]
    async fn netbox_dcim_devices_list(
        &self,
        Parameters(p): Parameters<dcim::DevicesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::devices_list, p, "devices")
    }
    #[tool(
        description = "Get a single device by its NetBox ID. Use netbox_dcim_devices_list to find the ID first."
    )]
    async fn netbox_dcim_devices_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/devices/", p.id, "device")
    }

    // DCIM — sites
    #[tool(
        description = "List sites. Filters: name, status, region, tag. Use fetch_all=true to get all results."
    )]
    async fn netbox_dcim_sites_list(
        &self,
        Parameters(p): Parameters<dcim::SitesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::sites_list, p, "sites")
    }
    #[tool(description = "Get a site by its NetBox ID.")]
    async fn netbox_dcim_sites_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/sites/", p.id, "site")
    }

    // DCIM — racks
    #[tool(description = "List racks in NetBox, optionally filtered by site, location, or status.")]
    async fn netbox_dcim_racks_list(
        &self,
        Parameters(p): Parameters<dcim::RacksListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::racks_list, p, "racks")
    }
    #[tool(description = "Get a rack by its NetBox ID.")]
    async fn netbox_dcim_racks_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/racks/", p.id, "rack")
    }

    // DCIM — interfaces
    #[tool(
        description = "List device interfaces. Use device=<name> to filter by device name directly — no need to look up the device ID first. Also filters: name, type, tag, fetch_all."
    )]
    async fn netbox_dcim_interfaces_list(
        &self,
        Parameters(p): Parameters<dcim::InterfacesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::interfaces_list, p, "interfaces")
    }
    #[tool(description = "Get a device interface by its NetBox ID.")]
    async fn netbox_dcim_interfaces_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/interfaces/", p.id, "interface")
    }

    // DCIM — cables
    #[tool(description = "List cables in NetBox, optionally filtered by site or status.")]
    async fn netbox_dcim_cables_list(
        &self,
        Parameters(p): Parameters<dcim::CablesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::cables_list, p, "cables")
    }
    #[tool(description = "Get a cable by its NetBox ID.")]
    async fn netbox_dcim_cables_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/cables/", p.id, "cable")
    }

    // DCIM — regions
    #[tool(description = "List regions in NetBox, optionally filtered by name, slug, or parent.")]
    async fn netbox_dcim_regions_list(
        &self,
        Parameters(p): Parameters<dcim::RegionsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::regions_list, p, "regions")
    }
    #[tool(description = "Get a region by its NetBox ID.")]
    async fn netbox_dcim_regions_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/regions/", p.id, "region")
    }

    // DCIM — locations
    #[tool(
        description = "List locations in NetBox, optionally filtered by site, parent, status, or tenant."
    )]
    async fn netbox_dcim_locations_list(
        &self,
        Parameters(p): Parameters<dcim::LocationsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::locations_list, p, "locations")
    }
    #[tool(description = "Get a location by its NetBox ID.")]
    async fn netbox_dcim_locations_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/locations/", p.id, "location")
    }

    // DCIM — manufacturers
    #[tool(description = "List manufacturers in NetBox, optionally filtered by name or slug.")]
    async fn netbox_dcim_manufacturers_list(
        &self,
        Parameters(p): Parameters<dcim::ManufacturersListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::manufacturers_list, p, "manufacturers")
    }
    #[tool(description = "Get a manufacturer by its NetBox ID.")]
    async fn netbox_dcim_manufacturers_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/manufacturers/", p.id, "manufacturer")
    }

    // DCIM — device types
    #[tool(
        description = "List device types in NetBox, optionally filtered by manufacturer or model."
    )]
    async fn netbox_dcim_device_types_list(
        &self,
        Parameters(p): Parameters<dcim::DeviceTypesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::device_types_list, p, "device types")
    }
    #[tool(description = "Get a device type by its NetBox ID.")]
    async fn netbox_dcim_device_types_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/device-types/", p.id, "device type")
    }

    // DCIM — device roles
    #[tool(
        description = "List device roles in NetBox, optionally filtered by name, slug, or VM eligibility."
    )]
    async fn netbox_dcim_device_roles_list(
        &self,
        Parameters(p): Parameters<dcim::DeviceRolesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::device_roles_list, p, "device roles")
    }
    #[tool(description = "Get a device role by its NetBox ID.")]
    async fn netbox_dcim_device_roles_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/device-roles/", p.id, "device role")
    }

    // DCIM — platforms
    #[tool(description = "List platforms in NetBox, optionally filtered by name or manufacturer.")]
    async fn netbox_dcim_platforms_list(
        &self,
        Parameters(p): Parameters<dcim::PlatformsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::platforms_list, p, "platforms")
    }
    #[tool(description = "Get a platform by its NetBox ID.")]
    async fn netbox_dcim_platforms_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/platforms/", p.id, "platform")
    }

    // DCIM — power panels
    #[tool(description = "List power panels in NetBox, optionally filtered by site.")]
    async fn netbox_dcim_power_panels_list(
        &self,
        Parameters(p): Parameters<dcim::PowerPanelsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::power_panels_list, p, "power panels")
    }
    #[tool(description = "Get a power panel by its NetBox ID.")]
    async fn netbox_dcim_power_panels_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/power-panels/", p.id, "power panel")
    }

    // DCIM — power feeds
    #[tool(
        description = "List power feeds in NetBox, optionally filtered by site, status, or type."
    )]
    async fn netbox_dcim_power_feeds_list(
        &self,
        Parameters(p): Parameters<dcim::PowerFeedsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::power_feeds_list, p, "power feeds")
    }
    #[tool(description = "Get a power feed by its NetBox ID.")]
    async fn netbox_dcim_power_feeds_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/power-feeds/", p.id, "power feed")
    }

    // DCIM — virtual chassis
    #[tool(description = "List virtual chassis in NetBox, optionally filtered by site or tenant.")]
    async fn netbox_dcim_virtual_chassis_list(
        &self,
        Parameters(p): Parameters<dcim::VirtualChassisListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::virtual_chassis_list, p, "virtual chassis")
    }
    #[tool(description = "Get a virtual chassis by its NetBox ID.")]
    async fn netbox_dcim_virtual_chassis_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/virtual-chassis/", p.id, "virtual chassis")
    }

    // DCIM — inventory items
    #[tool(
        description = "List inventory items in NetBox, optionally filtered by device, manufacturer, or discovery status."
    )]
    async fn netbox_dcim_inventory_items_list(
        &self,
        Parameters(p): Parameters<dcim::InventoryItemsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::inventory_items_list, p, "inventory items")
    }
    #[tool(description = "Get an inventory item by its NetBox ID.")]
    async fn netbox_dcim_inventory_items_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/inventory-items/", p.id, "inventory item")
    }

    // DCIM — cable terminations
    #[tool(description = "List cable terminations in NetBox, optionally filtered by cable ID.")]
    async fn netbox_dcim_cable_terminations_list(
        &self,
        Parameters(p): Parameters<dcim::CableTerminationsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::cable_terminations_list, p, "cable terminations")
    }
    #[tool(description = "Get a cable termination by its NetBox ID.")]
    async fn netbox_dcim_cable_terminations_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/dcim/cable-terminations/",
            p.id,
            "cable termination"
        )
    }

    // DCIM — console ports
    #[tool(
        description = "List console ports in NetBox, optionally filtered by name, device, or site."
    )]
    async fn netbox_dcim_console_ports_list(
        &self,
        Parameters(p): Parameters<dcim::ConsolePortsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::console_ports_list, p, "console ports")
    }
    #[tool(description = "Get a console port by its NetBox ID.")]
    async fn netbox_dcim_console_ports_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/console-ports/", p.id, "console port")
    }

    // DCIM — console server ports
    #[tool(
        description = "List console server ports in NetBox, optionally filtered by name, device, or site."
    )]
    async fn netbox_dcim_console_server_ports_list(
        &self,
        Parameters(p): Parameters<dcim::ConsoleServerPortsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(
            self,
            dcim::console_server_ports_list,
            p,
            "console server ports"
        )
    }
    #[tool(description = "Get a console server port by its NetBox ID.")]
    async fn netbox_dcim_console_server_ports_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/dcim/console-server-ports/",
            p.id,
            "console server port"
        )
    }

    // DCIM — device bays
    #[tool(
        description = "List device bays in NetBox, optionally filtered by name, device, or site."
    )]
    async fn netbox_dcim_device_bays_list(
        &self,
        Parameters(p): Parameters<dcim::DeviceBaysListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::device_bays_list, p, "device bays")
    }
    #[tool(description = "Get a device bay by its NetBox ID.")]
    async fn netbox_dcim_device_bays_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/device-bays/", p.id, "device bay")
    }

    // DCIM — front ports
    #[tool(description = "List front ports in NetBox, optionally filtered by name.")]
    async fn netbox_dcim_front_ports_list(
        &self,
        Parameters(p): Parameters<dcim::FrontPortsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::front_ports_list, p, "front ports")
    }
    #[tool(description = "Get a front port by its NetBox ID.")]
    async fn netbox_dcim_front_ports_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/front-ports/", p.id, "front port")
    }

    // DCIM — MAC addresses
    #[tool(description = "List MAC addresses in NetBox, optionally filtered by device ID.")]
    async fn netbox_dcim_mac_addresses_list(
        &self,
        Parameters(p): Parameters<dcim::MacAddressesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::mac_addresses_list, p, "MAC addresses")
    }
    #[tool(description = "Get a MAC address by its NetBox ID.")]
    async fn netbox_dcim_mac_addresses_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/mac-addresses/", p.id, "MAC address")
    }

    // DCIM — modules
    #[tool(description = "List modules in NetBox, optionally filtered by device, site, or status.")]
    async fn netbox_dcim_modules_list(
        &self,
        Parameters(p): Parameters<dcim::ModulesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::modules_list, p, "modules")
    }
    #[tool(description = "Get a module by its NetBox ID.")]
    async fn netbox_dcim_modules_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/modules/", p.id, "module")
    }

    // DCIM — module bays
    #[tool(description = "List module bays in NetBox, optionally filtered by device ID.")]
    async fn netbox_dcim_module_bays_list(
        &self,
        Parameters(p): Parameters<dcim::ModuleBaysListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::module_bays_list, p, "module bays")
    }
    #[tool(description = "Get a module bay by its NetBox ID.")]
    async fn netbox_dcim_module_bays_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/module-bays/", p.id, "module bay")
    }

    // DCIM — module types
    #[tool(description = "List module types in NetBox, optionally filtered by manufacturer.")]
    async fn netbox_dcim_module_types_list(
        &self,
        Parameters(p): Parameters<dcim::ModuleTypesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::module_types_list, p, "module types")
    }
    #[tool(description = "Get a module type by its NetBox ID.")]
    async fn netbox_dcim_module_types_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/module-types/", p.id, "module type")
    }

    // DCIM — power outlets
    #[tool(
        description = "List power outlets in NetBox, optionally filtered by name, device, or site."
    )]
    async fn netbox_dcim_power_outlets_list(
        &self,
        Parameters(p): Parameters<dcim::PowerOutletsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::power_outlets_list, p, "power outlets")
    }
    #[tool(description = "Get a power outlet by its NetBox ID.")]
    async fn netbox_dcim_power_outlets_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/power-outlets/", p.id, "power outlet")
    }

    // DCIM — power ports
    #[tool(
        description = "List power ports in NetBox, optionally filtered by name, device, or site."
    )]
    async fn netbox_dcim_power_ports_list(
        &self,
        Parameters(p): Parameters<dcim::PowerPortsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::power_ports_list, p, "power ports")
    }
    #[tool(description = "Get a power port by its NetBox ID.")]
    async fn netbox_dcim_power_ports_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/power-ports/", p.id, "power port")
    }

    // DCIM — rack reservations
    #[tool(
        description = "List rack reservations in NetBox, optionally filtered by rack, site, or tenant."
    )]
    async fn netbox_dcim_rack_reservations_list(
        &self,
        Parameters(p): Parameters<dcim::RackReservationsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::rack_reservations_list, p, "rack reservations")
    }
    #[tool(description = "Get a rack reservation by its NetBox ID.")]
    async fn netbox_dcim_rack_reservations_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/dcim/rack-reservations/",
            p.id,
            "rack reservation"
        )
    }

    // DCIM — rack roles
    #[tool(description = "List rack roles in NetBox, optionally filtered by name or slug.")]
    async fn netbox_dcim_rack_roles_list(
        &self,
        Parameters(p): Parameters<dcim::RackRolesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::rack_roles_list, p, "rack roles")
    }
    #[tool(description = "Get a rack role by its NetBox ID.")]
    async fn netbox_dcim_rack_roles_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/rack-roles/", p.id, "rack role")
    }

    // DCIM — rack types
    #[tool(description = "List rack types in NetBox, optionally filtered by slug or manufacturer.")]
    async fn netbox_dcim_rack_types_list(
        &self,
        Parameters(p): Parameters<dcim::RackTypesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::rack_types_list, p, "rack types")
    }
    #[tool(description = "Get a rack type by its NetBox ID.")]
    async fn netbox_dcim_rack_types_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/rack-types/", p.id, "rack type")
    }

    // DCIM — rear ports
    #[tool(description = "List rear ports in NetBox, optionally filtered by name.")]
    async fn netbox_dcim_rear_ports_list(
        &self,
        Parameters(p): Parameters<dcim::RearPortsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::rear_ports_list, p, "rear ports")
    }
    #[tool(description = "Get a rear port by its NetBox ID.")]
    async fn netbox_dcim_rear_ports_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/rear-ports/", p.id, "rear port")
    }

    // DCIM — site groups
    #[tool(description = "List site groups in NetBox, optionally filtered by name or slug.")]
    async fn netbox_dcim_site_groups_list(
        &self,
        Parameters(p): Parameters<dcim::SiteGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, dcim::site_groups_list, p, "site groups")
    }
    #[tool(description = "Get a site group by its NetBox ID.")]
    async fn netbox_dcim_site_groups_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/dcim/site-groups/", p.id, "site group")
    }

    // DCIM — virtual device contexts
    #[tool(
        description = "List virtual device contexts in NetBox, optionally filtered by device or tenant."
    )]
    async fn netbox_dcim_virtual_device_contexts_list(
        &self,
        Parameters(p): Parameters<dcim::VirtualDeviceContextsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(
            self,
            dcim::virtual_device_contexts_list,
            p,
            "virtual device contexts"
        )
    }
    #[tool(description = "Get a virtual device context by its NetBox ID.")]
    async fn netbox_dcim_virtual_device_contexts_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/dcim/virtual-device-contexts/",
            p.id,
            "virtual device context"
        )
    }

    // ---- IPAM ----

    #[tool(
        description = "List IP addresses. Filters: address, vrf (rd, e.g. 65000:100), status, role, parent (IPs within prefix incl. network/broadcast), device, device_id, virtual_machine, virtual_machine_id, tenant, tag. Use fetch_all=true for all results."
    )]
    async fn netbox_ipam_ip_addresses_list(
        &self,
        Parameters(p): Parameters<ipam::IpAddressesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::ip_addresses_list, p, "IP addresses")
    }
    #[tool(description = "Get an IP address by its NetBox ID.")]
    async fn netbox_ipam_ip_addresses_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/ip-addresses/", p.id, "IP address")
    }

    #[tool(
        description = "List prefixes. Filters: prefix, vrf (rd, e.g. 65000:100), status, role, site, tenant, family (4/6), tag. Use fetch_all=true for all results."
    )]
    async fn netbox_ipam_prefixes_list(
        &self,
        Parameters(p): Parameters<ipam::PrefixesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::prefixes_list, p, "prefixes")
    }
    #[tool(description = "Get a prefix by its NetBox ID.")]
    async fn netbox_ipam_prefixes_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/prefixes/", p.id, "prefix")
    }

    #[tool(description = "List VRFs (filter: q, name, rd, tenant).")]
    async fn netbox_ipam_vrfs_list(
        &self,
        Parameters(p): Parameters<ipam::VrfsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::vrfs_list, p, "VRFs")
    }
    #[tool(description = "Get a VRF by its NetBox ID.")]
    async fn netbox_ipam_vrfs_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/vrfs/", p.id, "VRF")
    }

    #[tool(description = "List VLANs (filter: q, vid, name, site, group, status).")]
    async fn netbox_ipam_vlans_list(
        &self,
        Parameters(p): Parameters<ipam::VlansListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::vlans_list, p, "VLANs")
    }
    #[tool(description = "Get a VLAN by its NetBox ID.")]
    async fn netbox_ipam_vlans_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/vlans/", p.id, "VLAN")
    }

    #[tool(description = "List aggregates (filter: q, family, rir, tenant).")]
    async fn netbox_ipam_aggregates_list(
        &self,
        Parameters(p): Parameters<ipam::AggregatesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::aggregates_list, p, "aggregates")
    }
    #[tool(description = "Get an aggregate by its NetBox ID.")]
    async fn netbox_ipam_aggregates_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/aggregates/", p.id, "aggregate")
    }

    #[tool(description = "List IP ranges (filter: q, vrf, status, tenant).")]
    async fn netbox_ipam_ip_ranges_list(
        &self,
        Parameters(p): Parameters<ipam::IpRangesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::ip_ranges_list, p, "IP ranges")
    }
    #[tool(description = "Get an IP range by its NetBox ID.")]
    async fn netbox_ipam_ip_ranges_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/ip-ranges/", p.id, "IP range")
    }

    #[tool(description = "List route targets (filter: q, name, tenant).")]
    async fn netbox_ipam_route_targets_list(
        &self,
        Parameters(p): Parameters<ipam::RouteTargetsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::route_targets_list, p, "route targets")
    }
    #[tool(description = "Get a route target by its NetBox ID.")]
    async fn netbox_ipam_route_targets_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/route-targets/", p.id, "route target")
    }

    #[tool(description = "List RIRs (filter: q, name, slug).")]
    async fn netbox_ipam_rirs_list(
        &self,
        Parameters(p): Parameters<ipam::RirsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::rirs_list, p, "RIRs")
    }
    #[tool(description = "Get a RIR by its NetBox ID.")]
    async fn netbox_ipam_rirs_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/rirs/", p.id, "RIR")
    }

    #[tool(description = "List VLAN groups (filter: q, name).")]
    async fn netbox_ipam_vlan_groups_list(
        &self,
        Parameters(p): Parameters<ipam::VlanGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::vlan_groups_list, p, "VLAN groups")
    }
    #[tool(description = "Get a VLAN group by its NetBox ID.")]
    async fn netbox_ipam_vlan_groups_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/vlan-groups/", p.id, "VLAN group")
    }

    #[tool(description = "List services (filter: q, device, virtual machine, protocol).")]
    async fn netbox_ipam_services_list(
        &self,
        Parameters(p): Parameters<ipam::ServicesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::services_list, p, "services")
    }
    #[tool(description = "Get a service by its NetBox ID.")]
    async fn netbox_ipam_services_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/services/", p.id, "service")
    }

    #[tool(description = "List ASNs (filter: q, site, tenant).")]
    async fn netbox_ipam_asns_list(
        &self,
        Parameters(p): Parameters<ipam::AsnsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::asns_list, p, "ASNs")
    }
    #[tool(description = "Get an ASN by its NetBox ID.")]
    async fn netbox_ipam_asns_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/asns/", p.id, "ASN")
    }

    #[tool(description = "List FHRP groups (filter: q, name, protocol).")]
    async fn netbox_ipam_fhrp_groups_list(
        &self,
        Parameters(p): Parameters<ipam::FhrpGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::fhrp_groups_list, p, "FHRP groups")
    }
    #[tool(description = "Get an FHRP group by its NetBox ID.")]
    async fn netbox_ipam_fhrp_groups_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/fhrp-groups/", p.id, "FHRP group")
    }

    #[tool(description = "List FHRP group assignments (filter: group_id, device_id).")]
    async fn netbox_ipam_fhrp_group_assignments_list(
        &self,
        Parameters(p): Parameters<ipam::FhrpGroupAssignmentsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(
            self,
            ipam::fhrp_group_assignments_list,
            p,
            "FHRP group assignments"
        )
    }
    #[tool(description = "Get an FHRP group assignment by its NetBox ID.")]
    async fn netbox_ipam_fhrp_group_assignments_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/ipam/fhrp-group-assignments/",
            p.id,
            "FHRP group assignment"
        )
    }

    #[tool(description = "List IP roles (filter: q, name, slug).")]
    async fn netbox_ipam_roles_list(
        &self,
        Parameters(p): Parameters<ipam::RolesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, ipam::roles_list, p, "IP roles")
    }
    #[tool(description = "Get an IP role by its NetBox ID.")]
    async fn netbox_ipam_roles_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/ipam/roles/", p.id, "IP role")
    }

    // ---- Circuits ----

    #[tool(
        description = "List circuits. Filters: provider, status, type, site, tenant, tag. Use fetch_all=true for all results."
    )]
    async fn netbox_circuits_circuits_list(
        &self,
        Parameters(p): Parameters<circuits::CircuitsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, circuits::circuits_list, p, "circuits")
    }
    #[tool(description = "Get a circuit by its NetBox ID.")]
    async fn netbox_circuits_circuits_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/circuits/circuits/", p.id, "circuit")
    }

    #[tool(description = "List circuit providers (filter: q, name).")]
    async fn netbox_circuits_providers_list(
        &self,
        Parameters(p): Parameters<circuits::ProvidersListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, circuits::providers_list, p, "providers")
    }
    #[tool(description = "Get a provider by its NetBox ID.")]
    async fn netbox_circuits_providers_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/circuits/providers/", p.id, "provider")
    }

    #[tool(description = "List circuit types (filter: q, name, slug).")]
    async fn netbox_circuits_circuit_types_list(
        &self,
        Parameters(p): Parameters<circuits::CircuitTypesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, circuits::circuit_types_list, p, "circuit types")
    }
    #[tool(description = "Get a circuit type by its NetBox ID.")]
    async fn netbox_circuits_circuit_types_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/circuits/circuit-types/", p.id, "circuit type")
    }

    #[tool(description = "List circuit terminations (filter: q, circuit, site).")]
    async fn netbox_circuits_circuit_terminations_list(
        &self,
        Parameters(p): Parameters<circuits::CircuitTerminationsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(
            self,
            circuits::circuit_terminations_list,
            p,
            "circuit terminations"
        )
    }
    #[tool(description = "Get a circuit termination by its NetBox ID.")]
    async fn netbox_circuits_circuit_terminations_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/circuits/circuit-terminations/",
            p.id,
            "circuit termination"
        )
    }

    #[tool(description = "List provider accounts (filter: q, provider).")]
    async fn netbox_circuits_provider_accounts_list(
        &self,
        Parameters(p): Parameters<circuits::ProviderAccountsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(
            self,
            circuits::provider_accounts_list,
            p,
            "provider accounts"
        )
    }
    #[tool(description = "Get a provider account by its NetBox ID.")]
    async fn netbox_circuits_provider_accounts_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/circuits/provider-accounts/",
            p.id,
            "provider account"
        )
    }

    #[tool(description = "List provider networks (filter: q, provider).")]
    async fn netbox_circuits_provider_networks_list(
        &self,
        Parameters(p): Parameters<circuits::ProviderNetworksListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(
            self,
            circuits::provider_networks_list,
            p,
            "provider networks"
        )
    }
    #[tool(description = "Get a provider network by its NetBox ID.")]
    async fn netbox_circuits_provider_networks_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/circuits/provider-networks/",
            p.id,
            "provider network"
        )
    }

    // ---- Tenancy ----

    #[tool(description = "List tenants (filter: q, name, group).")]
    async fn netbox_tenancy_tenants_list(
        &self,
        Parameters(p): Parameters<tenancy::TenantsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, tenancy::tenants_list, p, "tenants")
    }
    #[tool(description = "Get a tenant by its NetBox ID.")]
    async fn netbox_tenancy_tenants_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/tenancy/tenants/", p.id, "tenant")
    }

    #[tool(description = "List tenant groups (filter: q, name, parent).")]
    async fn netbox_tenancy_tenant_groups_list(
        &self,
        Parameters(p): Parameters<tenancy::TenantGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, tenancy::tenant_groups_list, p, "tenant groups")
    }
    #[tool(description = "Get a tenant group by its NetBox ID.")]
    async fn netbox_tenancy_tenant_groups_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/tenancy/tenant-groups/", p.id, "tenant group")
    }

    #[tool(description = "List contacts (filter: q, name, group).")]
    async fn netbox_tenancy_contacts_list(
        &self,
        Parameters(p): Parameters<tenancy::ContactsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, tenancy::contacts_list, p, "contacts")
    }
    #[tool(description = "Get a contact by its NetBox ID.")]
    async fn netbox_tenancy_contacts_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/tenancy/contacts/", p.id, "contact")
    }

    #[tool(description = "List contact groups (filter: q, name, parent).")]
    async fn netbox_tenancy_contact_groups_list(
        &self,
        Parameters(p): Parameters<tenancy::ContactGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, tenancy::contact_groups_list, p, "contact groups")
    }
    #[tool(description = "Get a contact group by its NetBox ID.")]
    async fn netbox_tenancy_contact_groups_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/tenancy/contact-groups/", p.id, "contact group")
    }

    #[tool(description = "List contact roles (filter: q, name, slug).")]
    async fn netbox_tenancy_contact_roles_list(
        &self,
        Parameters(p): Parameters<tenancy::ContactRolesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, tenancy::contact_roles_list, p, "contact roles")
    }
    #[tool(description = "Get a contact role by its NetBox ID.")]
    async fn netbox_tenancy_contact_roles_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/tenancy/contact-roles/", p.id, "contact role")
    }

    // ---- Virtualization ----

    #[tool(
        description = "List virtual machines. Filters: name, cluster, site, status, role, tenant, tag. Use fetch_all=true for all results."
    )]
    async fn netbox_virtualization_vms_list(
        &self,
        Parameters(p): Parameters<virtualization::VmsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, virtualization::vms_list, p, "VMs")
    }
    #[tool(description = "Get a virtual machine by its NetBox ID.")]
    async fn netbox_virtualization_vms_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/virtualization/virtual-machines/", p.id, "VM")
    }

    #[tool(description = "List clusters (filter: q, name, type, site).")]
    async fn netbox_virtualization_clusters_list(
        &self,
        Parameters(p): Parameters<virtualization::ClustersListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, virtualization::clusters_list, p, "clusters")
    }
    #[tool(description = "Get a cluster by its NetBox ID.")]
    async fn netbox_virtualization_clusters_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/virtualization/clusters/", p.id, "cluster")
    }

    #[tool(description = "List cluster groups (filter: q, name).")]
    async fn netbox_virtualization_cluster_groups_list(
        &self,
        Parameters(p): Parameters<virtualization::ClusterGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(
            self,
            virtualization::cluster_groups_list,
            p,
            "cluster groups"
        )
    }
    #[tool(description = "Get a cluster group by its NetBox ID.")]
    async fn netbox_virtualization_cluster_groups_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/virtualization/cluster-groups/",
            p.id,
            "cluster group"
        )
    }

    #[tool(description = "List cluster types (filter: q, name).")]
    async fn netbox_virtualization_cluster_types_list(
        &self,
        Parameters(p): Parameters<virtualization::ClusterTypesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, virtualization::cluster_types_list, p, "cluster types")
    }
    #[tool(description = "Get a cluster type by its NetBox ID.")]
    async fn netbox_virtualization_cluster_types_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/virtualization/cluster-types/",
            p.id,
            "cluster type"
        )
    }

    #[tool(
        description = "List VM interfaces. Use virtual_machine=<name> to filter by VM name directly. Also filters: name, enabled, mac_address, tag, fetch_all."
    )]
    async fn netbox_virtualization_interfaces_list(
        &self,
        Parameters(p): Parameters<virtualization::InterfacesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, virtualization::interfaces_list, p, "VM interfaces")
    }
    #[tool(description = "Get a VM interface by its NetBox ID.")]
    async fn netbox_virtualization_interfaces_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/virtualization/interfaces/",
            p.id,
            "VM interface"
        )
    }

    #[tool(description = "List virtual disks (filter: q, virtual machine, name).")]
    async fn netbox_virtualization_virtual_disks_list(
        &self,
        Parameters(p): Parameters<virtualization::VirtualDisksListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, virtualization::virtual_disks_list, p, "virtual disks")
    }
    #[tool(description = "Get a virtual disk by its NetBox ID.")]
    async fn netbox_virtualization_virtual_disks_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/virtualization/virtual-disks/",
            p.id,
            "virtual disk"
        )
    }

    // ---- Extras ----

    #[tool(description = "List tags (filter: q, name, slug).")]
    async fn netbox_extras_tags_list(
        &self,
        Parameters(p): Parameters<extras::TagsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, extras::tags_list, p, "tags")
    }
    #[tool(description = "Get a tag by its NetBox ID.")]
    async fn netbox_extras_tags_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/extras/tags/", p.id, "tag")
    }

    #[tool(description = "List config contexts (filter: q, name, is_active, site, role).")]
    async fn netbox_extras_config_contexts_list(
        &self,
        Parameters(p): Parameters<extras::ConfigContextsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, extras::config_contexts_list, p, "config contexts")
    }
    #[tool(description = "Get a config context by its NetBox ID.")]
    async fn netbox_extras_config_contexts_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/extras/config-contexts/", p.id, "config context")
    }

    #[tool(
        description = "List journal entries (filter: q, assigned_object_type, assigned_object_id, kind, created_by)."
    )]
    async fn netbox_extras_journal_entries_list(
        &self,
        Parameters(p): Parameters<extras::JournalEntriesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, extras::journal_entries_list, p, "journal entries")
    }
    #[tool(description = "Get a journal entry by its NetBox ID.")]
    async fn netbox_extras_journal_entries_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/extras/journal-entries/", p.id, "journal entry")
    }

    #[tool(description = "List custom fields (filter: q, name, type, object_type).")]
    async fn netbox_extras_custom_fields_list(
        &self,
        Parameters(p): Parameters<extras::CustomFieldsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, extras::custom_fields_list, p, "custom fields")
    }
    #[tool(description = "Get a custom field by its NetBox ID.")]
    async fn netbox_extras_custom_fields_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/extras/custom-fields/", p.id, "custom field")
    }

    #[tool(description = "List export templates (filter: q, name, object_type).")]
    async fn netbox_extras_export_templates_list(
        &self,
        Parameters(p): Parameters<extras::ExportTemplatesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, extras::export_templates_list, p, "export templates")
    }
    #[tool(description = "Get an export template by its NetBox ID.")]
    async fn netbox_extras_export_templates_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/extras/export-templates/",
            p.id,
            "export template"
        )
    }

    #[tool(description = "List webhooks (filter: q, name).")]
    async fn netbox_extras_webhooks_list(
        &self,
        Parameters(p): Parameters<extras::WebhooksListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, extras::webhooks_list, p, "webhooks")
    }
    #[tool(description = "Get a webhook by its NetBox ID.")]
    async fn netbox_extras_webhooks_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/extras/webhooks/", p.id, "webhook")
    }

    // ---- VPN ----

    #[tool(description = "List VPN tunnels (filter: q, status, group, tenant).")]
    async fn netbox_vpn_tunnels_list(
        &self,
        Parameters(p): Parameters<vpn::TunnelsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, vpn::tunnels_list, p, "VPN tunnels")
    }
    #[tool(description = "Get a VPN tunnel by its NetBox ID.")]
    async fn netbox_vpn_tunnels_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/vpn/tunnels/", p.id, "VPN tunnel")
    }

    #[tool(description = "List VPN tunnel groups (filter: q, name, slug).")]
    async fn netbox_vpn_tunnel_groups_list(
        &self,
        Parameters(p): Parameters<vpn::TunnelGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, vpn::tunnel_groups_list, p, "VPN tunnel groups")
    }
    #[tool(description = "Get a VPN tunnel group by its NetBox ID.")]
    async fn netbox_vpn_tunnel_groups_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/vpn/tunnel-groups/", p.id, "VPN tunnel group")
    }

    #[tool(description = "List L2VPNs (filter: q, type, tenant).")]
    async fn netbox_vpn_l2vpns_list(
        &self,
        Parameters(p): Parameters<vpn::L2vpnsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, vpn::l2vpns_list, p, "L2VPNs")
    }
    #[tool(description = "Get an L2VPN by its NetBox ID.")]
    async fn netbox_vpn_l2vpns_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/vpn/l2vpns/", p.id, "L2VPN")
    }

    #[tool(description = "List IKE policies (filter: q, name).")]
    async fn netbox_vpn_ike_policies_list(
        &self,
        Parameters(p): Parameters<vpn::IkePoliciesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, vpn::ike_policies_list, p, "IKE policies")
    }
    #[tool(description = "Get an IKE policy by its NetBox ID.")]
    async fn netbox_vpn_ike_policies_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/vpn/ike-policies/", p.id, "IKE policy")
    }

    #[tool(description = "List IPSec policies (filter: q, name).")]
    async fn netbox_vpn_ipsec_policies_list(
        &self,
        Parameters(p): Parameters<vpn::IpsecPoliciesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, vpn::ipsec_policies_list, p, "IPSec policies")
    }
    #[tool(description = "Get an IPSec policy by its NetBox ID.")]
    async fn netbox_vpn_ipsec_policies_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/vpn/ipsec-policies/", p.id, "IPSec policy")
    }

    #[tool(description = "List VPN tunnel terminations (filter: q, tunnel_id, role).")]
    async fn netbox_vpn_tunnel_terminations_list(
        &self,
        Parameters(p): Parameters<vpn::TunnelTerminationsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(
            self,
            vpn::tunnel_terminations_list,
            p,
            "VPN tunnel terminations"
        )
    }
    #[tool(description = "Get a VPN tunnel termination by its NetBox ID.")]
    async fn netbox_vpn_tunnel_terminations_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/vpn/tunnel-terminations/",
            p.id,
            "VPN tunnel termination"
        )
    }

    // ---- Wireless ----

    #[tool(description = "List wireless LANs (filter: q, ssid, group, status, tenant).")]
    async fn netbox_wireless_lans_list(
        &self,
        Parameters(p): Parameters<wireless::LansListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, wireless::lans_list, p, "wireless LANs")
    }
    #[tool(description = "Get a wireless LAN by its NetBox ID.")]
    async fn netbox_wireless_lans_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/wireless/wireless-lans/", p.id, "wireless LAN")
    }

    #[tool(description = "List wireless LAN groups (filter: q, name, parent).")]
    async fn netbox_wireless_lan_groups_list(
        &self,
        Parameters(p): Parameters<wireless::LanGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, wireless::lan_groups_list, p, "wireless LAN groups")
    }
    #[tool(description = "Get a wireless LAN group by its NetBox ID.")]
    async fn netbox_wireless_lan_groups_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(
            self,
            "/api/wireless/wireless-lan-groups/",
            p.id,
            "wireless LAN group"
        )
    }

    #[tool(description = "List wireless links (filter: q, status, tenant).")]
    async fn netbox_wireless_links_list(
        &self,
        Parameters(p): Parameters<wireless::LinksListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, wireless::links_list, p, "wireless links")
    }
    #[tool(description = "Get a wireless link by its NetBox ID.")]
    async fn netbox_wireless_links_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/wireless/wireless-links/", p.id, "wireless link")
    }

    // ---- Core ----

    #[tool(description = "List data sources (filter: q, name, status).")]
    async fn netbox_core_data_sources_list(
        &self,
        Parameters(p): Parameters<core::DataSourcesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, core::data_sources_list, p, "data sources")
    }
    #[tool(description = "Get a data source by its NetBox ID.")]
    async fn netbox_core_data_sources_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/core/data-sources/", p.id, "data source")
    }

    #[tool(description = "List background jobs (filter: q, status).")]
    async fn netbox_core_jobs_list(
        &self,
        Parameters(p): Parameters<core::JobsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, core::jobs_list, p, "jobs")
    }
    #[tool(description = "Get a background job by its NetBox ID.")]
    async fn netbox_core_jobs_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/core/jobs/", p.id, "job")
    }

    #[tool(description = "List object changes / audit log (filter: q, user).")]
    async fn netbox_core_object_changes_list(
        &self,
        Parameters(p): Parameters<core::ObjectChangesListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, core::object_changes_list, p, "object changes")
    }
    #[tool(description = "Get an object change record by its NetBox ID.")]
    async fn netbox_core_object_changes_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/core/object-changes/", p.id, "object change")
    }

    // ---- Users ----

    #[tool(description = "List users (filter: q, username, is_active).")]
    async fn netbox_users_users_list(
        &self,
        Parameters(p): Parameters<users::UsersListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, users::users_list, p, "users")
    }
    #[tool(description = "Get a user by their NetBox ID.")]
    async fn netbox_users_users_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/users/users/", p.id, "user")
    }

    #[tool(description = "List user groups (filter: q, name).")]
    async fn netbox_users_groups_list(
        &self,
        Parameters(p): Parameters<users::GroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, users::groups_list, p, "user groups")
    }
    #[tool(description = "Get a user group by its NetBox ID.")]
    async fn netbox_users_groups_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/users/groups/", p.id, "user group")
    }

    #[tool(description = "List API tokens (filter: q, user_id).")]
    async fn netbox_users_tokens_list(
        &self,
        Parameters(p): Parameters<users::TokensListParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_list!(self, users::tokens_list, p, "tokens")
    }
    #[tool(description = "Get an API token by its NetBox ID.")]
    async fn netbox_users_tokens_get(
        &self,
        Parameters(p): Parameters<GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        delegate_get!(self, "/api/users/tokens/", p.id, "token")
    }

    #[tool(
        description = "Look up a host by name across both physical devices and virtual machines. \
        Searches dcim/devices and virtualization/virtual-machines in parallel. \
        Accepts a hostname or FQDN — partial, case-insensitive match is used so 'web01' matches 'web01.example.com'. \
        Results are capped at 50 per resource type. \
        Returns { devices: [...], virtual_machines: [...], total_matches: N, has_more: bool }. \
        total_matches is the true NetBox count (may exceed returned results). \
        has_more: true means at least one result list was truncated — use the dedicated list tools with specific filters to retrieve all."
    )]
    async fn netbox_lookup_host(
        &self,
        Parameters(p): Parameters<LookupHostParams>,
    ) -> Result<CallToolResult, McpError> {
        let client = self.get_client()?;
        let params = vec![("name__ic", p.name), ("limit", DEFAULT_LIMIT.to_string())];
        let (devices_result, vms_result) = tokio::join!(
            client.list("/api/dcim/devices/", &params),
            client.list("/api/virtualization/virtual-machines/", &params),
        );
        let (devices, device_total) = match devices_result {
            Ok(v) => {
                let total = v["count"].as_u64().unwrap_or(0);
                let results = v["results"].as_array().cloned().unwrap_or_default();
                (results, total)
            }
            Err(e) => return tool_error(&format!("looking up devices: {e}")),
        };
        let (vms, vm_total) = match vms_result {
            Ok(v) => {
                let total = v["count"].as_u64().unwrap_or(0);
                let results = v["results"].as_array().cloned().unwrap_or_default();
                (results, total)
            }
            Err(e) => return tool_error(&format!("looking up virtual machines: {e}")),
        };
        let has_more = device_total > devices.len() as u64 || vm_total > vms.len() as u64;
        let combined = serde_json::json!({
            "devices": devices,
            "virtual_machines": vms,
            "total_matches": device_total + vm_total,
            "has_more": has_more,
        });
        json_result(combined)
    }
}

// --------------------------------------------------------------------------
// Prompts
// --------------------------------------------------------------------------

#[prompt_router]
impl NetboxMcpServer {
    #[prompt(
        name = "site-inventory",
        description = "Generate a structured inventory report for a NetBox site"
    )]
    async fn site_inventory(
        &self,
        Parameters(args): Parameters<SiteInventoryArgs>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<Vec<PromptMessage>, McpError> {
        Ok(vec![PromptMessage::new_text(
            PromptMessageRole::User,
            format!(
                "Using the netbox MCP tools, build a complete inventory for site: '{}'. \
                 Include: all devices (with roles and status), racks and their occupancy, \
                 IP prefixes assigned to the site, and active circuits. \
                 Present the results in a structured report.",
                args.site
            ),
        )])
    }

    #[prompt(
        name = "device-report",
        description = "Generate a detailed report for a specific NetBox device"
    )]
    async fn device_report(
        &self,
        Parameters(args): Parameters<DeviceReportArgs>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<Vec<PromptMessage>, McpError> {
        Ok(vec![PromptMessage::new_text(
            PromptMessageRole::User,
            format!(
                "Using the netbox MCP tools, generate a detailed report for device: '{}'. \
                 Include: device role, platform, site and rack, all interfaces and their IP addresses, \
                 connected cables, and any recent journal entries.",
                args.device
            ),
        )])
    }

    #[prompt(
        name = "prefix-utilization",
        description = "Analyze IP prefix utilization in NetBox"
    )]
    async fn prefix_utilization(
        &self,
        Parameters(args): Parameters<PrefixUtilizationArgs>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<Vec<PromptMessage>, McpError> {
        Ok(vec![PromptMessage::new_text(
            PromptMessageRole::User,
            format!(
                "Using the netbox MCP tools, analyze utilization for prefix: '{}'. \
                 List all child prefixes and IP addresses within the prefix, \
                 calculate utilization percentage, and identify any gaps or overlaps.",
                args.prefix
            ),
        )])
    }

    #[prompt(
        name = "tenant-summary",
        description = "Summarize all NetBox resources assigned to a tenant"
    )]
    async fn tenant_summary(
        &self,
        Parameters(args): Parameters<TenantSummaryArgs>,
        _ctx: RequestContext<RoleServer>,
    ) -> Result<Vec<PromptMessage>, McpError> {
        Ok(vec![PromptMessage::new_text(
            PromptMessageRole::User,
            format!(
                "Using the netbox MCP tools, provide a complete summary for tenant: '{}'. \
                 Include: all devices, virtual machines, IP addresses, prefixes, circuits, \
                 and VLANs assigned to this tenant. Summarize resource counts by category.",
                args.tenant
            ),
        )])
    }
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct SiteInventoryArgs {
    #[schemars(description = "Site name or slug to report on")]
    pub site: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct DeviceReportArgs {
    #[schemars(description = "Device name to report on")]
    pub device: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct PrefixUtilizationArgs {
    #[schemars(description = "IP prefix to analyze (e.g. 10.0.0.0/8)")]
    pub prefix: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct TenantSummaryArgs {
    #[schemars(description = "Tenant name or slug to summarize")]
    pub tenant: String,
}

// --------------------------------------------------------------------------
// ServerHandler
// --------------------------------------------------------------------------

#[tool_handler]
#[prompt_handler]
impl ServerHandler for NetboxMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_prompts()
                .enable_tools()
                .build(),
        )
        .with_server_info(Implementation::new("netbox-mcp", env!("CARGO_PKG_VERSION")))
    }

    async fn initialize(
        &self,
        _request: InitializeRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        // In HTTP mode, extract the bearer token from the request headers and
        // create the per-session NetboxClient. OnceLock::set is idempotent so
        // a re-initialize call is a silent no-op rather than a panic.
        if let Some(parts) = context.extensions.get::<axum::http::request::Parts>()
            && let Some(token) = parts
                .headers
                .get(axum::http::header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
        {
            let _ = self
                .client
                .set(NetboxClient::new(self.base_url.clone(), token));
        }
        Ok(self.get_info())
    }
}

// --------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // ------------------------------------------------------------------
    // clamp_limit
    // ------------------------------------------------------------------

    #[test]
    fn clamp_limit_none_uses_default() {
        assert_eq!(clamp_limit(None), DEFAULT_LIMIT);
    }

    #[test]
    fn clamp_limit_zero_uses_default() {
        assert_eq!(clamp_limit(Some(0)), DEFAULT_LIMIT);
    }

    #[test]
    fn clamp_limit_negative_uses_default() {
        assert_eq!(clamp_limit(Some(-1)), DEFAULT_LIMIT);
        assert_eq!(clamp_limit(Some(i32::MIN)), DEFAULT_LIMIT);
    }

    #[test]
    fn clamp_limit_above_max_is_clamped() {
        assert_eq!(clamp_limit(Some(MAX_LIMIT + 1)), MAX_LIMIT);
        assert_eq!(clamp_limit(Some(i32::MAX)), MAX_LIMIT);
    }

    #[test]
    fn clamp_limit_in_range_is_passed_through() {
        assert_eq!(clamp_limit(Some(1)), 1);
        assert_eq!(clamp_limit(Some(100)), 100);
        assert_eq!(clamp_limit(Some(MAX_LIMIT)), MAX_LIMIT);
    }

    // ------------------------------------------------------------------
    // slim_value
    // ------------------------------------------------------------------

    #[test]
    fn slim_value_passes_primitives_unchanged() {
        assert_eq!(slim_value(json!(42)), json!(42));
        assert_eq!(slim_value(json!("hello")), json!("hello"));
        assert_eq!(slim_value(json!(true)), json!(true));
        // Top-level null is preserved — slim only drops null *fields* of objects.
        assert_eq!(slim_value(json!(null)), json!(null));
    }

    #[test]
    fn slim_value_drops_null_fields_in_object() {
        let input = json!({"a": 1, "b": null, "c": "keep"});
        let expected = json!({"a": 1, "c": "keep"});
        assert_eq!(slim_value(input), expected);
    }

    #[test]
    fn slim_value_empty_object_survives() {
        assert_eq!(slim_value(json!({})), json!({}));
    }

    #[test]
    fn slim_value_recurses_into_nested_objects() {
        let input = json!({
            "a": {"x": null, "y": 2},
            "b": {"nested": {"deep": null, "kept": "yes"}},
        });
        let expected = json!({
            "a": {"y": 2},
            "b": {"nested": {"kept": "yes"}},
        });
        assert_eq!(slim_value(input), expected);
    }

    #[test]
    fn slim_value_cleans_objects_inside_arrays() {
        let input = json!([{"a": null, "b": 1}, {"c": 2}]);
        let expected = json!([{"b": 1}, {"c": 2}]);
        assert_eq!(slim_value(input), expected);
    }

    #[test]
    fn slim_value_preserves_null_elements_in_arrays() {
        // Arrays may legitimately carry null *elements* (positional). Only
        // null *fields* inside objects are pruned.
        let input = json!([1, null, 2]);
        assert_eq!(slim_value(input), json!([1, null, 2]));
    }

    #[test]
    fn slim_value_strips_deny_listed_keys() {
        let input = json!({
            "id": 1,
            "name": "vm-01",
            "config_context": {"env": "prod"},
            "local_context_data": {"env": "prod"},
            "primary_ip": {"id": 10, "address": "10.0.0.1/24"},
            "primary_ip4": {"id": 10, "address": "10.0.0.1/24"},
        });
        let expected = json!({
            "id": 1,
            "name": "vm-01",
            "config_context": {"env": "prod"},
            "primary_ip4": {"id": 10, "address": "10.0.0.1/24"},
        });
        assert_eq!(slim_value(input), expected);
    }

    #[test]
    fn slim_value_matches_real_netbox_response_shape() {
        // Sketch of a NetBox device payload: many nullable nested objects.
        let input = json!({
            "id": 1,
            "name": "rtr-01",
            "tenant": null,
            "site": {"id": 5, "name": "NYC", "tenant": null},
            "tags": [],
            "custom_fields": {"sla": null, "owner": "neteng"},
        });
        let expected = json!({
            "id": 1,
            "name": "rtr-01",
            "site": {"id": 5, "name": "NYC"},
            "tags": [],
            "custom_fields": {"owner": "neteng"},
        });
        assert_eq!(slim_value(input), expected);
    }

    // ------------------------------------------------------------------
    // QueryBuilder
    // ------------------------------------------------------------------

    #[test]
    fn querybuilder_new_is_empty() {
        let params = QueryBuilder::new().into_params();
        assert!(params.is_empty());
    }

    #[test]
    fn querybuilder_opt_some_appends() {
        let params = QueryBuilder::new()
            .opt("name", Some("rtr-01"))
            .into_params();
        assert_eq!(params, vec![("name", "rtr-01".to_string())]);
    }

    #[test]
    fn querybuilder_opt_none_skips() {
        let params = QueryBuilder::new().opt("name", None::<&str>).into_params();
        assert!(params.is_empty());
    }

    #[test]
    fn querybuilder_opt_uses_to_string() {
        let params = QueryBuilder::new()
            .opt("vid", Some(42i32))
            .opt("active", Some(true))
            .into_params();
        assert_eq!(
            params,
            vec![("vid", "42".to_string()), ("active", "true".to_string()),]
        );
    }

    #[test]
    fn querybuilder_many_none_skips() {
        let params = QueryBuilder::new().many("tag", None).into_params();
        assert!(params.is_empty());
    }

    #[test]
    fn querybuilder_many_empty_skips() {
        let params = QueryBuilder::new().many("tag", Some(vec![])).into_params();
        assert!(params.is_empty());
    }

    #[test]
    fn querybuilder_many_emits_one_pair_per_element() {
        let params = QueryBuilder::new()
            .many("tag", Some(vec!["edge".into(), "prod".into()]))
            .into_params();
        assert_eq!(
            params,
            vec![("tag", "edge".to_string()), ("tag", "prod".to_string()),]
        );
    }

    #[test]
    fn querybuilder_preserves_chain_order() {
        // NetBox tolerates any ordering, but tests pin the actual behavior so
        // downstream snapshot tests stay deterministic.
        let params = QueryBuilder::new()
            .opt("q", Some("foo"))
            .many("site", Some(vec!["nyc".into(), "lon".into()]))
            .opt("status", Some("active"))
            .into_params();
        assert_eq!(
            params,
            vec![
                ("q", "foo".to_string()),
                ("site", "nyc".to_string()),
                ("site", "lon".to_string()),
                ("status", "active".to_string()),
            ]
        );
    }

    // ------------------------------------------------------------------
    // finalize_params (the unit-testable half of paginate)
    // ------------------------------------------------------------------

    #[test]
    fn finalize_fetch_all_true_skips_limit_offset() {
        let params = finalize_params(vec![("q", "x".into())], Some(123), Some(45), Some(true));
        assert_eq!(params, vec![("q", "x".to_string())]);
    }

    #[test]
    fn finalize_fetch_all_false_appends_clamped_limit_and_offset() {
        let params = finalize_params(vec![], None, None, Some(false));
        assert_eq!(
            params,
            vec![
                ("limit", DEFAULT_LIMIT.to_string()),
                ("offset", "0".to_string()),
            ]
        );
    }

    #[test]
    fn finalize_fetch_all_unset_behaves_like_false() {
        let params = finalize_params(vec![], None, None, None);
        assert_eq!(
            params,
            vec![
                ("limit", DEFAULT_LIMIT.to_string()),
                ("offset", "0".to_string()),
            ]
        );
    }

    #[test]
    fn finalize_passes_through_explicit_limit_and_offset() {
        let params = finalize_params(vec![], Some(200), Some(10), Some(false));
        assert_eq!(
            params,
            vec![("limit", "200".to_string()), ("offset", "10".to_string()),]
        );
    }

    #[test]
    fn finalize_clamps_oversized_limit() {
        let params = finalize_params(vec![], Some(99_999), None, Some(false));
        assert_eq!(
            params,
            vec![
                ("limit", MAX_LIMIT.to_string()),
                ("offset", "0".to_string()),
            ]
        );
    }

    #[test]
    fn finalize_preserves_user_supplied_params_first() {
        // The user-supplied filters must appear *before* limit/offset, so a
        // future change to deduplicate by key (if NetBox ever cares) keeps
        // the user's pagination intent.
        let params = finalize_params(
            vec![("status", "active".into()), ("site", "nyc".into())],
            Some(25),
            Some(5),
            Some(false),
        );
        assert_eq!(
            params,
            vec![
                ("status", "active".to_string()),
                ("site", "nyc".to_string()),
                ("limit", "25".to_string()),
                ("offset", "5".to_string()),
            ]
        );
    }

    // ------------------------------------------------------------------
    // clean_page_response
    // ------------------------------------------------------------------

    #[test]
    fn clean_page_response_strips_next_and_previous() {
        let resp = json!({
            "count": 3,
            "next": "https://netbox.example.com/api/dcim/sites/?limit=2&offset=2",
            "previous": null,
            "results": [{"id": 1}, {"id": 2}],
        });
        let out = clean_page_response(resp, 0, 2);
        assert!(out.get("next").is_none());
        assert!(out.get("previous").is_none());
    }

    #[test]
    fn clean_page_response_has_more_true_when_results_remain() {
        let resp = json!({"count": 10, "next": "...", "previous": null, "results": []});
        let out = clean_page_response(resp, 0, 5);
        assert_eq!(out["has_more"], json!(true));
        assert_eq!(out["next_offset"], json!(5u64));
    }

    #[test]
    fn clean_page_response_has_more_false_on_last_page() {
        let resp = json!({"count": 10, "next": null, "previous": "...", "results": []});
        let out = clean_page_response(resp, 5, 5);
        assert_eq!(out["has_more"], json!(false));
        assert_eq!(out["next_offset"], json!(10u64));
    }

    #[test]
    fn clean_page_response_has_more_false_when_count_is_zero() {
        let resp = json!({"count": 0, "next": null, "previous": null, "results": []});
        let out = clean_page_response(resp, 0, 50);
        assert_eq!(out["has_more"], json!(false));
        assert_eq!(out["next_offset"], json!(0u64));
    }

    #[test]
    fn clean_page_response_has_more_false_when_offset_past_count() {
        // Defensive: if offset has somehow advanced beyond count, no more pages.
        let resp = json!({"count": 3, "next": null, "previous": "...", "results": []});
        let out = clean_page_response(resp, 10, 5);
        assert_eq!(out["has_more"], json!(false));
        assert_eq!(out["next_offset"], json!(3u64));
    }

    #[test]
    fn clean_page_response_preserves_count_and_results() {
        let resp = json!({
            "count": 2,
            "next": null,
            "previous": null,
            "results": [{"id": 1}, {"id": 2}],
        });
        let out = clean_page_response(resp, 0, 50);
        assert_eq!(out["count"], json!(2));
        assert_eq!(out["results"], json!([{"id": 1}, {"id": 2}]));
    }
}
