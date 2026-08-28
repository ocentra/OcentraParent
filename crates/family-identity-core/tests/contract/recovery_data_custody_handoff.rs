use ocentra_family_identity_core::family_identity::HouseholdRole;
use ocentra_family_identity_core::setup_lifecycle::{
    evaluate_recovery_operation, RecoveryDataCustodyHandoffState, RecoveryIdentityProofState,
    RecoveryKind, RecoveryOperation, RecoveryState, RecoverySupportChannel,
    SetupRecoveryAbuseState, SetupRecoveryResponseTimingState,
};

fn approved_parent_recovery(
    kind: RecoveryKind,
    delete_export_handoff_required: bool,
) -> RecoveryOperation {
    RecoveryOperation {
        requester_role: HouseholdRole::ParentOwner,
        same_family: true,
        kind,
        state: RecoveryState::Approved,
        owner_approval_required: false,
        identity_proof_state: RecoveryIdentityProofState::Verified,
        support_channel: RecoverySupportChannel::SelfServe,
        delete_export_handoff_required,
        abuse_state: SetupRecoveryAbuseState::WithinLimit,
        response_timing_state: SetupRecoveryResponseTimingState::Uniform,
    }
}

#[test]
fn forgot_login_delete_export_is_only_a_typed_data_custody_route() {
    let decision =
        evaluate_recovery_operation(approved_parent_recovery(RecoveryKind::ForgotLogin, true));
    assert_eq!(
        decision.data_custody_handoff_state,
        RecoveryDataCustodyHandoffState::ExportDeleteHandoffRequired
    );
    assert!(decision.failure_reason.is_none());
    assert_eq!(
        serde_json::to_value(decision).expect("serialize recovery decision")
            ["data_custody_handoff_state"],
        "export-delete-handoff-required"
    );
}

#[test]
fn household_transfer_uses_a_distinct_data_custody_handoff_kind() {
    let decision = evaluate_recovery_operation(approved_parent_recovery(
        RecoveryKind::HouseholdTransfer,
        false,
    ));
    assert_eq!(
        decision.data_custody_handoff_state,
        RecoveryDataCustodyHandoffState::HouseholdTransferHandoffRequired
    );
    assert_eq!(
        serde_json::to_value(decision).expect("serialize recovery decision")
            ["data_custody_handoff_state"],
        "household-transfer-handoff-required"
    );
}
