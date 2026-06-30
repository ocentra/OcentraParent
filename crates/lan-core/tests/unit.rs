#[path = "unit/support.rs"]
mod support;

#[path = "unit/lan_flow.rs"]
mod lan_flow;

#[path = "unit/discovery_flow.rs"]
mod discovery_flow;

#[path = "unit/canonical_household_merge.rs"]
mod canonical_household_merge;

#[path = "unit/canonical_household_classification.rs"]
mod canonical_household_classification;

#[path = "unit/lan_mdns_advertiser.rs"]
mod lan_mdns_advertiser;

#[path = "unit/mac_identity.rs"]
mod mac_identity;

#[path = "unit/network_inventory.rs"]
mod network_inventory;

#[path = "unit/network_inventory_active_refresh.rs"]
mod network_inventory_active_refresh;

#[path = "unit/network_inventory_command.rs"]
mod network_inventory_command;

#[path = "unit/network_inventory_hardware.rs"]
mod network_inventory_hardware;

#[path = "unit/network_inventory_linux_neighbors.rs"]
mod network_inventory_linux_neighbors;

#[path = "unit/network_inventory_macos_neighbors.rs"]
mod network_inventory_macos_neighbors;

#[path = "unit/network_inventory_mdns_dns_sd.rs"]
mod network_inventory_mdns_dns_sd;

#[path = "unit/network_inventory_name_evidence.rs"]
mod network_inventory_name_evidence;

#[path = "unit/network_inventory_neighbor_support.rs"]
mod network_inventory_neighbor_support;

#[path = "unit/network_inventory_passive_discovery.rs"]
mod network_inventory_passive_discovery;

#[path = "unit/network_inventory_ssdp_upnp.rs"]
mod network_inventory_ssdp_upnp;

#[path = "unit/network_inventory_windows_neighbors.rs"]
mod network_inventory_windows_neighbors;

#[path = "unit/passive_discovery.rs"]
mod passive_discovery;

#[path = "unit/read_model.rs"]
mod read_model;

#[path = "unit/service_identity.rs"]
mod service_identity;
