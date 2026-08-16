use ocentra_parent_agent_maintenance::constants::{MSI_INSTALLER_TYPE, WINDOWS_X64_TARGET};
use ocentra_parent_agent_maintenance::crypto::generate_key_pair;
use ocentra_parent_agent_maintenance::error::UpdaterError;
use ocentra_parent_agent_maintenance::manifest::{
    parse_payload, sign_payload, verify_manifest, ArtifactManifest, InstallerManifest,
    ServiceManifest, UpdateManifestPayload,
};
const SAMPLE_ARTIFACT_SHA256: &str =
    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
const SAMPLE_ARTIFACT_URL: &str =
    "https://github.com/ocentra/OcentraParent/releases/download/v0.2.0/agent.msi";

#[test]
fn manifest_policy_rejects_unsupported_schema_target_and_installer() {
    let keys = generate_key_pair();

    let mut unsupported_schema = sample_payload();
    unsupported_schema.schema_version = 2;
    assert!(matches!(
        sign_payload(unsupported_schema, &keys.private_key_base64),
        Err(UpdaterError::Policy(message)) if message == "unsupported update manifest schema: 2"
    ));

    let mut unsupported_target = sample_payload();
    unsupported_target.target = String::from("linux-x64");
    assert!(matches!(
        sign_payload(unsupported_target, &keys.private_key_base64),
        Err(UpdaterError::Policy(message)) if message == "unsupported update target: linux-x64"
    ));

    let mut unsupported_installer = sample_payload();
    unsupported_installer.installer.r#type = String::from("zip");
    assert!(matches!(
        sign_payload(unsupported_installer, &keys.private_key_base64),
        Err(UpdaterError::Policy(message)) if message == "unsupported update installer type: zip"
    ));
}

#[test]
fn manifest_parser_rejects_unknown_fields_in_payload_contract() {
    let payload = serde_json::json!({
        "schemaVersion": 1,
        "product": "Ocentra Parent",
        "package": "ocentra-parent-agent",
        "version": "0.2.0",
        "channel": "stable",
        "target": WINDOWS_X64_TARGET,
        "installer": {
            "type": MSI_INSTALLER_TYPE,
            "scope": "per-machine",
            "silentArgs": "/qn /norestart",
            "passiveArgs": "/passive /norestart"
        },
        "service": {
            "id": "OcentraParentAgent",
            "name": "Ocentra Parent Agent",
            "wrapper": "WinSW",
            "wrapperVersion": "2.12.0",
            "updaterId": "OcentraParentUpdater",
            "updaterName": "Ocentra Parent Updater"
        },
        "artifact": {
            "name": "ocentra-parent-agent-windows-x64-v0.2.0.msi",
            "sha256": SAMPLE_ARTIFACT_SHA256,
            "downloadUrl": SAMPLE_ARTIFACT_URL
        },
        "generatedAt": "2026-05-19T00:00:00Z",
        "extraLocalTruth": "must-not-parse"
    });

    let result = parse_payload(&payload.to_string());

    assert!(matches!(
        result,
        Err(UpdaterError::Json(error)) if error.classify() == serde_json::error::Category::Data
    ));
}

#[test]
fn signed_manifest_round_trip_verifies_with_trusted_key() -> Result<(), UpdaterError> {
    let keys = generate_key_pair();
    let payload = sample_payload();
    let signed = sign_payload(payload.clone(), &keys.private_key_base64)?;
    let verified = verify_manifest(signed, &keys.public_key_base64)?;

    assert_eq!(verified.version, payload.version);
    assert_eq!(verified.artifact.sha256, payload.artifact.sha256);
    Ok(())
}

#[test]
fn signed_manifest_rejects_unsigned_payload_changes() -> Result<(), UpdaterError> {
    let keys = generate_key_pair();
    let mut signed = sign_payload(sample_payload(), &keys.private_key_base64)?;
    signed.payload.version = "9.9.9".to_owned();

    let result = verify_manifest(signed, &keys.public_key_base64);

    assert!(matches!(
        result,
        Err(UpdaterError::Crypto(message))
            if message.starts_with("manifest signature verification failed:")
    ));
    Ok(())
}

#[test]
fn payload_policy_rejects_non_github_artifacts() {
    let keys = generate_key_pair();
    let mut payload = sample_payload();
    payload.artifact.download_url = "https://example.test/agent.msi".to_owned();

    let result = sign_payload(payload, &keys.private_key_base64);

    assert!(matches!(
        result,
        Err(UpdaterError::Policy(message))
            if message == "artifact download URL must use GitHub HTTPS releases"
    ));
}

#[test]
fn payload_parser_accepts_windows_utf8_bom() -> Result<(), UpdaterError> {
    let json = serde_json::json!(sample_payload()).to_string();
    let text = format!("\u{feff}{json}");

    let parsed = parse_payload(&text)?;

    assert_eq!(parsed.version, "0.2.0");
    Ok(())
}

fn sample_payload() -> UpdateManifestPayload {
    UpdateManifestPayload {
        schema_version: 1,
        product: "Ocentra Parent".to_owned(),
        package: "ocentra-parent-agent".to_owned(),
        version: "0.2.0".to_owned(),
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
