use ocentra_parent_agent_maintenance::constants::{MSI_INSTALLER_TYPE, WINDOWS_X64_TARGET};
use ocentra_parent_agent_maintenance::crypto::generate_key_pair;
use ocentra_parent_agent_maintenance::error::UpdaterError;
use ocentra_parent_agent_maintenance::manifest::{
    parse_payload, sign_payload, ArtifactManifest, InstallerManifest, ServiceManifest,
    UpdateManifestPayload,
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

fn sample_payload() -> UpdateManifestPayload {
    UpdateManifestPayload {
        schema_version: 1,
        product: String::from("Ocentra Parent"),
        package: String::from("ocentra-parent-agent"),
        version: String::from("0.2.0"),
        channel: String::from("stable"),
        target: String::from(WINDOWS_X64_TARGET),
        installer: InstallerManifest {
            r#type: String::from(MSI_INSTALLER_TYPE),
            scope: String::from("per-machine"),
            silent_args: String::from("/qn /norestart"),
            passive_args: String::from("/passive /norestart"),
        },
        service: ServiceManifest {
            id: String::from("OcentraParentAgent"),
            name: String::from("Ocentra Parent Agent"),
            wrapper: String::from("WinSW"),
            wrapper_version: String::from("2.12.0"),
            updater_id: String::from("OcentraParentUpdater"),
            updater_name: String::from("Ocentra Parent Updater"),
        },
        artifact: ArtifactManifest {
            name: String::from("ocentra-parent-agent-windows-x64-v0.2.0.msi"),
            sha256: String::from(SAMPLE_ARTIFACT_SHA256),
            download_url: String::from(SAMPLE_ARTIFACT_URL),
        },
        generated_at: String::from("2026-05-19T00:00:00Z"),
    }
}
