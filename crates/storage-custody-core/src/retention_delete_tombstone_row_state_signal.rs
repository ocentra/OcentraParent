use ocentra_schema::retention_delete_tombstone as contracts;

use crate::retention_delete_tombstone::RetentionDeleteSignal;

pub(super) fn signal_state(signal: RetentionDeleteSignal) -> contracts::RetentionDeleteState {
    match signal {
        RetentionDeleteSignal::DeleteRequested => contracts::RetentionDeleteState::DeleteRequested,
        RetentionDeleteSignal::DeleteValidated => contracts::RetentionDeleteState::DeleteValidated,
        RetentionDeleteSignal::TombstoneWritten => {
            contracts::RetentionDeleteState::TombstoneWritten
        }
        RetentionDeleteSignal::LocalRedacted => contracts::RetentionDeleteState::LocalRedacted,
        RetentionDeleteSignal::PropagationPending => {
            contracts::RetentionDeleteState::PropagationPending
        }
        RetentionDeleteSignal::Propagated => contracts::RetentionDeleteState::Propagated,
        RetentionDeleteSignal::ReplayProtected => contracts::RetentionDeleteState::ReplayProtected,
        RetentionDeleteSignal::AuditRetained => contracts::RetentionDeleteState::AuditRetained,
        RetentionDeleteSignal::HardDeleted => contracts::RetentionDeleteState::HardDeleted,
    }
}
