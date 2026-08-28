use ocentra_family_identity_core::account_identity_authority_repository::invite_recovery_repository::
    RecoveryOwnerEffect;

#[test]
fn effect_taxonomy_remains_explicit_and_non_collapsible() {
    let effects = [
        RecoveryOwnerEffect::ProviderCredentialSession,
        RecoveryOwnerEffect::DeviceTrustRevoke,
        RecoveryOwnerEffect::DeviceTrustReinstall,
        RecoveryOwnerEffect::HouseholdAuthorityMutation,
    ];
    let labels = effects
        .into_iter()
        .map(|effect| format!("{effect:?}"))
        .collect::<Vec<_>>();

    assert_eq!(
        labels,
        vec![
            "ProviderCredentialSession".to_owned(),
            "DeviceTrustRevoke".to_owned(),
            "DeviceTrustReinstall".to_owned(),
            "HouseholdAuthorityMutation".to_owned(),
        ]
    );
}
