use std::fmt;

use chrono::Utc;
use ocentra_lan_core::lan_pairing::signed_household_mesh_ingress::LanCryptographicallyVerifiedHouseholdMeshIngress;
use ocentra_parent_agent_protocol::lan_pairing::signed_household_mesh_ingress::transport::LanSignedHouseholdMeshMessageType;
use ocentra_parent_agent_protocol::lan_pairing::LanSignedChildAgentMessageKind;
use rusqlite::Connection;

use crate::trusted_device_registry::{
    signer_authority_types::LanRegisteredSignedChildAuthority, TrustedDeviceRegistry,
};

mod authorization;
mod error_display;
mod error_static_message;
mod rejection;
mod rejection_authority_classification;
mod rejection_classification;
mod rejection_reason;
mod reservation;
mod schema;

#[derive(Debug)]
pub enum LanHouseholdMeshIngressCustodyError {
    StorageUnavailable,
    SchemaRejected,
    IntegrityRejected,
    InvalidInput,
    IdentityMismatch,
    AuthorityStale,
    SequenceRegression,
    Rejected {
        outcome: LanHouseholdMeshIngressRejectionOutcome,
    },
    DuplicateReceipt {
        receipt_id: String,
    },
    ReconciliationRequired {
        receipt_id: String,
    },
}

impl std::error::Error for LanHouseholdMeshIngressCustodyError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanHouseholdMeshIngressRejectionReason {
    AuthorityStale,
    Expired,
    IdentityMismatch,
    InvalidInput,
    DuplicateReceipt,
    ReconciliationRequired,
    SequenceRegression,
}

#[derive(PartialEq, Eq)]
pub struct LanHouseholdMeshIngressRejectionIdentity {
    sha256: String,
}

impl fmt::Debug for LanHouseholdMeshIngressRejectionIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LanHouseholdMeshIngressRejectionIdentity")
            .field("identity", &"[redacted]")
            .finish()
    }
}

impl LanHouseholdMeshIngressRejectionIdentity {
    pub fn sha256(&self) -> &str {
        &self.sha256
    }
}

#[derive(PartialEq, Eq)]
pub struct LanHouseholdMeshIngressRejectionOutcome {
    outcome_id: String,
    identity: LanHouseholdMeshIngressRejectionIdentity,
    reason: LanHouseholdMeshIngressRejectionReason,
    observed_at: String,
}

impl fmt::Debug for LanHouseholdMeshIngressRejectionOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LanHouseholdMeshIngressRejectionOutcome")
            .field("reason", &self.reason)
            .field("identity", &"[redacted]")
            .finish()
    }
}

impl LanHouseholdMeshIngressRejectionOutcome {
    pub fn outcome_id(&self) -> &str {
        &self.outcome_id
    }

    pub fn identity(&self) -> &LanHouseholdMeshIngressRejectionIdentity {
        &self.identity
    }

    pub fn reason(&self) -> LanHouseholdMeshIngressRejectionReason {
        self.reason
    }

    pub fn observed_at(&self) -> &str {
        &self.observed_at
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LanHouseholdMeshIngressAuthorizationScope {
    SelectedEventRepublish,
}

/// Non-serializable local capability returned only after the three durable
/// replay identities have committed. Its scope is deliberately limited to a
/// selected local event republish.
#[derive(PartialEq, Eq)]
pub struct LanHouseholdMeshIngressAuthorization {
    receipt_id: String,
    scope: LanHouseholdMeshIngressAuthorizationScope,
    family_hash: String,
    child_device_id: String,
    target_device_id: String,
    parent_device_id: String,
    signer_public_key_id: String,
    signer_public_key_sha256: String,
    message_kind: LanSignedChildAgentMessageKind,
    local_event_ref: String,
    lan_message_type: LanSignedHouseholdMeshMessageType,
    route_id: String,
    message_id: String,
    idempotency_key: String,
    nonce: String,
    sequence: u64,
    payload_digest: String,
    install_id: String,
    pairing_id: String,
    registry_proof_digest: String,
    authority_generation: u64,
    issued_at: String,
    expires_at: String,
    reserved_at: String,
}

impl fmt::Debug for LanHouseholdMeshIngressAuthorization {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LanHouseholdMeshIngressAuthorization")
            .field("redacted", &true)
            .finish()
    }
}

pub struct LanHouseholdMeshIngressReceiptStore {
    connection: Connection,
}

impl LanHouseholdMeshIngressReceiptStore {
    pub fn create(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self, LanHouseholdMeshIngressCustodyError> {
        schema::create_store(path.as_ref())
    }

    pub fn open(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self, LanHouseholdMeshIngressCustodyError> {
        schema::open_store(path.as_ref())
    }

    pub fn reserve_selected_event_republish(
        &mut self,
        registry: &TrustedDeviceRegistry,
        ingress: &LanCryptographicallyVerifiedHouseholdMeshIngress,
        authority: &LanRegisteredSignedChildAuthority,
    ) -> Result<LanHouseholdMeshIngressAuthorization, LanHouseholdMeshIngressCustodyError> {
        let observed_at = Utc::now().to_rfc3339();
        let current_authority = match registry.authorize_signed_child_claim(ingress, &observed_at) {
            Ok(authority) => authority,
            Err(reason) => {
                return self.reject_authorization(
                    ingress,
                    rejection_authority_classification::for_authority_failure(&reason),
                    &observed_at,
                );
            }
        };
        match reservation::reserve_selected_event_republish(
            self,
            ingress,
            authority,
            &current_authority,
        ) {
            Ok(authorization) => Ok(authorization),
            Err(error) => match rejection_classification::for_custody_failure(&error) {
                Some(reason) => self.reject_authorization(ingress, reason, &observed_at),
                None => Err(error),
            },
        }
    }

    pub fn record_rejected_ingress(
        &mut self,
        ingress: &LanCryptographicallyVerifiedHouseholdMeshIngress,
        reason: LanHouseholdMeshIngressRejectionReason,
        observed_at: &str,
    ) -> Result<LanHouseholdMeshIngressRejectionOutcome, LanHouseholdMeshIngressCustodyError> {
        rejection::record_rejected_ingress(self, ingress, reason, observed_at)
    }

    fn reject_authorization<T>(
        &mut self,
        ingress: &LanCryptographicallyVerifiedHouseholdMeshIngress,
        reason: LanHouseholdMeshIngressRejectionReason,
        observed_at: &str,
    ) -> Result<T, LanHouseholdMeshIngressCustodyError> {
        let outcome = self.record_rejected_ingress(ingress, reason, observed_at)?;
        Err(LanHouseholdMeshIngressCustodyError::Rejected { outcome })
    }
}
