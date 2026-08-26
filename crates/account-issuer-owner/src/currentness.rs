//! Opaque family-owned currentness handle.

use ocentra_family_identity_core::account_identity_authority_issuer_client::
    AccountIdentityIssuerCurrentness;
use ocentra_family_identity_core::account_identity_authority_issuer_client::account_identity_authority_issuer_client_types::{
    AccountIdentityIssuerAccountId, AccountIdentityIssuerHouseholdId,
};

pub struct CurrentAuthority {
    pub(crate) inner: AccountIdentityIssuerCurrentness,
}

impl CurrentAuthority {
    pub fn account_id(&self) -> &AccountIdentityIssuerAccountId {
        self.inner.account_id()
    }

    pub fn household_id(&self) -> &AccountIdentityIssuerHouseholdId {
        self.inner.household_id()
    }

    pub fn authority_generation(&self) -> u64 {
        self.inner.authority_generation()
    }

    pub fn session_generation(&self) -> u64 {
        self.inner.session_generation()
    }
}
