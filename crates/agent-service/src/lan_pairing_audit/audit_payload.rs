use crate::fields::fields_from_pairs;
use ocentra_parent_agent_protocol::activity::policy::ParentEvidenceReference;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

pub(super) fn evidence_reference_fields(
    evidence_references: Option<&[ParentEvidenceReference]>,
) -> LogFields {
    let Some(evidence_references) = evidence_references else {
        return LogFields::default();
    };
    let evidence_reference_ids = evidence_references
        .iter()
        .map(|reference| reference.evidence_reference_id.as_str())
        .collect::<Vec<_>>()
        .join(&constants::delimiter::LIST.to_string());
    fields_from_pairs(vec![
        (
            constants::field::LAN_EVIDENCE_REFERENCE_COUNT,
            LogFieldValue::Number(evidence_references.len() as f64),
        ),
        (
            constants::field::LAN_EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(evidence_reference_ids),
        ),
    ])
}

pub(super) fn fallback_evidence_reference_fields(fields: &LogFields) -> LogFields {
    let evidence_reference_ids = fields
        .get(constants::field::LAN_EVIDENCE_REFERENCE_IDS)
        .and_then(|value| match value {
            LogFieldValue::String(value) if !value.is_empty() => Some(value.as_str()),
            _ => None,
        });
    let Some(evidence_reference_ids) = evidence_reference_ids else {
        return LogFields::default();
    };
    let evidence_reference_count = evidence_reference_ids
        .split(constants::delimiter::LIST)
        .filter(|evidence_id| !evidence_id.is_empty())
        .count();
    fields_from_pairs(vec![
        (
            constants::field::LAN_EVIDENCE_REFERENCE_COUNT,
            LogFieldValue::Number(evidence_reference_count as f64),
        ),
        (
            constants::field::LAN_EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(evidence_reference_ids.to_string()),
        ),
    ])
}
