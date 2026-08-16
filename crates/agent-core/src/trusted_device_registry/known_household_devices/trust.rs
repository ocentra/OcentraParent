use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;

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

fn trust_state_rank(state: &LanPairingTrustState) -> u8 {
    match state {
        LanPairingTrustState::Revoked => 5,
        LanPairingTrustState::Expired => 4,
        LanPairingTrustState::Paired => 3,
        LanPairingTrustState::Pairing => 2,
        LanPairingTrustState::Unpaired => 1,
    }
}
