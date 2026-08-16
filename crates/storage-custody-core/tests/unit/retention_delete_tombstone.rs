use crate::support::StorageCustodyTestValueExt;

use ocentra_schema::retention_delete_tombstone as contracts;
use ocentra_storage_custody_core::retention_delete_tombstone::{
    build_retention_delete_tombstone_proof, derive_retention_delete_tombstone_row,
    RetentionDeleteDerivationError, RetentionDeleteDerivationInput, RetentionDeleteSignal,
};

macro_rules! derivation_input {
    ($suffix:expr, $data_class:expr, $signal:expr $(,)?) => {
        RetentionDeleteDerivationInput {
            row_id: contracts::RetentionDeleteRowId::parse(format!("row-{}", $suffix)).assume_ok(),
            data_class: $data_class,
            signal: $signal,
            proof_ref: contracts::RetentionDeleteProofRef::parse(format!("proof-{}", $suffix))
                .assume_ok(),
            tombstone_ref: None,
            replay_ref: None,
            request_expired: false,
            local_payload_redacted: false,
            propagation_complete: false,
            replay_blocked: false,
            audit_payload_redacted: false,
            hard_delete_eligible: false,
        }
    };
}

macro_rules! tombstone_input {
    ($suffix:expr, $data_class:expr, $signal:expr $(,)?) => {
        RetentionDeleteDerivationInput {
            tombstone_ref: Some(
                contracts::RetentionDeleteTombstoneRef::parse(format!("tombstone-{}", $suffix))
                    .assume_ok(),
            ),
            ..derivation_input!($suffix, $data_class, $signal)
        }
    };
}

macro_rules! redacted_input {
    ($suffix:expr, $data_class:expr, $signal:expr $(,)?) => {
        RetentionDeleteDerivationInput {
            local_payload_redacted: true,
            ..tombstone_input!($suffix, $data_class, $signal)
        }
    };
}

macro_rules! propagated_input {
    ($suffix:expr, $data_class:expr, $signal:expr $(,)?) => {
        RetentionDeleteDerivationInput {
            propagation_complete: true,
            ..redacted_input!($suffix, $data_class, $signal)
        }
    };
}

macro_rules! replay_protected_input {
    ($suffix:expr, $data_class:expr, $signal:expr $(,)?) => {
        RetentionDeleteDerivationInput {
            replay_ref: Some(
                contracts::RetentionDeleteReplayRef::parse(format!("replay-{}", $suffix))
                    .assume_ok(),
            ),
            replay_blocked: true,
            ..propagated_input!($suffix, $data_class, $signal)
        }
    };
}

macro_rules! audit_retained_input {
    ($suffix:expr, $data_class:expr, $signal:expr $(,)?) => {
        RetentionDeleteDerivationInput {
            audit_payload_redacted: true,
            ..replay_protected_input!($suffix, $data_class, $signal)
        }
    };
}

macro_rules! hard_deleted_input {
    ($suffix:expr, $data_class:expr, $signal:expr $(,)?) => {
        RetentionDeleteDerivationInput {
            hard_delete_eligible: true,
            ..audit_retained_input!($suffix, $data_class, $signal)
        }
    };
}

#[test]
fn retention_delete_tombstone_derives_request_validate_and_tombstone_states() {
    let request = sample_request();

    let requested = derive_retention_delete_tombstone_row(
        &request,
        derivation_input!(
            "delete-requested",
            contracts::RetentionDeleteDataClass::EvidenceJournal,
            RetentionDeleteSignal::DeleteRequested,
        ),
    )
    .assume_ok();
    let validated = derive_retention_delete_tombstone_row(
        &request,
        derivation_input!(
            "delete-validated",
            contracts::RetentionDeleteDataClass::PolicyHistory,
            RetentionDeleteSignal::DeleteValidated,
        ),
    )
    .assume_ok();
    let tombstone = derive_retention_delete_tombstone_row(
        &request,
        tombstone_input!(
            "tombstone-written",
            contracts::RetentionDeleteDataClass::EvidenceJournal,
            RetentionDeleteSignal::TombstoneWritten,
        ),
    )
    .assume_ok();

    assert_eq!(
        requested.state,
        contracts::RetentionDeleteState::DeleteRequested
    );
    assert_eq!(
        validated.state,
        contracts::RetentionDeleteState::DeleteValidated
    );
    assert_eq!(
        tombstone.state,
        contracts::RetentionDeleteState::TombstoneWritten
    );
    assert!(tombstone.tombstone_written);
}

#[test]
fn retention_delete_tombstone_requires_local_redaction_before_propagation() {
    let request = sample_request();

    let missing_redaction = derive_retention_delete_tombstone_row(
        &request,
        tombstone_input!(
            "propagation-pending",
            contracts::RetentionDeleteDataClass::Screenshots,
            RetentionDeleteSignal::PropagationPending,
        ),
    );
    assert_eq!(
        missing_redaction,
        Err(RetentionDeleteDerivationError::LocalPayloadMustBeRedacted)
    );

    let pending = derive_retention_delete_tombstone_row(
        &request,
        redacted_input!(
            "propagation-pending",
            contracts::RetentionDeleteDataClass::Screenshots,
            RetentionDeleteSignal::PropagationPending,
        ),
    )
    .assume_ok();
    assert_eq!(
        pending.state,
        contracts::RetentionDeleteState::PropagationPending
    );
    assert!(pending.local_payload_redacted);
    assert!(pending.propagation_pending);
}

#[test]
fn retention_delete_tombstone_propagation_and_replay_protection_are_explicit() {
    let request = sample_request();

    let missing_replay = derive_retention_delete_tombstone_row(
        &request,
        propagated_input!(
            "replay-protected",
            contracts::RetentionDeleteDataClass::Reports,
            RetentionDeleteSignal::ReplayProtected,
        ),
    );
    assert_eq!(
        missing_replay,
        Err(RetentionDeleteDerivationError::MissingReplayRef)
    );

    let replay_protected = derive_retention_delete_tombstone_row(
        &request,
        replay_protected_input!(
            "replay-protected",
            contracts::RetentionDeleteDataClass::Reports,
            RetentionDeleteSignal::ReplayProtected,
        ),
    )
    .assume_ok();
    assert!(replay_protected.propagated);
    assert!(replay_protected.replay_blocked);
    assert!(replay_protected.restore_revival_blocked);
}

#[test]
fn retention_delete_tombstone_audit_retention_and_hard_delete_stay_minimal_and_claim_safe() {
    let request = sample_request();

    let missing_audit_redaction = derive_retention_delete_tombstone_row(
        &request,
        replay_protected_input!(
            "audit-retained",
            contracts::RetentionDeleteDataClass::Notifications,
            RetentionDeleteSignal::AuditRetained,
        ),
    );
    assert_eq!(
        missing_audit_redaction,
        Err(RetentionDeleteDerivationError::AuditMustBeMinimal)
    );

    let audit_retained = derive_retention_delete_tombstone_row(
        &request,
        audit_retained_input!(
            "audit-retained",
            contracts::RetentionDeleteDataClass::Notifications,
            RetentionDeleteSignal::AuditRetained,
        ),
    )
    .assume_ok();
    let hard_deleted = derive_retention_delete_tombstone_row(
        &request,
        hard_deleted_input!(
            "hard-deleted",
            contracts::RetentionDeleteDataClass::Logs,
            RetentionDeleteSignal::HardDeleted,
        ),
    )
    .assume_ok();

    assert!(audit_retained.minimal_audit_ref_retained);
    assert!(audit_retained.audit_payload_redacted);
    assert!(hard_deleted.hard_deleted);
    assert!(hard_deleted.claim_safe);
}

#[test]
fn retention_delete_tombstone_rejects_wrong_role_and_expired_requests() {
    let mut unauthorized_request = sample_request();
    unauthorized_request.parent_authorized = false;
    let unauthorized = derive_retention_delete_tombstone_row(
        &unauthorized_request,
        derivation_input!(
            "wrong-role",
            contracts::RetentionDeleteDataClass::EvidenceJournal,
            RetentionDeleteSignal::DeleteRequested,
        ),
    );
    assert_eq!(
        unauthorized,
        Err(RetentionDeleteDerivationError::ActorNotAuthorized)
    );

    let expired = derive_retention_delete_tombstone_row(
        &sample_request(),
        RetentionDeleteDerivationInput {
            request_expired: true,
            ..derivation_input!(
                "expired-request",
                contracts::RetentionDeleteDataClass::EvidenceJournal,
                RetentionDeleteSignal::DeleteRequested,
            )
        },
    );
    assert_eq!(
        expired,
        Err(RetentionDeleteDerivationError::DeleteRequestExpired)
    );
}

#[test]
fn retention_delete_tombstone_builds_full_proof_with_required_states() {
    let request = sample_request();
    let proof = build_retention_delete_tombstone_proof(
        &request,
        vec![
            derivation_input!(
                "delete-requested",
                contracts::RetentionDeleteDataClass::EvidenceJournal,
                RetentionDeleteSignal::DeleteRequested,
            ),
            derivation_input!(
                "delete-validated",
                contracts::RetentionDeleteDataClass::PolicyHistory,
                RetentionDeleteSignal::DeleteValidated,
            ),
            tombstone_input!(
                "tombstone-written",
                contracts::RetentionDeleteDataClass::EvidenceJournal,
                RetentionDeleteSignal::TombstoneWritten,
            ),
            redacted_input!(
                "local-redacted",
                contracts::RetentionDeleteDataClass::Screenshots,
                RetentionDeleteSignal::LocalRedacted,
            ),
            redacted_input!(
                "propagation-pending",
                contracts::RetentionDeleteDataClass::NetworkArtifacts,
                RetentionDeleteSignal::PropagationPending,
            ),
            propagated_input!(
                "propagated",
                contracts::RetentionDeleteDataClass::Reports,
                RetentionDeleteSignal::Propagated,
            ),
            replay_protected_input!(
                "replay-protected",
                contracts::RetentionDeleteDataClass::AiOutputs,
                RetentionDeleteSignal::ReplayProtected,
            ),
            audit_retained_input!(
                "audit-retained",
                contracts::RetentionDeleteDataClass::Notifications,
                RetentionDeleteSignal::AuditRetained,
            ),
            hard_deleted_input!(
                "hard-deleted",
                contracts::RetentionDeleteDataClass::Logs,
                RetentionDeleteSignal::HardDeleted,
            ),
        ],
        contracts::RetentionDeleteTimestamp::parse("2026-06-28T18:09:00.000Z").assume_ok(),
    )
    .assume_ok();

    assert_eq!(proof.retention_matrix.len(), 11);
    assert_eq!(
        proof
            .rows
            .iter()
            .map(|row| row.state.as_str())
            .collect::<Vec<_>>(),
        vec![
            "deleteRequested",
            "deleteValidated",
            "tombstoneWritten",
            "localRedacted",
            "propagationPending",
            "propagated",
            "replayProtected",
            "auditRetained",
            "hardDeleted",
        ]
    );
    assert_eq!(proof.rows[7].retention_class.as_str(), "audit-minimal");
    assert_eq!(proof.rows[8].retention_class.as_str(), "hard-deleted");
}

#[test]
fn retention_delete_tombstone_rejects_duplicate_state_and_hard_delete_without_replay_chain() {
    let request = sample_request();
    let duplicate = build_retention_delete_tombstone_proof(
        &request,
        vec![
            tombstone_input!(
                "tombstone-a",
                contracts::RetentionDeleteDataClass::EvidenceJournal,
                RetentionDeleteSignal::TombstoneWritten,
            ),
            tombstone_input!(
                "tombstone-b",
                contracts::RetentionDeleteDataClass::Screenshots,
                RetentionDeleteSignal::TombstoneWritten,
            ),
        ],
        contracts::RetentionDeleteTimestamp::parse("2026-06-28T18:10:00.000Z").assume_ok(),
    );
    assert_eq!(
        duplicate,
        Err(RetentionDeleteDerivationError::DuplicateState(
            contracts::RetentionDeleteState::TombstoneWritten
        ))
    );

    let missing_replay_chain = derive_retention_delete_tombstone_row(
        &request,
        RetentionDeleteDerivationInput {
            hard_delete_eligible: true,
            audit_payload_redacted: true,
            ..propagated_input!(
                "hard-delete-missing-replay",
                contracts::RetentionDeleteDataClass::Logs,
                RetentionDeleteSignal::HardDeleted,
            )
        },
    );
    assert_eq!(
        missing_replay_chain,
        Err(RetentionDeleteDerivationError::MissingReplayRef)
    );
}

fn sample_request() -> contracts::RetentionDeleteRequest {
    contracts::sample_retention_delete_tombstone_contract_proof().request
}
