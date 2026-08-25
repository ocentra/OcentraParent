use std::io;

use serde::{Deserialize, Serialize};

use super::{
    ChildAgentCleanupState, ChildAgentRemovalAction, ChildAgentRemovalAuditEntry,
    ChildAgentRemovalStatus, ChildAgentServiceIdentity, ChildAgentTamperSignal,
    ChildAgentTrustState,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct ChildAgentRemovalRecord {
    pub(super) version: u16,
    pub(super) trust_state: ChildAgentTrustState,
    pub(super) cleanup_state: ChildAgentCleanupState,
    pub(super) audit: Vec<ChildAgentRemovalAuditEntry>,
    #[serde(default)]
    pub(super) tamper_signals: Vec<ChildAgentTamperSignal>,
}

pub(super) fn empty_record() -> ChildAgentRemovalRecord {
    ChildAgentRemovalRecord {
        version: super::REMOVAL_STATE_VERSION,
        trust_state: ChildAgentTrustState::Active,
        cleanup_state: ChildAgentCleanupState::NotRequired,
        audit: Vec::new(),
        tamper_signals: Vec::new(),
    }
}

pub(super) fn append_audit(
    record: &mut ChildAgentRemovalRecord,
    action: ChildAgentRemovalAction,
    parent_authorization_ref: String,
    authorization_identity: ChildAgentServiceIdentity,
) -> io::Result<()> {
    let sequence = record.audit.len() + 1;
    record.audit.push(ChildAgentRemovalAuditEntry {
        audit_ref: format!("child-removal-audit-{sequence}"),
        action,
        parent_authorization_ref,
        household_id: authorization_identity.household_id,
        child_profile_id: authorization_identity.child_profile_id,
        target_device_id: authorization_identity.target_device_id,
        recorded_at_unix_seconds: super::removal_validation::current_unix_seconds()?,
    });
    Ok(())
}

pub(super) fn status_from_record(record: &ChildAgentRemovalRecord) -> ChildAgentRemovalStatus {
    let latest = record.audit.last();
    ChildAgentRemovalStatus {
        trust_state: record.trust_state.clone(),
        cleanup_state: record.cleanup_state.clone(),
        latest_audit_ref: latest.map(|entry| entry.audit_ref.clone()),
        latest_parent_authorization_ref: latest.map(|entry| entry.parent_authorization_ref.clone()),
        audit_entry_count: record.audit.len(),
        latest_tamper_signal_ref: record
            .tamper_signals
            .last()
            .map(|signal| signal.signal_ref.clone()),
        tamper_signal_count: record.tamper_signals.len(),
    }
}
