//! Fixed Microsoft Platform Crypto Provider key lifecycle mechanics.
//!
//! This module owns only CNG handles and calls. Enrollment, custody, caller
//! identity, and authority remain outside this unsafe ABI crate.

#![cfg(windows)]

use super::cng::{rsa_3072_modulus, set_fixed_key_security, FIXED_KEY_NAME, REQUIRED_RSA_BITS};
use super::cng_handles::{PcpProviderInner, PcpSigningKeyInner};
use super::cng_observation::{ensure_security_descriptor_support, observe_key, set_u32_property};
use super::cng_sign::sign_digest;
use crate::{Error, OwnedPcpProvider, OwnedPcpSigningKey, Result};
use windows_sys::Win32::Security::Cryptography::{
    NCryptCreatePersistedKey, NCryptDeleteKey, NCryptFinalizeKey, NCryptFreeObject, NCryptOpenKey,
    NCryptOpenStorageProvider, MS_PLATFORM_CRYPTO_PROVIDER, NCRYPT_ALLOW_SIGNING_FLAG,
    NCRYPT_EXPORT_POLICY_PROPERTY, NCRYPT_KEY_HANDLE, NCRYPT_KEY_USAGE_PROPERTY,
    NCRYPT_LENGTH_PROPERTY, NCRYPT_MACHINE_KEY_FLAG, NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY,
    NCRYPT_PCP_SIGNATURE_KEY, NCRYPT_PROV_HANDLE, NCRYPT_RSA_ALGORITHM, NCRYPT_SILENT_FLAG,
};

impl OwnedPcpProvider {
    /// Open exactly the machine-scoped Microsoft Platform Crypto Provider.
    pub fn open_machine() -> Result<Self> {
        let mut handle: NCRYPT_PROV_HANDLE = 0;
        let status =
            unsafe { NCryptOpenStorageProvider(&mut handle, MS_PLATFORM_CRYPTO_PROVIDER, 0) };
        if status != 0 || handle == 0 {
            release_object(handle);
            return Err(Error::Crypto(status as u32));
        }
        Ok(Self {
            inner: PcpProviderInner { handle },
        })
    }

    /// Open the compiled machine key and validate all strict properties.
    pub fn open_fixed_signing_key(self) -> Result<OwnedPcpSigningKey> {
        let provider = self.inner;
        let mut key: NCRYPT_KEY_HANDLE = 0;
        let status = unsafe {
            NCryptOpenKey(
                provider.handle,
                &mut key,
                FIXED_KEY_NAME,
                0,
                NCRYPT_MACHINE_KEY_FLAG | NCRYPT_SILENT_FLAG,
            )
        };
        if status != 0 || key == 0 {
            release_object(key);
            return Err(Error::Crypto(status as u32));
        }
        let owned = OwnedPcpSigningKey {
            inner: PcpSigningKeyInner {
                handle: key,
                _provider: provider,
            },
        };
        observe_key(owned.inner._provider.handle, owned.inner.handle)?;
        Ok(owned)
    }

    /// Create, configure, finalize, and validate the compiled machine key.
    pub fn create_fixed_signing_key(self) -> Result<OwnedPcpSigningKey> {
        let provider = self.inner;
        ensure_security_descriptor_support(provider.handle)?;
        let mut key: NCRYPT_KEY_HANDLE = 0;
        let status = unsafe {
            NCryptCreatePersistedKey(
                provider.handle,
                &mut key,
                NCRYPT_RSA_ALGORITHM,
                FIXED_KEY_NAME,
                0,
                NCRYPT_MACHINE_KEY_FLAG,
            )
        };
        if status != 0 || key == 0 {
            release_object(key);
            return Err(Error::Crypto(status as u32));
        }

        let result = finalize_created_key(key, provider);
        if result.is_err() {
            cleanup_created_key(key);
        }
        result
    }
}

fn release_object(handle: usize) {
    if handle != 0 {
        unsafe { NCryptFreeObject(handle) };
    }
}

fn cleanup_created_key(key: NCRYPT_KEY_HANDLE) {
    let delete_status = unsafe { NCryptDeleteKey(key, NCRYPT_SILENT_FLAG) };
    if delete_status != 0 {
        unsafe { NCryptFreeObject(key) };
    }
}

fn finalize_created_key(
    key: NCRYPT_KEY_HANDLE,
    provider: PcpProviderInner,
) -> Result<OwnedPcpSigningKey> {
    set_u32_property(key, NCRYPT_LENGTH_PROPERTY, REQUIRED_RSA_BITS)?;
    set_u32_property(key, NCRYPT_EXPORT_POLICY_PROPERTY, 0)?;
    set_u32_property(key, NCRYPT_KEY_USAGE_PROPERTY, NCRYPT_ALLOW_SIGNING_FLAG)?;
    set_u32_property(
        key,
        NCRYPT_PCP_KEY_USAGE_POLICY_PROPERTY,
        NCRYPT_PCP_SIGNATURE_KEY,
    )?;
    set_fixed_key_security(key)?;
    let status = unsafe { NCryptFinalizeKey(key, NCRYPT_SILENT_FLAG) };
    if status != 0 {
        return Err(Error::Crypto(status as u32));
    }
    observe_key(provider.handle, key)?;
    Ok(OwnedPcpSigningKey {
        inner: PcpSigningKeyInner {
            handle: key,
            _provider: provider,
        },
    })
}

impl OwnedPcpSigningKey {
    /// Return strict mechanical observations from the retained PCP key.
    pub fn observation(&self) -> Result<crate::PcpKeyObservation> {
        observe_key(self.inner._provider.handle, self.inner.handle)
    }

    /// Return the retained signing key's validated RSA-3072/65537 modulus.
    /// This is distinct from the PCP provider endorsement-key observation.
    pub fn signing_public_modulus(&self) -> Result<[u8; 384]> {
        let blob = super::cng::export_public_key(self.inner.handle)?;
        rsa_3072_modulus(&blob)
    }

    /// Sign one SHA-256 digest using PCP-backed PSS; no private key bytes cross
    /// the boundary and the result has no custody meaning by itself.
    pub fn sign_sha256_digest(&self, digest: &[u8; 32]) -> Result<Vec<u8>> {
        sign_digest(self.inner.handle, digest)
    }
}
