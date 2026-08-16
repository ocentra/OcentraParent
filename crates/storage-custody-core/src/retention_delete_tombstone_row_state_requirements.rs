use ocentra_schema::retention_delete_tombstone as contracts;

use super::RetentionDeleteStateRequirements;

pub(super) fn retention_delete_requirements(
    state: contracts::RetentionDeleteState,
) -> RetentionDeleteStateRequirements {
    RetentionDeleteStateRequirements {
        tombstone_written: matches!(
            state,
            contracts::RetentionDeleteState::TombstoneWritten
                | contracts::RetentionDeleteState::LocalRedacted
                | contracts::RetentionDeleteState::PropagationPending
                | contracts::RetentionDeleteState::Propagated
                | contracts::RetentionDeleteState::ReplayProtected
                | contracts::RetentionDeleteState::AuditRetained
                | contracts::RetentionDeleteState::HardDeleted
        ),
        requires_redaction: matches!(
            state,
            contracts::RetentionDeleteState::LocalRedacted
                | contracts::RetentionDeleteState::PropagationPending
                | contracts::RetentionDeleteState::Propagated
                | contracts::RetentionDeleteState::ReplayProtected
                | contracts::RetentionDeleteState::AuditRetained
                | contracts::RetentionDeleteState::HardDeleted
        ),
        requires_propagation: matches!(
            state,
            contracts::RetentionDeleteState::Propagated
                | contracts::RetentionDeleteState::ReplayProtected
                | contracts::RetentionDeleteState::AuditRetained
                | contracts::RetentionDeleteState::HardDeleted
        ),
        requires_replay_protection: matches!(
            state,
            contracts::RetentionDeleteState::ReplayProtected
                | contracts::RetentionDeleteState::AuditRetained
                | contracts::RetentionDeleteState::HardDeleted
        ),
        requires_minimal_audit: matches!(
            state,
            contracts::RetentionDeleteState::AuditRetained
                | contracts::RetentionDeleteState::HardDeleted
        ),
    }
}
