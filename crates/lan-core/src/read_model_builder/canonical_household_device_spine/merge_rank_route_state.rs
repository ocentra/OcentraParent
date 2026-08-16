use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState;

pub(super) fn rank(state: &LanCanonicalHouseholdRouteState) -> u8 {
    match state {
        LanCanonicalHouseholdRouteState::LocalNetwork => 4,
        LanCanonicalHouseholdRouteState::Localhost => 3,
        LanCanonicalHouseholdRouteState::ManualRequired => 2,
        LanCanonicalHouseholdRouteState::Unavailable => 1,
    }
}
