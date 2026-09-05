#![forbid(unsafe_code)]

//! Bounded, complete startup reconciliation before bridge admission.

use std::time::{Duration, Instant};

use ocentra_family_identity_core::account_identity_authority::VerifiedAccountIdentityAuthority;
use ocentra_family_identity_core::account_identity_authority_repository::AccountIdentityAuthorityService;
use ocentra_family_identity_core::session_lifecycle_custody::parent_local_bridge_audit::ParentLocalBridgeStartupRecovery;

use super::ParentLocalBridgeAdmissionError;

const MAX_BATCHES: u64 = 64;
const MAX_RECOVERED_ROWS: u64 = 49_152;
const MAX_ELAPSED: Duration = Duration::from_secs(2);

pub(super) fn complete(
    account_owner: &mut AccountIdentityAuthorityService,
    current_authority: &VerifiedAccountIdentityAuthority,
) -> Result<(), ParentLocalBridgeAdmissionError> {
    let started = Instant::now();
    let mut recovered_rows = 0_u64;
    for _ in 0..MAX_BATCHES {
        require_elapsed_budget(&started)?;
        let recovery = account_owner
            .recover_parent_local_bridge_startup(current_authority)
            .map_err(|_error| ParentLocalBridgeAdmissionError::StartupRecoveryRejected)?;
        recovered_rows = add_recovered_rows(recovered_rows, &recovery)?;
        if recovered_rows > MAX_RECOVERED_ROWS {
            return Err(ParentLocalBridgeAdmissionError::StartupRecoveryRejected);
        }
        require_elapsed_budget(&started)?;
        if !recovery.more_recovery_work() {
            return Ok(());
        }
    }
    Err(ParentLocalBridgeAdmissionError::StartupRecoveryRejected)
}

fn add_recovered_rows(
    accumulated: u64,
    recovery: &ParentLocalBridgeStartupRecovery,
) -> Result<u64, ParentLocalBridgeAdmissionError> {
    let batch = recovery
        .expired_claims_requeued()
        .checked_add(recovery.terminal_sessions_removed())
        .and_then(|rows| rows.checked_add(recovery.delivered_audits_removed()))
        .ok_or(ParentLocalBridgeAdmissionError::StartupRecoveryRejected)?;
    accumulated
        .checked_add(batch)
        .ok_or(ParentLocalBridgeAdmissionError::StartupRecoveryRejected)
}

fn require_elapsed_budget(started: &Instant) -> Result<(), ParentLocalBridgeAdmissionError> {
    (started.elapsed() < MAX_ELAPSED)
        .then_some(())
        .ok_or(ParentLocalBridgeAdmissionError::StartupRecoveryRejected)
}
