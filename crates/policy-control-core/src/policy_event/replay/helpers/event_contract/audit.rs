#![forbid(unsafe_code)]

use std::collections::BTreeSet;

use ocentra_eventing::error::EventingError;

use crate::policy_event::PolicyEvent;

const POLICY_EVENT_AUDIT_REFERENCES_FIELD: &str = "policy_event.audit_reference_ids";
const MISSING_AUDIT_REFERENCES: &str = "missing audit references";
const DUPLICATE_AUDIT_REFERENCE: &str = "duplicate audit reference";

pub(super) fn validate(event: &PolicyEvent) -> Result<(), EventingError> {
    if event.audit_reference_ids.is_empty() {
        return Err(EventingError::InvalidValue {
            field: POLICY_EVENT_AUDIT_REFERENCES_FIELD,
            value: MISSING_AUDIT_REFERENCES.to_string(),
        });
    }

    let mut seen = BTreeSet::new();
    for audit_reference_id in &event.audit_reference_ids {
        if !seen.insert(audit_reference_id) {
            return Err(EventingError::InvalidValue {
                field: POLICY_EVENT_AUDIT_REFERENCES_FIELD,
                value: DUPLICATE_AUDIT_REFERENCE.to_string(),
            });
        }
    }

    Ok(())
}
