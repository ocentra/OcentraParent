use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification;

pub(super) fn rank(classification: &LanCanonicalHouseholdDeviceClassification) -> u8 {
    match classification {
        LanCanonicalHouseholdDeviceClassification::ChildAgent => 13,
        LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure => 12,
        classification if common_device(classification) => 11,
        LanCanonicalHouseholdDeviceClassification::UnknownLanDevice => 2,
        LanCanonicalHouseholdDeviceClassification::UnsupportedLanDevice => 1,
        _ => 11,
    }
}

fn common_device(classification: &LanCanonicalHouseholdDeviceClassification) -> bool {
    matches!(
        classification,
        LanCanonicalHouseholdDeviceClassification::Phone
            | LanCanonicalHouseholdDeviceClassification::Tablet
            | LanCanonicalHouseholdDeviceClassification::Laptop
            | LanCanonicalHouseholdDeviceClassification::Desktop
            | LanCanonicalHouseholdDeviceClassification::Printer
            | LanCanonicalHouseholdDeviceClassification::Television
            | LanCanonicalHouseholdDeviceClassification::GameConsole
            | LanCanonicalHouseholdDeviceClassification::Camera
            | LanCanonicalHouseholdDeviceClassification::NetworkAttachedStorage
            | LanCanonicalHouseholdDeviceClassification::InternetOfThings
    )
}
