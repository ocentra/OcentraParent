mod candidates;
mod model;
mod parsing;

use serde_json::Value;

use super::linux_identity::LinuxDefaultRoute;
use super::LocalNetworkInterface;

pub(super) fn windows_local_network_interface_candidate(
    record: &Value,
) -> Option<LocalNetworkInterface> {
    candidates::windows_local_network_interface_candidate(record)
}

pub(super) fn linux_local_network_interface_candidate(
    record: &Value,
    default_route: Option<&LinuxDefaultRoute>,
    dns_servers: &[String],
) -> Option<LocalNetworkInterface> {
    candidates::linux_local_network_interface_candidate(record, default_route, dns_servers)
}

pub(super) fn merge_interface_candidate(
    interfaces: &mut Vec<LocalNetworkInterface>,
    candidate: LocalNetworkInterface,
) {
    candidates::merge_interface_candidate(interfaces, candidate);
}
