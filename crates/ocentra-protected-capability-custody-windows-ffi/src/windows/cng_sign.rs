//! Bounded PCP-backed SHA-256/PSS signing.

#![cfg(windows)]

use crate::{Error, Result, MAX_BUFFER_BYTES};
use windows_sys::Win32::Security::Cryptography::{
    NCryptSignHash, BCRYPT_PSS_PADDING_INFO, NCRYPT_KEY_HANDLE, NCRYPT_PAD_PSS_FLAG,
    NCRYPT_SHA256_ALGORITHM, NCRYPT_SILENT_FLAG,
};

pub(super) fn sign_digest(key: NCRYPT_KEY_HANDLE, digest: &[u8; 32]) -> Result<Vec<u8>> {
    let padding = BCRYPT_PSS_PADDING_INFO {
        pszAlgId: NCRYPT_SHA256_ALGORITHM,
        cbSalt: 32,
    };
    let mut signature = vec![0u8; MAX_BUFFER_BYTES];
    let mut written = 0u32;
    let status = unsafe {
        NCryptSignHash(
            key,
            (&padding as *const BCRYPT_PSS_PADDING_INFO).cast(),
            digest.as_ptr(),
            32,
            signature.as_mut_ptr(),
            u32::try_from(signature.len())?,
            &mut written,
            NCRYPT_PAD_PSS_FLAG | NCRYPT_SILENT_FLAG,
        )
    };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    let written = usize::try_from(written)?;
    if written == 0 || written > signature.len() {
        return Err(Error::CryptoPropertyViolation);
    }
    signature.truncate(written);
    Ok(signature)
}
