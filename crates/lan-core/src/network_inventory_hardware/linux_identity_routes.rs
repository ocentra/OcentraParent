use ocentra_parent_agent_protocol::constants;

use super::super::network_identity_support::{
    push_unique_string_if, supported_dns_server_text, supported_local_ipv4_text,
};
use super::LinuxDefaultRoute;
use crate::network_inventory_command::record_text;

pub(super) fn linux_default_route(
    route_records: &[serde_json::Value],
) -> Result<Option<LinuxDefaultRoute>, ()> {
    let mut routes = route_records.iter().filter_map(|record| {
        if record_text(record, constants::lan_pairing::JSON_KEY_DST).as_deref() != Some("default") {
            return None;
        }
        let device = record_text(record, constants::lan_pairing::JSON_KEY_DEV)?;
        let gateway = record_text(record, constants::lan_pairing::JSON_KEY_GATEWAY)
            .filter(|value| supported_local_ipv4_text(value));
        Some(LinuxDefaultRoute { device, gateway })
    });
    let route = routes.next();
    if route.is_some() && routes.next().is_some() {
        return Err(());
    }
    Ok(route)
}

pub(super) fn linux_dns_servers_from_resolv_conf_text(text: &str) -> Vec<String> {
    let mut dns_servers = Vec::new();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or_default().trim();
        let mut parts = line.split_whitespace();
        if parts.next() != Some("nameserver") {
            continue;
        }
        if let Some(server) = parts.next() {
            push_unique_string_if(&mut dns_servers, server, supported_dns_server_text(server));
        }
    }
    dns_servers
}
