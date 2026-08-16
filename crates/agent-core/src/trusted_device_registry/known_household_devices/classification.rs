use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification;

pub(super) fn preferred_classification(
    existing: LanCanonicalHouseholdDeviceClassification,
    incoming: LanCanonicalHouseholdDeviceClassification,
) -> LanCanonicalHouseholdDeviceClassification {
    use LanCanonicalHouseholdDeviceClassification::{
        ChildAgent, NetworkInfrastructure, UnknownLanDevice, UnsupportedLanDevice,
    };

    match (existing, incoming) {
        (ChildAgent, _) | (_, ChildAgent) => ChildAgent,
        (NetworkInfrastructure, _) | (_, NetworkInfrastructure) => NetworkInfrastructure,
        (UnknownLanDevice, other) => other,
        (existing, UnsupportedLanDevice) => existing,
        (_, incoming) => incoming,
    }
}
