use std::fmt;

use sha2::{Digest, Sha256};

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
    authority_generation: u64,
}

impl AccountIdentityIssuerServiceBinding {
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
