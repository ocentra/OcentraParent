use ocentra_eventing::envelope::DomainEvent;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_family_identity_core::family_identity::{
    authorize_child_device_scope, record_device_scope_decision, ActorAccountState,
    ChildProfileBindingState, DeviceOwnershipScope, DeviceScopeAuthorizationState,
    DeviceScopeEvaluationId, DeviceScopeEvaluationRequestedEvent, DeviceScopeInput,
    FamilyIdentityAggregateId, HouseholdMembershipState, HouseholdRole,
    ParentAuthorityRequirementState,
};

const FAMILY_AGGREGATE_ID: &str = "family-identity-household-default";
const DEVICE_SCOPE_EVALUATION_ID: &str = "family-device-scope-default";
const DEVICE_SCOPE_REQUESTED_EVENT_TYPE: &str = "family-identity.device-scope-evaluation.requested";
const DEVICE_SCOPE_DECISION_EVENT_TYPE: &str = "family-identity.device-scope-decision.recorded";

fn parent_child_device_input(actor_role: HouseholdRole) -> DeviceScopeInput {
    DeviceScopeInput {
        actor_role,
        same_family: true,
        actor_account_state: ActorAccountState::Active,
        membership_state: HouseholdMembershipState::Active,
        child_profile_binding_state: ChildProfileBindingState::Bound,
        device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
    }
}

#[test]
fn parent_household_member_can_target_owned_child_device() {
    let decision =
        authorize_child_device_scope(parent_child_device_input(HouseholdRole::ParentOwner));

    assert_eq!(
        decision.authorization_state,
        DeviceScopeAuthorizationState::Authorized
    );
    assert_eq!(
        decision.parent_authority_requirement_state,
        ParentAuthorityRequirementState::NotRequired
    );
}

#[test]
fn guardian_household_member_can_target_owned_child_device() {
    let decision =
        authorize_child_device_scope(parent_child_device_input(HouseholdRole::CoParentGuardian));

    assert_eq!(
        decision.authorization_state,
        DeviceScopeAuthorizationState::Authorized
    );
    assert_eq!(
        decision.parent_authority_requirement_state,
        ParentAuthorityRequirementState::NotRequired
    );
}

#[test]
fn child_actor_cannot_authorize_tracking_or_enforcement_scope() {
    let decision =
        authorize_child_device_scope(parent_child_device_input(HouseholdRole::ChildProfile));

    assert_eq!(
        decision.authorization_state,
        DeviceScopeAuthorizationState::Rejected
    );
    assert_eq!(
        decision.parent_authority_requirement_state,
        ParentAuthorityRequirementState::Required
    );
}

#[test]
fn parent_cannot_target_device_outside_child_profile_scope() {
    let decision = authorize_child_device_scope(DeviceScopeInput {
        device_ownership_scope: DeviceOwnershipScope::OtherDevice,
        ..parent_child_device_input(HouseholdRole::ParentOwner)
    });

    assert_eq!(
        decision.authorization_state,
        DeviceScopeAuthorizationState::Rejected
    );
    assert_eq!(
        decision.parent_authority_requirement_state,
        ParentAuthorityRequirementState::Required
    );
}

#[test]
fn non_household_actor_cannot_target_child_device() {
    let decision = authorize_child_device_scope(DeviceScopeInput {
        same_family: false,
        ..parent_child_device_input(HouseholdRole::CoParentGuardian)
    });

    assert_eq!(
        decision.authorization_state,
        DeviceScopeAuthorizationState::Rejected
    );
    assert_eq!(
        decision.parent_authority_requirement_state,
        ParentAuthorityRequirementState::Required
    );
}

#[test]
fn suspended_parent_cannot_target_child_device() {
    let decision = authorize_child_device_scope(DeviceScopeInput {
        actor_account_state: ActorAccountState::Suspended,
        ..parent_child_device_input(HouseholdRole::ParentOwner)
    });

    assert_eq!(
        decision.authorization_state,
        DeviceScopeAuthorizationState::Rejected
    );
    assert_eq!(
        decision.parent_authority_requirement_state,
        ParentAuthorityRequirementState::Required
    );
}

#[test]
fn missing_child_profile_binding_rejects_device_scope() {
    let decision = authorize_child_device_scope(DeviceScopeInput {
        child_profile_binding_state: ChildProfileBindingState::Missing,
        ..parent_child_device_input(HouseholdRole::ParentOwner)
    });

    assert_eq!(
        decision.authorization_state,
        DeviceScopeAuthorizationState::Rejected
    );
    assert_eq!(
        decision.parent_authority_requirement_state,
        ParentAuthorityRequirementState::Required
    );
}

#[test]
fn device_scope_request_records_typed_decision_event() {
    let request = DeviceScopeEvaluationRequestedEvent {
        aggregate_id: FamilyIdentityAggregateId::parse(FAMILY_AGGREGATE_ID)
            .expect_value("family aggregate id"),
        evaluation_id: DeviceScopeEvaluationId::parse(DEVICE_SCOPE_EVALUATION_ID)
            .expect_value("device scope evaluation id"),
        input: parent_child_device_input(HouseholdRole::ParentOwner),
    };

    let decision = record_device_scope_decision(&request);

    assert_eq!(decision.aggregate_id, request.aggregate_id);
    assert_eq!(decision.source_evaluation_id, request.evaluation_id);
    assert_eq!(
        decision.decision.authorization_state,
        DeviceScopeAuthorizationState::Authorized
    );
    assert_eq!(
        request
            .contract()
            .expect_value("device scope request contract")
            .event_type
            .as_str(),
        DEVICE_SCOPE_REQUESTED_EVENT_TYPE
    );
    assert_eq!(
        decision
            .contract()
            .expect_value("device scope decision contract")
            .event_type
            .as_str(),
        DEVICE_SCOPE_DECISION_EVENT_TYPE
    );
}
