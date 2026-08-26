use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

use super::account_identity_authority_issuer_client_types::{
    AccountIdentityIssuerAccountId, AccountIdentityIssuerHouseholdId,
};
use super::AccountIdentityIssuerCurrentness;

impl AccountIdentityIssuerCurrentness {
    pub fn account_id(&self) -> &AccountIdentityIssuerAccountId {
        &self.account_id
    }

    pub fn household_id(&self) -> &AccountIdentityIssuerHouseholdId {
        &self.household_id
    }

    pub fn authority_generation(&self) -> u64 {
        self.authority.authority_generation()
    }

    pub fn session_generation(&self) -> u64 {
        self.authority.session_generation()
    }

    pub(crate) fn authority(&self) -> &VerifiedAccountIdentityAuthority {
        &self.authority
    }
}
