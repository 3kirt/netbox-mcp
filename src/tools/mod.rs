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
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::client::NetboxClient;

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
    let text = serde_json::to_string_pretty(&v)
        .map_err(|e| McpError::internal_error(format!("marshalling response: {e}"), None))?;
    Ok(CallToolResult::success(vec![Content::text(text)]))
}

pub fn tool_error(msg: &str) -> Result<CallToolResult, McpError> {
    Ok(CallToolResult::error(vec![Content::text(msg)]))
}

// --------------------------------------------------------------------------
// Server struct
// --------------------------------------------------------------------------

/// The MCP server — holds a NetBox client and the generated tool/prompt routers.
#[derive(Clone)]
pub struct NetboxMcpServer {
    /// Shared NetBox client. In HTTP mode this is populated in `initialize()`.
    pub client: Arc<RwLock<Option<NetboxClient>>>,
    pub base_url: String,
    // Routers are populated by the rmcp macros and read internally via generated code.
    #[allow(dead_code)]
    tool_router: ToolRouter<NetboxMcpServer>,
    #[allow(dead_code)]
    prompt_router: PromptRouter<NetboxMcpServer>,
}

impl NetboxMcpServer {
    /// Create a server with an already-known token (stdio mode).
    pub fn new_stdio(base_url: String, token: String) -> Self {
        let client = NetboxClient::new(base_url.clone(), token);
        Self {
            client: Arc::new(RwLock::new(Some(client))),
            base_url,
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }

    /// Create a server without a token (HTTP mode — token injected in `initialize()`).
    pub fn new_http(base_url: String) -> Self {
        Self {
            client: Arc::new(RwLock::new(None)),
            base_url,
            tool_router: Self::tool_router(),
            prompt_router: Self::prompt_router(),
        }
    }

    /// Convenience: get the client or return a tool error.
    async fn get_client(
        &self,
    ) -> Result<tokio::sync::RwLockReadGuard<'_, Option<NetboxClient>>, McpError> {
        let guard = self.client.read().await;
        if guard.is_none() {
            return Err(McpError::internal_error(
                "NetBox client not initialized",
                None,
            ));
        }
        Ok(guard)
    }
}

// --------------------------------------------------------------------------
// Tool shims — one per tool, delegating to domain functions
// --------------------------------------------------------------------------

#[tool_router]
impl NetboxMcpServer {
    // DCIM — devices
    #[tool(
        description = "List devices in NetBox, optionally filtered by site, role, status, or rack."
    )]
    async fn netbox_dcim_devices_list(
        &self,
        Parameters(p): Parameters<dcim::DevicesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::devices_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing devices: {e}")),
        }
    }
    #[tool(description = "Get a device by its NetBox ID.")]
    async fn netbox_dcim_devices_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/devices/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting device {}: {e}", p.id)),
        }
    }

    // DCIM — sites
    #[tool(description = "List sites in NetBox, optionally filtered by name, status, or region.")]
    async fn netbox_dcim_sites_list(
        &self,
        Parameters(p): Parameters<dcim::SitesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::sites_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing sites: {e}")),
        }
    }
    #[tool(description = "Get a site by its NetBox ID.")]
    async fn netbox_dcim_sites_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/sites/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting site {}: {e}", p.id)),
        }
    }

    // DCIM — racks
    #[tool(description = "List racks in NetBox, optionally filtered by site, location, or status.")]
    async fn netbox_dcim_racks_list(
        &self,
        Parameters(p): Parameters<dcim::RacksListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::racks_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing racks: {e}")),
        }
    }
    #[tool(description = "Get a rack by its NetBox ID.")]
    async fn netbox_dcim_racks_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/racks/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting rack {}: {e}", p.id)),
        }
    }

    // DCIM — interfaces
    #[tool(description = "List device interfaces, optionally filtered by device, name, or type.")]
    async fn netbox_dcim_interfaces_list(
        &self,
        Parameters(p): Parameters<dcim::InterfacesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::interfaces_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing interfaces: {e}")),
        }
    }
    #[tool(description = "Get a device interface by its NetBox ID.")]
    async fn netbox_dcim_interfaces_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/interfaces/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting interface {}: {e}", p.id)),
        }
    }

    // DCIM — cables
    #[tool(description = "List cables in NetBox, optionally filtered by site or status.")]
    async fn netbox_dcim_cables_list(
        &self,
        Parameters(p): Parameters<dcim::CablesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::cables_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing cables: {e}")),
        }
    }
    #[tool(description = "Get a cable by its NetBox ID.")]
    async fn netbox_dcim_cables_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/cables/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting cable {}: {e}", p.id)),
        }
    }

    // DCIM — regions
    #[tool(description = "List regions in NetBox, optionally filtered by name, slug, or parent.")]
    async fn netbox_dcim_regions_list(
        &self,
        Parameters(p): Parameters<dcim::RegionsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::regions_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing regions: {e}")),
        }
    }
    #[tool(description = "Get a region by its NetBox ID.")]
    async fn netbox_dcim_regions_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/regions/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting region {}: {e}", p.id)),
        }
    }

    // DCIM — locations
    #[tool(
        description = "List locations in NetBox, optionally filtered by site, parent, status, or tenant."
    )]
    async fn netbox_dcim_locations_list(
        &self,
        Parameters(p): Parameters<dcim::LocationsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::locations_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing locations: {e}")),
        }
    }
    #[tool(description = "Get a location by its NetBox ID.")]
    async fn netbox_dcim_locations_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/locations/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting location {}: {e}", p.id)),
        }
    }

    // DCIM — manufacturers
    #[tool(description = "List manufacturers in NetBox, optionally filtered by name or slug.")]
    async fn netbox_dcim_manufacturers_list(
        &self,
        Parameters(p): Parameters<dcim::ManufacturersListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::manufacturers_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing manufacturers: {e}")),
        }
    }
    #[tool(description = "Get a manufacturer by its NetBox ID.")]
    async fn netbox_dcim_manufacturers_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/manufacturers/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting manufacturer {}: {e}", p.id)),
        }
    }

    // DCIM — device types
    #[tool(
        description = "List device types in NetBox, optionally filtered by manufacturer or model."
    )]
    async fn netbox_dcim_device_types_list(
        &self,
        Parameters(p): Parameters<dcim::DeviceTypesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::device_types_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing device types: {e}")),
        }
    }
    #[tool(description = "Get a device type by its NetBox ID.")]
    async fn netbox_dcim_device_types_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/device-types/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting device type {}: {e}", p.id)),
        }
    }

    // DCIM — device roles
    #[tool(
        description = "List device roles in NetBox, optionally filtered by name, slug, or VM eligibility."
    )]
    async fn netbox_dcim_device_roles_list(
        &self,
        Parameters(p): Parameters<dcim::DeviceRolesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::device_roles_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing device roles: {e}")),
        }
    }
    #[tool(description = "Get a device role by its NetBox ID.")]
    async fn netbox_dcim_device_roles_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/device-roles/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting device role {}: {e}", p.id)),
        }
    }

    // DCIM — platforms
    #[tool(description = "List platforms in NetBox, optionally filtered by name or manufacturer.")]
    async fn netbox_dcim_platforms_list(
        &self,
        Parameters(p): Parameters<dcim::PlatformsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::platforms_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing platforms: {e}")),
        }
    }
    #[tool(description = "Get a platform by its NetBox ID.")]
    async fn netbox_dcim_platforms_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/platforms/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting platform {}: {e}", p.id)),
        }
    }

    // DCIM — power panels
    #[tool(description = "List power panels in NetBox, optionally filtered by site.")]
    async fn netbox_dcim_power_panels_list(
        &self,
        Parameters(p): Parameters<dcim::PowerPanelsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::power_panels_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing power panels: {e}")),
        }
    }
    #[tool(description = "Get a power panel by its NetBox ID.")]
    async fn netbox_dcim_power_panels_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/power-panels/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting power panel {}: {e}", p.id)),
        }
    }

    // DCIM — power feeds
    #[tool(
        description = "List power feeds in NetBox, optionally filtered by site, status, or type."
    )]
    async fn netbox_dcim_power_feeds_list(
        &self,
        Parameters(p): Parameters<dcim::PowerFeedsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::power_feeds_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing power feeds: {e}")),
        }
    }
    #[tool(description = "Get a power feed by its NetBox ID.")]
    async fn netbox_dcim_power_feeds_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/power-feeds/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting power feed {}: {e}", p.id)),
        }
    }

    // DCIM — virtual chassis
    #[tool(description = "List virtual chassis in NetBox, optionally filtered by site or tenant.")]
    async fn netbox_dcim_virtual_chassis_list(
        &self,
        Parameters(p): Parameters<dcim::VirtualChassisListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::virtual_chassis_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing virtual chassis: {e}")),
        }
    }
    #[tool(description = "Get a virtual chassis by its NetBox ID.")]
    async fn netbox_dcim_virtual_chassis_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/virtual-chassis/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting virtual chassis {}: {e}", p.id)),
        }
    }

    // DCIM — inventory items
    #[tool(
        description = "List inventory items in NetBox, optionally filtered by device, manufacturer, or discovery status."
    )]
    async fn netbox_dcim_inventory_items_list(
        &self,
        Parameters(p): Parameters<dcim::InventoryItemsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::inventory_items_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing inventory items: {e}")),
        }
    }
    #[tool(description = "Get an inventory item by its NetBox ID.")]
    async fn netbox_dcim_inventory_items_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/inventory-items/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting inventory item {}: {e}", p.id)),
        }
    }

    // DCIM — cable terminations
    #[tool(description = "List cable terminations in NetBox, optionally filtered by cable ID.")]
    async fn netbox_dcim_cable_terminations_list(
        &self,
        Parameters(p): Parameters<dcim::CableTerminationsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::cable_terminations_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing cable terminations: {e}")),
        }
    }
    #[tool(description = "Get a cable termination by its NetBox ID.")]
    async fn netbox_dcim_cable_terminations_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/cable-terminations/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting cable termination {}: {e}", p.id)),
        }
    }

    // DCIM — console ports
    #[tool(
        description = "List console ports in NetBox, optionally filtered by name, device, or site."
    )]
    async fn netbox_dcim_console_ports_list(
        &self,
        Parameters(p): Parameters<dcim::ConsolePortsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::console_ports_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing console ports: {e}")),
        }
    }
    #[tool(description = "Get a console port by its NetBox ID.")]
    async fn netbox_dcim_console_ports_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/console-ports/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting console port {}: {e}", p.id)),
        }
    }

    // DCIM — console server ports
    #[tool(
        description = "List console server ports in NetBox, optionally filtered by name, device, or site."
    )]
    async fn netbox_dcim_console_server_ports_list(
        &self,
        Parameters(p): Parameters<dcim::ConsoleServerPortsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::console_server_ports_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing console server ports: {e}")),
        }
    }
    #[tool(description = "Get a console server port by its NetBox ID.")]
    async fn netbox_dcim_console_server_ports_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/console-server-ports/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting console server port {}: {e}", p.id)),
        }
    }

    // DCIM — device bays
    #[tool(
        description = "List device bays in NetBox, optionally filtered by name, device, or site."
    )]
    async fn netbox_dcim_device_bays_list(
        &self,
        Parameters(p): Parameters<dcim::DeviceBaysListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::device_bays_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing device bays: {e}")),
        }
    }
    #[tool(description = "Get a device bay by its NetBox ID.")]
    async fn netbox_dcim_device_bays_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/device-bays/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting device bay {}: {e}", p.id)),
        }
    }

    // DCIM — front ports
    #[tool(description = "List front ports in NetBox, optionally filtered by name.")]
    async fn netbox_dcim_front_ports_list(
        &self,
        Parameters(p): Parameters<dcim::FrontPortsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::front_ports_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing front ports: {e}")),
        }
    }
    #[tool(description = "Get a front port by its NetBox ID.")]
    async fn netbox_dcim_front_ports_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/front-ports/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting front port {}: {e}", p.id)),
        }
    }

    // DCIM — MAC addresses
    #[tool(description = "List MAC addresses in NetBox, optionally filtered by device ID.")]
    async fn netbox_dcim_mac_addresses_list(
        &self,
        Parameters(p): Parameters<dcim::MacAddressesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::mac_addresses_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing MAC addresses: {e}")),
        }
    }
    #[tool(description = "Get a MAC address by its NetBox ID.")]
    async fn netbox_dcim_mac_addresses_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/mac-addresses/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting MAC address {}: {e}", p.id)),
        }
    }

    // DCIM — modules
    #[tool(description = "List modules in NetBox, optionally filtered by device, site, or status.")]
    async fn netbox_dcim_modules_list(
        &self,
        Parameters(p): Parameters<dcim::ModulesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::modules_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing modules: {e}")),
        }
    }
    #[tool(description = "Get a module by its NetBox ID.")]
    async fn netbox_dcim_modules_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/modules/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting module {}: {e}", p.id)),
        }
    }

    // DCIM — module bays
    #[tool(description = "List module bays in NetBox, optionally filtered by device ID.")]
    async fn netbox_dcim_module_bays_list(
        &self,
        Parameters(p): Parameters<dcim::ModuleBaysListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::module_bays_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing module bays: {e}")),
        }
    }
    #[tool(description = "Get a module bay by its NetBox ID.")]
    async fn netbox_dcim_module_bays_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/module-bays/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting module bay {}: {e}", p.id)),
        }
    }

    // DCIM — module types
    #[tool(description = "List module types in NetBox, optionally filtered by manufacturer.")]
    async fn netbox_dcim_module_types_list(
        &self,
        Parameters(p): Parameters<dcim::ModuleTypesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::module_types_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing module types: {e}")),
        }
    }
    #[tool(description = "Get a module type by its NetBox ID.")]
    async fn netbox_dcim_module_types_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/module-types/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting module type {}: {e}", p.id)),
        }
    }

    // DCIM — power outlets
    #[tool(
        description = "List power outlets in NetBox, optionally filtered by name, device, or site."
    )]
    async fn netbox_dcim_power_outlets_list(
        &self,
        Parameters(p): Parameters<dcim::PowerOutletsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::power_outlets_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing power outlets: {e}")),
        }
    }
    #[tool(description = "Get a power outlet by its NetBox ID.")]
    async fn netbox_dcim_power_outlets_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/power-outlets/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting power outlet {}: {e}", p.id)),
        }
    }

    // DCIM — power ports
    #[tool(
        description = "List power ports in NetBox, optionally filtered by name, device, or site."
    )]
    async fn netbox_dcim_power_ports_list(
        &self,
        Parameters(p): Parameters<dcim::PowerPortsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::power_ports_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing power ports: {e}")),
        }
    }
    #[tool(description = "Get a power port by its NetBox ID.")]
    async fn netbox_dcim_power_ports_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/power-ports/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting power port {}: {e}", p.id)),
        }
    }

    // DCIM — rack reservations
    #[tool(
        description = "List rack reservations in NetBox, optionally filtered by rack, site, or tenant."
    )]
    async fn netbox_dcim_rack_reservations_list(
        &self,
        Parameters(p): Parameters<dcim::RackReservationsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::rack_reservations_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing rack reservations: {e}")),
        }
    }
    #[tool(description = "Get a rack reservation by its NetBox ID.")]
    async fn netbox_dcim_rack_reservations_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/rack-reservations/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting rack reservation {}: {e}", p.id)),
        }
    }

    // DCIM — rack roles
    #[tool(description = "List rack roles in NetBox, optionally filtered by name or slug.")]
    async fn netbox_dcim_rack_roles_list(
        &self,
        Parameters(p): Parameters<dcim::RackRolesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::rack_roles_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing rack roles: {e}")),
        }
    }
    #[tool(description = "Get a rack role by its NetBox ID.")]
    async fn netbox_dcim_rack_roles_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/rack-roles/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting rack role {}: {e}", p.id)),
        }
    }

    // DCIM — rack types
    #[tool(description = "List rack types in NetBox, optionally filtered by slug or manufacturer.")]
    async fn netbox_dcim_rack_types_list(
        &self,
        Parameters(p): Parameters<dcim::RackTypesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::rack_types_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing rack types: {e}")),
        }
    }
    #[tool(description = "Get a rack type by its NetBox ID.")]
    async fn netbox_dcim_rack_types_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/rack-types/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting rack type {}: {e}", p.id)),
        }
    }

    // DCIM — rear ports
    #[tool(description = "List rear ports in NetBox, optionally filtered by name.")]
    async fn netbox_dcim_rear_ports_list(
        &self,
        Parameters(p): Parameters<dcim::RearPortsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::rear_ports_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing rear ports: {e}")),
        }
    }
    #[tool(description = "Get a rear port by its NetBox ID.")]
    async fn netbox_dcim_rear_ports_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/dcim/rear-ports/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting rear port {}: {e}", p.id)),
        }
    }

    // DCIM — site groups
    #[tool(description = "List site groups in NetBox, optionally filtered by name or slug.")]
    async fn netbox_dcim_site_groups_list(
        &self,
        Parameters(p): Parameters<dcim::SiteGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::site_groups_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing site groups: {e}")),
        }
    }
    #[tool(description = "Get a site group by its NetBox ID.")]
    async fn netbox_dcim_site_groups_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/site-groups/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting site group {}: {e}", p.id)),
        }
    }

    // DCIM — virtual device contexts
    #[tool(
        description = "List virtual device contexts in NetBox, optionally filtered by device or tenant."
    )]
    async fn netbox_dcim_virtual_device_contexts_list(
        &self,
        Parameters(p): Parameters<dcim::VirtualDeviceContextsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match dcim::virtual_device_contexts_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing virtual device contexts: {e}")),
        }
    }
    #[tool(description = "Get a virtual device context by its NetBox ID.")]
    async fn netbox_dcim_virtual_device_contexts_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/dcim/virtual-device-contexts/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting virtual device context {}: {e}", p.id)),
        }
    }

    // ---- IPAM ----

    #[tool(description = "List IP addresses (filter: q, address, vrf, status, tenant, device).")]
    async fn netbox_ipam_ip_addresses_list(
        &self,
        Parameters(p): Parameters<ipam::IpAddressesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::ip_addresses_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing IP addresses: {e}")),
        }
    }
    #[tool(description = "Get an IP address by its NetBox ID.")]
    async fn netbox_ipam_ip_addresses_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/ipam/ip-addresses/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting IP address {}: {e}", p.id)),
        }
    }

    #[tool(description = "List prefixes (filter: q, prefix, vrf, status, site, tenant).")]
    async fn netbox_ipam_prefixes_list(
        &self,
        Parameters(p): Parameters<ipam::PrefixesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::prefixes_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing prefixes: {e}")),
        }
    }
    #[tool(description = "Get a prefix by its NetBox ID.")]
    async fn netbox_ipam_prefixes_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/ipam/prefixes/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting prefix {}: {e}", p.id)),
        }
    }

    #[tool(description = "List VRFs (filter: q, name, rd, tenant).")]
    async fn netbox_ipam_vrfs_list(
        &self,
        Parameters(p): Parameters<ipam::VrfsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::vrfs_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing VRFs: {e}")),
        }
    }
    #[tool(description = "Get a VRF by its NetBox ID.")]
    async fn netbox_ipam_vrfs_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/ipam/vrfs/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting VRF {}: {e}", p.id)),
        }
    }

    #[tool(description = "List VLANs (filter: q, vid, name, site, group, status).")]
    async fn netbox_ipam_vlans_list(
        &self,
        Parameters(p): Parameters<ipam::VlansListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::vlans_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing VLANs: {e}")),
        }
    }
    #[tool(description = "Get a VLAN by its NetBox ID.")]
    async fn netbox_ipam_vlans_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/ipam/vlans/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting VLAN {}: {e}", p.id)),
        }
    }

    #[tool(description = "List aggregates (filter: q, family, rir, tenant).")]
    async fn netbox_ipam_aggregates_list(
        &self,
        Parameters(p): Parameters<ipam::AggregatesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::aggregates_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing aggregates: {e}")),
        }
    }
    #[tool(description = "Get an aggregate by its NetBox ID.")]
    async fn netbox_ipam_aggregates_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/ipam/aggregates/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting aggregate {}: {e}", p.id)),
        }
    }

    #[tool(description = "List IP ranges (filter: q, vrf, status, tenant).")]
    async fn netbox_ipam_ip_ranges_list(
        &self,
        Parameters(p): Parameters<ipam::IpRangesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::ip_ranges_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing IP ranges: {e}")),
        }
    }
    #[tool(description = "Get an IP range by its NetBox ID.")]
    async fn netbox_ipam_ip_ranges_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/ipam/ip-ranges/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting IP range {}: {e}", p.id)),
        }
    }

    #[tool(description = "List route targets (filter: q, name, tenant).")]
    async fn netbox_ipam_route_targets_list(
        &self,
        Parameters(p): Parameters<ipam::RouteTargetsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::route_targets_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing route targets: {e}")),
        }
    }
    #[tool(description = "Get a route target by its NetBox ID.")]
    async fn netbox_ipam_route_targets_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/ipam/route-targets/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting route target {}: {e}", p.id)),
        }
    }

    #[tool(description = "List RIRs (filter: q, name, slug).")]
    async fn netbox_ipam_rirs_list(
        &self,
        Parameters(p): Parameters<ipam::RirsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::rirs_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing RIRs: {e}")),
        }
    }
    #[tool(description = "Get a RIR by its NetBox ID.")]
    async fn netbox_ipam_rirs_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/ipam/rirs/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting RIR {}: {e}", p.id)),
        }
    }

    #[tool(description = "List VLAN groups (filter: q, name).")]
    async fn netbox_ipam_vlan_groups_list(
        &self,
        Parameters(p): Parameters<ipam::VlanGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::vlan_groups_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing VLAN groups: {e}")),
        }
    }
    #[tool(description = "Get a VLAN group by its NetBox ID.")]
    async fn netbox_ipam_vlan_groups_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/ipam/vlan-groups/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting VLAN group {}: {e}", p.id)),
        }
    }

    #[tool(description = "List services (filter: q, device, virtual machine, protocol).")]
    async fn netbox_ipam_services_list(
        &self,
        Parameters(p): Parameters<ipam::ServicesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::services_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing services: {e}")),
        }
    }
    #[tool(description = "Get a service by its NetBox ID.")]
    async fn netbox_ipam_services_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/ipam/services/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting service {}: {e}", p.id)),
        }
    }

    #[tool(description = "List ASNs (filter: q, site, tenant).")]
    async fn netbox_ipam_asns_list(
        &self,
        Parameters(p): Parameters<ipam::AsnsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::asns_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing ASNs: {e}")),
        }
    }
    #[tool(description = "Get an ASN by its NetBox ID.")]
    async fn netbox_ipam_asns_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/ipam/asns/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting ASN {}: {e}", p.id)),
        }
    }

    #[tool(description = "List FHRP groups (filter: q, name, protocol).")]
    async fn netbox_ipam_fhrp_groups_list(
        &self,
        Parameters(p): Parameters<ipam::FhrpGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::fhrp_groups_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing FHRP groups: {e}")),
        }
    }
    #[tool(description = "Get an FHRP group by its NetBox ID.")]
    async fn netbox_ipam_fhrp_groups_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/ipam/fhrp-groups/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting FHRP group {}: {e}", p.id)),
        }
    }

    #[tool(description = "List FHRP group assignments (filter: group_id, device_id).")]
    async fn netbox_ipam_fhrp_group_assignments_list(
        &self,
        Parameters(p): Parameters<ipam::FhrpGroupAssignmentsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::fhrp_group_assignments_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing FHRP group assignments: {e}")),
        }
    }
    #[tool(description = "Get an FHRP group assignment by its NetBox ID.")]
    async fn netbox_ipam_fhrp_group_assignments_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/ipam/fhrp-group-assignments/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting FHRP group assignment {}: {e}", p.id)),
        }
    }

    #[tool(description = "List IP roles (filter: q, name, slug).")]
    async fn netbox_ipam_roles_list(
        &self,
        Parameters(p): Parameters<ipam::RolesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match ipam::roles_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing IP roles: {e}")),
        }
    }
    #[tool(description = "Get an IP role by its NetBox ID.")]
    async fn netbox_ipam_roles_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/ipam/roles/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting IP role {}: {e}", p.id)),
        }
    }

    // ---- Circuits ----

    #[tool(description = "List circuits (filter: q, provider, status, type, site, tenant).")]
    async fn netbox_circuits_circuits_list(
        &self,
        Parameters(p): Parameters<circuits::CircuitsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match circuits::circuits_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing circuits: {e}")),
        }
    }
    #[tool(description = "Get a circuit by its NetBox ID.")]
    async fn netbox_circuits_circuits_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/circuits/circuits/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting circuit {}: {e}", p.id)),
        }
    }

    #[tool(description = "List circuit providers (filter: q, name).")]
    async fn netbox_circuits_providers_list(
        &self,
        Parameters(p): Parameters<circuits::ProvidersListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match circuits::providers_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing providers: {e}")),
        }
    }
    #[tool(description = "Get a provider by its NetBox ID.")]
    async fn netbox_circuits_providers_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/circuits/providers/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting provider {}: {e}", p.id)),
        }
    }

    #[tool(description = "List circuit types (filter: q, name, slug).")]
    async fn netbox_circuits_circuit_types_list(
        &self,
        Parameters(p): Parameters<circuits::CircuitTypesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match circuits::circuit_types_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing circuit types: {e}")),
        }
    }
    #[tool(description = "Get a circuit type by its NetBox ID.")]
    async fn netbox_circuits_circuit_types_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/circuits/circuit-types/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting circuit type {}: {e}", p.id)),
        }
    }

    #[tool(description = "List circuit terminations (filter: q, circuit, site).")]
    async fn netbox_circuits_circuit_terminations_list(
        &self,
        Parameters(p): Parameters<circuits::CircuitTerminationsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match circuits::circuit_terminations_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing circuit terminations: {e}")),
        }
    }
    #[tool(description = "Get a circuit termination by its NetBox ID.")]
    async fn netbox_circuits_circuit_terminations_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/circuits/circuit-terminations/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting circuit termination {}: {e}", p.id)),
        }
    }

    #[tool(description = "List provider accounts (filter: q, provider).")]
    async fn netbox_circuits_provider_accounts_list(
        &self,
        Parameters(p): Parameters<circuits::ProviderAccountsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match circuits::provider_accounts_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing provider accounts: {e}")),
        }
    }
    #[tool(description = "Get a provider account by its NetBox ID.")]
    async fn netbox_circuits_provider_accounts_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/circuits/provider-accounts/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting provider account {}: {e}", p.id)),
        }
    }

    #[tool(description = "List provider networks (filter: q, provider).")]
    async fn netbox_circuits_provider_networks_list(
        &self,
        Parameters(p): Parameters<circuits::ProviderNetworksListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match circuits::provider_networks_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing provider networks: {e}")),
        }
    }
    #[tool(description = "Get a provider network by its NetBox ID.")]
    async fn netbox_circuits_provider_networks_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/circuits/provider-networks/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting provider network {}: {e}", p.id)),
        }
    }

    // ---- Tenancy ----

    #[tool(description = "List tenants (filter: q, name, group).")]
    async fn netbox_tenancy_tenants_list(
        &self,
        Parameters(p): Parameters<tenancy::TenantsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match tenancy::tenants_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing tenants: {e}")),
        }
    }
    #[tool(description = "Get a tenant by its NetBox ID.")]
    async fn netbox_tenancy_tenants_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/tenancy/tenants/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting tenant {}: {e}", p.id)),
        }
    }

    #[tool(description = "List tenant groups (filter: q, name, parent).")]
    async fn netbox_tenancy_tenant_groups_list(
        &self,
        Parameters(p): Parameters<tenancy::TenantGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match tenancy::tenant_groups_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing tenant groups: {e}")),
        }
    }
    #[tool(description = "Get a tenant group by its NetBox ID.")]
    async fn netbox_tenancy_tenant_groups_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/tenancy/tenant-groups/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting tenant group {}: {e}", p.id)),
        }
    }

    #[tool(description = "List contacts (filter: q, name, group).")]
    async fn netbox_tenancy_contacts_list(
        &self,
        Parameters(p): Parameters<tenancy::ContactsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match tenancy::contacts_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing contacts: {e}")),
        }
    }
    #[tool(description = "Get a contact by its NetBox ID.")]
    async fn netbox_tenancy_contacts_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/tenancy/contacts/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting contact {}: {e}", p.id)),
        }
    }

    #[tool(description = "List contact groups (filter: q, name, parent).")]
    async fn netbox_tenancy_contact_groups_list(
        &self,
        Parameters(p): Parameters<tenancy::ContactGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match tenancy::contact_groups_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing contact groups: {e}")),
        }
    }
    #[tool(description = "Get a contact group by its NetBox ID.")]
    async fn netbox_tenancy_contact_groups_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/tenancy/contact-groups/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting contact group {}: {e}", p.id)),
        }
    }

    #[tool(description = "List contact roles (filter: q, name, slug).")]
    async fn netbox_tenancy_contact_roles_list(
        &self,
        Parameters(p): Parameters<tenancy::ContactRolesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match tenancy::contact_roles_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing contact roles: {e}")),
        }
    }
    #[tool(description = "Get a contact role by its NetBox ID.")]
    async fn netbox_tenancy_contact_roles_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/tenancy/contact-roles/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting contact role {}: {e}", p.id)),
        }
    }

    // ---- Virtualization ----

    #[tool(description = "List virtual machines (filter: q, cluster, site, status, role, tenant).")]
    async fn netbox_virtualization_vms_list(
        &self,
        Parameters(p): Parameters<virtualization::VmsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match virtualization::vms_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing VMs: {e}")),
        }
    }
    #[tool(description = "Get a virtual machine by its NetBox ID.")]
    async fn netbox_virtualization_vms_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/virtualization/virtual-machines/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting VM {}: {e}", p.id)),
        }
    }

    #[tool(description = "List clusters (filter: q, name, type, site).")]
    async fn netbox_virtualization_clusters_list(
        &self,
        Parameters(p): Parameters<virtualization::ClustersListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match virtualization::clusters_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing clusters: {e}")),
        }
    }
    #[tool(description = "Get a cluster by its NetBox ID.")]
    async fn netbox_virtualization_clusters_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/virtualization/clusters/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting cluster {}: {e}", p.id)),
        }
    }

    #[tool(description = "List cluster groups (filter: q, name).")]
    async fn netbox_virtualization_cluster_groups_list(
        &self,
        Parameters(p): Parameters<virtualization::ClusterGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match virtualization::cluster_groups_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing cluster groups: {e}")),
        }
    }
    #[tool(description = "Get a cluster group by its NetBox ID.")]
    async fn netbox_virtualization_cluster_groups_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/virtualization/cluster-groups/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting cluster group {}: {e}", p.id)),
        }
    }

    #[tool(description = "List cluster types (filter: q, name).")]
    async fn netbox_virtualization_cluster_types_list(
        &self,
        Parameters(p): Parameters<virtualization::ClusterTypesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match virtualization::cluster_types_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing cluster types: {e}")),
        }
    }
    #[tool(description = "Get a cluster type by its NetBox ID.")]
    async fn netbox_virtualization_cluster_types_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/virtualization/cluster-types/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting cluster type {}: {e}", p.id)),
        }
    }

    #[tool(description = "List VM interfaces (filter: q, virtual machine, name, enabled).")]
    async fn netbox_virtualization_interfaces_list(
        &self,
        Parameters(p): Parameters<virtualization::InterfacesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match virtualization::interfaces_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing VM interfaces: {e}")),
        }
    }
    #[tool(description = "Get a VM interface by its NetBox ID.")]
    async fn netbox_virtualization_interfaces_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/virtualization/interfaces/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting VM interface {}: {e}", p.id)),
        }
    }

    #[tool(description = "List virtual disks (filter: q, virtual machine, name).")]
    async fn netbox_virtualization_virtual_disks_list(
        &self,
        Parameters(p): Parameters<virtualization::VirtualDisksListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match virtualization::virtual_disks_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing virtual disks: {e}")),
        }
    }
    #[tool(description = "Get a virtual disk by its NetBox ID.")]
    async fn netbox_virtualization_virtual_disks_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/virtualization/virtual-disks/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting virtual disk {}: {e}", p.id)),
        }
    }

    // ---- Extras ----

    #[tool(description = "List tags (filter: q, name, slug).")]
    async fn netbox_extras_tags_list(
        &self,
        Parameters(p): Parameters<extras::TagsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match extras::tags_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing tags: {e}")),
        }
    }
    #[tool(description = "Get a tag by its NetBox ID.")]
    async fn netbox_extras_tags_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/extras/tags/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting tag {}: {e}", p.id)),
        }
    }

    #[tool(description = "List config contexts (filter: q, name, is_active, site, role).")]
    async fn netbox_extras_config_contexts_list(
        &self,
        Parameters(p): Parameters<extras::ConfigContextsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match extras::config_contexts_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing config contexts: {e}")),
        }
    }
    #[tool(description = "Get a config context by its NetBox ID.")]
    async fn netbox_extras_config_contexts_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/extras/config-contexts/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting config context {}: {e}", p.id)),
        }
    }

    #[tool(
        description = "List journal entries (filter: q, assigned_object_type, assigned_object_id, kind, created_by)."
    )]
    async fn netbox_extras_journal_entries_list(
        &self,
        Parameters(p): Parameters<extras::JournalEntriesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match extras::journal_entries_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing journal entries: {e}")),
        }
    }
    #[tool(description = "Get a journal entry by its NetBox ID.")]
    async fn netbox_extras_journal_entries_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/extras/journal-entries/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting journal entry {}: {e}", p.id)),
        }
    }

    #[tool(description = "List custom fields (filter: q, name, type, object_type).")]
    async fn netbox_extras_custom_fields_list(
        &self,
        Parameters(p): Parameters<extras::CustomFieldsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match extras::custom_fields_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing custom fields: {e}")),
        }
    }
    #[tool(description = "Get a custom field by its NetBox ID.")]
    async fn netbox_extras_custom_fields_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/extras/custom-fields/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting custom field {}: {e}", p.id)),
        }
    }

    #[tool(description = "List export templates (filter: q, name, object_type).")]
    async fn netbox_extras_export_templates_list(
        &self,
        Parameters(p): Parameters<extras::ExportTemplatesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match extras::export_templates_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing export templates: {e}")),
        }
    }
    #[tool(description = "Get an export template by its NetBox ID.")]
    async fn netbox_extras_export_templates_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/extras/export-templates/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting export template {}: {e}", p.id)),
        }
    }

    #[tool(description = "List webhooks (filter: q, name).")]
    async fn netbox_extras_webhooks_list(
        &self,
        Parameters(p): Parameters<extras::WebhooksListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match extras::webhooks_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing webhooks: {e}")),
        }
    }
    #[tool(description = "Get a webhook by its NetBox ID.")]
    async fn netbox_extras_webhooks_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/extras/webhooks/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting webhook {}: {e}", p.id)),
        }
    }

    // ---- VPN ----

    #[tool(description = "List VPN tunnels (filter: q, status, group, tenant).")]
    async fn netbox_vpn_tunnels_list(
        &self,
        Parameters(p): Parameters<vpn::TunnelsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match vpn::tunnels_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing VPN tunnels: {e}")),
        }
    }
    #[tool(description = "Get a VPN tunnel by its NetBox ID.")]
    async fn netbox_vpn_tunnels_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/vpn/tunnels/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting VPN tunnel {}: {e}", p.id)),
        }
    }

    #[tool(description = "List VPN tunnel groups (filter: q, name, slug).")]
    async fn netbox_vpn_tunnel_groups_list(
        &self,
        Parameters(p): Parameters<vpn::TunnelGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match vpn::tunnel_groups_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing VPN tunnel groups: {e}")),
        }
    }
    #[tool(description = "Get a VPN tunnel group by its NetBox ID.")]
    async fn netbox_vpn_tunnel_groups_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/vpn/tunnel-groups/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting VPN tunnel group {}: {e}", p.id)),
        }
    }

    #[tool(description = "List L2VPNs (filter: q, type, tenant).")]
    async fn netbox_vpn_l2vpns_list(
        &self,
        Parameters(p): Parameters<vpn::L2vpnsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match vpn::l2vpns_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing L2VPNs: {e}")),
        }
    }
    #[tool(description = "Get an L2VPN by its NetBox ID.")]
    async fn netbox_vpn_l2vpns_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/vpn/l2vpns/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting L2VPN {}: {e}", p.id)),
        }
    }

    #[tool(description = "List IKE policies (filter: q, name).")]
    async fn netbox_vpn_ike_policies_list(
        &self,
        Parameters(p): Parameters<vpn::IkePoliciesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match vpn::ike_policies_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing IKE policies: {e}")),
        }
    }
    #[tool(description = "Get an IKE policy by its NetBox ID.")]
    async fn netbox_vpn_ike_policies_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/vpn/ike-policies/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting IKE policy {}: {e}", p.id)),
        }
    }

    #[tool(description = "List IPSec policies (filter: q, name).")]
    async fn netbox_vpn_ipsec_policies_list(
        &self,
        Parameters(p): Parameters<vpn::IpsecPoliciesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match vpn::ipsec_policies_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing IPSec policies: {e}")),
        }
    }
    #[tool(description = "Get an IPSec policy by its NetBox ID.")]
    async fn netbox_vpn_ipsec_policies_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/vpn/ipsec-policies/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting IPSec policy {}: {e}", p.id)),
        }
    }

    #[tool(description = "List VPN tunnel terminations (filter: q, tunnel_id, role).")]
    async fn netbox_vpn_tunnel_terminations_list(
        &self,
        Parameters(p): Parameters<vpn::TunnelTerminationsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match vpn::tunnel_terminations_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing VPN tunnel terminations: {e}")),
        }
    }
    #[tool(description = "Get a VPN tunnel termination by its NetBox ID.")]
    async fn netbox_vpn_tunnel_terminations_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/vpn/tunnel-terminations/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting VPN tunnel termination {}: {e}", p.id)),
        }
    }

    // ---- Wireless ----

    #[tool(description = "List wireless LANs (filter: q, ssid, group, status, tenant).")]
    async fn netbox_wireless_lans_list(
        &self,
        Parameters(p): Parameters<wireless::LansListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match wireless::lans_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing wireless LANs: {e}")),
        }
    }
    #[tool(description = "Get a wireless LAN by its NetBox ID.")]
    async fn netbox_wireless_lans_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/wireless/wireless-lans/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting wireless LAN {}: {e}", p.id)),
        }
    }

    #[tool(description = "List wireless LAN groups (filter: q, name, parent).")]
    async fn netbox_wireless_lan_groups_list(
        &self,
        Parameters(p): Parameters<wireless::LanGroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match wireless::lan_groups_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing wireless LAN groups: {e}")),
        }
    }
    #[tool(description = "Get a wireless LAN group by its NetBox ID.")]
    async fn netbox_wireless_lan_groups_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/wireless/wireless-lan-groups/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting wireless LAN group {}: {e}", p.id)),
        }
    }

    #[tool(description = "List wireless links (filter: q, status, tenant).")]
    async fn netbox_wireless_links_list(
        &self,
        Parameters(p): Parameters<wireless::LinksListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match wireless::links_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing wireless links: {e}")),
        }
    }
    #[tool(description = "Get a wireless link by its NetBox ID.")]
    async fn netbox_wireless_links_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/wireless/wireless-links/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting wireless link {}: {e}", p.id)),
        }
    }

    // ---- Core ----

    #[tool(description = "List data sources (filter: q, name, status).")]
    async fn netbox_core_data_sources_list(
        &self,
        Parameters(p): Parameters<core::DataSourcesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match core::data_sources_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing data sources: {e}")),
        }
    }
    #[tool(description = "Get a data source by its NetBox ID.")]
    async fn netbox_core_data_sources_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/core/data-sources/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting data source {}: {e}", p.id)),
        }
    }

    #[tool(description = "List background jobs (filter: q, status).")]
    async fn netbox_core_jobs_list(
        &self,
        Parameters(p): Parameters<core::JobsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match core::jobs_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing jobs: {e}")),
        }
    }
    #[tool(description = "Get a background job by its NetBox ID.")]
    async fn netbox_core_jobs_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/core/jobs/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting job {}: {e}", p.id)),
        }
    }

    #[tool(description = "List object changes / audit log (filter: q, user).")]
    async fn netbox_core_object_changes_list(
        &self,
        Parameters(p): Parameters<core::ObjectChangesListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match core::object_changes_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing object changes: {e}")),
        }
    }
    #[tool(description = "Get an object change record by its NetBox ID.")]
    async fn netbox_core_object_changes_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g
            .as_ref()
            .unwrap()
            .get("/api/core/object-changes/", p.id)
            .await
        {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting object change {}: {e}", p.id)),
        }
    }

    // ---- Users ----

    #[tool(description = "List users (filter: q, username, is_active).")]
    async fn netbox_users_users_list(
        &self,
        Parameters(p): Parameters<users::UsersListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match users::users_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing users: {e}")),
        }
    }
    #[tool(description = "Get a user by their NetBox ID.")]
    async fn netbox_users_users_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/users/users/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting user {}: {e}", p.id)),
        }
    }

    #[tool(description = "List user groups (filter: q, name).")]
    async fn netbox_users_groups_list(
        &self,
        Parameters(p): Parameters<users::GroupsListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match users::groups_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing user groups: {e}")),
        }
    }
    #[tool(description = "Get a user group by its NetBox ID.")]
    async fn netbox_users_groups_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/users/groups/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting user group {}: {e}", p.id)),
        }
    }

    #[tool(description = "List API tokens (filter: q, user_id).")]
    async fn netbox_users_tokens_list(
        &self,
        Parameters(p): Parameters<users::TokensListParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match users::tokens_list(g.as_ref().unwrap(), p).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("listing tokens: {e}")),
        }
    }
    #[tool(description = "Get an API token by its NetBox ID.")]
    async fn netbox_users_tokens_get(
        &self,
        Parameters(p): Parameters<dcim::GetByIdParams>,
    ) -> Result<CallToolResult, McpError> {
        let g = self.get_client().await?;
        match g.as_ref().unwrap().get("/api/users/tokens/", p.id).await {
            Ok(v) => json_result(v),
            Err(e) => tool_error(&format!("getting token {}: {e}", p.id)),
        }
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
        // create the per-session NetboxClient.
        if let Some(parts) = context.extensions.get::<axum::http::request::Parts>()
            && let Some(token) = parts
                .headers
                .get(axum::http::header::AUTHORIZATION)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.strip_prefix("Bearer "))
        {
            let client = NetboxClient::new(self.base_url.clone(), token);
            *self.client.write().await = Some(client);
        }
        Ok(self.get_info())
    }
}
