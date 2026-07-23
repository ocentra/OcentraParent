use ocentra_family_identity_core::household_authority::{
    authorize_device_trust_action, DeviceTrustAuthorizationRequest, HouseholdAuthorityAction,
    HouseholdAuthorizationFailureReason,
};

#[test]
fn public_device_trust_authority_issuance_fails_closed_without_a_runtime_adapter() {
    assert_eq!(
        authorize_device_trust_action(DeviceTrustAuthorizationRequest {
            family_id: "family".to_owned(),
            parent_account_id: "parent-account".to_owned(),
            target_child_device_id: "child-device".to_owned(),
            action: HouseholdAuthorityAction::PairChildDevice,
        }),
        Err(HouseholdAuthorizationFailureReason::AuthorityAdapterUnavailable)
    );
}

#[test]
fn public_device_trust_authority_issuance_rejects_an_unbound_target_before_adapter_lookup() {
    assert_eq!(
        authorize_device_trust_action(DeviceTrustAuthorizationRequest {
            family_id: "family".to_owned(),
            parent_account_id: "parent-account".to_owned(),
            target_child_device_id: "   ".to_owned(),
            action: HouseholdAuthorityAction::PairChildDevice,
        }),
        Err(HouseholdAuthorizationFailureReason::ChildProfileNotBound)
    );
}
