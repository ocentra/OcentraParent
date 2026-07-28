//! Windows-only local sealing for device-trust material.
//!
//! This adapter persists only DPAPI ciphertext plus non-secret binding metadata. It
//! has no plaintext or portable-key fallback: unavailable platform custody requires
//! recovery and re-pair through the owning flows.

use std::fmt;

use ocentra_family_identity_core::trust_bootstrap::{
    current_authority::{
        current_parent_device_trust_authority_for_sealed_device,
        CurrentParentDeviceTrustAuthorityInput,
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
    pub trust_subject: String,
    pub device_ref: String,
    pub device_role: String,
}

impl DpapiKeySealingContext {
    fn validate(&self) -> Result<(), DpapiKeySealingError> {
        if self.trust_subject.trim().is_empty()
            || self.device_ref.trim().is_empty()
            || self.device_role.trim().is_empty()
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
            .field("trust_subject", &"[redacted]")
            .field("device_ref", &"[redacted]")
            .field("device_role", &"[redacted]")
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
    context: DpapiKeySealingContext,
    trust_material: &[u8],
) -> Result<DpapiSealedKey, DpapiKeySealingError> {
    context.validate()?;
    if trust_material.is_empty() {
        return Err(DpapiKeySealingError::EmptyTrustMaterial);
    }

    let unsealing_credential = authorization.consume_for_platform_key_sealing();
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
    current_authority_input: &CurrentParentDeviceTrustAuthorityInput,
    expected_context: &DpapiKeySealingContext,
) -> Result<Vec<u8>, DpapiKeySealingError> {
    current_parent_device_trust_authority_for_sealed_device(
        current_authority_input,
        &expected_context.trust_subject,
        &expected_context.device_ref,
    )
    .map_err(|_error| DpapiKeySealingError::CurrentAuthorityRequired)?;
    expected_context.validate()?;
    if sealed_key.format_version != SEALED_KEY_FORMAT_VERSION {
        return Err(DpapiKeySealingError::UnsupportedFormat);
    }
    if sealed_key.context != *expected_context {
        return Err(DpapiKeySealingError::BindingMismatch);
    }
    let device_local_binding = current_windows_device_local_binding()?;
    if sealed_key.authorization_binding
        != authorization_binding(
            expected_context,
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
        expected_context,
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
        context.trust_subject.clone(),
        context.device_ref.clone(),
        context.device_role.clone(),
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
