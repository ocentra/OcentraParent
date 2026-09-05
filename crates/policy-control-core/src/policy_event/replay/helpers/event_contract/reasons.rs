#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;

use crate::policy_event::PolicyEvent;

const POLICY_EVENT_REASON_CODE_FIELD: &str = "policy_event.reason_code";
const MISSING_REASON_PREFIX: &str = "missing reason code for ";
const UNEXPECTED_REASON_PREFIX: &str = "unexpected reason code for ";
const INVALID_REASON_PREFIX: &str = "invalid reason code for ";

pub(super) fn validate_reason_code(event: &PolicyEvent) -> Result<(), EventingError> {
    let expected_reason = event.kind.reason_code_value();
    if super::super::sample::kind_requires_reason(event.kind) {
        return match event.reason_code.as_ref() {
            Some(reason_code) if reason_code.as_str() == expected_reason => Ok(()),
            Some(_) => Err(EventingError::InvalidValue {
                field: POLICY_EVENT_REASON_CODE_FIELD,
                value: format!("{INVALID_REASON_PREFIX}{expected_reason}"),
            }),
            None => Err(EventingError::InvalidValue {
                field: POLICY_EVENT_REASON_CODE_FIELD,
                value: format!("{MISSING_REASON_PREFIX}{expected_reason}"),
            }),
        };
    }

    if event.reason_code.is_some() {
        return Err(EventingError::InvalidValue {
            field: POLICY_EVENT_REASON_CODE_FIELD,
            value: format!("{UNEXPECTED_REASON_PREFIX}{}", event.kind.event_type_name()),
        });
    }
    Ok(())
}
