//! Fixed provider, persisted-key creation, and retained-handle lifetime.

#![cfg(windows)]

use super::cng_account_issuer_p256_capability::{
    AccountIssuerP256Handles, AccountIssuerP256Key, ACCOUNT_ISSUER_KEY_NAME_WIDE,
};
use crate::{Error, Result};
use windows_sys::Win32::Security::Cryptography::{
    NCryptCreatePersistedKey, NCryptDeleteKey, NCryptFinalizeKey, NCryptFreeObject, NCryptOpenKey,
    NCryptOpenStorageProvider, MS_PLATFORM_CRYPTO_PROVIDER, NCRYPT_ALLOW_SIGNING_FLAG,
    NCRYPT_ECDSA_P256_ALGORITHM, NCRYPT_EXPORT_POLICY_PROPERTY, NCRYPT_KEY_HANDLE,
    NCRYPT_KEY_USAGE_PROPERTY, NCRYPT_LENGTH_PROPERTY, NCRYPT_MACHINE_KEY_FLAG,
    NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY, NCRYPT_PCP_SIGNATURE_KEY, NCRYPT_PROV_HANDLE,
    NCRYPT_SILENT_FLAG,
};

pub(super) fn open_existing() -> Result<AccountIssuerP256Key> {
    let provider = open_provider()?;
    let key = open_persisted_key(provider).inspect_err(|_error| {
        release_provider(provider);
    })?;
    let baseline = super::cng_account_issuer_p256_security::observe_key(provider, key)
        .inspect_err(|_error| {
            release_key(key);
            release_provider(provider);
        })?;
    Ok(AccountIssuerP256Key {
        handles: AccountIssuerP256Handles { provider, key },
        baseline,
        permits_external_acl_transition: false,
    })
}

pub(super) fn create_for_external_provisioning() -> Result<AccountIssuerP256Key> {
    let provider = open_provider()?;
    super::cng_account_issuer_p256_security::ensure_security_descriptor_support(provider)
        .inspect_err(|_error| {
            release_provider(provider);
        })?;
    create_key(provider)
}

fn open_persisted_key(provider: NCRYPT_PROV_HANDLE) -> Result<NCRYPT_KEY_HANDLE> {
    let mut key = 0_usize;
    let status = unsafe {
        NCryptOpenKey(
            provider,
            &mut key,
            ACCOUNT_ISSUER_KEY_NAME_WIDE,
            0,
            NCRYPT_MACHINE_KEY_FLAG | NCRYPT_SILENT_FLAG,
        )
    };
    validate_handle_status(status, key).inspect_err(|_error| {
        release_key(key);
    })?;
    Ok(key)
}

fn create_persisted_key(provider: NCRYPT_PROV_HANDLE) -> Result<NCRYPT_KEY_HANDLE> {
    let mut key = 0_usize;
    let status = unsafe {
        NCryptCreatePersistedKey(
            provider,
            &mut key,
            NCRYPT_ECDSA_P256_ALGORITHM,
            ACCOUNT_ISSUER_KEY_NAME_WIDE,
            0,
            NCRYPT_MACHINE_KEY_FLAG,
        )
    };
    validate_handle_status(status, key).inspect_err(|_error| {
        release_key(key);
    })?;
    Ok(key)
}

fn create_key(provider: NCRYPT_PROV_HANDLE) -> Result<AccountIssuerP256Key> {
    let key = create_persisted_key(provider).inspect_err(|_error| {
        release_provider(provider);
    })?;
    finish_created_key(provider, key)
}

fn finish_created_key(
    provider: NCRYPT_PROV_HANDLE,
    key: NCRYPT_KEY_HANDLE,
) -> Result<AccountIssuerP256Key> {
    configure_and_finalize(key).inspect_err(|_error| {
        delete_or_release_key(key);
        release_provider(provider);
    })?;
    let baseline =
        super::cng_account_issuer_p256_security::observe_key_for_external_acl_transition(
            provider, key,
        )
        .inspect_err(|_error| {
            delete_or_release_key(key);
            release_provider(provider);
        })?;
    Ok(AccountIssuerP256Key {
        handles: AccountIssuerP256Handles { provider, key },
        baseline,
        permits_external_acl_transition: true,
    })
}

fn open_provider() -> Result<NCRYPT_PROV_HANDLE> {
    let mut provider = 0_usize;
    let status =
        unsafe { NCryptOpenStorageProvider(&mut provider, MS_PLATFORM_CRYPTO_PROVIDER, 0) };
    validate_handle_status(status, provider).inspect_err(|_error| {
        release_provider(provider);
    })?;
    super::cng_account_issuer_p256_algorithm::validate_provider_algorithm(provider).inspect_err(
        |_error| {
            release_provider(provider);
        },
    )?;
    Ok(provider)
}

fn validate_handle_status(status: i32, handle: usize) -> Result<()> {
    match (status, handle) {
        (0, 0) => Err(Error::CryptoPropertyViolation),
        (0, _) => Ok(()),
        (error, _) => Err(Error::Crypto(error as u32)),
    }
}

fn configure_and_finalize(key: NCRYPT_KEY_HANDLE) -> Result<()> {
    super::cng_account_issuer_p256_security::set_u32_property(key, NCRYPT_LENGTH_PROPERTY, 256)?;
    super::cng_account_issuer_p256_security::set_u32_property(
        key,
        NCRYPT_EXPORT_POLICY_PROPERTY,
        0,
    )?;
    super::cng_account_issuer_p256_security::set_u32_property(
        key,
        NCRYPT_KEY_USAGE_PROPERTY,
        NCRYPT_ALLOW_SIGNING_FLAG,
    )?;
    super::cng_account_issuer_p256_security::set_u32_property(
        key,
        NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY,
        NCRYPT_PCP_SIGNATURE_KEY,
    )?;
    let status = unsafe { NCryptFinalizeKey(key, NCRYPT_SILENT_FLAG) };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    Ok(())
}

fn delete_or_release_key(key: NCRYPT_KEY_HANDLE) {
    let status = unsafe { NCryptDeleteKey(key, NCRYPT_SILENT_FLAG) };
    if status != 0 {
        release_key(key);
    }
}

fn release_provider(provider: NCRYPT_PROV_HANDLE) {
    if provider != 0 {
        unsafe { NCryptFreeObject(provider) };
    }
}

fn release_key(key: NCRYPT_KEY_HANDLE) {
    if key != 0 {
        unsafe { NCryptFreeObject(key) };
    }
}

impl Drop for AccountIssuerP256Handles {
    fn drop(&mut self) {
        release_key(self.key);
        release_provider(self.provider);
    }
}
