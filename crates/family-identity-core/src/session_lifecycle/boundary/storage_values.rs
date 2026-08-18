#![forbid(unsafe_code)]

//! Raw persisted-value and entropy boundary for browser-session custody.

use sha2::{Digest, Sha256};

use super::browser_credentials::{
    IssuedBrowserAccessCredential, IssuedBrowserRefreshCredential,
    PresentedBrowserAccessCredential, PresentedBrowserRefreshCredential,
};

pub(crate) const SESSION_CREDENTIAL_CLASS: &str = "browser-user-session";
pub(crate) const SESSION_DIGEST_ALGORITHM: &str = "sha256";
pub(crate) const SESSION_ACCESS_DIGEST_DOMAIN: &str = "ocentra-account-session-access-v1";
pub(crate) const SESSION_REFRESH_DIGEST_DOMAIN: &str = "ocentra-account-session-refresh-v1";

const ACCESS_BEARER_PREFIX: &str = "ocentra_access_";
const REFRESH_BEARER_PREFIX: &str = "ocentra_refresh_";
const SESSION_FAMILY_PREFIX: &str = "session-family-";
const ENTROPY_BYTES: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionAccessDigest(String);

impl SessionAccessDigest {
    pub(crate) fn from_presented(value: &PresentedBrowserAccessCredential) -> Self {
        Self(domain_hash(
            SESSION_ACCESS_DIGEST_DOMAIN,
            value.bearer().as_bytes(),
        ))
    }

    fn from_bearer(value: &str) -> Self {
        Self(domain_hash(SESSION_ACCESS_DIGEST_DOMAIN, value.as_bytes()))
    }

    pub(crate) fn parse(value: String) -> Option<Self> {
        digest_is_valid(&value).then_some(Self(value))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionRefreshDigest(String);

impl SessionRefreshDigest {
    pub(crate) fn from_presented(value: &PresentedBrowserRefreshCredential) -> Self {
        Self(domain_hash(
            SESSION_REFRESH_DIGEST_DOMAIN,
            value.bearer().as_bytes(),
        ))
    }

    fn from_bearer(value: &str) -> Self {
        Self(domain_hash(SESSION_REFRESH_DIGEST_DOMAIN, value.as_bytes()))
    }

    pub(crate) fn parse(value: String) -> Option<Self> {
        digest_is_valid(&value).then_some(Self(value))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SessionRefreshFamilyId(String);

impl SessionRefreshFamilyId {
    pub(crate) fn generate() -> Result<Self, getrandom::Error> {
        generate_opaque_identifier(SESSION_FAMILY_PREFIX).map(Self)
    }

    pub(crate) fn parse(value: String) -> Option<Self> {
        opaque_identifier_is_valid(&value, SESSION_FAMILY_PREFIX).then_some(Self(value))
    }

    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

pub(crate) struct SessionCredentialMaterial {
    pub(crate) issued_access: IssuedBrowserAccessCredential,
    pub(crate) issued_refresh: IssuedBrowserRefreshCredential,
    pub(crate) access_digest: SessionAccessDigest,
    pub(crate) refresh_digest: SessionRefreshDigest,
}

impl SessionCredentialMaterial {
    pub(crate) fn issue() -> Result<Self, getrandom::Error> {
        let access_bearer = generate_opaque_identifier(ACCESS_BEARER_PREFIX)?;
        let refresh_bearer = generate_opaque_identifier(REFRESH_BEARER_PREFIX)?;
        Ok(Self {
            access_digest: SessionAccessDigest::from_bearer(&access_bearer),
            refresh_digest: SessionRefreshDigest::from_bearer(&refresh_bearer),
            issued_access: IssuedBrowserAccessCredential::from_bearer(access_bearer),
            issued_refresh: IssuedBrowserRefreshCredential::from_bearer(refresh_bearer),
        })
    }
}

pub(crate) fn digest_is_valid(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(super) fn generate_opaque_identifier(prefix: &str) -> Result<String, getrandom::Error> {
    let mut bytes = [0_u8; ENTROPY_BYTES];
    getrandom::fill(&mut bytes)?;
    Ok(format!("{prefix}{}", encode_lower_hex(&bytes)))
}

pub(super) fn opaque_identifier_is_valid(value: &str, prefix: &str) -> bool {
    value.strip_prefix(prefix).is_some_and(digest_is_valid)
}

fn domain_hash(domain: &str, value: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(domain.as_bytes());
    hasher.update([0]);
    hasher.update(value);
    encode_lower_hex(&hasher.finalize())
}

fn encode_lower_hex(value: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(value.len() * 2);
    for byte in value {
        encoded.push(HEX[usize::from(byte >> 4)] as char);
        encoded.push(HEX[usize::from(byte & 0x0f)] as char);
    }
    encoded
}
