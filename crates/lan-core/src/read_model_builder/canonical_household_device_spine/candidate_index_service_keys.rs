use std::collections::HashSet;

use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceRecord;

use super::super::push_normalized_key;

pub(super) fn push_service_hint_key(
    keys: &mut HashSet<String>,
    record: &LanDiscoveryEvidenceRecord,
) {
    for prefix in [
        "mdns-instance-name:",
        "ssdp-udn:",
        "mdns-service-type:",
        "ssdp-device-type:",
    ] {
        if record
            .value
            .get(..prefix.len())
            .is_some_and(|value| value.eq_ignore_ascii_case(prefix))
        {
            push_normalized_key(keys, prefix, &record.normalized_value);
        }
    }
}
