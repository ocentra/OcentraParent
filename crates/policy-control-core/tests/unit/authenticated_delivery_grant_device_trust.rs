use super::authenticated_delivery_grant::IssuanceFixture;
use super::authenticated_delivery_grant_fixture::{
    current_household_authority_state, issuer_with_current_state_and_device_trust,
};
use super::TestResult;
use ocentra_family_identity_core::family_identity::DeviceTrustState;
use ocentra_family_identity_core::parent_step_up_proof::ParentDeviceTrustCurrentState;
use ocentra_policy_control_core::authenticated_delivery_grant::AuthenticatedDeliveryGrantIssuanceError;
use std::sync::{Arc, Mutex};

#[test]
fn issuer_accepts_current_trusted_device_and_rejects_the_same_proof_after_revocation() -> TestResult
{
    let state = Arc::new(Mutex::new(ParentDeviceTrustCurrentState {
        parent_device_id: "parent-device-1".to_owned(),
        trust_state: DeviceTrustState::Trusted,
        revocation_epoch: 0,
    }));
    let resolver = Arc::clone(&state);
    let issuer = test_ok!(
        issuer_with_current_state_and_device_trust(current_household_authority_state, move || {
            test_ok!(resolver.lock(), "current device trust state").clone()
        },),
        "issuer with live device trust state"
    );
    let accepted = test_ok!(
        issuer.issue(IssuanceFixture::new().request()),
        "current trusted device issuance"
    );
    assert_eq!(accepted.parent_device_id, "parent-device-1");
    *test_ok!(state.lock(), "current device trust state") = ParentDeviceTrustCurrentState {
        parent_device_id: "parent-device-1".to_owned(),
        trust_state: DeviceTrustState::Revoked,
        revocation_epoch: 1,
    };
    assert_eq!(
        issuer.issue(IssuanceFixture::new().request()),
        Err(AuthenticatedDeliveryGrantIssuanceError::ParentStepUpRejected)
    );
    Ok(())
}
