use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanCanonicalHouseholdDeviceClassification,
    LanCanonicalHouseholdDeviceSource,
};

pub(super) fn has_agent_or_registry_evidence(device: &LanCanonicalHouseholdDevice) -> bool {
    device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent
        || device.child_agent_inventory.is_some()
        || device.source_labels.iter().any(|source| {
            matches!(
                source,
                LanCanonicalHouseholdDeviceSource::LocalService
                    | LanCanonicalHouseholdDeviceSource::TrustedRegistry
            )
        })
}

pub(super) fn has_local_service_identity_anchor(
    existing: &LanCanonicalHouseholdDevice,
    incoming: &LanCanonicalHouseholdDevice,
) -> bool {
    existing
        .source_labels
        .iter()
        .chain(incoming.source_labels.iter())
        .any(|source| matches!(source, LanCanonicalHouseholdDeviceSource::LocalService))
}
