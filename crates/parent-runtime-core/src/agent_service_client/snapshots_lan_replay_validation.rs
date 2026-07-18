use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventHistoryState;
use ocentra_parent_agent_protocol::logging::LogFields;
use serde_json::Value;

use super::payload_fields::log_field_string;
use super::snapshots_common::{optional_bool_field, optional_string_field, optional_u64_field};

pub(super) const LAN_REPLAY_CONTEXT: &str = "agent-service LAN runtime replay payload";

pub(super) fn validate_report_metadata(
    payload: &LogFields,
    entry_count: usize,
    latest_event_id: Option<&str>,
    latest_observed_at: Option<&str>,
) -> Result<(), String> {
    required_text_field(payload, constants::field::GENERATED_AT)?;
    let observed = required_count(payload, constants::field::LAN_RUNTIME_OBSERVED_EVENTS)?;
    let streamed = required_count(payload, constants::field::LAN_RUNTIME_STREAMED_EVENTS)?;
    let failed = required_count(payload, constants::field::LAN_RUNTIME_FAILED_EVENTS)?;
    let entry_count = u64::try_from(entry_count)
        .map_err(|error| format!("{LAN_REPLAY_CONTEXT} entry count overflow: {error}"))?;

    if failed != 0 || streamed != entry_count || observed != streamed {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected inconsistent counts: observed={observed}, streamed={streamed}, failed={failed}, entries={entry_count}"
        ));
    }

    validate_history_state(payload)?;
    validate_latest_metadata(
        optional_string_field(payload, constants::field::LATEST_EVENT_ID).as_deref(),
        latest_event_id,
        constants::field::LATEST_EVENT_ID,
    )?;
    validate_latest_metadata(
        optional_string_field(payload, constants::field::LATEST_OBSERVED_AT).as_deref(),
        latest_observed_at,
        constants::field::LATEST_OBSERVED_AT,
    )
}

fn validate_history_state(payload: &LogFields) -> Result<(), String> {
    let history_state_label =
        required_text_field(payload, constants::field::LAN_RUNTIME_EVENT_HISTORY_STATE)?;
    let history_state = serde_json::from_value::<LanDiscoveryEventHistoryState>(Value::String(
        history_state_label.to_string(),
    ))
    .map_err(|error| format!("{LAN_REPLAY_CONTEXT} history state parse failed: {error}"))?;
    let manual_required =
        optional_bool_field(payload, constants::field::LAN_RUNTIME_MANUAL_REQUIRED_STATE)
            .ok_or_else(|| {
                format!(
                    "{LAN_REPLAY_CONTEXT} missing {}",
                    constants::field::LAN_RUNTIME_MANUAL_REQUIRED_STATE
                )
            })?;
    if manual_required != matches!(history_state, LanDiscoveryEventHistoryState::ManualRequired) {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected inconsistent history and manual-required state"
        ));
    }
    Ok(())
}

fn validate_latest_metadata(
    reported: Option<&str>,
    actual: Option<&str>,
    field: &'static str,
) -> Result<(), String> {
    if reported.is_some_and(|reported| Some(reported) != actual) {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected {field} that does not match the final stream entry"
        ));
    }
    Ok(())
}

fn required_text_field<'a>(payload: &'a LogFields, field: &'static str) -> Result<&'a str, String> {
    payload
        .get(field)
        .and_then(log_field_string)
        .and_then(|value| canonical_text(value, field).ok())
        .ok_or_else(|| format!("{LAN_REPLAY_CONTEXT} missing or invalid {field}"))
}

fn required_count(payload: &LogFields, field: &'static str) -> Result<u64, String> {
    optional_u64_field(payload, field)
        .ok_or_else(|| format!("{LAN_REPLAY_CONTEXT} missing or invalid {field}"))
}

pub(super) fn validate_optional_text(
    value: Option<&str>,
    field: &'static str,
) -> Result<(), String> {
    value.map_or(Ok(()), |value| canonical_text(value, field).map(|_| ()))
}

pub(super) fn canonical_text<'a>(value: &'a str, field: &'static str) -> Result<&'a str, String> {
    if value.is_empty() || value.trim() != value {
        Err(format!("{LAN_REPLAY_CONTEXT} rejected invalid {field}"))
    } else {
        Ok(value)
    }
}
