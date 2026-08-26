//! Deep, fixed-interface AccountIssuer P-256 CNG capability.
//!
//! Provider selection, key naming, property validation, lifetime, and
//! service-bound admission stay behind this small interface. The type never
//! exposes a provider/key handle or private material.

#![cfg(windows)]

use crate::account_issuer_types::AccountIssuerP256Observation;
use crate::{OwnedService, Result};
use windows_sys::core::PCWSTR;
use windows_sys::Win32::Security::Cryptography::{NCRYPT_KEY_HANDLE, NCRYPT_PROV_HANDLE};

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
pub(super) const ACCOUNT_ISSUER_ALGORITHM_GROUP_NAME: &[u8] = &[69, 67, 68, 83, 65];
pub(super) const BROKER_SERVICE_NAME: &[u8] = &[
    79, 99, 101, 110, 116, 114, 97, 80, 114, 111, 116, 101, 99, 116, 101, 100, 67, 97, 112, 97, 98,
    105, 108, 105, 116, 121, 67, 117, 115, 116, 111, 100, 121, 66, 114, 111, 107, 101, 114,
];
pub(super) struct AccountIssuerP256Handles {
    pub(super) provider: NCRYPT_PROV_HANDLE,
    pub(super) key: NCRYPT_KEY_HANDLE,
}

/// An opened or externally created fixed AccountIssuer key that has not yet
/// been bound to the retained broker service identity.
pub struct AccountIssuerP256Key {
    pub(super) handles: AccountIssuerP256Handles,
    pub(super) baseline: AccountIssuerP256Observation,
    pub(super) permits_external_acl_transition: bool,
}

/// A fixed AccountIssuer key whose complete immutable identity and exact
/// service-only ACL were observed together. This type is deliberately
/// non-Clone and is the only state that may export public material. The
/// private signing mechanic remains unreachable until an owning protocol
/// defines a non-mintable canonical request type.
pub struct BoundAccountIssuerP256Key {
    pub(super) handles: AccountIssuerP256Handles,
    pub(super) baseline: AccountIssuerP256Observation,
    pub(super) service_sid: Vec<u8>,
}

impl AccountIssuerP256Key {
    /// Open the compiled machine key from the Microsoft Platform Crypto
    /// Provider. No provider, key name, path, or private material is caller
    /// selectable.
    pub fn open_machine() -> Result<Self> {
        super::cng_account_issuer_p256_storage::open_existing()
    }

    /// Create the fixed non-exportable key for an external installer
    /// ceremony. The returned capability remains unusable for signing until
    /// the ceremony installs the exact service ACL and `bind_to_service`
    /// revalidates it from SCM.
    pub fn create_for_external_provisioning() -> Result<Self> {
        super::cng_account_issuer_p256_storage::create_for_external_provisioning()
    }

    /// Bind this retained key to the actual broker service observed through
    /// SCM. The unbound state is consumed, so no caller can retain a second
    /// weaker handle or provide an SDDL fragment/service SID.
    pub fn bind_to_service(self, service: &OwnedService) -> Result<BoundAccountIssuerP256Key> {
        super::cng_account_issuer_p256_lifecycle::bind_to_service(self, service)
    }
}

impl BoundAccountIssuerP256Key {
    /// Re-read and compare the complete immutable key/provider observation and
    /// exact service-only descriptor from the retained handles.
    pub fn observation(&self) -> Result<AccountIssuerP256Observation> {
        super::cng_account_issuer_p256_lifecycle::observe_bound(self)
    }

    pub fn revalidate(&self) -> Result<()> {
        super::cng_account_issuer_p256_lifecycle::revalidate_bound(self)
    }

    /// Export only the canonical 65-byte SEC1 public point.
    pub fn public_key_sec1(&self) -> Result<[u8; 65]> {
        self.observation()
            .map(|observation| *observation.public_key_sec1())
    }

    /// Consume one owner-created canonical request and return only the
    /// protocol-owned signed capability.  No raw payload, digest, key, or
    /// provider handle crosses this boundary.
    pub fn sign_prepared_account_issuer_v2(
        &self,
        request: ocentra_protected_capability_custody_protocol::account_issuer_contract::PreparedAccountIssuerV2Request,
    ) -> Result<ocentra_protected_capability_custody_protocol::account_issuer_contract::ProtectedAccountIssuerSignerCapability>
    {
        self.revalidate()?;
        let signature = super::cng_account_issuer_p256_sign::sign_request_digest(
            self.handles.key,
            request.request_digest().as_bytes(),
        )?;
        Ok(
            ocentra_protected_capability_custody_protocol::account_issuer_contract::ProtectedAccountIssuerSignerCapability::from_prepared_request(&request, *signature.as_bytes()),
        )
    }
}
