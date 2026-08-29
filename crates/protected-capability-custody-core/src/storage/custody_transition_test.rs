use super::record as record_module;
use super::{from_broker, Record, StorageError};
use crate::binding::{
    Action, Binding, BindingLocator, GenerationSlot, GenerationSlotName, OperationId,
    TargetEnvelope, TargetKind, BINDING_VERSION,
};
use crate::platform::identity::{DatabaseIdentity, PhysicalDatabaseIdentity};
use crate::platform::record::BrokerRecord;
use crate::platform::SealedState;

fn identity() -> Result<DatabaseIdentity, crate::platform::PlatformError> {
    let physical = PhysicalDatabaseIdentity::from_parts([1_u8; 32], [2_u8; 32], [3_u8; 32])?;
    DatabaseIdentity::from_parts(physical, [4_u8; 32])
}

fn binding(operation_byte: u8) -> Result<Binding, crate::binding::BindingError> {
    let operation = OperationId::try_new(vec![operation_byte])?;
    let target = TargetEnvelope::try_new(TargetKind::Device, vec![2], vec![3], vec![4])?;
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

fn record(
    binding: &Binding,
    state: SealedState,
    sequence: u64,
    key_epoch: u64,
    writer_epoch: u64,
    watermark: u64,
    sealed_byte: u8,
) -> Result<Record, StorageError> {
    let database_identity = identity().map_err(|_| StorageError::Tampered)?;
    from_broker(&BrokerRecord {
        record_namespace: crate::RECORD_NAMESPACE.to_vec(),
        schema_version: crate::STORAGE_SCHEMA_VERSION,
        binding_version: BINDING_VERSION,
        database_identity,
        record_id: [9_u8; 32],
        lookup_digest: binding.locator().lookup_digest(),
        binding_digest: binding.digest(),
        canonical_binding: binding.canonical_bytes().to_vec(),
        state,
        sequence,
        key_epoch,
        writer_epoch,
        anti_rollback_watermark: watermark,
        sealed: vec![sealed_byte],
    })
}

#[test]
fn transition_accepts_only_sequential_ambiguous_and_terminal_edges() -> Result<(), StorageError> {
    let binding = binding(1).map_err(|_| StorageError::Tampered)?;
    let prepared = record(&binding, SealedState::Prepared, 1, 3, 4, 5, 1)?;
    let commit_intent = record(&binding, SealedState::CommitAmbiguous, 2, 3, 4, 6, 2)?;
    let committed = record(&binding, SealedState::Committed, 3, 3, 4, 7, 3)?;

    assert!(record_module::validate_transition(&prepared, &commit_intent).is_ok());
    assert!(record_module::validate_transition(&commit_intent, &committed).is_ok());
    Ok(())
}

#[test]
fn transition_rejects_replay_nonmonotonic_watermark_and_wrong_edge() -> Result<(), StorageError> {
    let binding = binding(1).map_err(|_| StorageError::Tampered)?;
    let prepared = record(&binding, SealedState::Prepared, 1, 3, 4, 5, 1)?;
    let replay = record(&binding, SealedState::Prepared, 1, 3, 4, 6, 2)?;
    let regressed_watermark = record(&binding, SealedState::CommitAmbiguous, 2, 3, 4, 5, 2)?;
    let skipped_terminal = record(&binding, SealedState::Committed, 3, 3, 4, 7, 3)?;

    assert!(matches!(
        record_module::validate_transition(&prepared, &replay),
        Err(StorageError::IllegalTransition)
    ));
    assert!(matches!(
        record_module::validate_transition(&prepared, &regressed_watermark),
        Err(StorageError::IllegalTransition)
    ));
    assert!(matches!(
        record_module::validate_transition(&prepared, &skipped_terminal),
        Err(StorageError::IllegalTransition)
    ));
    Ok(())
}

#[test]
fn transition_rejects_binding_and_epoch_drift_after_each_record_is_authenticated(
) -> Result<(), StorageError> {
    let first_binding = binding(1).map_err(|_| StorageError::Tampered)?;
    let other_binding = binding(2).map_err(|_| StorageError::Tampered)?;
    let prepared = record(&first_binding, SealedState::Prepared, 1, 3, 4, 5, 1)?;
    let binding_drift = record(&other_binding, SealedState::CommitAmbiguous, 2, 3, 4, 6, 2)?;
    let key_drift = record(&first_binding, SealedState::CommitAmbiguous, 2, 4, 4, 6, 2)?;
    let writer_drift = record(&first_binding, SealedState::CommitAmbiguous, 2, 3, 5, 6, 2)?;

    assert!(matches!(
        record_module::validate_transition(&prepared, &binding_drift),
        Err(StorageError::IllegalTransition)
    ));
    assert!(matches!(
        record_module::validate_transition(&prepared, &key_drift),
        Err(StorageError::IllegalTransition)
    ));
    assert!(record_module::validate_transition(&prepared, &writer_drift).is_ok());
    Ok(())
}

#[test]
fn state_sequence_mismatch_is_tampered_before_transition_evaluation() -> Result<(), StorageError> {
    let binding = binding(1).map_err(|_| StorageError::Tampered)?;
    assert!(matches!(
        record(&binding, SealedState::Committed, 1, 3, 4, 5, 1),
        Err(StorageError::Tampered)
    ));
    Ok(())
}
