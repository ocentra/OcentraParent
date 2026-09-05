#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use crate::policy_event::{PolicyEvent, PolicyEventKind};

const POLICY_EVENT_DEAD_LETTER_REASON_FIELD: &str = "policy_event.dead_letter_reason";
const DEAD_LETTER_REASON_REQUIRED: &str = "dead-letter reason required";
const DEAD_LETTER_REASON_UNEXPECTED: &str =
    "dead-letter reason only valid for policy.dead-letter.recorded";

pub(super) fn validate(event: &PolicyEvent) -> Result<(), EventingError> {
    if matches!(event.kind, PolicyEventKind::DeadLetterRecorded) {
        if event.dead_letter_reason.is_none() {
            return Err(EventingError::InvalidValue {
                field: POLICY_EVENT_DEAD_LETTER_REASON_FIELD,
                value: DEAD_LETTER_REASON_REQUIRED.to_string(),
            });
        }
    } else if event.dead_letter_reason.is_some() {
        return Err(EventingError::InvalidValue {
            field: POLICY_EVENT_DEAD_LETTER_REASON_FIELD,
            value: DEAD_LETTER_REASON_UNEXPECTED.to_string(),
        });
    }
    Ok(())
}
