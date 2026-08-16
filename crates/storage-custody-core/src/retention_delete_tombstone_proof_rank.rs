use ocentra_schema::retention_delete_tombstone as contracts;

pub(super) fn state_rank(state: contracts::RetentionDeleteState) -> usize {
    match state {
        contracts::RetentionDeleteState::DeleteRequested => 0,
        contracts::RetentionDeleteState::DeleteValidated => 1,
        contracts::RetentionDeleteState::TombstoneWritten => 2,
        contracts::RetentionDeleteState::LocalRedacted => 3,
        contracts::RetentionDeleteState::PropagationPending => 4,
        contracts::RetentionDeleteState::Propagated => 5,
        contracts::RetentionDeleteState::ReplayProtected => 6,
        contracts::RetentionDeleteState::AuditRetained => 7,
        contracts::RetentionDeleteState::HardDeleted => 8,
    }
}
