use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;

pub(super) fn rank(state: &LanPairingProductionDiscoveryState) -> u8 {
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
