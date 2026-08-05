#![cfg(windows)]

use ocentra_eventing::ids::CorrelationId;
use ocentra_family_identity_core::{
    household_authority::{HouseholdAuthorityAction, ParentStepUpAssertionSnapshot},
    parent_presence::{
        ParentPresenceChallenge, ParentPresenceVerificationInput, ParentPresenceVerificationPort,
    },
    trust_bootstrap::{begin_parent_device_key_sealing, TrustBootstrapDecision},
};
use ocentra_storage_custody_core::windows_device_trust_custody::{
    Error, WindowsDeviceTrustCustody,
};

fn test_failure(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

#[test]
fn verified_parent_ceremony_persists_before_epoch_activation_and_revocation_rejects_restored_record(
) -> Result<(), String> {
    let root = std::env::temp_dir().join(format!("ocentra-wp02-{}", std::process::id()));
    let _cleanup = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).map_err(test_failure)?;
    let mut port = ParentPresenceVerificationPort::open_unsealed_test_custody_at(
        root.join("presence.sqlite"),
        "2026-08-05T00:00:00.000Z",
    )
    .map_err(test_failure)?;
    let challenge = ParentPresenceChallenge {
        challenge_ref: "seal-challenge".into(),
        nonce_ref: "seal-nonce".into(),
        family_id: "family-a".into(),
        parent_account_id: "parent-a".into(),
        privileged_action: HouseholdAuthorityAction::SealParentDeviceTrust,
        action_device_id: "parent-device-a".into(),
        action_device_child_profile_id: None,
        target_child_profile_id: None,
        expires_at: "2099-01-01T00:00:00.000Z".into(),
    };
    port.issue_challenge(challenge.clone())
        .map_err(test_failure)?;
    let accepted = port
        .verify_and_consume(ParentPresenceVerificationInput {
            correlation_id: CorrelationId::parse("wp02-windows-custody").map_err(test_failure)?,
            challenge_ref: challenge.challenge_ref.clone(),
            assertion: ParentStepUpAssertionSnapshot {
                family_id: challenge.family_id.clone(),
                parent_account_id: challenge.parent_account_id.clone(),
                action_device_id: challenge.action_device_id.clone(),
                action_device_child_profile_id: None,
                target_child_profile_id: None,
                action: HouseholdAuthorityAction::SealParentDeviceTrust,
                nonce: challenge.nonce_ref.clone(),
                expires_at: challenge.expires_at,
            },
        })
        .map_err(test_failure)?;
    let TrustBootstrapDecision::AwaitingPlatformKeySealing(request) =
        begin_parent_device_key_sealing("bootstrap-a".into(), accepted)
    else {
        return Err("approved ceremony did not request sealing".into());
    };
    let custody = WindowsDeviceTrustCustody::open(root.join("sealed")).map_err(test_failure)?;
    custody
        .seal_persist_activate(request, b"trust-material")
        .map_err(test_failure)?;
    assert_eq!(
        custody
            .unseal_current("family-a", "parent-a", "parent-device-a")
            .map_err(test_failure)?,
        b"trust-material"
    );
    assert_eq!(
        custody.unseal_current("family-a", "parent-a", "other-parent-device"),
        Err(Error::Missing)
    );

    let mut record = None;
    for entry in std::fs::read_dir(root.join("sealed")).map_err(test_failure)? {
        let path = entry.map_err(test_failure)?.path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("sealed") {
            record = Some(path);
        }
    }
    let record = record.ok_or_else(|| "sealed record path unavailable".to_owned())?;
    let copied_record = std::fs::read(&record).map_err(test_failure)?;
    custody
        .revoke_or_reset("family-a", "parent-a", "parent-device-a")
        .map_err(test_failure)?;
    std::fs::write(&record, copied_record).map_err(test_failure)?;
    assert_eq!(
        custody.unseal_current("family-a", "parent-a", "parent-device-a"),
        Err(Error::Missing)
    );
    let _cleanup = std::fs::remove_dir_all(root);
    Ok(())
}
