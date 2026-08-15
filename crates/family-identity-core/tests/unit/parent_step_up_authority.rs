use ocentra_family_identity_core::household_authority::HouseholdAuthorityAction;
use ocentra_family_identity_core::parent_step_up_authority::{
    verify_parent_step_up_receipt, ParentStepUpAuthorityFailure, ParentStepUpAuthorityRequest,
    UnavailableParentStepUpAuthorityVerifier,
};
use ocentra_schema::parent_step_up_receipt::{
    ParentStepUpAuthorityReceipt, PARENT_STEP_UP_RECEIPT_SCHEMA_VERSION,
};

fn request() -> ParentStepUpAuthorityRequest {
    ParentStepUpAuthorityRequest {
        issuer: "account-authority".to_owned(),
        audience: "ocentra-parent".to_owned(),
        family_id: "family-1".to_owned(),
        parent_account_id: "parent-1".to_owned(),
        action_device_id: "device-1".to_owned(),
        action_device_child_profile_id: None,
        target_child_profile_id: Some("child-1".to_owned()),
        action: HouseholdAuthorityAction::PairChildDevice,
        expected_nonce: "nonce-1".to_owned(),
        observed_at: "2026-08-05T23:00:01.000Z".to_owned(),
    }
}

fn receipt() -> ParentStepUpAuthorityReceipt {
    ParentStepUpAuthorityReceipt {
        schema_version: PARENT_STEP_UP_RECEIPT_SCHEMA_VERSION.to_owned(),
        receipt_id: "receipt-1".to_owned(),
        issuer: "account-authority".to_owned(),
        audience: "ocentra-parent".to_owned(),
        key_id: "key-1".to_owned(),
        family_id: "family-1".to_owned(),
        parent_account_id: "parent-1".to_owned(),
        action_device_id: "device-1".to_owned(),
        action_device_child_profile_id: None,
        target_child_profile_id: Some("child-1".to_owned()),
        action: "pair-child-device".to_owned(),
        nonce: "nonce-1".to_owned(),
        issued_at: "2026-08-05T22:59:59.000Z".to_owned(),
        expires_at: "2026-08-05T23:05:00.000Z".to_owned(),
        signature: "encoded-signature".to_owned(),
    }
}

#[test]
fn production_adapter_never_authorizes_self_reported_receipt() {
    let mut verifier = UnavailableParentStepUpAuthorityVerifier;
    assert_eq!(
        verify_parent_step_up_receipt(&mut verifier, &receipt(), &request()),
        Err(ParentStepUpAuthorityFailure::AuthorityUnavailable)
    );
}

#[test]
fn malformed_receipt_is_rejected_before_provider_call() {
    let mut verifier = UnavailableParentStepUpAuthorityVerifier;
    let mut malformed = receipt();
    malformed.nonce = "wrong-nonce".to_owned();
    assert_eq!(
        verify_parent_step_up_receipt(&mut verifier, &malformed, &request()),
        Err(ParentStepUpAuthorityFailure::InvalidReceiptShape)
    );
}

#[test]
fn noncanonical_timestamp_is_rejected_before_provider_call() {
    let mut verifier = UnavailableParentStepUpAuthorityVerifier;
    let mut malformed = receipt();
    malformed.expires_at = "2026-08-05T23:05:00+00:00".to_owned();
    assert_eq!(
        verify_parent_step_up_receipt(&mut verifier, &malformed, &request()),
        Err(ParentStepUpAuthorityFailure::TimestampInvalid)
    );
}

#[test]
fn future_issued_timestamp_is_rejected_before_provider_call() {
    let mut verifier = UnavailableParentStepUpAuthorityVerifier;
    let mut malformed = receipt();
    malformed.issued_at = "2026-08-05T23:00:02.000Z".to_owned();
    assert_eq!(
        verify_parent_step_up_receipt(&mut verifier, &malformed, &request()),
        Err(ParentStepUpAuthorityFailure::InvalidReceiptShape)
    );
}
