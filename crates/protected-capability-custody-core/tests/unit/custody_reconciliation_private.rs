#![cfg(test)]

use super::{support, CustodyError, Decision};
use crate::binding::{
    Action, Binding, BindingError, BindingLocator, GenerationSlot, GenerationSlotName, OperationId,
    TargetEnvelope, TargetKind,
};
use crate::platform::identity::{DatabaseIdentity, PhysicalDatabaseIdentity};
use crate::platform::record::BrokerRecord;
use crate::platform::{PlatformAttestation, PlatformError, SealedState, TransitionFailure};
use crate::storage;

fn identity() -> Result<DatabaseIdentity, PlatformError> {
    let physical = PhysicalDatabaseIdentity::from_parts([1_u8; 32], [2_u8; 32], [3_u8; 32])?;
    DatabaseIdentity::from_parts(physical, [4_u8; 32])
}

fn binding() -> Result<Binding, BindingError> {
    let operation = OperationId::try_new(vec![1, 2, 3])?;
    let target = TargetEnvelope::try_new(TargetKind::Device, vec![4], vec![5], vec![6])?;
    let locator = BindingLocator::try_new(operation, Action::Seal, target)?;
    Binding::try_new(
        locator,
        [
            GenerationSlot::try_new(GenerationSlotName::Authority, 1)?,
            GenerationSlot::try_new(GenerationSlotName::Target, 2)?,
            GenerationSlot::try_new(GenerationSlotName::Key, 3)?,
            GenerationSlot::try_new(GenerationSlotName::Writer, 4)?,
        ],
    )
}

fn record() -> Result<crate::storage::Record, CustodyError> {
    record_with(SealedState::Prepared, 1)
}

fn record_with(state: SealedState, sequence: u64) -> Result<crate::storage::Record, CustodyError> {
    let binding = binding().map_err(|_error| CustodyError::Tampered)?;
    let database_identity = identity().map_err(|_error| CustodyError::Tampered)?;
    storage::from_broker(&BrokerRecord {
        record_namespace: crate::RECORD_NAMESPACE.to_vec(),
        schema_version: crate::STORAGE_SCHEMA_VERSION,
        binding_version: crate::binding::BINDING_VERSION,
        database_identity,
        record_id: [9_u8; 32],
        lookup_digest: binding.locator().lookup_digest(),
        binding_digest: binding.digest(),
        canonical_binding: binding.canonical_bytes().to_vec(),
        state,
        sequence,
        key_epoch: 3,
        writer_epoch: 4,
        anti_rollback_watermark: 5,
        sealed: vec![7],
    })
    .map_err(|_error| CustodyError::Database)
}

#[test]
fn reconciliation_currentness_rejects_cross_binding_and_accepts_exact_binding(
) -> Result<(), CustodyError> {
    let binding = binding().map_err(|_error| CustodyError::Tampered)?;
    let current = record()?;
    assert!(matches!(
        support::validate_current(&current, &binding),
        Ok(())
    ));

    let other = Binding::try_new(
        BindingLocator::try_new(
            OperationId::try_new(vec![8]).map_err(|_error| CustodyError::Tampered)?,
            Action::Seal,
            TargetEnvelope::try_new(TargetKind::Device, vec![4], vec![5], vec![6])
                .map_err(|_error| CustodyError::Tampered)?,
        )
        .map_err(|_error| CustodyError::Tampered)?,
        [
            GenerationSlot::try_new(GenerationSlotName::Authority, 1)
                .map_err(|_error| CustodyError::Tampered)?,
            GenerationSlot::try_new(GenerationSlotName::Target, 2)
                .map_err(|_error| CustodyError::Tampered)?,
            GenerationSlot::try_new(GenerationSlotName::Key, 3)
                .map_err(|_error| CustodyError::Tampered)?,
            GenerationSlot::try_new(GenerationSlotName::Writer, 4)
                .map_err(|_error| CustodyError::Tampered)?,
        ],
    )
    .map_err(|_error| CustodyError::Tampered)?;
    assert!(matches!(
        support::validate_current(&current, &other),
        Err(CustodyError::WrongBinding)
    ));
    Ok(())
}

#[test]
fn reconciliation_attestation_rejects_rotation_regression_and_identity_drift(
) -> Result<(), CustodyError> {
    let current = record()?;
    let matching = PlatformAttestation::isolated_broker(
        current.key_epoch,
        current.writer_epoch,
        current.anti_rollback_watermark,
        current.database_identity,
    );
    assert!(matches!(
        support::validate_attestation(&current, matching),
        Ok(())
    ));

    let rotated = PlatformAttestation::isolated_broker(
        current.key_epoch + 1,
        current.writer_epoch,
        current.anti_rollback_watermark,
        current.database_identity,
    );
    assert!(matches!(
        support::validate_attestation(&current, rotated),
        Err(CustodyError::Rotated)
    ));

    let regressed = PlatformAttestation::isolated_broker(
        current.key_epoch,
        current.writer_epoch - 1,
        current.anti_rollback_watermark - 1,
        current.database_identity,
    );
    assert!(matches!(
        support::validate_attestation(&current, regressed),
        Err(CustodyError::Tampered)
    ));

    let other_physical = PhysicalDatabaseIdentity::from_parts([8_u8; 32], [9_u8; 32], [10_u8; 32])
        .map_err(|_error| CustodyError::Tampered)?;
    let other_identity = DatabaseIdentity::from_parts(other_physical, [11_u8; 32])
        .map_err(|_error| CustodyError::Tampered)?;
    let replaced = PlatformAttestation::isolated_broker(
        current.key_epoch,
        current.writer_epoch,
        current.anti_rollback_watermark,
        other_identity,
    );
    assert!(matches!(
        support::validate_attestation(&current, replaced),
        Err(CustodyError::Tampered)
    ));
    Ok(())
}

#[test]
fn reconciliation_preserves_ambiguous_phases_and_fail_closed_error_mapping() {
    assert_eq!(
        support::ambiguous_state(Decision::Commit),
        SealedState::CommitAmbiguous
    );
    assert_eq!(
        support::ambiguous_state(Decision::Abort),
        SealedState::AbortAmbiguous
    );
    assert!(matches!(
        support::terminal_state(SealedState::CommitAmbiguous),
        Ok(SealedState::Committed)
    ));
    assert!(matches!(
        support::terminal_state(SealedState::AbortAmbiguous),
        Ok(SealedState::Aborted)
    ));
    assert!(matches!(
        support::terminal_state(SealedState::Prepared),
        Err(CustodyError::Conflict)
    ));

    let commit_ambiguous = record_with(SealedState::CommitAmbiguous, 2);
    assert!(matches!(
        commit_ambiguous.and_then(|record| support::finalize_outcome(&record)),
        Ok(super::FinalizeOutcome::CommitAmbiguous)
    ));
    let abort_ambiguous = record_with(SealedState::AbortAmbiguous, 2);
    assert!(matches!(
        abort_ambiguous.and_then(|record| support::finalize_outcome(&record)),
        Ok(super::FinalizeOutcome::AbortAmbiguous)
    ));
    let committed = record_with(SealedState::Committed, 3);
    assert!(matches!(
        committed.and_then(|record| support::finalize_outcome(&record)),
        Ok(super::FinalizeOutcome::Committed(_))
    ));

    assert!(matches!(
        support::map_platform_error(&PlatformError::DeploymentRequired),
        CustodyError::Unavailable
    ));
    assert!(matches!(
        support::map_transition_failure(
            TransitionFailure::OutcomeUnknown,
            super::TransitionPhase::CommitIntent,
        ),
        CustodyError::CommitAmbiguous
    ));
    assert!(matches!(
        support::map_transition_failure(
            TransitionFailure::OutcomeUnknown,
            super::TransitionPhase::AbortTerminal,
        ),
        CustodyError::AbortAmbiguous
    ));
}
