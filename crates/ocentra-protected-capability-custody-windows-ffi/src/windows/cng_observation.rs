//! Bounded strict CNG/PCP property observation.

#![cfg(windows)]

use crate::{Error, InputFault, PcpKeyObservation, Result, WindowsText, MAX_BUFFER_BYTES};
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Security::Cryptography::{
    NCryptGetProperty, NCryptSetProperty, NCRYPT_ALLOW_SIGNING_FLAG, NCRYPT_EXPORT_POLICY_PROPERTY,
    NCRYPT_HANDLE, NCRYPT_IMPL_HARDWARE_FLAG, NCRYPT_IMPL_REMOVABLE_FLAG,
    NCRYPT_IMPL_SOFTWARE_FLAG, NCRYPT_IMPL_TYPE_PROPERTY, NCRYPT_KEY_HANDLE,
    NCRYPT_KEY_USAGE_PROPERTY, NCRYPT_NAME_PROPERTY, NCRYPT_PCP_EKPUB_PROPERTY,
    NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY, NCRYPT_PCP_PLATFORM_TYPE_PROPERTY,
    NCRYPT_PCP_SIGNATURE_KEY, NCRYPT_PCP_TPM2BNAME_PROPERTY, NCRYPT_PROV_HANDLE,
    NCRYPT_SECURITY_DESCR_PROPERTY,
};
use windows_sys::Win32::Security::{
    DACL_SECURITY_INFORMATION, OBJECT_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION,
    PROTECTED_DACL_SECURITY_INFORMATION,
};

const PCP_SECURITY_FLAGS: OBJECT_SECURITY_INFORMATION =
    OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION;

pub(super) fn observe_key(
    provider: NCRYPT_PROV_HANDLE,
    key: NCRYPT_KEY_HANDLE,
) -> Result<PcpKeyObservation> {
    let expected_key_name = fixed_key_name_text()?;
    let key_name = decode_name(&get_property(key, NCRYPT_NAME_PROPERTY, 0)?)?;
    let implementation_type = get_u32_property(key, NCRYPT_IMPL_TYPE_PROPERTY)?;
    let export_policy = get_u32_property(key, NCRYPT_EXPORT_POLICY_PROPERTY)?;
    let key_usage = get_u32_property(key, NCRYPT_KEY_USAGE_PROPERTY)?;
    let pcp_key_usage_policy = get_u32_property(key, NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY)?;
    let platform_type = decode_name(&get_property(
        provider,
        NCRYPT_PCP_PLATFORM_TYPE_PROPERTY,
        0,
    )?)?;
    let security_descriptor =
        get_property(key, NCRYPT_SECURITY_DESCR_PROPERTY, PCP_SECURITY_FLAGS)?;
    let ek_public = get_property(provider, NCRYPT_PCP_EKPUB_PROPERTY, 0)?;
    let tpm2b_name = get_property(key, NCRYPT_PCP_TPM2BNAME_PROPERTY, 0)?;

    if !strict_properties(
        &key_name,
        &expected_key_name,
        implementation_type,
        export_policy,
        key_usage,
        pcp_key_usage_policy,
        &platform_type,
        &security_descriptor,
        &ek_public,
        &tpm2b_name,
    ) {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(PcpKeyObservation {
        key_name,
        implementation_type,
        export_policy,
        key_usage,
        pcp_key_usage_policy,
        platform_type,
        security_descriptor,
        ek_public,
        tpm2b_name,
    })
}

pub(super) fn set_u32_property(key: NCRYPT_KEY_HANDLE, property: PCWSTR, value: u32) -> Result<()> {
    let bytes = value.to_le_bytes();
    let status = unsafe { NCryptSetProperty(key, property, bytes.as_ptr(), 4, 0) };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    Ok(())
}

fn strict_properties(
    key_name: &WindowsText,
    expected_key_name: &WindowsText,
    implementation_type: u32,
    export_policy: u32,
    key_usage: u32,
    pcp_key_usage_policy: u32,
    platform_type: &WindowsText,
    security_descriptor: &[u8],
    ek_public: &[u8],
    tpm2b_name: &[u8],
) -> bool {
    key_name == expected_key_name
        && implementation_type & NCRYPT_IMPL_HARDWARE_FLAG != 0
        && implementation_type & NCRYPT_IMPL_SOFTWARE_FLAG == 0
        && implementation_type & NCRYPT_IMPL_REMOVABLE_FLAG == 0
        && export_policy == 0
        && key_usage == NCRYPT_ALLOW_SIGNING_FLAG
        && pcp_key_usage_policy == NCRYPT_PCP_SIGNATURE_KEY
        && !platform_type.as_str().is_empty()
        && !security_descriptor.is_empty()
        && valid_rsa_public_blob(ek_public)
        && valid_tpm2b(tpm2b_name)
}

fn fixed_key_name_text() -> Result<WindowsText> {
    WindowsText::from_utf16(
        &super::cng::FIXED_KEY_NAME_WIDE[..super::cng::FIXED_KEY_NAME_WIDE.len() - 1],
        InputFault::WindowsTextInvalid,
    )
}

fn get_property(
    key: NCRYPT_HANDLE,
    property: PCWSTR,
    flags: OBJECT_SECURITY_INFORMATION,
) -> Result<Vec<u8>> {
    let mut buffer = vec![0u8; MAX_BUFFER_BYTES];
    let mut written = 0u32;
    let status = unsafe {
        NCryptGetProperty(
            key,
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

fn get_u32_property(key: NCRYPT_KEY_HANDLE, property: PCWSTR) -> Result<u32> {
    let value = get_property(key, property, 0)?;
    if value.len() != 4 {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

fn decode_name(bytes: &[u8]) -> Result<WindowsText> {
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
    let value = String::from_utf16(&units[..units.len() - 1])
        .map_err(|_| Error::CryptoPropertyViolation)?;
    WindowsText::try_from_str(&value).map_err(|_| Error::CryptoPropertyViolation)
}

fn valid_tpm2b(value: &[u8]) -> bool {
    if value.len() < 3 {
        return false;
    }
    let declared = usize::from(u16::from_be_bytes([value[0], value[1]]));
    declared > 0 && declared + 2 == value.len()
}

fn valid_rsa_public_blob(value: &[u8]) -> bool {
    if value.len() < 24 {
        return false;
    }
    let magic = u32::from_le_bytes([value[0], value[1], value[2], value[3]]);
    let bit_length = u32::from_le_bytes([value[4], value[5], value[6], value[7]]);
    let exponent_bytes = u32::from_le_bytes([value[8], value[9], value[10], value[11]]) as usize;
    let modulus_bytes = u32::from_le_bytes([value[12], value[13], value[14], value[15]]) as usize;
    let prime_one_bytes = u32::from_le_bytes([value[16], value[17], value[18], value[19]]);
    let prime_two_bytes = u32::from_le_bytes([value[20], value[21], value[22], value[23]]);
    magic == windows_sys::Win32::Security::Cryptography::BCRYPT_RSAPUBLIC_MAGIC
        && bit_length != 0
        && bit_length % 8 == 0
        && exponent_bytes != 0
        && modulus_bytes != 0
        && prime_one_bytes == 0
        && prime_two_bytes == 0
        && 24usize
            .checked_add(exponent_bytes)
            .and_then(|length| length.checked_add(modulus_bytes))
            == Some(value.len())
}
