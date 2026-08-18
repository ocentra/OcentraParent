#![forbid(unsafe_code)]

//! Durable-adapter-neutral recovery lifecycle state.

use crate::family_identity::RecoveryId;
use crate::family_identity_contract_text::required_contract_text;
use crate::setup_lifecycle::{
    evaluate_recovery_operation, RecoveryDecision, RecoveryIdentityProofState, RecoveryKind,
    RecoveryOperation, RecoveryState,
};
use ocentra_eventing::error::EventingError;
use ocentra_schema::account_identity_authority::{
    AccountIdentityDeviceId, AccountIdentityMemberId,
};
use ocentra_schema::report_query_custody::{FamilyId, ParentAccountId};

/// Recovery state held by the account adapter while an operation is proved
/// and approved. Completion never grants account or device authority; it only
/// makes the downstream custody/setup handoff explicit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryLifecycleRecord {
    pub(crate) operation: RecoveryOperation,
    pub(crate) created_at: String,
    pub(crate) last_transition_at: String,
}

impl RecoveryLifecycleRecord {
    pub(crate) fn new(
        operation: RecoveryOperation,
        created_at: impl Into<String>,
    ) -> Result<Self, EventingError> {
        let created_at = required_contract_text("family_identity.recovery.created_at", created_at)?;
        Ok(Self {
            operation,
            last_transition_at: created_at.clone(),
            created_at,
        })
    }

    pub(crate) fn authorize(&self) -> RecoveryDecision {
        evaluate_recovery_operation(self.operation)
    }

    pub(crate) fn record_identity_proof(
        &mut self,
        proof_state: RecoveryIdentityProofState,
        transitioned_at: impl Into<String>,
    ) -> Result<(), EventingError> {
        if self.operation.state != RecoveryState::PendingIdentityProof {
            return Err(invalid_state("identity-proof-transition-not-available"));
        }
        self.last_transition_at = transition_time(transitioned_at)?;
        self.operation.identity_proof_state = proof_state;
        self.operation.state = next_state_after_identity_proof(self.operation, proof_state);
        Ok(())
    }

    pub(crate) fn approve_owner(
        &mut self,
        transitioned_at: impl Into<String>,
    ) -> Result<(), EventingError> {
        if self.operation.state != RecoveryState::OwnerApprovalRequired {
            return Err(invalid_state("owner-approval-transition-not-available"));
        }
        let decision = self.authorize();
        if self.operation.identity_proof_state != RecoveryIdentityProofState::Verified
            || !decision.owner_approval_required
        {
            return Err(invalid_state("owner-approval-not-available"));
        }
        self.last_transition_at = transition_time(transitioned_at)?;
        self.operation.state = RecoveryState::Approved;
        Ok(())
    }

    pub(crate) fn revoke(
        &mut self,
        transitioned_at: impl Into<String>,
    ) -> Result<(), EventingError> {
        if matches!(
            self.operation.state,
            RecoveryState::Completed | RecoveryState::Revoked
        ) {
            return Err(invalid_state("terminal-state-transition-not-available"));
        }
        self.last_transition_at = transition_time(transitioned_at)?;
        self.operation.state = RecoveryState::Revoked;
        Ok(())
    }
}

/// Opaque, account-owned handoff queued only after recovery approval commits.
/// It is a custody request reference, not evidence of custody execution or a
/// replacement for the current authority check.
#[derive(Debug, PartialEq, Eq)]
pub struct RecoveryCustodyHandoff {
    handoff_id: String,
    correlation_id: String,
    recovery_id: RecoveryId,
    household_id: FamilyId,
    account_id: ParentAccountId,
    member_id: AccountIdentityMemberId,
    device_id: AccountIdentityDeviceId,
    kind: RecoveryKind,
    requested_at: String,
}

fn next_state_after_identity_proof(
    operation: RecoveryOperation,
    proof_state: RecoveryIdentityProofState,
) -> RecoveryState {
    match proof_state {
        RecoveryIdentityProofState::Failed => RecoveryState::Revoked,
        RecoveryIdentityProofState::Pending => operation.state,
        RecoveryIdentityProofState::Verified => {
            if evaluate_recovery_operation(operation).owner_approval_required {
                RecoveryState::OwnerApprovalRequired
            } else {
                RecoveryState::Approved
            }
        }
    }
}

fn invalid_state(value: &'static str) -> EventingError {
    EventingError::InvalidValue {
        field: "family_identity.recovery.state",
        value: value.to_owned(),
    }
}

fn transition_time(value: impl Into<String>) -> Result<String, EventingError> {
    required_contract_text("family_identity.recovery.last_transition_at", value)
}

#[path = "recovery_lifecycle_handoff.rs"]
mod handoff;
