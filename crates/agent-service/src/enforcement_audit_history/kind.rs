use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAdapterKind, EnforcementAuditEventKind, EnforcementAuditJournalEvent,
    EnforcementAuditJournalProvenance,
};

use super::EnforcementAuditHistoryKind;

#[path = "kind_legacy.rs"]
mod legacy;

pub(super) fn for_event(event: &EnforcementAuditJournalEvent) -> EnforcementAuditHistoryKind {
    match event.provenance {
        EnforcementAuditJournalProvenance::Legacy => legacy::for_event(event),
        EnforcementAuditJournalProvenance::RejectedIntent => {
            EnforcementAuditHistoryKind::RejectedIntent
        }
        EnforcementAuditJournalProvenance::AcceptedIntent => {
            EnforcementAuditHistoryKind::AcceptedIntent
        }
        EnforcementAuditJournalProvenance::AdapterResult => adapter_result(event),
    }
}

fn adapter_result(event: &EnforcementAuditJournalEvent) -> EnforcementAuditHistoryKind {
    if event.adapter_kind != EnforcementAdapterKind::TimerControl {
        return EnforcementAuditHistoryKind::AdapterResult;
    }
    match event.audit_event_kind {
        EnforcementAuditEventKind::Expired => EnforcementAuditHistoryKind::TimerExpired,
        EnforcementAuditEventKind::RollbackRequested
        | EnforcementAuditEventKind::RollbackCompleted => {
            EnforcementAuditHistoryKind::TimerRollback
        }
        EnforcementAuditEventKind::Cancelled => EnforcementAuditHistoryKind::TimerCancelled,
        _ => EnforcementAuditHistoryKind::AdapterResult,
    }
}
