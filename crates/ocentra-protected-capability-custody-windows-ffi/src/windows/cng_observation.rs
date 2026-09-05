//! Bounded strict CNG/PCP property observation.

#![cfg(windows)]

use super::cng;
use crate::{
    Error, InputFault, PcpKeyObservation, Result, SecurityDescriptorObservation, WindowsText,
    MAX_BUFFER_BYTES,
};
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Security::Cryptography::{
    NCryptGetProperty, NCryptSetProperty, NCRYPT_ALLOW_SIGNING_FLAG, NCRYPT_EXPORT_POLICY_PROPERTY,
    NCRYPT_HANDLE, NCRYPT_IMPL_HARDWARE_FLAG, NCRYPT_IMPL_REMOVABLE_FLAG,
    NCRYPT_IMPL_SOFTWARE_FLAG, NCRYPT_IMPL_TYPE_PROPERTY, NCRYPT_KEY_HANDLE,
    NCRYPT_KEY_USAGE_PROPERTY, NCRYPT_NAME_PROPERTY, NCRYPT_PCP_EKPUB_PROPERTY,
    NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY, NCRYPT_PCP_PLATFORM_TYPE_PROPERTY,
    NCRYPT_PCP_SIGNATURE_KEY, NCRYPT_PCP_TPM2BNAME_PROPERTY, NCRYPT_PROV_HANDLE,
    NCRYPT_SECURITY_DESCR_PROPERTY, NCRYPT_SECURITY_DESCR_SUPPORT_PROPERTY, NCRYPT_SILENT_FLAG,
};
use windows_sys::Win32::Security::{
    DACL_SECURITY_INFORMATION, OBJECT_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION,
};

const PCP_SECURITY_FLAGS: OBJECT_SECURITY_INFORMATION =
    OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION | NCRYPT_SILENT_FLAG;

pub(super) fn observe_key(
    provider: NCRYPT_PROV_HANDLE,
    key: NCRYPT_KEY_HANDLE,
) -> Result<PcpKeyObservation> {
    ensure_security_descriptor_support(provider)?;
    let expected_key_name = fixed_key_name_text()?;
    let key_name = decode_name(&get_property(
        key,
        NCRYPT_NAME_PROPERTY,
        NCRYPT_SILENT_FLAG,
    )?)?;
    let implementation_type = get_u32_property(provider, NCRYPT_IMPL_TYPE_PROPERTY)?;
    let export_policy = get_u32_property(key, NCRYPT_EXPORT_POLICY_PROPERTY)?;
    let key_usage = get_u32_property(key, NCRYPT_KEY_USAGE_PROPERTY)?;
    let pcp_key_usage_policy = get_u32_property(key, NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY)?;
    // NCRYPT_PCP_PLATFORM_TYPE_PROPERTY is provider-owned UTF-16 text.
    let platform_type = decode_name(&get_property(
        provider,
        NCRYPT_PCP_PLATFORM_TYPE_PROPERTY,
        NCRYPT_SILENT_FLAG,
    )?)?;
    let security_descriptor =
        get_property(key, NCRYPT_SECURITY_DESCR_PROPERTY, PCP_SECURITY_FLAGS)?;
    let ek_public = get_property(provider, NCRYPT_PCP_EKPUB_PROPERTY, NCRYPT_SILENT_FLAG)?;
    let signing_public_key = cng::export_public_key(key)?;
    let tpm2b_name = get_property(key, NCRYPT_PCP_TPM2BNAME_PROPERTY, NCRYPT_SILENT_FLAG)?;
    let security = crate::security::copy_descriptor(security_descriptor.clone())?;

    let properties = StrictProperties {
        key_name: &key_name,
        expected_key_name: &expected_key_name,
        implementation_type,
        export_policy,
        key_usage,
        pcp_key_usage_policy,
        platform_type: &platform_type,
        security: &security,
        ek_public: &ek_public,
        signing_public_key: &signing_public_key,
        tpm2b_name: &tpm2b_name,
    };
    if !properties.is_valid() {
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
    let status = unsafe { NCryptSetProperty(key, property, bytes.as_ptr(), 4, NCRYPT_SILENT_FLAG) };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    Ok(())
}

pub(super) fn ensure_security_descriptor_support(provider: NCRYPT_PROV_HANDLE) -> Result<()> {
    let support = get_u32_property(provider, NCRYPT_SECURITY_DESCR_SUPPORT_PROPERTY)?;
    if support != 1 {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(())
}

struct StrictProperties<'a> {
    key_name: &'a WindowsText,
    expected_key_name: &'a WindowsText,
    implementation_type: u32,
    export_policy: u32,
    key_usage: u32,
    pcp_key_usage_policy: u32,
    platform_type: &'a WindowsText,
    security: &'a SecurityDescriptorObservation,
    ek_public: &'a [u8],
    signing_public_key: &'a [u8],
    tpm2b_name: &'a [u8],
}

impl StrictProperties<'_> {
    fn is_valid(&self) -> bool {
        self.key_name == self.expected_key_name
            && self.implementation_type & NCRYPT_IMPL_HARDWARE_FLAG != 0
            && self.implementation_type & NCRYPT_IMPL_SOFTWARE_FLAG == 0
            && self.implementation_type & NCRYPT_IMPL_REMOVABLE_FLAG == 0
            && self.export_policy == 0
            && self.key_usage == NCRYPT_ALLOW_SIGNING_FLAG
            && self.pcp_key_usage_policy == NCRYPT_PCP_SIGNATURE_KEY
            && !self.platform_type.as_str().is_empty()
            && cng::valid_system_key_security(self.security)
            && cng::valid_rsa_public_blob(self.ek_public)
            && cng::valid_rsa_3072_public_blob(self.signing_public_key)
            && valid_tpm2b(self.tpm2b_name)
    }
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
    let value = get_property(key, property, NCRYPT_SILENT_FLAG)?;
    if value.len() != 4 {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(u32::from_le_bytes([value[0], value[1], value[2], value[3]]))
}

fn decode_name(bytes: &[u8]) -> Result<WindowsText> {
    if bytes.len() < 2 || !bytes.len().is_multiple_of(2) {
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
        .map_err(|_error| Error::CryptoPropertyViolation)?;
    WindowsText::try_from_str(&value).map_err(|_error| Error::CryptoPropertyViolation)
}

fn valid_tpm2b(value: &[u8]) -> bool {
    if value.len() < 3 {
        return false;
    }
    let declared = usize::from(u16::from_be_bytes([value[0], value[1]]));
    declared > 0 && declared + 2 == value.len()
}
