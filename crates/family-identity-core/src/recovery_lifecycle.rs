#![forbid(unsafe_code)]

//! Durable-adapter-neutral recovery lifecycle state.

use serde::{Deserialize, Serialize};

use crate::family_identity_contract_text::required_contract_text;
use crate::setup_lifecycle::{
    evaluate_recovery_operation, RecoveryDecision, RecoveryIdentityProofState, RecoveryOperation,
    RecoveryState,
};
use ocentra_eventing::error::EventingError;

/// Recovery state held by the account adapter while an operation is proved
/// and approved. Completion never grants account or device authority; it only
/// makes the downstream custody/setup handoff explicit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLifecycleRecord {
    pub operation: RecoveryOperation,
    pub created_at: String,
    pub last_transition_at: String,
}

impl RecoveryLifecycleRecord {
    pub fn new(
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

    pub fn authorize(&self) -> RecoveryDecision {
        evaluate_recovery_operation(self.operation)
    }

    pub fn record_identity_proof(
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

    pub fn approve_owner(
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

    pub fn complete(&mut self, transitioned_at: impl Into<String>) -> Result<(), EventingError> {
        if self.operation.state != RecoveryState::Approved {
            return Err(invalid_state("approval-required"));
        }
        self.last_transition_at = transition_time(transitioned_at)?;
        self.operation.state = RecoveryState::Completed;
        Ok(())
    }

    pub fn revoke(&mut self, transitioned_at: impl Into<String>) -> Result<(), EventingError> {
        self.last_transition_at = transition_time(transitioned_at)?;
        self.operation.state = RecoveryState::Revoked;
        Ok(())
    }
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
