use ocentra_parent_agent_protocol::{
    constants, BrowserEvidenceReadModel, BrowserTabEvidence, LogFieldValue, LogFields,
};

use crate::fields::fields_from_pairs;

type FieldPair = (&'static str, LogFieldValue);

pub fn browser_evidence_read_model_payload(read_model: &BrowserEvidenceReadModel) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = read_model_pairs(read_model);
    pairs.extend(latest_identity_pairs(latest));
    pairs.extend(latest_target_pairs(latest));
    pairs.extend(latest_state_pairs(latest));
    fields_from_pairs(pairs)
}

fn read_model_pairs(read_model: &BrowserEvidenceReadModel) -> Vec<FieldPair> {
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::LIMIT,
            LogFieldValue::Number(read_model.limit as f64),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            constants::field::LATEST_EVENT_ID,
            optional_string(read_model.latest_event_id.as_ref()),
        ),
        (
            constants::field::LATEST_OBSERVED_AT,
            optional_string(read_model.latest_observed_at.as_ref()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            optional_enum(
                read_model
                    .capability_status
                    .as_ref()
                    .map(|status| status.as_protocol_str()),
            ),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(read_model.custody_label.as_protocol_str().to_string()),
        ),
        (
            constants::field::QUERY_VISIBILITY,
            LogFieldValue::String(read_model.query_visibility.as_protocol_str().to_string()),
        ),
    ]
}

fn latest_identity_pairs(row: Option<&BrowserTabEvidence>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::BROWSER_EVIDENCE_ID,
            optional_string(row.map(|value| &value.browser_evidence_id)),
        ),
        (
            constants::field::SOURCE_ID,
            optional_string(row.map(|value| &value.source_id)),
        ),
        (
            constants::field::ADAPTER_ID,
            optional_string(row.map(|value| &value.adapter_id)),
        ),
        (
            constants::field::MANAGED_BROWSER_SESSION_ID,
            optional_string(row.map(|value| &value.managed_browser_session_id)),
        ),
        (
            constants::field::BROWSER_FAMILY,
            optional_enum(row.map(|value| value.browser_family.as_protocol_str())),
        ),
        (
            constants::field::BROWSER_CHANNEL,
            optional_enum(row.map(|value| value.browser_channel.as_protocol_str())),
        ),
        (
            constants::field::PROFILE_ID,
            optional_string(row.map(|value| &value.profile_id)),
        ),
        (
            constants::field::PROCESS_ID,
            optional_u32(row.map(|value| value.process_id)),
        ),
    ]
}

fn latest_target_pairs(row: Option<&BrowserTabEvidence>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::WINDOW_ID,
            optional_string(row.and_then(|value| value.window_id.as_ref())),
        ),
        (
            constants::field::TAB_ID,
            optional_string(row.and_then(|value| value.tab_id.as_ref())),
        ),
        (
            constants::field::TARGET_ID,
            optional_string(row.and_then(|value| value.target_id.as_ref())),
        ),
        (
            constants::field::URL,
            optional_string(row.map(|value| &value.url)),
        ),
        (
            constants::field::ORIGIN,
            optional_string(row.map(|value| &value.origin)),
        ),
        (
            constants::field::DOMAIN,
            optional_string(row.map(|value| &value.domain)),
        ),
        (
            constants::field::TITLE,
            optional_string(row.and_then(|value| value.title.as_ref())),
        ),
    ]
}

fn latest_state_pairs(row: Option<&BrowserTabEvidence>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::ACTIVE_STATE,
            optional_enum(row.map(|value| value.active_state.as_protocol_str())),
        ),
        (
            constants::field::ACTIVE_PROOF_SOURCE,
            optional_enum(row.map(|value| value.active_proof_source.as_protocol_str())),
        ),
        (
            constants::field::FRESH_UNTIL,
            optional_string(row.map(|value| &value.fresh_until)),
        ),
        (
            constants::field::STALE_AT,
            optional_string(row.map(|value| &value.stale_at)),
        ),
        (
            constants::field::DEGRADED_REASON,
            optional_string(row.and_then(|value| value.degraded_reason.as_ref())),
        ),
    ]
}

fn optional_string(value: Option<&String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_enum(value: Option<&str>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.to_string()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_u32(value: Option<u32>) -> LogFieldValue {
    match value {
        Some(number) => LogFieldValue::Number(number as f64),
        None => LogFieldValue::Null(()),
    }
}

#[cfg(test)]
mod tests {
    use ocentra_parent_agent_protocol::{
        BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
        BrowserCustodyLabel, BrowserFamily, BrowserQueryVisibilityLabel,
        BROWSER_EVIDENCE_SCHEMA_VERSION,
    };

    use super::*;

    #[test]
    fn browser_evidence_payload_uses_degraded_reason_field() {
        let payload = browser_evidence_read_model_payload(&read_model());

        assert_eq!(
            payload[constants::field::DEGRADED_REASON],
            LogFieldValue::String(constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string())
        );
        assert_eq!(
            payload[constants::field::ACTIVE_PROOF_SOURCE],
            LogFieldValue::String(
                constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY.to_string()
            )
        );
        assert_eq!(payload.get(constants::field::REASON), None);
    }

    fn read_model() -> BrowserEvidenceReadModel {
        BrowserEvidenceReadModel {
            schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
            generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
            limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
            returned: 1,
            latest_event_id: Some(constants::event_id::HEALTH_REPORTED.to_string()),
            latest_observed_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
            capability_status: Some(BrowserCapabilityStatus::TabListOnly),
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
            query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
            rows: vec![BrowserTabEvidence {
                schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
                browser_evidence_id: constants::browser::EVIDENCE_ID_PREFIX.to_string(),
                observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
                fresh_until: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
                source_id: constants::browser::SOURCE_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string(),
                adapter_id: constants::browser::ADAPTER_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string(),
                device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                browser_family: BrowserFamily::Chrome,
                browser_channel: BrowserChannel::Stable,
                managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
                profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
                process_id: constants::activity_store::TEST_BROWSER_PROCESS_ID,
                window_id: None,
                tab_id: Some(
                    constants::activity_store::TEST_BROWSER_TAB_ID_FROM_TARGET.to_string(),
                ),
                target_id: Some(constants::activity_store::TEST_BROWSER_TARGET_ID.to_string()),
                active_state: BrowserActiveTabState::Unknown,
                active_proof_source: BrowserActiveProofSource::TargetListOnly,
                url: constants::activity_store::TEST_BROWSER_URL.to_string(),
                origin: constants::activity_store::TEST_BROWSER_ORIGIN.to_string(),
                domain: constants::activity_store::TEST_BROWSER_DOMAIN.to_string(),
                title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
                capability_status: BrowserCapabilityStatus::TabListOnly,
                degraded_reason: Some(constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string()),
                stale_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
                custody_label: BrowserCustodyLabel::ChildDeviceLocal,
                query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
            }],
        }
    }
}
