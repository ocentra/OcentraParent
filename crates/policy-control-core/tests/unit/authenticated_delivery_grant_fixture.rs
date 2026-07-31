use super::authenticated_delivery_grant::IssuanceFixture;
use ocentra_eventing::bus::EventBus;
use ocentra_eventing::envelope::StoredEventEnvelope;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::EventType;
use ocentra_eventing::journal::policy::{JournalPolicy, JournalSelector};
use ocentra_eventing::journal::{
    EventJournal, JournalAppend, JournalAppendDurability, JournalAppendFuture, JournalHashVersion,
};
use ocentra_family_identity_core::household_authority::HouseholdAuthorityInput;
use ocentra_family_identity_core::household_authority_proof::{
    HouseholdAuthorityCurrentState, HouseholdAuthorityProofIdentityBinding,
    HouseholdAuthorityProofSigner,
};
use ocentra_family_identity_core::parent_step_up_proof::ParentStepUpProofSigner;
use ocentra_policy_control_core::authenticated_delivery_grant::authority::AuthenticatedDeliveryGrantAuthoritySigner;
use ocentra_policy_control_core::authenticated_delivery_grant::{
    AuthenticatedDeliveryGrantIssuanceError, AuthenticatedDeliveryGrantIssuer,
    DeliveryGrantBindings,
};
use ocentra_policy_control_core::policy_authority::{
    PolicyConflictDecision, PolicyConflictResolutionState, PolicyControlDecision,
    PolicyManualReviewState,
};
use ocentra_policy_control_core::policy_authority_resolved_decision::ResolvedPolicyDecision;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Default)]
pub(crate) struct InMemoryMilestoneJournal {
    next_sequence: AtomicU64,
}

pub(crate) struct FailingMilestoneJournal {
    calls: AtomicU64,
    fail_once_on: u64,
    persisted: Mutex<Vec<StoredEventEnvelope>>,
}

/// Models an untrusted V3 receipt whose mutable durability field was changed
/// after the buffered journal entry was produced. A real V3 synchronization
/// receipt must carry the authenticated completion marker instead.
#[derive(Default)]
pub(crate) struct ForgedV3MilestoneJournal {
    next_sequence: AtomicU64,
}

/// Models a legacy receipt whose mutable synchronized field is not backed by
/// a V3 completion proof and therefore cannot authorize a grant.
#[derive(Default)]
pub(crate) struct LegacyV2MilestoneJournal {
    next_sequence: AtomicU64,
}

impl FailingMilestoneJournal {
    pub(crate) fn fail_once_on(fail_once_on: u64) -> Self {
        Self {
            calls: AtomicU64::new(0),
            fail_once_on,
            persisted: Mutex::new(Vec::new()),
        }
    }

    pub(crate) fn persisted(&self) -> Result<Vec<StoredEventEnvelope>, EventingError> {
        let persisted = self
            .persisted
            .lock()
            .map_err(|_error| EventingError::JournalIo {
                path: "policy-control-failing-milestone-journal".to_owned(),
                reason: "persisted-record lock unavailable".to_owned(),
            })?;
        Ok(persisted.clone())
    }

    fn append_result(
        &self,
        envelope: &StoredEventEnvelope,
    ) -> Result<JournalAppend, EventingError> {
        let call = self.calls.fetch_add(1, Ordering::Relaxed) + 1;
        if call == self.fail_once_on {
            return Err(EventingError::JournalIo {
                path: "policy-control-failing-milestone-journal".to_owned(),
                reason: "intentional terminal append failure".to_owned(),
            });
        }
        self.persisted
            .lock()
            .map_err(|_error| EventingError::JournalIo {
                path: "policy-control-failing-milestone-journal".to_owned(),
                reason: "persisted-record lock unavailable".to_owned(),
            })?
            .push(envelope.clone());
        JournalAppend {
            sequence: call,
            previous_hash: None,
            current_hash: None,
            hash_version: JournalHashVersion::V3,
            durability: JournalAppendDurability::Buffered,
            requested_durability: JournalAppendDurability::Synchronized,
            synchronization_hash: None,
        }
        .with_synchronization_proof()
    }
}

impl EventJournal for FailingMilestoneJournal {
    fn is_production_durable(&self) -> bool {
        true
    }

    fn append<'a>(&'a self, envelope: &'a StoredEventEnvelope) -> JournalAppendFuture<'a> {
        Box::pin(async move { self.append_result(envelope) })
    }
}

impl EventJournal for InMemoryMilestoneJournal {
    fn is_production_durable(&self) -> bool {
        true
    }

    fn append<'a>(&'a self, _envelope: &'a StoredEventEnvelope) -> JournalAppendFuture<'a> {
        Box::pin(async move {
            JournalAppend {
                sequence: self.next_sequence.fetch_add(1, Ordering::Relaxed) + 1,
                previous_hash: None,
                current_hash: None,
                hash_version: JournalHashVersion::V3,
                durability: JournalAppendDurability::Buffered,
                requested_durability: JournalAppendDurability::Synchronized,
                synchronization_hash: None,
            }
            .with_synchronization_proof()
        })
    }
}

impl EventJournal for ForgedV3MilestoneJournal {
    fn is_production_durable(&self) -> bool {
        true
    }

    fn append<'a>(&'a self, _envelope: &'a StoredEventEnvelope) -> JournalAppendFuture<'a> {
        Box::pin(async move {
            Ok(JournalAppend {
                sequence: self.next_sequence.fetch_add(1, Ordering::Relaxed) + 1,
                previous_hash: None,
                current_hash: None,
                hash_version: JournalHashVersion::V3,
                durability: JournalAppendDurability::Synchronized,
                requested_durability: JournalAppendDurability::Synchronized,
                synchronization_hash: None,
            })
        })
    }
}

impl EventJournal for LegacyV2MilestoneJournal {
    fn is_production_durable(&self) -> bool {
        true
    }

    fn append<'a>(&'a self, _envelope: &'a StoredEventEnvelope) -> JournalAppendFuture<'a> {
        Box::pin(async move {
            Ok(JournalAppend {
                sequence: self.next_sequence.fetch_add(1, Ordering::Relaxed) + 1,
                previous_hash: None,
                current_hash: None,
                hash_version: JournalHashVersion::V2,
                durability: JournalAppendDurability::Synchronized,
                requested_durability: JournalAppendDurability::Synchronized,
                synchronization_hash: None,
            })
        })
    }
}

pub(super) fn issuer_without_milestone_publisher(
) -> Result<AuthenticatedDeliveryGrantIssuer, AuthenticatedDeliveryGrantIssuanceError> {
    let authority = AuthenticatedDeliveryGrantAuthoritySigner::from_platform_key([7; 32]);
    let household_authority = HouseholdAuthorityProofSigner::from_platform_key([6; 32]);
    let step_up = ParentStepUpProofSigner::from_platform_key([8; 32]);
    AuthenticatedDeliveryGrantIssuer::from_platform_key_with_provenance_verifiers(
        "parent-key-1",
        [3; 32],
        authority.verifying_key(),
        household_authority.verifying_key(),
        current_household_authority_state(),
        step_up.verifying_key(),
    )
    .map(|issuer| issuer.with_trusted_issuance_now_for_debug_test("2026-07-28T00:01:00Z"))
}

pub(super) fn issuer(
) -> Result<AuthenticatedDeliveryGrantIssuer, AuthenticatedDeliveryGrantIssuanceError> {
    let event_type = EventType::parse("authenticated-delivery-grant.issuance.milestone")
        .map_err(|_error| AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed)?;
    let event_bus = EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
        Arc::new(InMemoryMilestoneJournal::default()),
    );
    issuer_without_milestone_publisher()
        .and_then(|issuer| {
            issuer
                .with_event_bus_issuance_publisher(event_bus)
                .map_err(|_error| {
                    AuthenticatedDeliveryGrantIssuanceError::MilestonePublicationFailed
                })
        })
        .map(|issuer| issuer.with_trusted_issuance_now_for_debug_test("2026-07-28T00:01:00Z"))
}

pub(super) fn household_authority_proof(
    authority: HouseholdAuthorityInput,
) -> ocentra_family_identity_core::household_authority_proof::HouseholdAuthorityProof {
    test_ok!(
        HouseholdAuthorityProofSigner::from_platform_key([6; 32]).sign_bound_at(
            &HouseholdAuthorityCurrentState {
                authority,
                family_revocation_epoch: 1,
            },
            HouseholdAuthorityProofIdentityBinding {
                household_id: "household-1".to_owned(),
                parent_actor_id: "parent-1".to_owned(),
                parent_device_id: "parent-device-1".to_owned(),
                child_profile_id: "child-1".to_owned(),
                target_device_id: "child-device-1".to_owned(),
            },
            "2026-07-28T00:00:00Z",
            "2026-07-28T00:05:00Z",
        ),
        "family identity authority proof"
    )
}

pub(super) fn current_household_authority_state() -> HouseholdAuthorityCurrentState {
    HouseholdAuthorityCurrentState {
        authority: HouseholdAuthorityInput {
            actor_role: ocentra_family_identity_core::family_identity::HouseholdRole::ParentOwner,
            same_family: true,
            actor_account_state: ocentra_family_identity_core::family_identity::ActorAccountState::Active,
            membership_state: ocentra_family_identity_core::family_identity::HouseholdMembershipState::Active,
            child_profile_binding_state: ocentra_family_identity_core::family_identity::ChildProfileBindingState::Bound,
            device_ownership_scope: ocentra_family_identity_core::family_identity::DeviceOwnershipScope::ChildProfileDevice,
            device_trust_state: ocentra_family_identity_core::family_identity::DeviceTrustState::Trusted,
            session_freshness_state: ocentra_family_identity_core::family_identity::SessionFreshnessState::Fresh,
            capability_granted: true,
            controller_lease_state: None,
            action: ocentra_family_identity_core::household_authority::HouseholdAuthorityAction::ChangePolicy,
        },
        family_revocation_epoch: 1,
    }
}

pub(super) fn resolved_decision(
    bindings: &DeliveryGrantBindings,
    decision: PolicyControlDecision,
) -> ResolvedPolicyDecision {
    test_ok!(
        ResolvedPolicyDecision::for_delivery_grant(
            format!(
                "policy-control-aggregate:{}:{}",
                bindings.target_device_id, bindings.action_id
            ),
            bindings.policy_decision_id.clone(),
            decision,
            executable_conflict_decision(),
        ),
        "resolved policy decision identity"
    )
}

pub(super) fn executable_conflict_decision() -> PolicyConflictDecision {
    PolicyConflictDecision {
        resolution_state: PolicyConflictResolutionState::UseParentPolicy,
        manual_review_state: PolicyManualReviewState::NotRequired,
    }
}

pub(super) fn durable_milestone_bus(
    _journal_path: &std::path::Path,
) -> Result<EventBus, ocentra_eventing::error::EventingError> {
    let event_type = EventType::parse("authenticated-delivery-grant.issuance.milestone")?;
    Ok(EventBus::with_journal(
        JournalPolicy::before_dispatch(JournalSelector::EventTypes(vec![event_type])),
        Arc::new(InMemoryMilestoneJournal::default()),
    ))
}

pub(super) fn issuance_fixture_with_expiry(
    mut fixture: IssuanceFixture,
    expires_at: &str,
) -> Result<IssuanceFixture, String> {
    fixture.bindings.expires_at = expires_at.to_owned();
    let assertion = fixture
        .parent_step_up
        .validation
        .assertion
        .as_mut()
        .ok_or_else(|| "fixture is missing its parent step-up assertion".to_owned())?;
    assertion.expires_at = expires_at.to_owned();
    Ok(fixture)
}
