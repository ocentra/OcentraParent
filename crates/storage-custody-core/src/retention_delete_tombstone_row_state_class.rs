use ocentra_schema::retention_delete_tombstone as contracts;

pub(super) fn state_retention_class(
    state: contracts::RetentionDeleteState,
) -> contracts::RetentionDeleteRetentionClass {
    match state {
        contracts::RetentionDeleteState::DeleteRequested
        | contracts::RetentionDeleteState::DeleteValidated => {
            contracts::RetentionDeleteRetentionClass::DeleteRequested
        }
        contracts::RetentionDeleteState::TombstoneWritten
        | contracts::RetentionDeleteState::LocalRedacted
        | contracts::RetentionDeleteState::PropagationPending
        | contracts::RetentionDeleteState::Propagated
        | contracts::RetentionDeleteState::ReplayProtected => {
            contracts::RetentionDeleteRetentionClass::DeleteConfirmed
        }
        contracts::RetentionDeleteState::AuditRetained => {
            contracts::RetentionDeleteRetentionClass::AuditMinimal
        }
        contracts::RetentionDeleteState::HardDeleted => {
            contracts::RetentionDeleteRetentionClass::HardDeleted
        }
    }
}
