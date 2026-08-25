//! Fixed PCP identity constants shared by the CNG mechanics modules.

#![cfg(windows)]

use crate::{Error, Result, SecurityDescriptorObservation, MAX_BUFFER_BYTES};
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Foundation::{LocalFree, GENERIC_ALL};
use windows_sys::Win32::Security::Authorization::{
    ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
};
use windows_sys::Win32::Security::Cryptography::{
    NCryptExportKey, NCryptSetProperty, BCRYPT_RSAPUBLIC_BLOB, NCRYPT_KEY_HANDLE,
    NCRYPT_SECURITY_DESCR_PROPERTY, NCRYPT_SILENT_FLAG,
};
use windows_sys::Win32::Security::{
    DACL_SECURITY_INFORMATION, OWNER_SECURITY_INFORMATION, PSECURITY_DESCRIPTOR,
};

pub(super) const REQUIRED_RSA_BITS: u32 = 3072;
pub(super) const RSA_3072_MODULUS_BYTES: usize = REQUIRED_RSA_BITS as usize / 8;

pub(super) static FIXED_KEY_NAME_WIDE: [u16; 52] = [
    79, 99, 101, 110, 116, 114, 97, 80, 97, 114, 101, 110, 116, 46, 80, 114, 111, 116, 101, 99,
    116, 101, 100, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 67, 117, 115, 116, 111, 100, 121,
    46, 83, 105, 103, 110, 105, 110, 103, 46, 118, 49, 0,
];
pub(super) const FIXED_KEY_NAME: PCWSTR = FIXED_KEY_NAME_WIDE.as_ptr();
const PCP_SECURITY_SET_FLAGS: u32 =
    OWNER_SECURITY_INFORMATION | DACL_SECURITY_INFORMATION | NCRYPT_SILENT_FLAG;
const SYSTEM_SID: [u8; 12] = [1, 1, 0, 0, 0, 0, 0, 5, 18, 0, 0, 0];
const ACCESS_ALLOWED_ACE_TYPE: u8 = 0;
const SYSTEM_KEY_ACCESS_MASK: u32 = GENERIC_ALL;
static SYSTEM_ONLY_KEY_SDDL_WIDE: [u16; 20] = [
    79, 58, 83, 89, 68, 58, 80, 40, 65, 59, 59, 71, 65, 59, 59, 59, 83, 89, 41, 0,
];
const SYSTEM_ONLY_KEY_SDDL: PCWSTR = SYSTEM_ONLY_KEY_SDDL_WIDE.as_ptr();

pub(super) fn export_public_key(key: NCRYPT_KEY_HANDLE) -> Result<Vec<u8>> {
    let mut required = 0u32;
    let status = unsafe {
        NCryptExportKey(
            key,
            0,
            BCRYPT_RSAPUBLIC_BLOB,
            core::ptr::null(),
            core::ptr::null_mut(),
            0,
            &mut required,
            NCRYPT_SILENT_FLAG,
        )
    };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    let required = usize::try_from(required)?;
    if required == 0 || required > MAX_BUFFER_BYTES {
        return Err(Error::CryptoPropertyViolation);
    }
    let mut blob = vec![0u8; required];
    let mut written = 0u32;
    let status = unsafe {
        NCryptExportKey(
            key,
            0,
            BCRYPT_RSAPUBLIC_BLOB,
            core::ptr::null(),
            blob.as_mut_ptr(),
            u32::try_from(blob.len())?,
            &mut written,
            NCRYPT_SILENT_FLAG,
        )
    };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    let written = usize::try_from(written)?;
    if written == 0 || written > blob.len() {
        return Err(Error::CryptoPropertyViolation);
    }
    blob.truncate(written);
    if !valid_rsa_3072_public_blob(&blob) {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(blob)
}

pub(super) fn valid_rsa_public_blob(value: &[u8]) -> bool {
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

pub(super) fn valid_rsa_3072_public_blob(value: &[u8]) -> bool {
    if !valid_rsa_public_blob(value) {
        return false;
    }
    let bit_length = u32::from_le_bytes([value[4], value[5], value[6], value[7]]);
    let exponent_bytes = u32::from_le_bytes([value[8], value[9], value[10], value[11]]);
    let modulus_bytes = u32::from_le_bytes([value[12], value[13], value[14], value[15]]);
    bit_length == REQUIRED_RSA_BITS
        && modulus_bytes == REQUIRED_RSA_BITS / 8
        && exponent_bytes == 3
        && value.len() >= 27
        && value[24..27] == [1, 0, 1]
}

pub(super) fn rsa_3072_modulus(value: &[u8]) -> Result<[u8; RSA_3072_MODULUS_BYTES]> {
    value
        .get(27..)
        .ok_or(Error::CryptoPropertyViolation)?
        .try_into()
        .map_err(|_| Error::CryptoPropertyViolation)
}

pub(super) fn set_fixed_key_security(key: NCRYPT_KEY_HANDLE) -> Result<()> {
    let mut descriptor_ptr: PSECURITY_DESCRIPTOR = core::ptr::null_mut();
    let mut descriptor_size = 0u32;
    let conversion_status = unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            SYSTEM_ONLY_KEY_SDDL,
            SDDL_REVISION_1,
            &mut descriptor_ptr,
            &mut descriptor_size,
        )
    };
    let descriptor = LocalSecurityDescriptor(descriptor_ptr);
    if conversion_status == 0 || descriptor_ptr.is_null() {
        return Err(Error::Win32(last_error()));
    }
    let descriptor_size = usize::try_from(descriptor_size)?;
    if descriptor_size == 0 || descriptor_size > MAX_BUFFER_BYTES {
        return Err(Error::BufferTooLarge);
    }
    let status = unsafe {
        NCryptSetProperty(
            key,
            NCRYPT_SECURITY_DESCR_PROPERTY,
            descriptor_ptr.cast(),
            u32::try_from(descriptor_size)?,
            PCP_SECURITY_SET_FLAGS,
        )
    };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    drop(descriptor);
    Ok(())
}

pub(super) fn valid_system_key_security(security: &SecurityDescriptorObservation) -> bool {
    let Some(ace) = security.dacl().first() else {
        return false;
    };
    security.owner_sid() == SYSTEM_SID
        && !security.owner_was_defaulted()
        && security.dacl_is_present()
        && !security.dacl_was_defaulted()
        && security.dacl_is_protected()
        && security.dacl().len() == 1
        && ace.ace_type() == ACCESS_ALLOWED_ACE_TYPE
        && ace.flags() == 0
        && ace.access_mask() == SYSTEM_KEY_ACCESS_MASK
        && ace.sid() == SYSTEM_SID
}

struct LocalSecurityDescriptor(PSECURITY_DESCRIPTOR);

impl Drop for LocalSecurityDescriptor {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                LocalFree(self.0.cast());
            }
        }
    }
}

fn last_error() -> u32 {
    unsafe { windows_sys::Win32::Foundation::GetLastError() }
}
