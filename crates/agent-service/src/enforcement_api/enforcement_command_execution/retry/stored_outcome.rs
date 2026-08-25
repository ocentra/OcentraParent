use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome;
use ocentra_parent_agent_protocol::{
    constants,
    enforcement::{
        EnforcementAction, EnforcementAuditEvent, EnforcementResult, EnforcementTimerEvent,
    },
    logging::{LogFieldValue, LogFields},
};
use serde::de::DeserializeOwned;

use super::{recovery_reconciliation_error, EnforcementRetryRecoveryError};
use crate::enforcement_payload::EnforcementFieldKey;

pub(super) fn outcome_from_fields(
    fields: &LogFields,
) -> Result<EnforcementBoundaryOutcome, EnforcementRetryRecoveryError> {
    let action = required_json::<EnforcementAction>(
        fields,
        EnforcementFieldKey(constants::field::ENFORCEMENT_ACTION),
    )?;
    let result = required_json::<EnforcementResult>(
        fields,
        EnforcementFieldKey(constants::field::ENFORCEMENT_RESULT),
    )?;
    let audit_event = required_json::<EnforcementAuditEvent>(
        fields,
        EnforcementFieldKey(constants::field::ENFORCEMENT_AUDIT_EVENT),
    )?;
    let timer_event = optional_json::<EnforcementTimerEvent>(
        fields,
        EnforcementFieldKey(constants::field::ENFORCEMENT_TIMER_EVENT),
    )?;
    if audit_event.action != action || audit_event.result != result {
        return Err(EnforcementRetryRecoveryError::ReconciliationRequired);
    }
    if timer_event.as_ref().is_some_and(|timer| {
        timer.action_id != action.action_id
            || timer.policy_decision_id != action.policy_decision_id
            || timer.evidence_references != action.evidence_references
    }) {
        return Err(EnforcementRetryRecoveryError::ReconciliationRequired);
    }
    Ok(EnforcementBoundaryOutcome {
        action,
        result,
        audit_event,
        timer_event,
        adapter_request: None,
    })
}

fn required_json<T: DeserializeOwned>(
    fields: &LogFields,
    key: EnforcementFieldKey,
) -> Result<T, EnforcementRetryRecoveryError> {
    match fields.get(key.0) {
        Some(LogFieldValue::String(value)) => {
            serde_json::from_str(value).map_err(recovery_reconciliation_error)
        }
        _ => Err(EnforcementRetryRecoveryError::ReconciliationRequired),
    }
}

fn optional_json<T: DeserializeOwned>(
    fields: &LogFields,
    key: EnforcementFieldKey,
) -> Result<Option<T>, EnforcementRetryRecoveryError> {
    match fields.get(key.0) {
        Some(LogFieldValue::String(value)) => serde_json::from_str(value)
            .map(Some)
            .map_err(recovery_reconciliation_error),
        Some(LogFieldValue::Null(())) => Ok(None),
        _ => Err(EnforcementRetryRecoveryError::ReconciliationRequired),
    }
}
