use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingText, LanSelectedRouteTarget,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::fields::fields_from_pairs;

use super::{selection, LanPairingStatus};

pub(super) fn discovery_state(status: &LanPairingStatus) -> LanPairingText {
    match status
        .selected_target
        .as_ref()
        .map(|target| &target.reachability)
    {
        Some(LanPairingDeviceReachability::Offline) => {
            constants::value::LAN_DISCOVERY_STATE_OFFLINE
                .to_string()
                .into()
        }
        Some(LanPairingDeviceReachability::Stale) => constants::value::LAN_DISCOVERY_STATE_STALE
            .to_string()
            .into(),
        Some(LanPairingDeviceReachability::Online) => constants::value::LAN_DISCOVERY_STATE_PAIRED
            .to_string()
            .into(),
        None if status.trusted_device_count > 0 => constants::value::LAN_DISCOVERY_STATE_PAIRED
            .to_string()
            .into(),
        None if status.active_challenge_count > 0 => constants::value::LAN_DISCOVERY_STATE_PENDING
            .to_string()
            .into(),
        None if status.has_revoked_pairing => constants::value::LAN_DISCOVERY_STATE_REVOKED
            .to_string()
            .into(),
        None => constants::value::LAN_DISCOVERY_STATE_DISCOVERED
            .to_string()
            .into(),
    }
}

pub(super) fn state_fields(status: &LanPairingStatus) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::LAN_PAIRING_STATE,
            LogFieldValue::String(status.pairing_state.to_string()),
        ),
        (
            constants::field::LAN_AUTHENTICATION_STATE,
            LogFieldValue::String(status.authentication_state.to_string()),
        ),
        (
            constants::field::LAN_TRUSTED_DEVICE_COUNT,
            LogFieldValue::Number(status.trusted_device_count as f64),
        ),
        (
            constants::field::LAN_TRUSTED_DEVICE_IDS,
            LogFieldValue::String(
                status
                    .trusted_device_ids
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_REVOKED_DEVICE_IDS,
            LogFieldValue::String(
                status
                    .revoked_device_ids
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_SELECTED_CHILD_DEVICE_ID,
            LogFieldValue::String(selection::child_device_id(status.selected_target.as_ref()).0),
        ),
        (
            constants::field::LAN_SELECTED_PAIRING_ID,
            LogFieldValue::String(selection::pairing_id(status.selected_target.as_ref()).0),
        ),
        (
            constants::field::LAN_SELECTED_ROUTE_TRUST_STATE,
            LogFieldValue::String(
                selection::route_trust_state(status.selected_target.as_ref()).to_string(),
            ),
        ),
        (
            constants::field::LAN_SELECTED_DEVICE_REACHABILITY,
            LogFieldValue::String(
                selection::reachability(status.selected_target.as_ref()).to_string(),
            ),
        ),
        (
            constants::field::LAN_SELECTED_DEVICE_STALE_AT,
            LogFieldValue::String(selection::stale_at(status.selected_target.as_ref()).0),
        ),
        (
            constants::field::LAN_SELECTED_ROUTE_ID,
            LogFieldValue::String(selection::route_id(status.selected_target.as_ref()).0),
        ),
        (
            constants::field::LAN_SELECTED_ROUTE_STALE_AT,
            LogFieldValue::String(selection::stale_at(status.selected_target.as_ref()).0),
        ),
        (
            constants::field::LAN_SELECTED_ROUTE_OFFLINE_AT,
            LogFieldValue::String(selection::offline_at(status.selected_target.as_ref()).0),
        ),
    ])
}

pub(super) fn pairing_state(status: &LanPairingStatus) -> LanPairingText {
    if status.trusted_device_count > 0 {
        constants::value::LAN_PAIRING_PAIRED.to_string().into()
    } else if status.active_challenge_count > 0 {
        constants::value::LAN_PAIRING_PAIRING.to_string().into()
    } else if status.has_revoked_pairing {
        constants::value::LAN_PAIRING_REVOKED.to_string().into()
    } else {
        constants::value::LAN_PAIRING_UNPAIRED.to_string().into()
    }
}

pub(super) fn authentication_state(selected: &Option<LanSelectedRouteTarget>) -> LanPairingText {
    if selected.is_some() {
        constants::value::LAN_AUTH_PAIRED.to_string().into()
    } else {
        constants::value::LAN_AUTH_UNPAIRED.to_string().into()
    }
}
