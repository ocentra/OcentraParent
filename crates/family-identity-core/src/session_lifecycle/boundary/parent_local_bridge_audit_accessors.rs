#![forbid(unsafe_code)]

//! Read-only startup recovery report accessors.

use super::parent_local_bridge_audit::ParentLocalBridgeStartupRecovery;

impl ParentLocalBridgeStartupRecovery {
    pub fn expired_claims_requeued(&self) -> u64 {
        self.expired_claims_requeued
    }

    pub fn terminal_sessions_removed(&self) -> u64 {
        self.terminal_sessions_removed
    }

    pub fn delivered_audits_removed(&self) -> u64 {
        self.delivered_audits_removed
    }

    pub fn more_recovery_work(&self) -> bool {
        self.more_recovery_work
    }
}
