use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use super::AppGameDispatchEvidenceRejection;

pub(super) struct AppGameDispatchEvidencePayload {
    pub(super) runtime_evidence_id: RuntimeEvidenceId,
    pub(super) process_id: u32,
    pub(super) target_value: TargetValue,
}

pub(super) struct RuntimeEvidenceId(pub(super) String);

pub(super) struct TargetValue(pub(super) String);

struct EvidenceReferenceIds(String);

impl AppGameDispatchEvidencePayload {
    pub(super) fn parse(fields: &LogFields) -> Result<Self, AppGameDispatchEvidenceRejection> {
        let runtime_evidence_id = required_runtime_evidence_id(fields)?;
        let process_id = required_process_id(fields)?;
        let target_value = required_target_value(fields)?;
        let evidence_reference_ids = required_evidence_reference_ids(fields)?;
        evidence_reference_ids
            .0
            .split(',')
            .map(str::trim)
            .any(|value| value == runtime_evidence_id.0)
            .then_some(Self {
                runtime_evidence_id,
                process_id,
                target_value,
            })
            .ok_or(AppGameDispatchEvidenceRejection::Mismatch)
    }
}

fn required_runtime_evidence_id(
    fields: &LogFields,
) -> Result<RuntimeEvidenceId, AppGameDispatchEvidenceRejection> {
    match fields.get(constants::field::APP_GAME_RUNTIME_EVIDENCE_ID) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Ok(RuntimeEvidenceId(value.trim().to_string()))
        }
        _ => Err(AppGameDispatchEvidenceRejection::Required),
    }
}

fn required_target_value(
    fields: &LogFields,
) -> Result<TargetValue, AppGameDispatchEvidenceRejection> {
    match fields.get(constants::field::POLICY_TARGET_VALUE) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Ok(TargetValue(value.trim().to_string()))
        }
        _ => Err(AppGameDispatchEvidenceRejection::Required),
    }
}

fn required_evidence_reference_ids(
    fields: &LogFields,
) -> Result<EvidenceReferenceIds, AppGameDispatchEvidenceRejection> {
    match fields.get(constants::field::EVIDENCE_REFERENCE_IDS) {
        Some(LogFieldValue::String(value)) if !value.trim().is_empty() => {
            Ok(EvidenceReferenceIds(value.trim().to_string()))
        }
        _ => Err(AppGameDispatchEvidenceRejection::Required),
    }
}

fn required_process_id(fields: &LogFields) -> Result<u32, AppGameDispatchEvidenceRejection> {
    match fields.get(constants::field::PROCESS_ID) {
        Some(LogFieldValue::Number(value)) if *value >= 0.0 && *value <= f64::from(u32::MAX) => {
            Ok(*value as u32)
        }
        _ => Err(AppGameDispatchEvidenceRejection::Required),
    }
}
