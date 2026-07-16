use ocentra_entitlement_core::entitlement_access::{
    evaluate_entitlement_capability, record_entitlement_capability_decision,
    EntitlementAggregateId, EntitlementCapability, EntitlementCapabilityAccessState,
    EntitlementCapabilityEvaluationRequestedEvent, EntitlementCapabilityInput,
    EntitlementCapabilityRejectionReason, EntitlementCapabilityScope, EntitlementDecisionId,
    EntitlementEvaluationId, EntitlementManualReviewState, EntitlementPolicyState,
    FamilySetupState, OfflineGraceState, SubscriptionState,
};
use ocentra_entitlement_core::entitlement_snapshot::EntitlementSnapshotContext;
use ocentra_entitlement_core::entitlement_snapshot_values::{
    EntitlementDeviceTrustRequirementState, EntitlementDeviceTrustState,
    EntitlementPackageBuildState, EntitlementSnapshotBindingState,
    EntitlementSnapshotFreshnessState, EntitlementSnapshotSignatureState,
};
use ocentra_eventing::envelope::DomainEvent;

const ENTITLEMENT_AGGREGATE_ID: &str = "entitlement-household-default";
const ENTITLEMENT_EVALUATION_ID: &str = "entitlement-evaluation-default";
const ENTITLEMENT_REQUESTED_EVENT_TYPE: &str = "entitlement.capability-evaluation.requested";
const ENTITLEMENT_DECISION_EVENT_TYPE: &str = "entitlement.capability-decision.recorded";

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

fn entitlement_input(capability_scope: EntitlementCapabilityScope) -> EntitlementCapabilityInput {
    EntitlementCapabilityInput {
        capability: EntitlementCapability::Tracking,
        subscription_state: SubscriptionState::Active,
        offline_grace_state: OfflineGraceState::Inactive,
        family_setup_state: FamilySetupState::Complete,
        policy_state: EntitlementPolicyState::Clean,
        capability_scope,
        snapshot_context: trusted_snapshot_context(),
    }
}

#[test]
fn local_child_runtime_capability_is_allowed_for_active_clean_family() {
    let decision = evaluate_entitlement_capability(entitlement_input(
        EntitlementCapabilityScope::LocalChildRuntime,
    ));

    assert_eq!(decision.capability, EntitlementCapability::Tracking);
    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Allowed
    );
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::NotRequired
    );
    assert_eq!(decision.rejection_reason, None);
}

#[test]
fn parent_portal_only_scope_is_blocked_for_child_runtime_capability() {
    let decision = evaluate_entitlement_capability(entitlement_input(
        EntitlementCapabilityScope::ParentPortalOnly,
    ));

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
fn offline_grace_still_allows_local_child_runtime_capability() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        offline_grace_state: OfflineGraceState::Active,
        subscription_state: SubscriptionState::Inactive,
        capability: EntitlementCapability::Enforcement,
        ..entitlement_input(EntitlementCapabilityScope::LocalChildRuntime)
    });

    assert_eq!(decision.capability, EntitlementCapability::Enforcement);
    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Allowed
    );
    assert_eq!(
        decision.manual_review_state,
        EntitlementManualReviewState::NotRequired
    );
    assert_eq!(decision.rejection_reason, None);
}

#[test]
fn payment_dispute_blocks_even_active_local_child_runtime_capability() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        policy_state: EntitlementPolicyState::PaymentDispute,
        ..entitlement_input(EntitlementCapabilityScope::LocalChildRuntime)
    });

    assert_eq!(decision.capability, EntitlementCapability::Tracking);
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
fn incomplete_family_setup_blocks_even_with_offline_grace() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        family_setup_state: FamilySetupState::Incomplete,
        offline_grace_state: OfflineGraceState::Active,
        subscription_state: SubscriptionState::Inactive,
        capability: EntitlementCapability::RemoteAccess,
        ..entitlement_input(EntitlementCapabilityScope::LocalChildRuntime)
    });

    assert_eq!(decision.capability, EntitlementCapability::RemoteAccess);
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
fn stale_snapshot_blocks_local_child_runtime_capability() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        snapshot_context: EntitlementSnapshotContext {
            freshness_state: EntitlementSnapshotFreshnessState::Stale,
            ..trusted_snapshot_context()
        },
        ..entitlement_input(EntitlementCapabilityScope::LocalChildRuntime)
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
        Some(EntitlementCapabilityRejectionReason::StaleSnapshot)
    );
}

#[test]
fn revoked_snapshot_blocks_local_child_runtime_capability() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        snapshot_context: EntitlementSnapshotContext {
            freshness_state: EntitlementSnapshotFreshnessState::Revoked,
            ..trusted_snapshot_context()
        },
        ..entitlement_input(EntitlementCapabilityScope::LocalChildRuntime)
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::RevokedSnapshot)
    );
}

#[test]
fn invalid_package_build_blocks_local_child_runtime_capability() {
    let decision = evaluate_entitlement_capability(EntitlementCapabilityInput {
        snapshot_context: EntitlementSnapshotContext {
            package_build_state: EntitlementPackageBuildState::Invalid,
            ..trusted_snapshot_context()
        },
        capability: EntitlementCapability::ScreenEvidence,
        ..entitlement_input(EntitlementCapabilityScope::LocalChildRuntime)
    });

    assert_eq!(
        decision.access_state,
        EntitlementCapabilityAccessState::Blocked
    );
    assert_eq!(
        decision.rejection_reason,
        Some(EntitlementCapabilityRejectionReason::InvalidPackageBuild)
    );
}

#[test]
fn capability_request_records_typed_entitlement_decision_event() -> Result<(), EventingError> {
    let request = EntitlementCapabilityEvaluationRequestedEvent {
        aggregate_id: EntitlementAggregateId::parse(ENTITLEMENT_AGGREGATE_ID)?,
        evaluation_id: EntitlementEvaluationId::parse(ENTITLEMENT_EVALUATION_ID)?,
        input: entitlement_input(EntitlementCapabilityScope::LocalChildRuntime),
    };

    let decision = record_entitlement_capability_decision(&request);

    assert_eq!(decision.aggregate_id, request.aggregate_id);
    assert_eq!(decision.source_evaluation_id, request.evaluation_id);
    assert!(
        EntitlementDecisionId::parse(decision.decision_id.as_str()).is_ok(),
        "decision id remains branded"
    );
    assert_eq!(
        request.contract()?.event_type.as_str(),
        ENTITLEMENT_REQUESTED_EVENT_TYPE
    );
    assert_eq!(
        decision.contract()?.event_type.as_str(),
        ENTITLEMENT_DECISION_EVENT_TYPE
    );
    assert_eq!(
        decision.decision.manual_review_state,
        EntitlementManualReviewState::NotRequired
    );
    assert_eq!(decision.decision.rejection_reason, None);

    Ok(())
}
use ocentra_eventing::error::EventingError;
