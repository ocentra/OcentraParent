use ocentra_schema::retention_delete_tombstone as contracts;

#[path = "retention_delete_tombstone_proof.rs"]
mod retention_delete_tombstone_proof;
#[path = "retention_delete_tombstone_row.rs"]
mod retention_delete_tombstone_row;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetentionDeleteSignal {
    DeleteRequested,
    DeleteValidated,
    TombstoneWritten,
    LocalRedacted,
    PropagationPending,
    Propagated,
    ReplayProtected,
    AuditRetained,
    HardDeleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetentionDeleteDerivationInput {
    pub row_id: contracts::RetentionDeleteRowId,
    pub request_expired: bool,
    pub signal: RetentionDeleteSignal,
    pub data_class: contracts::RetentionDeleteDataClass,
    pub proof_ref: contracts::RetentionDeleteProofRef,
    pub tombstone_ref: Option<contracts::RetentionDeleteTombstoneRef>,
    pub replay_ref: Option<contracts::RetentionDeleteReplayRef>,
    pub local_payload_redacted: bool,
    pub propagation_complete: bool,
    pub replay_blocked: bool,
    pub audit_payload_redacted: bool,
    pub hard_delete_eligible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetentionDeleteDerivationError {
    ActorNotAuthorized,
    DeleteRequestExpired,
    DeleteRequestMustTargetRawPayload,
    DerivedOutputsMustRedact,
    MissingTombstoneRef,
    LocalPayloadMustBeRedacted,
    PropagationStillPending,
    MissingReplayRef,
    ReplayProtectionRequired,
    AuditMustBeMinimal,
    HardDeleteNotEligible,
    DuplicateState(contracts::RetentionDeleteState),
    MissingRequiredState(contracts::RetentionDeleteState),
    InvalidContractVersion,
}

pub fn derive_retention_delete_tombstone_row(
    request: &contracts::RetentionDeleteRequest,
    input: RetentionDeleteDerivationInput,
) -> Result<contracts::RetentionDeleteRow, RetentionDeleteDerivationError> {
    retention_delete_tombstone_row::derive_retention_delete_tombstone_row(request, input)
}

pub fn build_retention_delete_tombstone_proof(
    request: &contracts::RetentionDeleteRequest,
    inputs: Vec<RetentionDeleteDerivationInput>,
    updated_at: contracts::RetentionDeleteTimestamp,
) -> Result<contracts::RetentionDeleteTombstoneContractProof, RetentionDeleteDerivationError> {
    retention_delete_tombstone_proof::build_retention_delete_tombstone_proof(
        request, inputs, updated_at,
    )
}
