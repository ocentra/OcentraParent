use chrono::{DateTime, FixedOffset};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventKind;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use super::payload_fields::log_field_string;
use super::snapshots_common::optional_u64_field;

pub(super) mod envelope;
mod history_state;

use self::history_state::validate_history_state;

pub(super) const LAN_REPLAY_CONTEXT: &str = "agent-service LAN runtime replay payload";

pub(super) fn validate_report_metadata<'a>(
    payload: &LogFields,
    entry_count: usize,
    event_kinds: impl Iterator<Item = &'a LanDiscoveryEventKind>,
    latest_event_id: Option<&str>,
    latest_observed_at: Option<&str>,
) -> Result<(), String> {
    let generated_at = required_text_field(payload, constants::field::GENERATED_AT)?;
    parse_rfc3339_timestamp(generated_at, constants::field::GENERATED_AT)?;
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

    validate_history_state(payload, entry_count, event_kinds)?;
    let reported_event_id = optional_reported_text(payload, constants::field::LATEST_EVENT_ID)?;
    validate_latest_metadata(
        reported_event_id,
        latest_event_id,
        constants::field::LATEST_EVENT_ID,
    )?;
    let reported_observed_at =
        optional_reported_text(payload, constants::field::LATEST_OBSERVED_AT)?;
    if let Some(reported_observed_at) = reported_observed_at {
        parse_rfc3339_timestamp(reported_observed_at, constants::field::LATEST_OBSERVED_AT)?;
    }
    validate_latest_metadata(
        reported_observed_at,
        latest_observed_at,
        constants::field::LATEST_OBSERVED_AT,
    )
}

fn validate_latest_metadata(
    reported: Option<&str>,
    actual: Option<&str>,
    field: &'static str,
) -> Result<(), String> {
    if reported != actual {
        return Err(format!(
            "{LAN_REPLAY_CONTEXT} rejected {field} that does not match the final stream entry"
        ));
    }
    Ok(())
}

fn optional_reported_text<'a>(
    payload: &'a LogFields,
    field: &'static str,
) -> Result<Option<&'a str>, String> {
    match payload.get(field) {
        None => Ok(None),
        Some(LogFieldValue::String(value)) if value.is_empty() => Ok(None),
        Some(LogFieldValue::String(value)) => canonical_text(value, field).map(Some),
        Some(_) => Err(format!("{LAN_REPLAY_CONTEXT} missing or invalid {field}")),
    }
}

fn required_text_field<'a>(payload: &'a LogFields, field: &'static str) -> Result<&'a str, String> {
    let value = payload
        .get(field)
        .and_then(log_field_string)
        .ok_or_else(|| format!("{LAN_REPLAY_CONTEXT} missing or invalid {field}"))?;
    canonical_text(value, field)
}

fn required_count(payload: &LogFields, field: &'static str) -> Result<u64, String> {
    optional_u64_field(payload, field)
        .ok_or_else(|| format!("{LAN_REPLAY_CONTEXT} missing or invalid {field}"))
}

pub(super) fn parse_rfc3339_timestamp(
    value: &str,
    field: &'static str,
) -> Result<DateTime<FixedOffset>, String> {
    let value = canonical_text(value, field)?;
    DateTime::parse_from_rfc3339(value)
        .map_err(|error| format!("{LAN_REPLAY_CONTEXT} rejected invalid {field}: {error}"))
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
