use ocentra_parent_agent_maintenance::constants::{MSI_INSTALLER_TYPE, WINDOWS_X64_TARGET};
use ocentra_parent_agent_maintenance::crypto::generate_key_pair;
use ocentra_parent_agent_maintenance::error::UpdaterError;
use ocentra_parent_agent_maintenance::manifest::{
    parse_payload, sign_payload, verify_manifest, ArtifactManifest, InstallerManifest,
    ServiceManifest, UpdateManifestPayload,
};

#[test]
fn signed_manifest_round_trip_verifies_with_trusted_key() {
    let keys = generate_key_pair();
    let payload = sample_payload("0.2.0");
    let signed_result = sign_payload(payload.clone(), &keys.private_key_base64);
    assert!(
        signed_result.is_ok(),
        "manifest signs failed: {signed_result:?}"
    );
    let Ok(signed) = signed_result else {
        return;
    };

    let verified_result = verify_manifest(signed, &keys.public_key_base64);
    assert!(
        verified_result.is_ok(),
        "manifest verifies failed: {verified_result:?}"
    );
    let Ok(verified) = verified_result else {
        return;
    };

    assert_eq!(verified.version, payload.version);
    assert_eq!(verified.artifact.sha256, payload.artifact.sha256);
}

#[test]
fn signed_manifest_rejects_unsigned_payload_changes() {
    let keys = generate_key_pair();
    let signed_result = sign_payload(sample_payload("0.2.0"), &keys.private_key_base64);
    assert!(
        signed_result.is_ok(),
        "manifest signs failed: {signed_result:?}"
    );
    let Ok(mut signed) = signed_result else {
        return;
    };
    signed.payload.version = "9.9.9".to_owned();

    let result = verify_manifest(signed, &keys.public_key_base64);

    assert!(matches!(
        result,
        Err(UpdaterError::Crypto(message))
            if message.starts_with("manifest signature verification failed:")
    ));
}

#[test]
fn payload_policy_rejects_non_github_artifacts() {
    let keys = generate_key_pair();
    let mut payload = sample_payload("0.2.0");
    payload.artifact.download_url = "https://example.test/agent.msi".to_owned();

    let result = sign_payload(payload, &keys.private_key_base64);

    assert!(matches!(
        result,
        Err(UpdaterError::Policy(message))
            if message == "artifact download URL must use GitHub HTTPS releases"
    ));
}

#[test]
fn payload_parser_accepts_windows_utf8_bom() {
    let json_result = serde_json::to_string(&sample_payload("0.2.0"));
    assert!(
        json_result.is_ok(),
        "json serialization failed: {json_result:?}"
    );
    let Ok(json) = json_result else {
        return;
    };
    let text = format!("\u{feff}{json}");

    let parsed_result = parse_payload(&text);
    assert!(
        parsed_result.is_ok(),
        "payload parses failed: {parsed_result:?}"
    );
    let Ok(parsed) = parsed_result else {
        return;
    };

    assert_eq!(parsed.version, "0.2.0");
}

fn sample_payload(version: &str) -> UpdateManifestPayload {
    UpdateManifestPayload {
        schema_version: 1,
        product: "Ocentra Parent".to_owned(),
        package: "ocentra-parent-agent".to_owned(),
        version: version.to_owned(),
        channel: "stable".to_owned(),
        target: WINDOWS_X64_TARGET.to_owned(),
        installer: InstallerManifest {
            r#type: MSI_INSTALLER_TYPE.to_owned(),
            scope: "per-machine".to_owned(),
            silent_args: "/qn /norestart".to_owned(),
            passive_args: "/passive /norestart".to_owned(),
        },
        service: ServiceManifest {
            id: "OcentraParentAgent".to_owned(),
            name: "Ocentra Parent Agent".to_owned(),
            wrapper: "WinSW".to_owned(),
            wrapper_version: "2.12.0".to_owned(),
            updater_id: "OcentraParentUpdater".to_owned(),
            updater_name: "Ocentra Parent Updater".to_owned(),
        },
        artifact: ArtifactManifest {
            name: "ocentra-parent-agent-windows-x64-v0.2.0.msi".to_owned(),
            sha256: "A".repeat(64),
            download_url:
                "https://github.com/ocentra/OcentraParent/releases/download/v0.2.0/agent.msi"
                    .to_owned(),
        },
        generated_at: "2026-05-19T00:00:00Z".to_owned(),
    }
}
