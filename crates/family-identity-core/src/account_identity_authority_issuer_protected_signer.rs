//! Account-owned boundary for a protected issuer signer.
//!
//! This module deliberately contains no key generation, key import, scalar
//! storage, or signing implementation.  The only accepted value is the
//! already-opaque signer adapter owned by a platform/HSM integration.  Until
//! that integration is accepted, the issuer remains fail-closed with
//! `SignerCustodyUnavailable`.

use super::key_custody::{AccountIdentityIssuerKeyCustody, AccountIdentityIssuerSignerAdapter};

/// A narrow wrapper that prevents the issuer lifecycle from confusing an
/// adapter boundary with signer custody itself.  Private key material never
/// enters this value; the platform owner retains it outside this crate.
pub(crate) struct AccountIdentityIssuerProtectedSigner {
    custody: AccountIdentityIssuerKeyCustody,
}

impl AccountIdentityIssuerProtectedSigner {
    pub(crate) fn from_platform_owner(signer: Box<dyn AccountIdentityIssuerSignerAdapter>) -> Self {
        Self {
            custody: AccountIdentityIssuerKeyCustody::from_signer(signer),
        }
    }

    pub(crate) fn into_custody(self) -> AccountIdentityIssuerKeyCustody {
        self.custody
    }
}
