use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingEvidenceRef;

pub(super) fn evidence_reference_ids(
    fields: &LogFields,
    evidence: &[ActivityEvidenceRef],
) -> Vec<TrackingEvidenceRef> {
    let mut ids = string_field(fields, constants::field::EVIDENCE_REFERENCE_IDS)
        .map(|value| split_evidence_reference_ids(&value))
        .unwrap_or_default();

    for reference in evidence {
        let evidence_ref = TrackingEvidenceRef::parse(reference.evidence_id.clone())
            .expect_value("tracking read-model evidence reference parses");
        if !ids.iter().any(|id| id == &evidence_ref) {
            ids.push(evidence_ref);
        }
    }
    ids
}

pub(super) fn split_evidence_reference_ids(value: &str) -> Vec<TrackingEvidenceRef> {
    value
        .split(constants::delimiter::LIST)
        .map(str::trim)
        .filter(|id| !id.is_empty())
        .map(|id| {
            TrackingEvidenceRef::parse(id)
                .expect_value("tracking read-model split evidence reference parses")
        })
        .collect()
}

pub(super) fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}
