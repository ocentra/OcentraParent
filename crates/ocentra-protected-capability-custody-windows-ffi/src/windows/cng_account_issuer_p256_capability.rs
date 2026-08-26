//! Deep, fixed-interface AccountIssuer P-256 CNG capability.
//!
//! Provider selection, key naming, property validation, lifetime, and
//! service-bound admission stay behind this small interface. The type never
//! exposes a provider/key handle or private material.

#![cfg(windows)]

use crate::account_issuer_types::{AccountIssuerP256Observation, AccountIssuerP256Signature};
use crate::{Error, OwnedService, Result};
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Security::Cryptography::{
    NCryptCreatePersistedKey, NCryptDeleteKey, NCryptFinalizeKey, NCryptFreeObject,
    NCryptIsAlgSupported, NCryptOpenKey, NCryptOpenStorageProvider, MS_PLATFORM_CRYPTO_PROVIDER,
    NCRYPT_ALLOW_SIGNING_FLAG, NCRYPT_ECDSA_P256_ALGORITHM, NCRYPT_EXPORT_POLICY_PROPERTY,
    NCRYPT_KEY_HANDLE, NCRYPT_KEY_USAGE_PROPERTY, NCRYPT_LENGTH_PROPERTY, NCRYPT_MACHINE_KEY_FLAG,
    NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY, NCRYPT_PCP_SIGNATURE_KEY, NCRYPT_PROV_HANDLE,
    NCRYPT_SILENT_FLAG,
};

pub(super) const ACCOUNT_ISSUER_KEY_NAME: &[u8] = &[
    79, 99, 101, 110, 116, 114, 97, 80, 97, 114, 101, 110, 116, 46, 80, 114, 111, 116, 101, 99,
    116, 101, 100, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 67, 117, 115, 116, 111, 100, 121,
    46, 65, 99, 99, 111, 117, 110, 116, 73, 115, 115, 117, 101, 114, 46, 118, 50,
];
pub(super) static ACCOUNT_ISSUER_KEY_NAME_WIDE_UNITS: [u16; 58] = [
    79, 99, 101, 110, 116, 114, 97, 80, 97, 114, 101, 110, 116, 46, 80, 114, 111, 116, 101, 99,
    116, 101, 100, 67, 97, 112, 97, 98, 105, 108, 105, 116, 121, 67, 117, 115, 116, 111, 100, 121,
    46, 65, 99, 99, 111, 117, 110, 116, 73, 115, 115, 117, 101, 114, 46, 118, 50, 0,
];
pub(super) const ACCOUNT_ISSUER_KEY_NAME_WIDE: PCWSTR = ACCOUNT_ISSUER_KEY_NAME_WIDE_UNITS.as_ptr();
pub(super) const ACCOUNT_ISSUER_ALGORITHM_NAME: &[u8] = &[69, 67, 68, 83, 65, 95, 80, 50, 53, 54];
pub(super) const BROKER_SERVICE_NAME: &[u8] = &[
    79, 99, 101, 110, 116, 114, 97, 80, 114, 111, 116, 101, 99, 116, 101, 100, 67, 97, 112, 97, 98,
    105, 108, 105, 116, 121, 67, 117, 115, 116, 111, 100, 121, 66, 114, 111, 107, 101, 114,
];
pub(super) const ACCOUNT_ISSUER_SIGNING_DOMAIN: &[u8] = &[
    111, 99, 101, 110, 116, 114, 97, 46, 97, 99, 99, 111, 117, 110, 116, 45, 97, 117, 116, 104,
    111, 114, 105, 116, 121, 45, 112, 114, 111, 100, 117, 99, 101, 114, 46, 115, 105, 103, 110,
    105, 110, 103, 46, 118, 50, 0,
];

pub struct AccountIssuerP256Key {
    pub(super) provider: NCRYPT_PROV_HANDLE,
    pub(super) key: NCRYPT_KEY_HANDLE,
    pub(super) observation: Option<AccountIssuerP256Observation>,
    pub(super) service_sid: Option<Vec<u8>>,
}

impl AccountIssuerP256Key {
    /// Open the compiled machine key from the Microsoft Platform Crypto
    /// Provider. No provider, key name, path, or private material is caller
    /// selectable.
    pub fn open_machine() -> Result<Self> {
        open_existing()
    }

    /// Create the fixed non-exportable key for an external installer
    /// ceremony. The returned capability remains unusable for signing until
    /// the ceremony installs the exact service ACL and `bind_to_service`
    /// revalidates it from SCM.
    pub fn create_for_external_provisioning() -> Result<Self> {
        create_for_external_provisioning()
    }

    /// Re-read the key's strict mechanical properties from the retained CNG
    /// handle. This does not mint enrollment or authority.
    pub fn observation(&self) -> Result<AccountIssuerP256Observation> {
        super::cng_account_issuer_p256_lifecycle::observe(self)
    }

    /// Revalidate the retained key identity and, when bound, its exact
    /// service-only descriptor before any public export or signing operation.
    pub fn revalidate(&self) -> Result<()> {
        super::cng_account_issuer_p256_lifecycle::revalidate(self)
    }

    /// Bind this retained key to the actual broker service observed through
    /// SCM. A caller cannot provide an SDDL fragment or service SID.
    pub fn bind_to_service(&mut self, service: &OwnedService) -> Result<()> {
        super::cng_account_issuer_p256_lifecycle::bind_to_service(self, service)
    }

    /// Export only the canonical 65-byte SEC1 public point.
    pub fn public_key_sec1(&self) -> Result<[u8; 65]> {
        self.revalidate()?;
        self.observation()
            .map(|observation| *observation.public_key_sec1())
    }

    /// Hash the exact domain-bound canonical Account payload once and return
    /// a fixed-width low-S P1363 signature. Caller-supplied digests, padding,
    /// private export, and algorithm fallback are not accepted.
    pub fn sign_domain_bound_request(
        &self,
        canonical_payload: &[u8],
    ) -> Result<AccountIssuerP256Signature> {
        self.revalidate()?;
        super::cng_account_issuer_p256_sign::sign_domain_bound_request(
            self.key,
            canonical_payload,
            self.service_sid.is_some(),
        )
    }
}

fn open_existing() -> Result<AccountIssuerP256Key> {
    let provider = open_provider()?;
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
    if status != 0 || key == 0 {
        release_key(key);
        release_provider(provider);
        return Err(Error::Crypto(status as u32));
    }
    let mut owned = AccountIssuerP256Key {
        provider,
        key,
        observation: None,
        service_sid: None,
    };
    let observation = super::cng_account_issuer_p256_security::observe_key(provider, key)?;
    owned.observation = Some(observation);
    Ok(owned)
}

fn create_for_external_provisioning() -> Result<AccountIssuerP256Key> {
    let provider = open_provider()?;
    if let Err(error) =
        super::cng_account_issuer_p256_security::ensure_security_descriptor_support(provider)
    {
        release_provider(provider);
        return Err(error);
    }
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
    if status != 0 || key == 0 {
        release_key(key);
        release_provider(provider);
        return Err(Error::Crypto(status as u32));
    }
    configure_and_finalize(key).map_err(|error| {
        delete_or_release_key(key);
        release_provider(provider);
        error
    })?;
    Ok(AccountIssuerP256Key {
        provider,
        key,
        observation: None,
        service_sid: None,
    })
}

fn open_provider() -> Result<NCRYPT_PROV_HANDLE> {
    let mut provider = 0_usize;
    let status =
        unsafe { NCryptOpenStorageProvider(&mut provider, MS_PLATFORM_CRYPTO_PROVIDER, 0) };
    if status != 0 || provider == 0 {
        release_provider(provider);
        return Err(Error::Crypto(status as u32));
    }
    let support_status =
        unsafe { NCryptIsAlgSupported(provider, NCRYPT_ECDSA_P256_ALGORITHM, NCRYPT_SILENT_FLAG) };
    if support_status != 0 {
        release_provider(provider);
        return Err(Error::Crypto(support_status as u32));
    }
    Ok(provider)
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

impl Drop for AccountIssuerP256Key {
    fn drop(&mut self) {
        unsafe { NCryptFreeObject(self.key) };
        unsafe { NCryptFreeObject(self.provider) };
    }
}
