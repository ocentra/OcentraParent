//! Strict observation and security checks for the fixed AccountIssuer key.

#![cfg(windows)]

use super::cng_account_issuer_p256_export::export_public_key;
use crate::account_issuer_types::AccountIssuerP256Observation;
use crate::{
    Error, InputFault, Result, SecurityDescriptorObservation, WindowsText, MAX_BUFFER_BYTES,
};
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Security::Cryptography::{
    NCryptGetProperty, NCryptSetProperty, NCRYPT_ALGORITHM_GROUP_PROPERTY,
    NCRYPT_ALGORITHM_PROPERTY, NCRYPT_EXPORT_POLICY_PROPERTY, NCRYPT_IMPL_TYPE_PROPERTY,
    NCRYPT_KEY_HANDLE, NCRYPT_KEY_USAGE_PROPERTY, NCRYPT_LENGTH_PROPERTY, NCRYPT_NAME_PROPERTY,
    NCRYPT_PCP_EKPUB_PROPERTY, NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY,
    NCRYPT_PCP_PLATFORM_TYPE_PROPERTY, NCRYPT_PCP_PROVIDER_VERSION_PROPERTY,
    NCRYPT_PCP_TPM2BNAME_PROPERTY, NCRYPT_PROV_HANDLE, NCRYPT_SECURITY_DESCR_PROPERTY,
    NCRYPT_SECURITY_DESCR_SUPPORT_PROPERTY, NCRYPT_SILENT_FLAG, NCRYPT_UNIQUE_NAME_PROPERTY,
};
use windows_sys::Win32::Security::{
    DACL_SECURITY_INFORMATION, OBJECT_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION,
};

const SECURITY_FLAGS: OBJECT_SECURITY_INFORMATION =
    OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION | NCRYPT_SILENT_FLAG;

struct KeyTextProperties {
    key_name: WindowsText,
    algorithm: WindowsText,
    algorithm_group: WindowsText,
    unique_name: WindowsText,
}

#[derive(Eq, PartialEq)]
struct ProviderProperties {
    implementation_type: u32,
    provider_version: WindowsText,
    platform_type: WindowsText,
    ek_public: Vec<u8>,
}

struct KeyNumericProperties {
    export_policy: u32,
    key_usage: u32,
    pcp_key_usage_policy: u32,
    key_length_bits: u32,
}

struct KeyMaterial {
    tpm2b_name: Vec<u8>,
    public_key_sec1: [u8; 65],
    security: SecurityDescriptorObservation,
}

struct RawObservation {
    observation: AccountIssuerP256Observation,
}

pub(super) fn observe_key(
    provider: NCRYPT_PROV_HANDLE,
    key: NCRYPT_KEY_HANDLE,
) -> Result<AccountIssuerP256Observation> {
    observe_key_with_security_policy(provider, key, true)
}

pub(super) fn observe_key_for_external_acl_transition(
    provider: NCRYPT_PROV_HANDLE,
    key: NCRYPT_KEY_HANDLE,
) -> Result<AccountIssuerP256Observation> {
    observe_key_with_security_policy(provider, key, false)
}

fn observe_key_with_security_policy(
    provider: NCRYPT_PROV_HANDLE,
    key: NCRYPT_KEY_HANDLE,
    require_protected_base_security: bool,
) -> Result<AccountIssuerP256Observation> {
    ensure_security_descriptor_support(provider)?;
    let raw = read_observation(provider, key)?;
    let identity_valid = super::cng_account_issuer_p256_identity::valid_identity(&raw.observation);
    let base_security_valid =
        super::cng_account_issuer_p256_acl::valid_base_security(raw.observation.security());
    let security_valid = (require_protected_base_security, base_security_valid) != (true, false);
    if (identity_valid, security_valid) != (true, true) {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(raw.observation)
}

fn read_observation(
    provider: NCRYPT_PROV_HANDLE,
    key: NCRYPT_KEY_HANDLE,
) -> Result<RawObservation> {
    let text = read_key_text_properties(key)?;
    let provider_properties = read_provider_properties(provider)?;
    validate_key_provider_identity(key, &provider_properties)?;
    let numeric = read_key_numeric_properties(key)?;
    let material = read_key_material(key)?;
    Ok(RawObservation {
        observation: AccountIssuerP256Observation {
            key_name: text.key_name,
            algorithm: text.algorithm,
            algorithm_group: text.algorithm_group,
            unique_name: text.unique_name,
            implementation_type: provider_properties.implementation_type,
            provider_version: provider_properties.provider_version,
            export_policy: numeric.export_policy,
            key_usage: numeric.key_usage,
            pcp_key_usage_policy: numeric.pcp_key_usage_policy,
            key_length_bits: numeric.key_length_bits,
            platform_type: provider_properties.platform_type,
            ek_public: provider_properties.ek_public,
            tpm2b_name: material.tpm2b_name,
            public_key_sec1: material.public_key_sec1,
            security: material.security,
        },
    })
}

fn validate_key_provider_identity(
    key: NCRYPT_KEY_HANDLE,
    expected: &ProviderProperties,
) -> Result<()> {
    // The expected handle was opened by the compiled Microsoft Platform
    // Provider name. The key-acquired handle must independently expose the
    // same provider properties and algorithm surface; numeric handle equality
    // is neither required nor treated as identity.
    let observed = super::cng_account_issuer_p256_provider_observation::from_key(key)?;
    super::cng_account_issuer_p256_algorithm::validate_provider_algorithm(observed.handle())?;
    ensure_security_descriptor_support(observed.handle())?;
    let observed_properties = read_provider_properties(observed.handle())?;
    if &observed_properties != expected {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(())
}

fn read_key_text_properties(key: NCRYPT_KEY_HANDLE) -> Result<KeyTextProperties> {
    Ok(KeyTextProperties {
        key_name: read_text_property(key, NCRYPT_NAME_PROPERTY)?,
        algorithm: read_text_property(key, NCRYPT_ALGORITHM_PROPERTY)?,
        algorithm_group: read_text_property(key, NCRYPT_ALGORITHM_GROUP_PROPERTY)?,
        unique_name: read_text_property(key, NCRYPT_UNIQUE_NAME_PROPERTY)?,
    })
}

fn read_provider_properties(provider: NCRYPT_PROV_HANDLE) -> Result<ProviderProperties> {
    Ok(ProviderProperties {
        implementation_type: get_u32_property(provider, NCRYPT_IMPL_TYPE_PROPERTY)?,
        provider_version: read_text_property(provider, NCRYPT_PCP_PROVIDER_VERSION_PROPERTY)?,
        platform_type: read_text_property(provider, NCRYPT_PCP_PLATFORM_TYPE_PROPERTY)?,
        ek_public: get_property(provider, NCRYPT_PCP_EKPUB_PROPERTY, NCRYPT_SILENT_FLAG)?,
    })
}

fn read_key_numeric_properties(key: NCRYPT_KEY_HANDLE) -> Result<KeyNumericProperties> {
    Ok(KeyNumericProperties {
        export_policy: get_u32_property(key, NCRYPT_EXPORT_POLICY_PROPERTY)?,
        key_usage: get_u32_property(key, NCRYPT_KEY_USAGE_PROPERTY)?,
        pcp_key_usage_policy: get_u32_property(key, NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY)?,
        key_length_bits: get_u32_property(key, NCRYPT_LENGTH_PROPERTY)?,
    })
}

fn read_key_material(key: NCRYPT_KEY_HANDLE) -> Result<KeyMaterial> {
    let descriptor = get_property(key, NCRYPT_SECURITY_DESCR_PROPERTY, SECURITY_FLAGS)?;
    Ok(KeyMaterial {
        tpm2b_name: get_property(key, NCRYPT_PCP_TPM2BNAME_PROPERTY, NCRYPT_SILENT_FLAG)?,
        public_key_sec1: export_public_key(key)?,
        security: crate::security::copy_descriptor(descriptor)?,
    })
}

pub(super) fn ensure_security_descriptor_support(provider: NCRYPT_PROV_HANDLE) -> Result<()> {
    if get_u32_property(provider, NCRYPT_SECURITY_DESCR_SUPPORT_PROPERTY)? != 1 {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(())
}

pub(super) fn set_u32_property(key: NCRYPT_KEY_HANDLE, property: PCWSTR, value: u32) -> Result<()> {
    let bytes = value.to_le_bytes();
    let status = unsafe {
        NCryptSetProperty(
            key,
            property,
            bytes.as_ptr(),
            u32::try_from(bytes.len())?,
            NCRYPT_SILENT_FLAG,
        )
    };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    Ok(())
}

fn read_text_property(handle: usize, property: PCWSTR) -> Result<WindowsText> {
    decode_text(&get_property(handle, property, NCRYPT_SILENT_FLAG)?)
}

pub(super) fn get_property(
    handle: usize,
    property: PCWSTR,
    flags: OBJECT_SECURITY_INFORMATION,
) -> Result<Vec<u8>> {
    let mut buffer = vec![0_u8; MAX_BUFFER_BYTES];
    let mut written = 0_u32;
    let status = unsafe {
        NCryptGetProperty(
            handle,
            property,
            buffer.as_mut_ptr(),
            u32::try_from(buffer.len())?,
            &mut written,
            flags,
        )
    };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    let written = usize::try_from(written)?;
    if !(1..=buffer.len()).contains(&written) {
        return Err(Error::CryptoPropertyViolation);
    }
    buffer.truncate(written);
    Ok(buffer)
}

fn get_u32_property(handle: usize, property: PCWSTR) -> Result<u32> {
    let value = get_property(handle, property, NCRYPT_SILENT_FLAG)?;
    if value.len() != 4 {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

fn decode_text(bytes: &[u8]) -> Result<WindowsText> {
    if (bytes.len() < 2, bytes.len() % 2) != (false, 0) {
        return Err(Error::CryptoPropertyViolation);
    }
    let units = bytes
        .chunks_exact(2)
        .map(|part| u16::from_le_bytes([part[0], part[1]]))
        .collect::<Vec<_>>();
    if units.last().copied() != Some(0) {
        return Err(Error::CryptoPropertyViolation);
    }
    WindowsText::from_utf16(&units[..units.len() - 1], InputFault::WindowsTextInvalid)
}
