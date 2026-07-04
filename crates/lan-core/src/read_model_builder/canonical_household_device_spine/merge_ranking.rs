use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceClassification, LanCanonicalHouseholdRouteState,
};

pub(super) fn stronger_classification(
    existing: LanCanonicalHouseholdDeviceClassification,
    incoming: LanCanonicalHouseholdDeviceClassification,
) -> LanCanonicalHouseholdDeviceClassification {
    if classification_rank(&incoming) > classification_rank(&existing) {
        incoming
    } else {
        existing
    }
}

pub(super) fn stronger_discovery_state(
    existing: LanPairingProductionDiscoveryState,
    incoming: LanPairingProductionDiscoveryState,
) -> LanPairingProductionDiscoveryState {
    if discovery_state_rank(&incoming) > discovery_state_rank(&existing) {
        incoming
    } else {
        existing
    }
}

pub(super) fn stronger_trust_state(
    existing: LanPairingTrustState,
    incoming: LanPairingTrustState,
) -> LanPairingTrustState {
    if trust_state_rank(&incoming) > trust_state_rank(&existing) {
        incoming
    } else {
        existing
    }
}

pub(super) fn stronger_route_state(
    existing: LanCanonicalHouseholdRouteState,
    incoming: LanCanonicalHouseholdRouteState,
) -> LanCanonicalHouseholdRouteState {
    if route_state_rank(&incoming) > route_state_rank(&existing) {
        incoming
    } else {
        existing
    }
}

fn classification_rank(classification: &LanCanonicalHouseholdDeviceClassification) -> u8 {
    match classification {
        LanCanonicalHouseholdDeviceClassification::ChildAgent => 13,
        LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure => 12,
        LanCanonicalHouseholdDeviceClassification::Phone
        | LanCanonicalHouseholdDeviceClassification::Tablet
        | LanCanonicalHouseholdDeviceClassification::Laptop
        | LanCanonicalHouseholdDeviceClassification::Desktop
        | LanCanonicalHouseholdDeviceClassification::Printer
        | LanCanonicalHouseholdDeviceClassification::Television
        | LanCanonicalHouseholdDeviceClassification::GameConsole
        | LanCanonicalHouseholdDeviceClassification::Camera
        | LanCanonicalHouseholdDeviceClassification::NetworkAttachedStorage
        | LanCanonicalHouseholdDeviceClassification::InternetOfThings => 11,
        LanCanonicalHouseholdDeviceClassification::UnknownLanDevice => 2,
        LanCanonicalHouseholdDeviceClassification::UnsupportedLanDevice => 1,
    }
}

fn discovery_state_rank(state: &LanPairingProductionDiscoveryState) -> u8 {
    match state {
        LanPairingProductionDiscoveryState::Revoked => 8,
        LanPairingProductionDiscoveryState::Rejected => 7,
        LanPairingProductionDiscoveryState::Expired => 6,
        LanPairingProductionDiscoveryState::Paired => 5,
        LanPairingProductionDiscoveryState::Discovered => 4,
        LanPairingProductionDiscoveryState::Stale => 3,
        LanPairingProductionDiscoveryState::Offline => 2,
        LanPairingProductionDiscoveryState::ManualRequired => 1,
        LanPairingProductionDiscoveryState::Pending
        | LanPairingProductionDiscoveryState::Unavailable => 0,
    }
}

fn trust_state_rank(state: &LanPairingTrustState) -> u8 {
    match state {
        LanPairingTrustState::Revoked => 5,
        LanPairingTrustState::Expired => 4,
        LanPairingTrustState::Paired => 3,
        LanPairingTrustState::Pairing => 2,
        LanPairingTrustState::Unpaired => 1,
    }
}

fn route_state_rank(state: &LanCanonicalHouseholdRouteState) -> u8 {
    match state {
        LanCanonicalHouseholdRouteState::LocalNetwork => 4,
        LanCanonicalHouseholdRouteState::Localhost => 3,
        LanCanonicalHouseholdRouteState::ManualRequired => 2,
        LanCanonicalHouseholdRouteState::Unavailable => 1,
    }
}
