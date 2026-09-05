use ocentra_parent_agent_protocol::browser_read_model::{
    BrowserEvidenceReadModel, BrowserTabEvidence,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

#[path = "browser_evidence_payload/field_pairs.rs"]
mod field_pairs;

use field_pairs::{
    browser_evidence_fields_from_pairs, field_pair, optional_text, optional_u32,
    BrowserEvidenceFieldKey, BrowserEvidenceFieldPair, BrowserEvidenceTextRef,
};

pub fn browser_evidence_read_model_payload(read_model: &BrowserEvidenceReadModel) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = read_model_pairs(read_model);
    pairs.extend(latest_identity_pairs(latest));
    pairs.extend(latest_target_pairs(latest));
    pairs.extend(latest_state_pairs(latest));
    if let Ok(serialized) = serde_json::to_string(read_model) {
        pairs.push(field_pair(
            BrowserEvidenceFieldKey(constants::field::BROWSER_EVIDENCE_READ_MODEL_JSON),
            LogFieldValue::String(serialized),
        ));
    }
    browser_evidence_fields_from_pairs(pairs)
}

fn read_model_pairs(read_model: &BrowserEvidenceReadModel) -> Vec<BrowserEvidenceFieldPair> {
    vec![
        field_pair(
            BrowserEvidenceFieldKey(constants::field::GENERATED_AT),
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::LIMIT),
            LogFieldValue::Number(read_model.limit as f64),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::RETURNED),
            LogFieldValue::Number(read_model.returned as f64),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::LATEST_EVENT_ID),
            optional_text(
                read_model
                    .latest_event_id
                    .as_deref()
                    .map(BrowserEvidenceTextRef),
            ),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::LATEST_OBSERVED_AT),
            optional_text(
                read_model
                    .latest_observed_at
                    .as_deref()
                    .map(BrowserEvidenceTextRef),
            ),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::CAPABILITY_STATUS),
            optional_text(
                read_model
                    .capability_status
                    .as_ref()
                    .map(|status| BrowserEvidenceTextRef(status.as_protocol_str())),
            ),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::CUSTODY_LABEL),
            LogFieldValue::String(read_model.custody_label.as_protocol_str().to_string()),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::QUERY_VISIBILITY),
            LogFieldValue::String(read_model.query_visibility.as_protocol_str().to_string()),
        ),
    ]
}

fn latest_identity_pairs(row: Option<&BrowserTabEvidence>) -> Vec<BrowserEvidenceFieldPair> {
    vec![
        field_pair(
            BrowserEvidenceFieldKey(constants::field::BROWSER_EVIDENCE_ID),
            optional_text(row.map(|value| BrowserEvidenceTextRef(&value.browser_evidence_id))),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::SOURCE_ID),
            optional_text(row.map(|value| BrowserEvidenceTextRef(&value.source_id))),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::ADAPTER_ID),
            optional_text(row.map(|value| BrowserEvidenceTextRef(&value.adapter_id))),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::MANAGED_BROWSER_SESSION_ID),
            optional_text(
                row.map(|value| BrowserEvidenceTextRef(&value.managed_browser_session_id)),
            ),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::BROWSER_FAMILY),
            optional_text(
                row.map(|value| BrowserEvidenceTextRef(value.browser_family.as_protocol_str())),
            ),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::BROWSER_CHANNEL),
            optional_text(
                row.map(|value| BrowserEvidenceTextRef(value.browser_channel.as_protocol_str())),
            ),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::PROFILE_ID),
            optional_text(row.map(|value| BrowserEvidenceTextRef(&value.profile_id))),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::PROCESS_ID),
            optional_u32(row.map(|value| value.process_id)),
        ),
    ]
}

fn latest_target_pairs(row: Option<&BrowserTabEvidence>) -> Vec<BrowserEvidenceFieldPair> {
    vec![
        field_pair(
            BrowserEvidenceFieldKey(constants::field::WINDOW_ID),
            optional_text(
                row.and_then(|value| value.window_id.as_deref().map(BrowserEvidenceTextRef)),
            ),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::TAB_ID),
            optional_text(
                row.and_then(|value| value.tab_id.as_deref().map(BrowserEvidenceTextRef)),
            ),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::TARGET_ID),
            optional_text(
                row.and_then(|value| value.target_id.as_deref().map(BrowserEvidenceTextRef)),
            ),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::URL),
            optional_text(row.map(|value| BrowserEvidenceTextRef(&value.url))),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::ORIGIN),
            optional_text(row.map(|value| BrowserEvidenceTextRef(&value.origin))),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::DOMAIN),
            optional_text(row.map(|value| BrowserEvidenceTextRef(&value.domain))),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::TITLE),
            optional_text(row.and_then(|value| value.title.as_deref().map(BrowserEvidenceTextRef))),
        ),
    ]
}

fn latest_state_pairs(row: Option<&BrowserTabEvidence>) -> Vec<BrowserEvidenceFieldPair> {
    vec![
        field_pair(
            BrowserEvidenceFieldKey(constants::field::ACTIVE_STATE),
            optional_text(
                row.map(|value| BrowserEvidenceTextRef(value.active_state.as_protocol_str())),
            ),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::ACTIVE_PROOF_SOURCE),
            optional_text(
                row.map(|value| {
                    BrowserEvidenceTextRef(value.active_proof_source.as_protocol_str())
                }),
            ),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::FRESH_UNTIL),
            optional_text(row.map(|value| BrowserEvidenceTextRef(&value.fresh_until))),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::STALE_AT),
            optional_text(row.map(|value| BrowserEvidenceTextRef(&value.stale_at))),
        ),
        field_pair(
            BrowserEvidenceFieldKey(constants::field::DEGRADED_REASON),
            optional_text(
                row.and_then(|value| value.degraded_reason.as_deref().map(BrowserEvidenceTextRef)),
            ),
        ),
    ]
}
