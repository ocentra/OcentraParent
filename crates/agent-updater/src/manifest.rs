use serde::{Deserialize, Serialize};

use crate::constants::{
    CHILD_PACKAGE, CHILD_PRODUCT, CHILD_SERVICE_ID, CHILD_UPDATER_ID, ED25519_ALGORITHM,
    MSI_INSTALLER_TYPE, WINDOWS_X64_TARGET,
};
use crate::crypto::{sign_bytes, verify_bytes};
use crate::error::UpdaterError;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SignedUpdateManifest {
    pub payload: UpdateManifestPayload,
    pub signature: ManifestSignature,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UpdateManifestPayload {
    pub schema_version: u32,
    pub product: String,
    pub package: String,
    pub version: String,
    pub channel: String,
    pub target: String,
    pub installer: InstallerManifest,
    pub service: ServiceManifest,
    pub artifact: ArtifactManifest,
    pub generated_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InstallerManifest {
    pub r#type: String,
    pub scope: String,
    pub silent_args: String,
    pub passive_args: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ServiceManifest {
    pub id: String,
    pub name: String,
    pub wrapper: String,
    pub wrapper_version: String,
    pub updater_id: String,
    pub updater_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactManifest {
    pub name: String,
    pub sha256: String,
    pub download_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ManifestSignature {
    pub algorithm: String,
    pub key_id: String,
    pub value: String,
}

pub fn sign_payload(
    payload: UpdateManifestPayload,
    private_key_base64: &str,
) -> Result<SignedUpdateManifest, UpdaterError> {
    validate_payload_policy(&payload)?;
    let bytes = canonical_payload_bytes(&payload)?;
    let (value, key_id) = sign_bytes(&bytes, private_key_base64)?;
    Ok(SignedUpdateManifest {
        payload,
        signature: ManifestSignature {
            algorithm: ED25519_ALGORITHM.to_owned(),
            key_id,
            value,
        },
    })
}

pub fn verify_manifest(
    manifest: SignedUpdateManifest,
    public_key_base64: &str,
) -> Result<UpdateManifestPayload, UpdaterError> {
    validate_payload_policy(&manifest.payload)?;
    let bytes = canonical_payload_bytes(&manifest.payload)?;
    verify_bytes(
        &bytes,
        &manifest.signature.value,
        public_key_base64,
        &manifest.signature.key_id,
        &manifest.signature.algorithm,
    )?;
    Ok(manifest.payload)
}

pub fn parse_signed_manifest(text: &str) -> Result<SignedUpdateManifest, UpdaterError> {
    Ok(serde_json::from_str(strip_utf8_bom(text))?)
}

pub fn parse_payload(text: &str) -> Result<UpdateManifestPayload, UpdaterError> {
    Ok(serde_json::from_str(strip_utf8_bom(text))?)
}

pub fn canonical_payload_bytes(payload: &UpdateManifestPayload) -> Result<Vec<u8>, UpdaterError> {
    Ok(serde_json::to_vec(payload)?)
}

fn validate_payload_policy(payload: &UpdateManifestPayload) -> Result<(), UpdaterError> {
    if payload.schema_version != 1 {
        return Err(UpdaterError::Policy(format!(
            "unsupported update manifest schema: {}",
            payload.schema_version
        )));
    }
    if payload.target != WINDOWS_X64_TARGET {
        return Err(UpdaterError::Policy(format!(
            "unsupported update target: {}",
            payload.target
        )));
    }
    require_field(&payload.product, CHILD_PRODUCT, "child product")?;
    require_field(&payload.package, CHILD_PACKAGE, "child package")?;
    require_field(&payload.channel, "stable", "update channel")?;
    if payload.installer.r#type != MSI_INSTALLER_TYPE {
        return Err(UpdaterError::Policy(format!(
            "unsupported update installer type: {}",
            payload.installer.r#type
        )));
    }
    require_field(&payload.installer.scope, "per-machine", "installer scope")?;
    require_field(
        &payload.installer.silent_args,
        "/qn /norestart",
        "silent installer arguments",
    )?;
    require_field(
        &payload.installer.passive_args,
        "/passive /norestart",
        "passive installer arguments",
    )?;
    require_field(&payload.service.id, CHILD_SERVICE_ID, "child service id")?;
    require_field(
        &payload.service.updater_id,
        CHILD_UPDATER_ID,
        "child updater id",
    )?;
    require_field(
        &payload.service.name,
        CHILD_PRODUCT,
        "child service display name",
    )?;
    require_field(
        &payload.service.updater_name,
        "Ocentra Child Updater",
        "child updater display name",
    )?;
    require_non_empty(&payload.service.wrapper, "service wrapper")?;
    require_non_empty(&payload.service.wrapper_version, "service wrapper version")?;
    require_non_empty(&payload.version, "manifest version")?;
    require_non_empty(&payload.generated_at, "manifest generatedAt")?;
    require_artifact_name(&payload.artifact.name)?;
    require_sha256(&payload.artifact.sha256)?;
    if !payload
        .artifact
        .download_url
        .starts_with("https://github.com/")
    {
        return Err(UpdaterError::Policy(
            "artifact download URL must use GitHub HTTPS releases".to_owned(),
        ));
    }
    Ok(())
}

fn require_field(actual: &str, expected: &str, field: &str) -> Result<(), UpdaterError> {
    if actual != expected {
        return Err(UpdaterError::Policy(format!(
            "{field} must be {expected}, found {actual}"
        )));
    }
    Ok(())
}

fn require_non_empty(value: &str, field: &str) -> Result<(), UpdaterError> {
    if value.trim().is_empty() {
        return Err(UpdaterError::Policy(format!("{field} must not be empty")));
    }
    Ok(())
}

fn require_artifact_name(value: &str) -> Result<(), UpdaterError> {
    require_non_empty(value, "artifact name")?;
    if value.contains('/') || value.contains('\\') || value == "." || value == ".." {
        return Err(UpdaterError::Policy(
            "artifact name must be a single safe file name".to_owned(),
        ));
    }
    if !value.ends_with(".msi") {
        return Err(UpdaterError::Policy(
            "Windows child update artifact must be an .msi file".to_owned(),
        ));
    }
    Ok(())
}

pub fn require_sha256(value: &str) -> Result<(), UpdaterError> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(UpdaterError::Policy(
            "artifact sha256 must be exactly 64 hexadecimal characters".to_owned(),
        ));
    }
    Ok(())
}

fn strip_utf8_bom(text: &str) -> &str {
    text.strip_prefix('\u{feff}').unwrap_or(text)
}
