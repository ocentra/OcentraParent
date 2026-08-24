use std::fmt;

use sha2::{Digest, Sha256};

use crate::account_identity_authority::VerifiedAccountIdentityAuthority;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum AccountIdentityIssuerService {
    CloudflareAccountAuthority,
}

impl AccountIdentityIssuerService {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::CloudflareAccountAuthority => "ocentra.account-authority-producer.cloudflare",
        }
    }

    pub(crate) fn from_label(label: &str) -> Option<Self> {
        match label {
            "ocentra.account-authority-producer.cloudflare" => {
                Some(Self::CloudflareAccountAuthority)
            }
            _ => None,
        }
    }
}

/// An Account-owned binding for one authenticated consumer boundary.  The
/// digest is an identifier, not a bearer credential; authentication still
/// requires a real service adapter installed by the owner.
pub(crate) struct AccountIdentityIssuerServiceBinding {
    service: AccountIdentityIssuerService,
    account_id: String,
    household_id: String,
    authority_generation: u64,
    binding_id: String,
}

impl AccountIdentityIssuerServiceBinding {
    pub(crate) fn from_authority(
        authority: &VerifiedAccountIdentityAuthority,
        service: AccountIdentityIssuerService,
    ) -> Result<Self, super::AccountIdentityIssuerError> {
        let account_id = authority.account_id().to_string();
        let household_id = authority.household_id().to_string();
        let authority_generation = authority.authority_generation();
        if account_id.trim().is_empty()
            || household_id.trim().is_empty()
            || authority_generation == 0
        {
            return Err(super::AccountIdentityIssuerError::InvalidServiceBinding);
        }
        let binding_id =
            Self::expected_binding_id(service, &account_id, &household_id, authority_generation);
        Ok(Self {
            service,
            account_id,
            household_id,
            authority_generation,
            binding_id,
        })
    }

    pub(crate) fn service(&self) -> AccountIdentityIssuerService {
        self.service
    }

    pub(crate) fn expected_binding_id(
        service: AccountIdentityIssuerService,
        account_id: &str,
        household_id: &str,
        authority_generation: u64,
    ) -> String {
        let mut digest = Sha256::new();
        digest.update(b"ocentra.account-issuer.service-binding.v1\0");
        for value in [
            service.label().as_bytes(),
            account_id.as_bytes(),
            household_id.as_bytes(),
        ] {
            digest.update((value.len() as u64).to_be_bytes());
            digest.update(value);
        }
        digest.update(authority_generation.to_be_bytes());
        format!("sha256:{:x}", digest.finalize())
    }

    pub(crate) fn account_id(&self) -> &str {
        &self.account_id
    }

    pub(crate) fn household_id(&self) -> &str {
        &self.household_id
    }

    pub(crate) fn authority_generation(&self) -> u64 {
        self.authority_generation
    }

    pub(crate) fn binding_id(&self) -> &str {
        &self.binding_id
    }

    pub(crate) fn matches_authority(&self, authority: &VerifiedAccountIdentityAuthority) -> bool {
        self.account_id == authority.account_id().to_string()
            && self.household_id == authority.household_id().to_string()
            && self.authority_generation == authority.authority_generation()
    }
}

impl fmt::Debug for AccountIdentityIssuerServiceBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccountIdentityIssuerServiceBinding")
            .field("service", &self.service)
            .field("account_id", &"redacted")
            .field("household_id", &"redacted")
            .field("authority_generation", &self.authority_generation)
            .field("binding_id", &"redacted")
            .finish()
    }
}

pub(crate) struct AccountIdentityIssuerAuthenticatedBinding {
    binding_id: String,
}

impl AccountIdentityIssuerAuthenticatedBinding {
    /// Minted only next to the Account-owned authenticator implementation.
    /// Sibling crate modules cannot echo a caller-supplied digest into success.
    fn new(binding: &AccountIdentityIssuerServiceBinding) -> Self {
        Self {
            binding_id: binding.binding_id.clone(),
        }
    }

    pub(crate) fn binding_id(&self) -> &str {
        &self.binding_id
    }
}

/// The platform/consumer adapter that proves the service-binding context.
/// Success evidence is mintable only inside this Account-owned module. A
/// sibling can provide transport to a future adapter, but cannot echo a
/// caller-supplied digest into an authenticated result.
pub(crate) trait AccountIdentityIssuerServiceBindingAuthenticator: Send + Sync {
    fn authenticate(
        &self,
        binding: &AccountIdentityIssuerServiceBinding,
    ) -> Result<AccountIdentityIssuerAuthenticatedBinding, super::AccountIdentityIssuerError>;
}
