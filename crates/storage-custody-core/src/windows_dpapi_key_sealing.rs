//! Windows-only local sealing for device-trust material.
//!
//! This adapter persists only DPAPI ciphertext plus non-secret binding metadata. It
//! has no plaintext or portable-key fallback: unavailable platform custody requires
//! recovery and re-pair through the owning flows.

use std::fmt;

use ocentra_family_identity_core::trust_bootstrap::{
    current_authority::{
        require_current_parent_device_trust_authority, CurrentParentDeviceTrustAuthoritySource,
    },
    AwaitingPlatformKeySealingRequest, PersistedPlatformKeyUnsealingCredential,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const SEALED_KEY_FORMAT_VERSION: u8 = 1;
#[cfg(windows)]
const WINDOWS_MACHINE_CRYPTOGRAPHY_PATH: &str = "SOFTWARE\\Microsoft\\Cryptography";
#[cfg(windows)]
const WINDOWS_MACHINE_GUID_VALUE: &str = "MachineGuid";

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DpapiKeySealingContext {
    pub family_id: String,
    pub trust_subject: String,
    pub device_ref: String,
    pub device_role: String,
    pub lifecycle_generation: u64,
    pub installation_binding_generation: u64,
}

impl DpapiKeySealingContext {
    fn from_approved_ceremony(
        credential: &PersistedPlatformKeyUnsealingCredential,
        authority: ocentra_family_identity_core::trust_bootstrap::current_authority::CurrentParentDeviceTrustAuthority,
    ) -> Self {
        let ceremony = credential.approved_parent_device_ceremony();
        Self {
            family_id: ceremony.family_id.to_owned(),
            trust_subject: ceremony.trust_subject().to_owned(),
            device_ref: ceremony.device_ref().to_owned(),
            device_role: ceremony.device_role().to_owned(),
            lifecycle_generation: authority.lifecycle_generation,
            installation_binding_generation: authority.installation_binding_generation,
        }
    }
    fn validate(&self) -> Result<(), DpapiKeySealingError> {
        if self.family_id.trim().is_empty()
            || self.trust_subject.trim().is_empty()
            || self.device_ref.trim().is_empty()
            || self.device_role.trim().is_empty()
            || self.lifecycle_generation == 0
            || self.installation_binding_generation == 0
        {
            return Err(DpapiKeySealingError::InvalidBinding);
        }
        Ok(())
    }

    #[cfg(windows)]
    fn entropy(
        &self,
        credential: &PersistedPlatformKeyUnsealingCredential,
        device_local_binding: &[u8; 32],
    ) -> Vec<u8> {
        hex_encode(&authorization_binding(
            self,
            credential,
            device_local_binding,
        ))
        .into_bytes()
    }
}

impl fmt::Debug for DpapiKeySealingContext {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DpapiKeySealingContext")
            .field("family_id", &"[redacted]")
            .field("trust_subject", &"[redacted]")
            .field("device_ref", &"[redacted]")
            .field("device_role", &"[redacted]")
            .field("lifecycle_generation", &self.lifecycle_generation)
            .field(
                "installation_binding_generation",
                &self.installation_binding_generation,
            )
            .finish()
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize)]
pub struct DpapiSealedKey {
    format_version: u8,
    context: DpapiKeySealingContext,
    authorization_binding: [u8; 32],
    unsealing_credential: PersistedPlatformKeyUnsealingCredential,
    ciphertext: Vec<u8>,
}

impl DpapiSealedKey {
    pub fn context(&self) -> &DpapiKeySealingContext {
        &self.context
    }

    pub fn ciphertext(&self) -> &[u8] {
        &self.ciphertext
    }
}

impl fmt::Debug for DpapiSealedKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DpapiSealedKey")
            .field("format_version", &self.format_version)
            .field("context", &"[redacted]")
            .field("authorization_binding", &"[redacted]")
            .field("ciphertext", &"[redacted]")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpapiKeySealingError {
    EmptyTrustMaterial,
    InvalidBinding,
    UnsupportedFormat,
    BindingMismatch,
    AuthorizationMismatch,
    CurrentAuthorityRequired,
    PlatformUnavailable,
    UnsealFailed,
}

pub fn seal_for_current_windows_user(
    authorization: AwaitingPlatformKeySealingRequest,
    current_authority_source: &impl CurrentParentDeviceTrustAuthoritySource,
    trust_material: &[u8],
) -> Result<DpapiSealedKey, DpapiKeySealingError> {
    if trust_material.is_empty() {
        return Err(DpapiKeySealingError::EmptyTrustMaterial);
    }

    let unsealing_credential = authorization.consume_for_platform_key_sealing();
    let ceremony = unsealing_credential.approved_parent_device_ceremony();
    let authority = current_authority_source
        .current_authorized_parent_device(
            &ceremony.family_id,
            ceremony.trust_subject(),
            ceremony.device_ref(),
        )
        .map_err(|_error| DpapiKeySealingError::CurrentAuthorityRequired)?;
    let context = DpapiKeySealingContext::from_approved_ceremony(&unsealing_credential, authority);
    context.validate()?;
    let device_local_binding = current_windows_device_local_binding()?;
    let authorization_binding =
        authorization_binding(&context, &unsealing_credential, &device_local_binding);
    let ciphertext = seal_for_current_windows_user_inner(
        trust_material,
        &context,
        &unsealing_credential,
        &device_local_binding,
    )?;
    Ok(DpapiSealedKey {
        format_version: SEALED_KEY_FORMAT_VERSION,
        context,
        authorization_binding,
        unsealing_credential,
        ciphertext,
    })
}

pub fn unseal_for_current_windows_user(
    sealed_key: &DpapiSealedKey,
    current_authority_source: &impl CurrentParentDeviceTrustAuthoritySource,
) -> Result<Vec<u8>, DpapiKeySealingError> {
    require_current_parent_device_trust_authority(
        current_authority_source,
        &sealed_key.context.family_id,
        &sealed_key.context.trust_subject,
        &sealed_key.context.device_ref,
        sealed_key.context.lifecycle_generation,
        sealed_key.context.installation_binding_generation,
    )
    .map_err(|_error| DpapiKeySealingError::CurrentAuthorityRequired)?;
    sealed_key.context.validate()?;
    if sealed_key.format_version != SEALED_KEY_FORMAT_VERSION {
        return Err(DpapiKeySealingError::UnsupportedFormat);
    }
    let device_local_binding = current_windows_device_local_binding()?;
    if sealed_key.authorization_binding
        != authorization_binding(
            &sealed_key.context,
            &sealed_key.unsealing_credential,
            &device_local_binding,
        )
    {
        return Err(DpapiKeySealingError::AuthorizationMismatch);
    }
    if sealed_key.ciphertext.is_empty() {
        return Err(DpapiKeySealingError::UnsealFailed);
    }

    unseal_for_current_windows_user_inner(
        &sealed_key.ciphertext,
        &sealed_key.context,
        &sealed_key.unsealing_credential,
        &device_local_binding,
    )
}

#[cfg(windows)]
fn seal_for_current_windows_user_inner(
    trust_material: &[u8],
    context: &DpapiKeySealingContext,
    credential: &PersistedPlatformKeyUnsealingCredential,
    device_local_binding: &[u8; 32],
) -> Result<Vec<u8>, DpapiKeySealingError> {
    windows_dpapi::encrypt_data(
        trust_material,
        windows_dpapi::Scope::User,
        Some(&context.entropy(credential, device_local_binding)),
    )
    .map_err(|_error| DpapiKeySealingError::PlatformUnavailable)
}

#[cfg(not(windows))]
fn seal_for_current_windows_user_inner(
    _trust_material: &[u8],
    _context: &DpapiKeySealingContext,
    _credential: &PersistedPlatformKeyUnsealingCredential,
    _device_local_binding: &[u8; 32],
) -> Result<Vec<u8>, DpapiKeySealingError> {
    Err(DpapiKeySealingError::PlatformUnavailable)
}

#[cfg(windows)]
fn unseal_for_current_windows_user_inner(
    ciphertext: &[u8],
    context: &DpapiKeySealingContext,
    credential: &PersistedPlatformKeyUnsealingCredential,
    device_local_binding: &[u8; 32],
) -> Result<Vec<u8>, DpapiKeySealingError> {
    windows_dpapi::decrypt_data(
        ciphertext,
        windows_dpapi::Scope::User,
        Some(&context.entropy(credential, device_local_binding)),
    )
    .map_err(|_error| DpapiKeySealingError::UnsealFailed)
}

#[cfg(not(windows))]
fn unseal_for_current_windows_user_inner(
    _ciphertext: &[u8],
    _context: &DpapiKeySealingContext,
    _credential: &PersistedPlatformKeyUnsealingCredential,
    _device_local_binding: &[u8; 32],
) -> Result<Vec<u8>, DpapiKeySealingError> {
    Err(DpapiKeySealingError::PlatformUnavailable)
}

fn authorization_binding(
    context: &DpapiKeySealingContext,
    credential: &PersistedPlatformKeyUnsealingCredential,
    device_local_binding: &[u8; 32],
) -> [u8; 32] {
    Sha256::digest(canonical_binding_bytes(
        context,
        credential,
        device_local_binding,
    ))
    .into()
}

fn canonical_binding_bytes(
    context: &DpapiKeySealingContext,
    credential: &PersistedPlatformKeyUnsealingCredential,
    device_local_binding: &[u8; 32],
) -> Vec<u8> {
    let fields = [
        format!("ocentra-device-trust-dpapi-v{SEALED_KEY_FORMAT_VERSION}"),
        context.family_id.clone(),
        context.trust_subject.clone(),
        context.device_ref.clone(),
        context.device_role.clone(),
        context.lifecycle_generation.to_string(),
        context.installation_binding_generation.to_string(),
        credential.trust_bootstrap_ref().to_owned(),
        credential.device_trust_ref().as_str().to_owned(),
        hex_encode(device_local_binding),
    ];
    let mut encoded = Vec::new();
    for field in fields {
        let field = hex_encode(field.as_bytes());
        encoded.extend_from_slice(format!("{:016x}:", field.len()).as_bytes());
        encoded.extend_from_slice(field.as_bytes());
    }
    encoded
}

#[cfg(windows)]
fn current_windows_device_local_binding() -> Result<[u8; 32], DpapiKeySealingError> {
    use winreg::{enums::HKEY_LOCAL_MACHINE, RegKey};

    let machine_hive = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cryptography_key = machine_hive
        .open_subkey(WINDOWS_MACHINE_CRYPTOGRAPHY_PATH)
        .map_err(|_error| DpapiKeySealingError::PlatformUnavailable)?;
    let machine_guid = cryptography_key
        .get_value::<String, _>(WINDOWS_MACHINE_GUID_VALUE)
        .map_err(|_error| DpapiKeySealingError::PlatformUnavailable)?;
    (!machine_guid.trim().is_empty())
        .then(|| Sha256::digest(machine_guid.as_bytes()).into())
        .ok_or(DpapiKeySealingError::PlatformUnavailable)
}

#[cfg(not(windows))]
fn current_windows_device_local_binding() -> Result<[u8; 32], DpapiKeySealingError> {
    Err(DpapiKeySealingError::PlatformUnavailable)
}

fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}
