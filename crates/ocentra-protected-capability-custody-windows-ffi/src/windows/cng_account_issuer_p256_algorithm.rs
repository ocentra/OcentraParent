//! Exact provider algorithm admission for the AccountIssuer P-256 key.

#![cfg(windows)]

use crate::{Error, Result, MAX_WIDE_CHARS};
use windows_sys::Win32::Security::Cryptography::{
    NCryptAlgorithmName, NCryptEnumAlgorithms, NCryptFreeBuffer, NCryptIsAlgSupported,
    NCRYPT_ECDSA_P256_ALGORITHM, NCRYPT_PROV_HANDLE, NCRYPT_SIGNATURE_INTERFACE,
    NCRYPT_SIGNATURE_OPERATION,
};

const ECDSA_P256_NAME: [u16; 11] = [69, 67, 68, 83, 65, 95, 80, 50, 53, 54, 0];

pub(super) fn validate_provider_algorithm(provider: NCRYPT_PROV_HANDLE) -> Result<()> {
    let status = unsafe { NCryptIsAlgSupported(provider, NCRYPT_ECDSA_P256_ALGORITHM, 0) };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }

    let mut count = 0_u32;
    let mut algorithms = core::ptr::null_mut::<NCryptAlgorithmName>();
    let status = unsafe {
        NCryptEnumAlgorithms(
            provider,
            NCRYPT_SIGNATURE_OPERATION,
            &mut count,
            &mut algorithms,
            0,
        )
    };
    if status != 0 {
        let _ = release_algorithm_buffer(algorithms);
        return Err(Error::Crypto(status as u32));
    }

    let validation = validate_algorithm_list(algorithms, count);
    let release = release_algorithm_buffer(algorithms);
    validation.and(release)
}

fn validate_algorithm_list(algorithms: *mut NCryptAlgorithmName, count: u32) -> Result<()> {
    let count = usize::try_from(count)?;
    if algorithms.is_null() || count == 0 || count > MAX_WIDE_CHARS {
        return Err(Error::CryptoPropertyViolation);
    }
    let algorithms = unsafe { core::slice::from_raw_parts(algorithms, count) };
    for algorithm in algorithms {
        if algorithm.dwClass == NCRYPT_SIGNATURE_INTERFACE
            && algorithm.dwAlgOperations == NCRYPT_SIGNATURE_OPERATION
            && exact_algorithm_name(algorithm.pszName)?
        {
            return Ok(());
        }
    }
    Err(Error::CryptoPropertyViolation)
}

fn exact_algorithm_name(name: windows_sys::core::PWSTR) -> Result<bool> {
    if name.is_null() {
        return Err(Error::CryptoPropertyViolation);
    }
    for (index, expected) in ECDSA_P256_NAME.iter().enumerate() {
        if unsafe { *name.add(index) } != *expected {
            return Ok(false);
        }
    }
    Ok(true)
}

fn release_algorithm_buffer(algorithms: *mut NCryptAlgorithmName) -> Result<()> {
    if algorithms.is_null() {
        return Ok(());
    }
    let status = unsafe { NCryptFreeBuffer(algorithms.cast()) };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    Ok(())
}
