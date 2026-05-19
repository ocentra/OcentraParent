use serde::{Deserialize, Serialize};

use crate::constants::{ED25519_ALGORITHM, MSI_INSTALLER_TYPE, WINDOWS_X64_TARGET};
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
    if payload.installer.r#type != MSI_INSTALLER_TYPE {
        return Err(UpdaterError::Policy(format!(
            "unsupported update installer type: {}",
            payload.installer.r#type
        )));
    }
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

fn strip_utf8_bom(text: &str) -> &str {
    text.strip_prefix('\u{feff}').unwrap_or(text)
}
