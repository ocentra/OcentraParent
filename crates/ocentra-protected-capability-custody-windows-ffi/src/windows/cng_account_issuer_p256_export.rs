//! Public-only SEC1 export for the fixed AccountIssuer P-256 key.

#![cfg(windows)]

use crate::{Error, Result, MAX_BUFFER_BYTES};
use windows_sys::Win32::Security::Cryptography::{
    NCryptExportKey, BCRYPT_ECCKEY_BLOB, BCRYPT_ECCPUBLIC_BLOB, BCRYPT_ECDSA_PUBLIC_P256_MAGIC,
    NCRYPT_KEY_HANDLE, NCRYPT_SILENT_FLAG,
};

const P256_COORDINATE_BYTES: usize = 32;
const ECC_PUBLIC_HEADER_BYTES: usize = core::mem::size_of::<BCRYPT_ECCKEY_BLOB>();
const ECC_PUBLIC_BLOB_BYTES: usize = ECC_PUBLIC_HEADER_BYTES + 2 * P256_COORDINATE_BYTES;

pub(super) fn export_public_key(key: NCRYPT_KEY_HANDLE) -> Result<[u8; 65]> {
    let mut required = 0_u32;
    let status = unsafe {
        NCryptExportKey(
            key,
            0,
            BCRYPT_ECCPUBLIC_BLOB,
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
    if required != ECC_PUBLIC_BLOB_BYTES || required > MAX_BUFFER_BYTES {
        return Err(Error::CryptoPropertyViolation);
    }

    let mut blob = vec![0_u8; required];
    let mut written = 0_u32;
    let status = unsafe {
        NCryptExportKey(
            key,
            0,
            BCRYPT_ECCPUBLIC_BLOB,
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
    if usize::try_from(written)? != ECC_PUBLIC_BLOB_BYTES {
        return Err(Error::CryptoPropertyViolation);
    }
    decode_public_blob(&blob)
}

fn decode_public_blob(blob: &[u8]) -> Result<[u8; 65]> {
    if blob.len() != ECC_PUBLIC_BLOB_BYTES {
        return Err(Error::CryptoPropertyViolation);
    }
    let magic = u32::from_le_bytes(
        blob[0..4]
            .try_into()
            .map_err(|_error| Error::CryptoPropertyViolation)?,
    );
    let coordinate_bytes = u32::from_le_bytes(
        blob[4..8]
            .try_into()
            .map_err(|_error| Error::CryptoPropertyViolation)?,
    );
    if magic != BCRYPT_ECDSA_PUBLIC_P256_MAGIC
        || usize::try_from(coordinate_bytes)? != P256_COORDINATE_BYTES
    {
        return Err(Error::CryptoPropertyViolation);
    }

    let mut sec1 = [0_u8; 65];
    sec1[0] = 0x04;
    sec1[1..].copy_from_slice(&blob[ECC_PUBLIC_HEADER_BYTES..]);
    if sec1[1..].iter().all(|byte| *byte == 0) {
        return Err(Error::CryptoPropertyViolation);
    }
    Ok(sec1)
}
