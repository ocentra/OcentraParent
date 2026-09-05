use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryOutcome;
use ocentra_parent_agent_protocol::{
    constants,
    enforcement::EnforcementActiveTimerState,
    logging::{LogFieldValue, LogFields},
};

use super::EnforcementRetryRecoveryError;
use crate::enforcement_payload::EnforcementFieldKey;

pub(super) fn validate_complete_report_payload(
    fields: &LogFields,
    outcome: &EnforcementBoundaryOutcome,
    active_state: Option<&EnforcementActiveTimerState>,
) -> Result<(), EnforcementRetryRecoveryError> {
    let valid = fields.get(constants::field::DATABASE_READY) == Some(&LogFieldValue::Boolean(true))
        && number(
            fields,
            EnforcementFieldKey(constants::field::EVENTS_INGESTED),
        )
        .is_some()
        && number(fields, EnforcementFieldKey(constants::field::EVENTS_STORED))
            .is_some_and(|count| count >= 1)
        && report_event_id_matches(fields, outcome)
        && timer_fields_match(fields, outcome, active_state);
    valid
        .then_some(())
        .ok_or(EnforcementRetryRecoveryError::ReconciliationRequired)
}

fn report_event_id_matches(fields: &LogFields, outcome: &EnforcementBoundaryOutcome) -> bool {
    fields.get(constants::field::ENFORCEMENT_JOURNAL_EVENT_ID)
        == Some(&LogFieldValue::String(
            outcome.audit_event.audit_event_id.clone(),
        ))
}

fn timer_fields_match(
    fields: &LogFields,
    outcome: &EnforcementBoundaryOutcome,
    active_state: Option<&EnforcementActiveTimerState>,
) -> bool {
    let timer = match &outcome.timer_event {
        Some(timer) => {
            fields.get(constants::field::ENFORCEMENT_TIMER_EVENT_ID)
                == Some(&LogFieldValue::String(timer.timer_event_id.clone()))
                && fields.get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND)
                    == Some(&LogFieldValue::String(
                        timer.timer_event_kind.as_protocol_str().to_string(),
                    ))
        }
        None => {
            fields
                .get(constants::field::ENFORCEMENT_TIMER_EVENT_ID)
                .is_none()
                && fields
                    .get(constants::field::ENFORCEMENT_TIMER_EVENT_KIND)
                    .is_none()
        }
    };
    timer && active_state_field_matches(fields, active_state)
}

fn active_state_field_matches(
    fields: &LogFields,
    active_state: Option<&EnforcementActiveTimerState>,
) -> bool {
    match (
        fields.get(constants::field::ENFORCEMENT_TIMER_STATE),
        active_state,
    ) {
        (Some(LogFieldValue::String(value)), Some(expected)) => {
            serde_json::from_str::<EnforcementActiveTimerState>(value)
                .is_ok_and(|stored| stored == *expected)
        }
        (Some(LogFieldValue::Null(())), None) => true,
        _ => false,
    }
}

fn number(fields: &LogFields, key: EnforcementFieldKey) -> Option<u64> {
    match fields.get(key.0) {
        Some(LogFieldValue::Number(value))
            if value.is_finite()
                && *value >= 0.0
                && value.fract() == 0.0
                && *value <= u64::MAX as f64 =>
        {
            Some(*value as u64)
        }
        _ => None,
    }
}
