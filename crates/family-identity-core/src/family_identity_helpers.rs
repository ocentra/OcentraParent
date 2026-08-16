use crate::family_identity::{
    DeviceScopeAuthorizationState, DeviceScopeDecisionRecordedEvent, DeviceScopeEvaluationId,
    DeviceScopeEvaluationRequestedEvent, DeviceScopeInput, ParentAuthorityRequirementState,
};
use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::envelope::EventContract;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{EventType, IdempotencyKey, SchemaVersion};

pub(crate) fn device_scope_is_allowed(input: &DeviceScopeInput) -> bool {
    matches!(
        input.actor_role,
        crate::family_identity::HouseholdRole::ParentOwner
            | crate::family_identity::HouseholdRole::CoParentGuardian
    ) && input.same_family
        && input.membership_state == crate::family_identity::HouseholdMembershipState::Active
        && input.actor_account_state == crate::family_identity::ActorAccountState::Active
        && input.child_profile_binding_state
            == crate::family_identity::ChildProfileBindingState::Bound
        && input.device_ownership_scope
            == crate::family_identity::DeviceOwnershipScope::ChildProfileDevice
}

pub(crate) fn device_scope_authorization_state(allowed: bool) -> DeviceScopeAuthorizationState {
    if allowed {
        DeviceScopeAuthorizationState::Authorized
    } else {
        DeviceScopeAuthorizationState::Rejected
    }
}

pub(crate) fn parent_authority_requirement_state(allowed: bool) -> ParentAuthorityRequirementState {
    if allowed {
        ParentAuthorityRequirementState::NotRequired
    } else {
        ParentAuthorityRequirementState::Required
    }
}

pub(crate) fn family_identity_event_contract(
    event_type: &str,
) -> Result<EventContract, EventingError> {
    Ok(EventContract::new(
        EventType::parse(event_type)?,
        SchemaVersion::new(1)?,
    ))
}

pub(crate) fn family_identity_idempotency_key(
    event_type: &str,
    unique_ref: impl std::fmt::Display,
) -> Result<IdempotencyKey, EventingError> {
    IdempotencyKey::parse(format!("{}{}{}", event_type, ":", unique_ref))
}

pub(crate) fn device_scope_decision_ref(evaluation_id: &DeviceScopeEvaluationId) -> String {
    let mut value = String::from("family-identity-device-scope-decision:");
    value.push_str(evaluation_id.as_str());
    value
}

impl DomainEvent for DeviceScopeEvaluationRequestedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        family_identity_event_contract("family-identity.device-scope-evaluation.requested")
    }

    fn aggregate_key(&self) -> Result<ocentra_eventing::ids::AggregateKey, EventingError> {
        ocentra_eventing::ids::AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        family_identity_idempotency_key(
            "family-identity.device-scope-evaluation.requested",
            &self.evaluation_id,
        )
    }
}

impl DomainEvent for DeviceScopeDecisionRecordedEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        family_identity_event_contract("family-identity.device-scope-decision.recorded")
    }

    fn aggregate_key(&self) -> Result<ocentra_eventing::ids::AggregateKey, EventingError> {
        ocentra_eventing::ids::AggregateKey::parse(self.aggregate_id.as_str())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        family_identity_idempotency_key(
            "family-identity.device-scope-decision.recorded",
            &self.decision_id,
        )
    }
}
