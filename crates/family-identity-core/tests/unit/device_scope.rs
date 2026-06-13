use ocentra_eventing::DomainEvent;
use ocentra_family_identity_core::{
    authorize_child_device_scope, record_device_scope_decision, ActorAccountState,
    ChildProfileBindingState, DeviceOwnershipScope, DeviceScopeAuthorizationState,
    DeviceScopeEvaluationId, DeviceScopeEvaluationRequestedEvent, DeviceScopeInput,
    FamilyActorRole, FamilyIdentityAggregateId, HouseholdMembership,
    ParentAuthorityRequirementState,
};

#[test]
fn parent_household_member_can_target_owned_child_device() {
    let decision = authorize_child_device_scope(DeviceScopeInput {
        actor_role: FamilyActorRole::Parent,
        actor_account_state: ActorAccountState::Active,
        household_membership: HouseholdMembership::Member,
        child_profile_binding_state: ChildProfileBindingState::Bound,
        device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
    });

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
    let decision = authorize_child_device_scope(DeviceScopeInput {
        actor_role: FamilyActorRole::Guardian,
        actor_account_state: ActorAccountState::Active,
        household_membership: HouseholdMembership::Member,
        child_profile_binding_state: ChildProfileBindingState::Bound,
        device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
    });

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
    let decision = authorize_child_device_scope(DeviceScopeInput {
        actor_role: FamilyActorRole::Child,
        actor_account_state: ActorAccountState::Active,
        household_membership: HouseholdMembership::Member,
        child_profile_binding_state: ChildProfileBindingState::Bound,
        device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
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
fn parent_cannot_target_device_outside_child_profile_scope() {
    let decision = authorize_child_device_scope(DeviceScopeInput {
        actor_role: FamilyActorRole::Parent,
        actor_account_state: ActorAccountState::Active,
        household_membership: HouseholdMembership::Member,
        child_profile_binding_state: ChildProfileBindingState::Bound,
        device_ownership_scope: DeviceOwnershipScope::OtherDevice,
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
        actor_role: FamilyActorRole::Guardian,
        actor_account_state: ActorAccountState::Active,
        household_membership: HouseholdMembership::External,
        child_profile_binding_state: ChildProfileBindingState::Bound,
        device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
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
        actor_role: FamilyActorRole::Parent,
        actor_account_state: ActorAccountState::Suspended,
        household_membership: HouseholdMembership::Member,
        child_profile_binding_state: ChildProfileBindingState::Bound,
        device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
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
        actor_role: FamilyActorRole::Parent,
        actor_account_state: ActorAccountState::Active,
        household_membership: HouseholdMembership::Member,
        child_profile_binding_state: ChildProfileBindingState::Missing,
        device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
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
fn device_scope_evaluation_request_records_typed_decision_event() {
    let request = DeviceScopeEvaluationRequestedEvent {
        aggregate_id: FamilyIdentityAggregateId::parse("family-identity-household-default")
            .expect("family identity aggregate"),
        evaluation_id: DeviceScopeEvaluationId::parse("family-identity-evaluation-default")
            .expect("family identity evaluation"),
        input: DeviceScopeInput {
            actor_role: FamilyActorRole::Parent,
            actor_account_state: ActorAccountState::Active,
            household_membership: HouseholdMembership::Member,
            child_profile_binding_state: ChildProfileBindingState::Bound,
            device_ownership_scope: DeviceOwnershipScope::ChildProfileDevice,
        },
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
            .expect("family identity request contract")
            .event_type
            .as_str(),
        "family-identity.device-scope-evaluation.requested"
    );
    assert_eq!(
        decision
            .contract()
            .expect("family identity decision contract")
            .event_type
            .as_str(),
        "family-identity.device-scope-decision.recorded"
    );
}
