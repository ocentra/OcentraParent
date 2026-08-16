use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;

pub(super) fn state_from_trust(
    trust_state: &LanPairingTrustState,
) -> LanPairingProductionDiscoveryState {
    match trust_state {
        LanPairingTrustState::Paired => LanPairingProductionDiscoveryState::Paired,
        LanPairingTrustState::Pairing => LanPairingProductionDiscoveryState::Pending,
        LanPairingTrustState::Revoked => LanPairingProductionDiscoveryState::Revoked,
        LanPairingTrustState::Expired => LanPairingProductionDiscoveryState::Expired,
        LanPairingTrustState::Unpaired => LanPairingProductionDiscoveryState::Discovered,
    }
}
