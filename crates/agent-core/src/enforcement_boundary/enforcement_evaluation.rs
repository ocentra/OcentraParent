use super::enforcement_action::enforcement_action;
use super::enforcement_adapter_request::adapter_request;
use super::enforcement_audit::enforcement_audit_event;
use super::enforcement_mode::enforcement_mode;
use super::enforcement_result::enforcement_result;
use super::enforcement_timer_event::timer_event;
use super::enforcement_validation::validate_intent_decision;
use super::{EnforcementBoundaryInput, EnforcementBoundaryOutcome, EnforcementBoundaryRejection};

pub(super) fn evaluate_enforcement_boundary(
    input: impl Borrow<EnforcementBoundaryInput>,
) -> Result<EnforcementBoundaryOutcome, EnforcementBoundaryRejection> {
    let input = input.borrow();
    validate_intent_decision(&input.intent, &input.decision)?;
    let mode = enforcement_mode(&input.intent)?;
    let action = enforcement_action(input, mode);
    let result = enforcement_result(input, &action)?;
    let timer_event = timer_event(input, &action, &result);
    let audit_event = enforcement_audit_event(input, &action, &result);
    let adapter_request = adapter_request(&action, &result);

    Ok(EnforcementBoundaryOutcome {
        action,
        result,
        audit_event,
        timer_event,
        adapter_request,
    })
}
use std::borrow::Borrow;
