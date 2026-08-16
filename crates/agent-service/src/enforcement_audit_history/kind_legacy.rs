use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAuditEventKind, EnforcementAuditJournalEvent, EnforcementResultStatus,
};

use super::super::EnforcementAuditHistoryKind;

pub(super) fn for_event(event: &EnforcementAuditJournalEvent) -> EnforcementAuditHistoryKind {
    if event.started_at.is_some() || event.completed_at.is_some() {
        return super::adapter_result(event);
    }
    match (event.audit_event_kind, event.result_status) {
        (EnforcementAuditEventKind::Failed, EnforcementResultStatus::Failed) => {
            EnforcementAuditHistoryKind::RejectedIntent
        }
        (EnforcementAuditEventKind::Attempted, EnforcementResultStatus::WouldEnforce) => {
            EnforcementAuditHistoryKind::AcceptedIntent
        }
        (EnforcementAuditEventKind::Expired, EnforcementResultStatus::WouldEnforce) => {
            EnforcementAuditHistoryKind::TimerExpired
        }
        _ => super::adapter_result(event),
    }
}
