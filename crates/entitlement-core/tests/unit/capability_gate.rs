use ocentra_entitlement_core::entitlement_access::{
    evaluate_entitlement_capability, record_entitlement_capability_decision,
    EntitlementAggregateId, EntitlementCapability, EntitlementCapabilityAccessState,
    EntitlementCapabilityEvaluationRequestedEvent, EntitlementCapabilityInput,
    EntitlementCapabilityRejectionReason, EntitlementCapabilityScope, EntitlementEvaluationId,
    EntitlementManualReviewState, EntitlementPolicyState, FamilySetupState, OfflineGraceState,
    SubscriptionState,
};
use ocentra_entitlement_core::entitlement_snapshot::EntitlementSnapshotContext;
use ocentra_entitlement_core::entitlement_snapshot_values::{
    EntitlementDeviceTrustRequirementState, EntitlementDeviceTrustState,
    EntitlementPackageBuildState, EntitlementSnapshotBindingState,
    EntitlementSnapshotFreshnessState, EntitlementSnapshotSignatureState,
};
use ocentra_eventing::envelope::DomainEvent;

fn trusted_snapshot_context() -> EntitlementSnapshotContext {
    EntitlementSnapshotContext {
        signature_state: EntitlementSnapshotSignatureState::Trusted,
        freshness_state: EntitlementSnapshotFreshnessState::Fresh,
        household_binding_state: EntitlementSnapshotBindingState::Matched,
        device_binding_state: EntitlementSnapshotBindingState::Matched,
        device_trust_requirement_state: EntitlementDeviceTrustRequirementState::Required,
        device_trust_state: EntitlementDeviceTrustState::Present,
        package_build_state: EntitlementPackageBuildState::Valid,
    }
}

#[test]
fn active_subscription_allows_capability_after_family_setup() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::Tracking,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
        snapshot_context: trusted_snapshot_context(),
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Allowed
    );
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::NotRequired
    );
    assert_eq!(decision.capability, EntitlementCapability::Tracking);
    assert_eq!(decision.rejection_reason, None);
}

#[test]
fn offline_grace_preserves_child_runtime_capability() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::Enforcement,
        subscription_state: SubscriptionState::Inactive,
        offline_grace_state: OfflineGraceState::Active,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
        snapshot_context: trusted_snapshot_context(),
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Allowed
    );
    assert_eq!(decision.capability, EntitlementCapability::Enforcement);
    assert_eq!(decision.rejection_reason, None);
}

#[test]
fn incomplete_family_setup_blocks_capability_even_with_subscription() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::RemoteAccess,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Incomplete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
        snapshot_context: trusted_snapshot_context(),
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::Required
    );
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::IncompleteFamilySetup)
    );
}

#[test]
fn inactive_subscription_without_grace_blocks_capability_after_family_setup() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::ScreenEvidence,
        subscription_state: SubscriptionState::Inactive,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
        snapshot_context: trusted_snapshot_context(),
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::Required
    );
    assert_eq!(decision.capability, EntitlementCapability::ScreenEvidence);
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::InactiveSubscription)
    );
}

#[test]
fn payment_dispute_blocks_capability_even_during_grace() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::RemoteAccess,
        subscription_state: SubscriptionState::Inactive,
        offline_grace_state: OfflineGraceState::Active,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::PaymentDispute,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
        snapshot_context: trusted_snapshot_context(),
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::Required
    );
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::PaymentDispute)
    );
}

#[test]
fn parent_portal_only_capability_does_not_unlock_child_runtime() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        capability: EntitlementCapability::ScreenEvidence,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::ParentPortalOnly,
        snapshot_context: trusted_snapshot_context(),
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::Required
    );
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::ParentPortalOnlyScope)
    );
}

#[test]
fn wrong_household_snapshot_blocks_capability_even_when_subscription_is_active() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        snapshot_context: EntitlementSnapshotContext {
            household_binding_state: EntitlementSnapshotBindingState::Mismatched,
            ..trusted_snapshot_context()
        },
        capability: EntitlementCapability::Tracking,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::WrongHousehold)
    );
}

#[test]
fn wrong_device_snapshot_blocks_capability_even_when_subscription_is_active() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        snapshot_context: EntitlementSnapshotContext {
            device_binding_state: EntitlementSnapshotBindingState::Mismatched,
            ..trusted_snapshot_context()
        },
        capability: EntitlementCapability::Tracking,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::WrongDevice)
    );
}

#[test]
fn missing_device_trust_blocks_when_snapshot_requires_sealed_device_trust() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        snapshot_context: EntitlementSnapshotContext {
            device_trust_state: EntitlementDeviceTrustState::Missing,
            ..trusted_snapshot_context()
        },
        capability: EntitlementCapability::RemoteAccess,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::MissingDeviceTrust)
    );
}

#[test]
fn invalid_signature_blocks_capability_before_subscription_state_is_considered() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        snapshot_context: EntitlementSnapshotContext {
            signature_state: EntitlementSnapshotSignatureState::Invalid,
            ..trusted_snapshot_context()
        },
        capability: EntitlementCapability::Tracking,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Active,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::InvalidSignature)
    );
}

#[test]
fn entitlement_evaluation_request_records_typed_decision_event() -> Result<(), EventingError> {
    let request = EntitlementCapabilityEvaluationRequestedEvent {
        aggregate_id: EntitlementAggregateId::parse("entitlement-family-default")?,
        evaluation_id: EntitlementEvaluationId::parse("entitlement-evaluation-default")?,
        input: EntitlementCapabilityInput {
            capability: EntitlementCapability::Tracking,
            subscription_state: SubscriptionState::Active,
            offline_grace_state: OfflineGraceState::Inactive,
            family_setup_state: FamilySetupState::Complete,
            policy_state: EntitlementPolicyState::Clean,
            capability_scope: EntitlementCapabilityScope::LocalChildRuntime,
            snapshot_context: trusted_snapshot_context(),
        },
    };

    let decision = record_entitlement_capability_decision(&request);

    assert_eq!(decision.aggregate_id, request.aggregate_id);
    assert_eq!(decision.source_evaluation_id, request.evaluation_id);
    assert_eq!(
        decision.decision.capability,
        EntitlementCapability::Tracking
    );
    assert_eq!(
        decision.decision.access_state,
        EntitlementCapabilityAccessState::Allowed
    );
    assert_eq!(decision.decision.rejection_reason, None);
    assert_eq!(
        request.contract()?.event_type.as_str(),
        "entitlement.capability-evaluation.requested"
    );
    assert_eq!(
        decision.contract()?.event_type.as_str(),
        "entitlement.capability-decision.recorded"
    );

    Ok(())
}
use ocentra_eventing::error::EventingError;
