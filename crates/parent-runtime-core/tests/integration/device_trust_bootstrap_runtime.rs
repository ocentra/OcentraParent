#![cfg(windows)]

use ocentra_eventing::ids::CorrelationId;
use ocentra_family_identity_core::{
    household_authority::{HouseholdAuthorityAction, ParentStepUpAssertionSnapshot},
    parent_presence::{
        ParentPresenceChallenge, ParentPresenceVerificationInput, ParentPresenceVerificationPort,
    },
};
use ocentra_parent_runtime_core::device_trust_bootstrap_runtime::{
    ParentDeviceTrustBootstrapError, ParentDeviceTrustCommandError, ParentDeviceTrustCommandFacade,
};
use ocentra_parent_runtime_core::parent_ui_bridge::dispatch_parent_ui_action_with_device_trust;
use ocentra_schema::parent_ui_bridge::{ParentRouteId, ParentUiAction, ParentUiActionKind};
use ocentra_storage_custody_core::windows_device_trust_custody::Error as CustodyError;
use serde_json::json;

fn test_failure(error: impl std::fmt::Debug) -> String {
    format!("{error:?}")
}

#[test]
fn parent_dispatch_seals_a_staged_verified_ceremony_and_rejects_a_restored_record(
) -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "ocentra-parent-runtime-device-trust-{}",
        std::process::id()
    ));
    let _cleanup = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).map_err(test_failure)?;
    let mut parent_presence = ParentPresenceVerificationPort::open_unsealed_test_custody_at(
        root.join("parent-presence.sqlite"),
        "2026-08-05T00:00:00.000Z",
    )
    .map_err(test_failure)?;
    let challenge = ParentPresenceChallenge {
        challenge_ref: "runtime-seal-challenge".into(),
        nonce_ref: "runtime-seal-nonce".into(),
        family_id: "family-runtime".into(),
        parent_account_id: "parent-runtime".into(),
        privileged_action: HouseholdAuthorityAction::SealParentDeviceTrust,
        action_device_id: "parent-runtime-device".into(),
        action_device_child_profile_id: None,
        target_child_profile_id: None,
        expires_at: "2099-01-01T00:00:00.000Z".into(),
    };
    parent_presence
        .issue_challenge(challenge.clone())
        .map_err(test_failure)?;
    let accepted = parent_presence
        .verify_and_consume(ParentPresenceVerificationInput {
            correlation_id: CorrelationId::parse("parent-runtime-device-trust")
                .map_err(test_failure)?,
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
    let facade = ParentDeviceTrustCommandFacade::open(root.join("sealed")).map_err(test_failure)?;
    let ceremony_ref = facade
        .stage_accepted_parent_device_trust_ceremony("runtime-bootstrap".into(), accepted)
        .map_err(test_failure)?;
    let action = ParentUiAction {
        action: ParentUiActionKind::DeviceTrustSealStagedCeremonyRequested,
        route: ParentRouteId::Approvals,
        context: None,
        command: None,
        payload: json!({ "ceremonyRef": ceremony_ref.as_str() }),
    };

    assert!(dispatch_parent_ui_action_with_device_trust(&action, &facade).accepted);
    assert!(matches!(
        facade
            .unseal_current_parent_device_trust(
                "family-runtime",
                "parent-runtime",
                "parent-runtime-device",
            )
            .map_err(test_failure)?,
        material if material.len() == 32
    ));
    assert!(!dispatch_parent_ui_action_with_device_trust(&action, &facade).accepted);

    let record = sealed_record(&root)?;
    let copied_record = std::fs::read(&record).map_err(test_failure)?;
    facade
        .revoke_or_reset_parent_device_trust(
            "family-runtime",
            "parent-runtime",
            "parent-runtime-device",
        )
        .map_err(test_failure)?;
    std::fs::write(&record, copied_record).map_err(test_failure)?;
    assert_eq!(
        facade.unseal_current_parent_device_trust(
            "family-runtime",
            "parent-runtime",
            "parent-runtime-device",
        ),
        Err(ParentDeviceTrustCommandError::Runtime(
            ParentDeviceTrustBootstrapError::Custody(CustodyError::Missing)
        ))
    );
    let _cleanup = std::fs::remove_dir_all(root);
    Ok(())
}

fn sealed_record(root: &std::path::Path) -> Result<std::path::PathBuf, String> {
    for entry in std::fs::read_dir(root.join("sealed")).map_err(test_failure)? {
        let path = entry.map_err(test_failure)?.path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("sealed") {
            return Ok(path);
        }
    }
    Err("sealed record path unavailable".into())
}
