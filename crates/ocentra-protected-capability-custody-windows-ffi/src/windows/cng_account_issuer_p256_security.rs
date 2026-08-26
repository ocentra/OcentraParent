//! Strict observation and security checks for the fixed AccountIssuer key.

#![cfg(windows)]

use super::cng_account_issuer_p256_capability::{
    ACCOUNT_ISSUER_ALGORITHM_NAME, ACCOUNT_ISSUER_KEY_NAME,
};
use super::cng_account_issuer_p256_export::export_public_key;
use crate::account_issuer_types::AccountIssuerP256Observation;
use crate::{
    Error, InputFault, OwnedService, Result, SecurityDescriptorObservation, WindowsText,
    MAX_BUFFER_BYTES,
};
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Foundation::GENERIC_ALL;
use windows_sys::Win32::Security::Cryptography::{
    NCryptGetProperty, NCRYPT_ALGORITHM_PROPERTY, NCRYPT_ALLOW_SIGNING_FLAG,
    NCRYPT_EXPORT_POLICY_PROPERTY, NCRYPT_IMPL_HARDWARE_FLAG, NCRYPT_IMPL_REMOVABLE_FLAG,
    NCRYPT_IMPL_SOFTWARE_FLAG, NCRYPT_IMPL_TYPE_PROPERTY, NCRYPT_KEY_HANDLE,
    NCRYPT_KEY_USAGE_PROPERTY, NCRYPT_LENGTH_PROPERTY, NCRYPT_PCP_EKPUB_PROPERTY,
    NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY, NCRYPT_PCP_PLATFORM_TYPE_PROPERTY,
    NCRYPT_PCP_SIGNATURE_KEY, NCRYPT_PCP_TPM2BNAME_PROPERTY, NCRYPT_PROV_HANDLE,
    NCRYPT_SECURITY_DESCR_PROPERTY, NCRYPT_SECURITY_DESCR_SUPPORT_PROPERTY, NCRYPT_SILENT_FLAG,
};
use windows_sys::Win32::Security::{
    DACL_SECURITY_INFORMATION, OBJECT_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION,
};

const SECURITY_FLAGS: OBJECT_SECURITY_INFORMATION =
    OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION | NCRYPT_SILENT_FLAG;
const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1;
const ALLOWED_ACE_TYPE: u8 = 0;
const EMPTY_SID: &[u8] = &[];
const TRUSTED_INSTALLER_SID: &[u8] = &[
    1, 6, 0, 0, 0, 0, 0, 5, 80, 0, 0, 0, 181, 137, 251, 56, 25, 132, 194, 203, 92, 108, 35, 109,
    87, 0, 119, 110, 192, 2, 100, 135,
];
const SYSTEM_SID: &[u8] = &[1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0];
const EVERYONE_SID: &[u8] = &[1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0];
const AUTHENTICATED_USERS_SID: &[u8] = &[1, 1, 0, 0, 0, 0, 0, 5, 11, 0, 0, 0];
const BUILTIN_ADMINISTRATORS_SID: &[u8] = &[1, 2, 0, 0, 0, 0, 0, 5, 32, 0, 0, 0, 32, 2, 0, 0];
const BUILTIN_USERS_SID: &[u8] = &[1, 2, 0, 0, 0, 0, 0, 5, 32, 0, 0, 0, 33, 2, 0, 0];

pub(super) fn observe_key(
    provider: NCRYPT_PROV_HANDLE,
    key: NCRYPT_KEY_HANDLE,
) -> Result<AccountIssuerP256Observation> {
    ensure_security_descriptor_support(provider)?;
    let key_name = decode_text(&get_property(
        key,
        NCRYPT_PROPERTY_NAME,
        NCRYPT_SILENT_FLAG,
    )?)?;
    let algorithm = decode_text(&get_property(
        key,
        NCRYPT_ALGORITHM_PROPERTY,
        NCRYPT_SILENT_FLAG,
    )?)?;
    let implementation_type = get_u32_property(provider, NCRYPT_IMPL_TYPE_PROPERTY)?;
    let export_policy = get_u32_property(key, NCRYPT_EXPORT_POLICY_PROPERTY)?;
    let key_usage = get_u32_property(key, NCRYPT_KEY_USAGE_PROPERTY)?;
    let pcp_key_usage_policy = get_u32_property(key, NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY)?;
    let key_length_bits = get_u32_property(key, NCRYPT_LENGTH_PROPERTY)?;
    let platform_type = decode_text(&get_property(
        provider,
        NCRYPT_PCP_PLATFORM_TYPE_PROPERTY,
        NCRYPT_SILENT_FLAG,
    )?)?;
    let ek_public = get_property(provider, NCRYPT_PCP_EKPUB_PROPERTY, NCRYPT_SILENT_FLAG)?;
    let tpm2b_name = get_property(key, NCRYPT_PCP_TPM2BNAME_PROPERTY, NCRYPT_SILENT_FLAG)?;
    let security_descriptor = get_property(key, NCRYPT_SECURITY_DESCR_PROPERTY, SECURITY_FLAGS)?;
    let security = crate::security::copy_descriptor(security_descriptor)?;
    let public_key_sec1 = export_public_key(key)?;

    if !strict_properties(
        &key_name,
        &algorithm,
        implementation_type,
        export_policy,
        key_usage,
        pcp_key_usage_policy,
        key_length_bits,
        &platform_type,
        &security,
        &ek_public,
        &tpm2b_name,
    ) {
        return Err(Error::CryptoPropertyViolation);
    }

    Ok(AccountIssuerP256Observation {
        key_name,
        algorithm,
        implementation_type,
        export_policy,
        key_usage,
        pcp_key_usage_policy,
        key_length_bits,
        platform_type,
        ek_public,
        tpm2b_name,
        public_key_sec1,
        security,
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
        windows_sys::Win32::Security::Cryptography::NCryptSetProperty(
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

/// Validate the exact service-bound ACL observed from the retained SCM
/// service. The service SID must come from `OwnedService::service_sid`; this
/// function does not parse caller-supplied SDDL or manufacture an identity.
pub(super) fn valid_service_security(
    security: &SecurityDescriptorObservation,
    service_sid: &[u8],
) -> bool {
    if service_sid.is_empty()
        || security.owner_sid() != TRUSTED_INSTALLER_SID
        || security.owner_was_defaulted()
        || !security.dacl_is_present()
        || security.dacl_was_defaulted()
        || !security.dacl_is_protected()
        || security.dacl().len() != 1
    {
        return false;
    }
    let Some(ace) = security.dacl().first() else {
        return false;
    };
    ace.ace_type() == ALLOWED_ACE_TYPE
        && ace.flags() == 0
        && ace.access_mask() == GENERIC_ALL
        && ace.sid() == service_sid
        && !ace.raw().is_empty()
        && !is_broad_sid(ace.sid())
}

pub(super) fn validate_service_binding(service: &OwnedService) -> Result<Vec<u8>> {
    let observation = service.observation()?;
    if observation.service_name().as_str().as_bytes()
        != super::cng_account_issuer_p256_capability::BROKER_SERVICE_NAME
        || observation.service_sid_type() != SERVICE_SID_TYPE_UNRESTRICTED
    {
        return Err(Error::CryptoPropertyViolation);
    }
    service.service_sid()
}

fn strict_properties(
    key_name: &WindowsText,
    algorithm: &WindowsText,
    implementation_type: u32,
    export_policy: u32,
    key_usage: u32,
    pcp_key_usage_policy: u32,
    key_length_bits: u32,
    platform_type: &WindowsText,
    security: &SecurityDescriptorObservation,
    ek_public: &[u8],
    tpm2b_name: &[u8],
) -> bool {
    key_name.as_str().as_bytes() == ACCOUNT_ISSUER_KEY_NAME
        && algorithm.as_str().as_bytes() == ACCOUNT_ISSUER_ALGORITHM_NAME
        && implementation_type & NCRYPT_IMPL_HARDWARE_FLAG != 0
        && implementation_type & NCRYPT_IMPL_SOFTWARE_FLAG == 0
        && implementation_type & NCRYPT_IMPL_REMOVABLE_FLAG == 0
        && export_policy == 0
        && key_usage == NCRYPT_ALLOW_SIGNING_FLAG
        && pcp_key_usage_policy == NCRYPT_PCP_SIGNATURE_KEY
        && key_length_bits == 256
        && !platform_type.as_str().is_empty()
        && super::cng::valid_rsa_public_blob(ek_public)
        && valid_tpm2b(tpm2b_name)
        && valid_base_security(security)
}

fn valid_tpm2b(value: &[u8]) -> bool {
    if value.len() < 3 {
        return false;
    }
    let declared = usize::from(u16::from_be_bytes([value[0], value[1]]));
    declared > 0 && declared.checked_add(2) == Some(value.len())
}

fn valid_base_security(security: &SecurityDescriptorObservation) -> bool {
    security.owner_sid() != EMPTY_SID
        && !security.owner_was_defaulted()
        && security.dacl_is_present()
        && !security.dacl_was_defaulted()
        && security.dacl_is_protected()
        && !security.dacl().is_empty()
        && security.dacl().iter().all(|ace| {
            ace.ace_type() == ALLOWED_ACE_TYPE && ace.flags() == 0 && !is_broad_sid(ace.sid())
        })
}

fn is_broad_sid(sid: &[u8]) -> bool {
    sid == SYSTEM_SID
        || sid == EVERYONE_SID
        || sid == AUTHENTICATED_USERS_SID
        || sid == BUILTIN_ADMINISTRATORS_SID
        || sid == BUILTIN_USERS_SID
}

fn get_property(
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
    if written == 0 || written > buffer.len() {
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
    if bytes.len() < 2 || bytes.len() % 2 != 0 {
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

const NCRYPT_PROPERTY_NAME_WIDE_UNITS: [u16; 5] = [78, 97, 109, 101, 0];
const NCRYPT_PROPERTY_NAME: PCWSTR = NCRYPT_PROPERTY_NAME_WIDE_UNITS.as_ptr();
