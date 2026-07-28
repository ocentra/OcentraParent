//! Windows-only local sealing for device-trust material.
//!
//! This adapter persists only DPAPI ciphertext plus non-secret binding metadata. It
//! has no plaintext or portable-key fallback: unavailable platform custody requires
//! recovery and re-pair through the owning flows.

use serde::{Deserialize, Serialize};

const SEALED_KEY_FORMAT_VERSION: u8 = 1;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

    fn entropy(&self) -> Vec<u8> {
        format!(
            "ocentra-device-trust-dpapi-v{SEALED_KEY_FORMAT_VERSION}\u{1f}{}\u{1f}{}\u{1f}{}",
            self.trust_subject, self.device_ref, self.device_role
        )
        .into_bytes()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DpapiSealedKey {
    format_version: u8,
    context: DpapiKeySealingContext,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DpapiKeySealingError {
    EmptyTrustMaterial,
    InvalidBinding,
    UnsupportedFormat,
    BindingMismatch,
    PlatformUnavailable,
    UnsealFailed,
}

pub fn seal_for_current_windows_user(
    context: DpapiKeySealingContext,
    trust_material: &[u8],
) -> Result<DpapiSealedKey, DpapiKeySealingError> {
    context.validate()?;
    if trust_material.is_empty() {
        return Err(DpapiKeySealingError::EmptyTrustMaterial);
    }

    let ciphertext = seal_for_current_windows_user_inner(trust_material, &context)?;
    Ok(DpapiSealedKey {
        format_version: SEALED_KEY_FORMAT_VERSION,
        context,
        ciphertext,
    })
}

pub fn unseal_for_current_windows_user(
    sealed_key: &DpapiSealedKey,
    expected_context: &DpapiKeySealingContext,
) -> Result<Vec<u8>, DpapiKeySealingError> {
    expected_context.validate()?;
    if sealed_key.format_version != SEALED_KEY_FORMAT_VERSION {
        return Err(DpapiKeySealingError::UnsupportedFormat);
    }
    if sealed_key.context != *expected_context {
        return Err(DpapiKeySealingError::BindingMismatch);
    }
    if sealed_key.ciphertext.is_empty() {
        return Err(DpapiKeySealingError::UnsealFailed);
    }

    unseal_for_current_windows_user_inner(&sealed_key.ciphertext, expected_context)
}

#[cfg(windows)]
fn seal_for_current_windows_user_inner(
    trust_material: &[u8],
    context: &DpapiKeySealingContext,
) -> Result<Vec<u8>, DpapiKeySealingError> {
    windows_dpapi::encrypt_data(
        trust_material,
        windows_dpapi::Scope::User,
        Some(&context.entropy()),
    )
    .map_err(|_error| DpapiKeySealingError::PlatformUnavailable)
}

#[cfg(not(windows))]
fn seal_for_current_windows_user_inner(
    _trust_material: &[u8],
    _context: &DpapiKeySealingContext,
) -> Result<Vec<u8>, DpapiKeySealingError> {
    Err(DpapiKeySealingError::PlatformUnavailable)
}

#[cfg(windows)]
fn unseal_for_current_windows_user_inner(
    ciphertext: &[u8],
    context: &DpapiKeySealingContext,
) -> Result<Vec<u8>, DpapiKeySealingError> {
    windows_dpapi::decrypt_data(
        ciphertext,
        windows_dpapi::Scope::User,
        Some(&context.entropy()),
    )
    .map_err(|_error| DpapiKeySealingError::UnsealFailed)
}

#[cfg(not(windows))]
fn unseal_for_current_windows_user_inner(
    _ciphertext: &[u8],
    _context: &DpapiKeySealingContext,
) -> Result<Vec<u8>, DpapiKeySealingError> {
    Err(DpapiKeySealingError::PlatformUnavailable)
}
