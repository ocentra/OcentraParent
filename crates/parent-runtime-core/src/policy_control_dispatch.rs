use ocentra_eventing::{
    envelope::DomainEvent, envelope::EventContract, error::EventingError, ids::AggregateKey,
    ids::EventType, ids::IdempotencyKey, ids::SchemaVersion,
};
use ocentra_policy_control_core::policy_authority::{
    PolicyActionAuthorizationState, PolicyConflictResolutionState, PolicyDecisionResolvedEvent,
    PolicyEnforcementExecutionState, PolicyManualReviewState,
};
use ocentra_policy_control_core::policy_delivery::{
    PolicyDeliveryParentVisibleState, PolicyDeliveryRecord, PolicyDeliveryState,
};
use serde::{Deserialize, Serialize};

#[path = "policy_control_dispatch/helpers.rs"]
mod helpers;
use self::helpers::{
    parent_runtime_policy_control_dispatch_ref, parent_visible_state_when_dispatch_is_blocked,
    parent_visible_state_while_dispatching,
};

const PARENT_RUNTIME_POLICY_CONTROL_SCHEMA_VERSION: u16 = 1;
pub const PARENT_RUNTIME_POLICY_CONTROL_DISPATCH_EVALUATED_EVENT_TYPE: &str =
    "parent-runtime.policy-control-dispatch.evaluated";
const PARENT_RUNTIME_POLICY_CONTROL_IDEMPOTENCY_SEPARATOR: &str = ":";
const PARENT_RUNTIME_POLICY_CONTROL_DISPATCH_PREFIX: &str =
    "parent-runtime-policy-control-dispatch:";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentPolicyControlAcknowledgementState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentRuntimePolicyControlOriginState {
    #[serde(rename = "trusted-local-ui")]
    TrustedLocalUi,
    #[serde(rename = "untrusted")]
    Untrusted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentRuntimePolicyControlPublishState {
    #[serde(rename = "publish")]
    Publish,
    #[serde(rename = "do-not-publish")]
    DoNotPublish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentRuntimePolicyControlAuditRetentionState {
    #[serde(rename = "retain")]
    Retain,
    #[serde(rename = "do-not-retain")]
    DoNotRetain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentRuntimePolicyControlWaitState {
    #[serde(rename = "await")]
    Await,
    #[serde(rename = "do-not-await")]
    DoNotAwait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentRuntimePolicyControlDispatchRequest {
    pub current_delivery_state: PolicyDeliveryState,
    pub current_parent_visible_state: PolicyDeliveryParentVisibleState,
    pub origin_state: ParentRuntimePolicyControlOriginState,
    pub child_acknowledgement_state: ParentPolicyControlAcknowledgementState,
    pub action_authorization_state: PolicyActionAuthorizationState,
    pub enforcement_execution_state: PolicyEnforcementExecutionState,
    pub decision_manual_review_state: PolicyManualReviewState,
    pub conflict_resolution_state: PolicyConflictResolutionState,
    pub conflict_manual_review_state: PolicyManualReviewState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentRuntimePolicyControlDispatchDecision {
    pub child_runtime_publish_state: ParentRuntimePolicyControlPublishState,
    pub parent_audit_retention_state: ParentRuntimePolicyControlAuditRetentionState,
    pub child_acknowledgement_wait_state: ParentRuntimePolicyControlWaitState,
    pub parent_visible_state: PolicyDeliveryParentVisibleState,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct ParentRuntimePolicyControlDispatchId(String);

impl ParentRuntimePolicyControlDispatchId {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: "parent_runtime.policy_control.dispatch_id",
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for ParentRuntimePolicyControlDispatchId {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<ParentRuntimePolicyControlDispatchId> for String {
    fn from(value: ParentRuntimePolicyControlDispatchId) -> Self {
        value.0
    }
}

impl std::fmt::Display for ParentRuntimePolicyControlDispatchId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParentRuntimePolicyControlDispatchEvaluatedEvent {
    pub dispatch_id: ParentRuntimePolicyControlDispatchId,
    pub source_delivery: PolicyDeliveryRecord,
    pub source_decision: PolicyDecisionResolvedEvent,
    pub child_acknowledgement_state: ParentPolicyControlAcknowledgementState,
    pub decision: ParentRuntimePolicyControlDispatchDecision,
}

impl DomainEvent for ParentRuntimePolicyControlDispatchEvaluatedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(PARENT_RUNTIME_POLICY_CONTROL_DISPATCH_EVALUATED_EVENT_TYPE)?,
            SchemaVersion::new(PARENT_RUNTIME_POLICY_CONTROL_SCHEMA_VERSION)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(self.source_decision.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(format!(
            "{}{}{}",
            PARENT_RUNTIME_POLICY_CONTROL_DISPATCH_EVALUATED_EVENT_TYPE,
            PARENT_RUNTIME_POLICY_CONTROL_IDEMPOTENCY_SEPARATOR,
            self.dispatch_id
        ))
    }
}

pub fn route_parent_runtime_policy_control_dispatch(
    request: ParentRuntimePolicyControlDispatchRequest,
) -> ParentRuntimePolicyControlDispatchDecision {
    let trusted_origin =
        request.origin_state == ParentRuntimePolicyControlOriginState::TrustedLocalUi;
    let delivery_can_progress = matches!(
        request.current_delivery_state,
        PolicyDeliveryState::Queued | PolicyDeliveryState::Degraded | PolicyDeliveryState::Offline
    );
    let decision_allows_publish = request.action_authorization_state
        == PolicyActionAuthorizationState::Authorized
        && request.enforcement_execution_state == PolicyEnforcementExecutionState::MayExecute
        && request.decision_manual_review_state == PolicyManualReviewState::NotRequired
        && request.conflict_resolution_state == PolicyConflictResolutionState::UseParentPolicy
        && request.conflict_manual_review_state == PolicyManualReviewState::NotRequired;
    let publish_to_child_runtime =
        trusted_origin && delivery_can_progress && decision_allows_publish;

    ParentRuntimePolicyControlDispatchDecision {
        child_runtime_publish_state: if publish_to_child_runtime {
            ParentRuntimePolicyControlPublishState::Publish
        } else {
            ParentRuntimePolicyControlPublishState::DoNotPublish
        },
        parent_audit_retention_state: ParentRuntimePolicyControlAuditRetentionState::Retain,
        child_acknowledgement_wait_state: if publish_to_child_runtime
            && request.child_acknowledgement_state
                == ParentPolicyControlAcknowledgementState::Required
        {
            ParentRuntimePolicyControlWaitState::Await
        } else {
            ParentRuntimePolicyControlWaitState::DoNotAwait
        },
        parent_visible_state: if publish_to_child_runtime {
            parent_visible_state_while_dispatching(request.current_delivery_state)
        } else {
            parent_visible_state_when_dispatch_is_blocked(
                request.current_delivery_state,
                request.current_parent_visible_state,
            )
        },
    }
}

pub fn route_parent_policy_control_delivery(
    delivery: &PolicyDeliveryRecord,
    decision_event: &PolicyDecisionResolvedEvent,
    child_acknowledgement_state: ParentPolicyControlAcknowledgementState,
) -> ParentRuntimePolicyControlDispatchDecision {
    route_parent_policy_control_delivery_from_origin(
        delivery,
        decision_event,
        child_acknowledgement_state,
        ParentRuntimePolicyControlOriginState::TrustedLocalUi,
    )
}

pub fn parent_runtime_policy_control_dispatch_evaluated_event(
    delivery: &PolicyDeliveryRecord,
    decision_event: &PolicyDecisionResolvedEvent,
    child_acknowledgement_state: ParentPolicyControlAcknowledgementState,
) -> ParentRuntimePolicyControlDispatchEvaluatedEvent {
    parent_runtime_policy_control_dispatch_evaluated_event_from_origin(
        delivery,
        decision_event,
        child_acknowledgement_state,
        ParentRuntimePolicyControlOriginState::TrustedLocalUi,
    )
}

pub fn parent_runtime_policy_control_dispatch_evaluated_event_from_origin(
    delivery: &PolicyDeliveryRecord,
    decision_event: &PolicyDecisionResolvedEvent,
    child_acknowledgement_state: ParentPolicyControlAcknowledgementState,
    origin_state: ParentRuntimePolicyControlOriginState,
) -> ParentRuntimePolicyControlDispatchEvaluatedEvent {
    ParentRuntimePolicyControlDispatchEvaluatedEvent {
        dispatch_id: ParentRuntimePolicyControlDispatchId(
            parent_runtime_policy_control_dispatch_ref(delivery, decision_event),
        ),
        source_delivery: delivery.clone(),
        source_decision: decision_event.clone(),
        child_acknowledgement_state,
        decision: route_parent_policy_control_delivery_from_origin(
            delivery,
            decision_event,
            child_acknowledgement_state,
            origin_state,
        ),
    }
}

pub fn route_parent_policy_control_delivery_from_origin(
    delivery: &PolicyDeliveryRecord,
    decision_event: &PolicyDecisionResolvedEvent,
    child_acknowledgement_state: ParentPolicyControlAcknowledgementState,
    origin_state: ParentRuntimePolicyControlOriginState,
) -> ParentRuntimePolicyControlDispatchDecision {
    route_parent_runtime_policy_control_dispatch(ParentRuntimePolicyControlDispatchRequest {
        current_delivery_state: delivery.state,
        current_parent_visible_state: delivery.parent_visible_state(),
        origin_state,
        child_acknowledgement_state,
        action_authorization_state: decision_event.decision.action_authorization_state,
        enforcement_execution_state: decision_event.decision.enforcement_execution_state,
        decision_manual_review_state: decision_event.decision.manual_review_state,
        conflict_resolution_state: decision_event.conflict_decision.resolution_state,
        conflict_manual_review_state: decision_event.conflict_decision.manual_review_state,
    })
}
