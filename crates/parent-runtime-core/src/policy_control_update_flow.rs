use ocentra_eventing::error::EventingError;
use ocentra_policy_control_core::policy_authority::{
    resolve_policy_evaluation_request, PolicyDecisionResolvedEvent, PolicyEvaluationRequestedEvent,
};
use ocentra_policy_control_core::policy_delivery::{
    apply_policy_delivery_transition_without_execution_receipt, PolicyDeliveryApplyOutcome,
    PolicyDeliveryAttemptId, PolicyDeliveryRecord, PolicyDeliveryState, PolicyDeliveryTransition,
};
use ocentra_policy_control_core::policy_source::{PolicyAuditReferenceId, PolicyReasonCode};

use crate::policy_control_dispatch::{
    parent_runtime_policy_control_dispatch_evaluated_event_from_origin,
    ParentPolicyControlAcknowledgementState, ParentRuntimePolicyControlDispatchEvaluatedEvent,
    ParentRuntimePolicyControlOriginState, ParentRuntimePolicyControlPublishState,
};

const POLICY_CONTROL_ATTEMPT_DELIVERED_SUFFIX: &str = "delivered";
const POLICY_CONTROL_ATTEMPT_BLOCKED_SUFFIX: &str = "dispatch-blocked";
const POLICY_CONTROL_AUDIT_DELIVERED_SUFFIX: &str = "delivered";
const POLICY_CONTROL_AUDIT_BLOCKED_SUFFIX: &str = "dispatch-blocked";
const POLICY_CONTROL_REASON_DISPATCH_BLOCKED: &str = "parent-runtime-dispatch-blocked";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParentPolicyControlUpdateFlowReport {
    pub policy_evaluation_event: PolicyEvaluationRequestedEvent,
    pub policy_decision_event: PolicyDecisionResolvedEvent,
    pub dispatch_event: ParentRuntimePolicyControlDispatchEvaluatedEvent,
    pub attempted_transitions: Vec<PolicyDeliveryTransition>,
    pub delivery_outcomes: Vec<PolicyDeliveryApplyOutcome>,
    pub final_record: PolicyDeliveryRecord,
}

pub struct ParentPolicyControlUpdateFlow {
    child_acknowledgement_state: ParentPolicyControlAcknowledgementState,
    origin_state: ParentRuntimePolicyControlOriginState,
}

impl ParentPolicyControlUpdateFlow {
    pub fn new(
        child_acknowledgement_state: ParentPolicyControlAcknowledgementState,
        origin_state: ParentRuntimePolicyControlOriginState,
    ) -> Self {
        Self {
            child_acknowledgement_state,
            origin_state,
        }
    }

    pub fn publish_parent_policy_control_delivery(
        &self,
        queued_delivery: &PolicyDeliveryRecord,
        policy_evaluation_event: &PolicyEvaluationRequestedEvent,
        child_runtime_transitions: &[PolicyDeliveryTransition],
    ) -> Result<ParentPolicyControlUpdateFlowReport, EventingError> {
        let policy_decision_event = resolve_policy_evaluation_request(policy_evaluation_event);
        let dispatch_event = parent_runtime_policy_control_dispatch_evaluated_event_from_origin(
            queued_delivery,
            &policy_decision_event,
            self.child_acknowledgement_state,
            self.origin_state,
        );
        let (attempted_transitions, delivery_outcomes) = apply_policy_control_delivery_flow(
            queued_delivery,
            &dispatch_event,
            child_runtime_transitions,
        )?;
        let final_record = delivery_outcomes
            .last()
            .cloned()
            .map(PolicyDeliveryApplyOutcome::into_record)
            .unwrap_or_else(|| queued_delivery.clone());

        Ok(ParentPolicyControlUpdateFlowReport {
            policy_evaluation_event: policy_evaluation_event.clone(),
            policy_decision_event,
            dispatch_event,
            attempted_transitions,
            delivery_outcomes,
            final_record,
        })
    }
}

pub fn publish_parent_policy_control_delivery_event_flow(
    queued_delivery: &PolicyDeliveryRecord,
    policy_evaluation_event: &PolicyEvaluationRequestedEvent,
    child_runtime_transitions: &[PolicyDeliveryTransition],
    child_acknowledgement_state: ParentPolicyControlAcknowledgementState,
    origin_state: ParentRuntimePolicyControlOriginState,
) -> Result<ParentPolicyControlUpdateFlowReport, EventingError> {
    ParentPolicyControlUpdateFlow::new(child_acknowledgement_state, origin_state)
        .publish_parent_policy_control_delivery(
            queued_delivery,
            policy_evaluation_event,
            child_runtime_transitions,
        )
}

fn apply_policy_control_delivery_flow(
    queued_delivery: &PolicyDeliveryRecord,
    dispatch_event: &ParentRuntimePolicyControlDispatchEvaluatedEvent,
    child_runtime_transitions: &[PolicyDeliveryTransition],
) -> Result<
    (
        Vec<PolicyDeliveryTransition>,
        Vec<PolicyDeliveryApplyOutcome>,
    ),
    EventingError,
> {
    let mut current = queued_delivery.clone();
    let mut attempted_transitions = Vec::new();
    let mut delivery_outcomes = Vec::new();

    if dispatch_event.decision.child_runtime_publish_state
        == ParentRuntimePolicyControlPublishState::Publish
    {
        let delivered_transition = delivered_transition(&current, dispatch_event)?;
        let delivered_outcome = apply_policy_delivery_transition_without_execution_receipt(
            &current,
            delivered_transition.clone(),
        )?;
        current = delivered_outcome.clone().into_record();
        attempted_transitions.push(delivered_transition);
        delivery_outcomes.push(delivered_outcome);

        for transition in child_runtime_transitions {
            let outcome = apply_policy_delivery_transition_without_execution_receipt(
                &current,
                transition.clone(),
            )?;
            current = outcome.clone().into_record();
            attempted_transitions.push(transition.clone());
            delivery_outcomes.push(outcome);
        }
    } else {
        let blocked_transition = blocked_transition(&current, dispatch_event)?;
        let blocked_outcome = apply_policy_delivery_transition_without_execution_receipt(
            &current,
            blocked_transition.clone(),
        )?;
        attempted_transitions.push(blocked_transition);
        delivery_outcomes.push(blocked_outcome);
    }

    Ok((attempted_transitions, delivery_outcomes))
}

fn delivered_transition(
    current: &PolicyDeliveryRecord,
    dispatch_event: &ParentRuntimePolicyControlDispatchEvaluatedEvent,
) -> Result<PolicyDeliveryTransition, EventingError> {
    Ok(PolicyDeliveryTransition {
        attempt_id: attempt_id(dispatch_event, POLICY_CONTROL_ATTEMPT_DELIVERED_SUFFIX)?,
        sequence: next_sequence(current)?,
        state: PolicyDeliveryState::Delivered,
        audit_reference_ids: vec![audit_reference_id(
            current,
            POLICY_CONTROL_AUDIT_DELIVERED_SUFFIX,
        )?],
        reason_code: None,
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    })
}

fn blocked_transition(
    current: &PolicyDeliveryRecord,
    dispatch_event: &ParentRuntimePolicyControlDispatchEvaluatedEvent,
) -> Result<PolicyDeliveryTransition, EventingError> {
    Ok(PolicyDeliveryTransition {
        attempt_id: attempt_id(dispatch_event, POLICY_CONTROL_ATTEMPT_BLOCKED_SUFFIX)?,
        sequence: next_sequence(current)?,
        state: PolicyDeliveryState::Rejected,
        audit_reference_ids: vec![audit_reference_id(
            current,
            POLICY_CONTROL_AUDIT_BLOCKED_SUFFIX,
        )?],
        reason_code: Some(PolicyReasonCode::parse(
            POLICY_CONTROL_REASON_DISPATCH_BLOCKED,
        )?),
        superseded_by_policy_version: None,
        rollback_reference_state: None,
    })
}

fn next_sequence(
    current: &PolicyDeliveryRecord,
) -> Result<ocentra_policy_control_core::policy_delivery::PolicyDeliverySequence, EventingError> {
    ocentra_policy_control_core::policy_delivery::PolicyDeliverySequence::new(
        current.last_sequence.value() + 1,
    )
}

fn attempt_id(
    dispatch_event: &ParentRuntimePolicyControlDispatchEvaluatedEvent,
    suffix: &str,
) -> Result<PolicyDeliveryAttemptId, EventingError> {
    PolicyDeliveryAttemptId::parse(format!("{}-{suffix}", dispatch_event.dispatch_id.as_str()))
}

fn audit_reference_id(
    current: &PolicyDeliveryRecord,
    suffix: &str,
) -> Result<PolicyAuditReferenceId, EventingError> {
    PolicyAuditReferenceId::parse(format!("audit-{}-{suffix}", current.delivery_id.as_str()))
}
