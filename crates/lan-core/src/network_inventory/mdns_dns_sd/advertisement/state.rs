use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanMdnsAdvertisementLifecycleState, LanMdnsAdvertisementSupportState, LanPairingTrustState,
};

pub(super) fn parse_pairing_state(value: &str) -> Option<LanPairingTrustState> {
    if value.eq_ignore_ascii_case(constants::value::LAN_PAIRING_UNPAIRED) {
        Some(LanPairingTrustState::Unpaired)
    } else if value.eq_ignore_ascii_case(constants::value::LAN_PAIRING_PAIRING) {
        Some(LanPairingTrustState::Pairing)
    } else if value.eq_ignore_ascii_case(constants::value::LAN_PAIRING_PAIRED) {
        Some(LanPairingTrustState::Paired)
    } else if value.eq_ignore_ascii_case(constants::value::LAN_PAIRING_REVOKED) {
        Some(LanPairingTrustState::Revoked)
    } else if value.eq_ignore_ascii_case(constants::value::LAN_PAIRING_EXPIRED) {
        Some(LanPairingTrustState::Expired)
    } else {
        None
    }
}

pub(super) fn parse_lifecycle_state(value: &str) -> Option<LanMdnsAdvertisementLifecycleState> {
    if value.eq_ignore_ascii_case(constants::lan_pairing::MDNS_TXT_VALUE_START) {
        Some(LanMdnsAdvertisementLifecycleState::Start)
    } else if value.eq_ignore_ascii_case(constants::lan_pairing::MDNS_TXT_VALUE_UPDATE) {
        Some(LanMdnsAdvertisementLifecycleState::Update)
    } else if value.eq_ignore_ascii_case(constants::lan_pairing::MDNS_TXT_VALUE_STOP) {
        Some(LanMdnsAdvertisementLifecycleState::Stop)
    } else if value.eq_ignore_ascii_case(constants::lan_pairing::MDNS_TXT_VALUE_DEGRADED) {
        Some(LanMdnsAdvertisementLifecycleState::Degraded)
    } else {
        None
    }
}

pub(super) fn parse_support_state(value: &str) -> Option<LanMdnsAdvertisementSupportState> {
    if value.eq_ignore_ascii_case(constants::lan_pairing::MDNS_TXT_VALUE_SUPPORTED) {
        Some(LanMdnsAdvertisementSupportState::Supported)
    } else if value.eq_ignore_ascii_case(constants::lan_pairing::MDNS_TXT_VALUE_DEGRADED) {
        Some(LanMdnsAdvertisementSupportState::Degraded)
    } else if value
        .eq_ignore_ascii_case(constants::lan_pairing::MDNS_TXT_VALUE_UNSUPPORTED_PLATFORM)
    {
        Some(LanMdnsAdvertisementSupportState::UnsupportedPlatform)
    } else {
        None
    }
}
