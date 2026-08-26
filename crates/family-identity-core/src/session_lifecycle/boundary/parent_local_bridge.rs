#![forbid(unsafe_code)]

//! Opaque Account-owned parent-local bridge capability custody boundary.
//!
//! A capability is issued only by the Account repository after it has
//! revalidated a `VerifiedAccountIdentityAuthority`. The transport handshake
//! is intentionally separate: it contains the opaque bearer, fixed audience,
//! and owner-generated connection nonce, while the authenticated result below
//! is created only after durable currentness checks.

use std::fmt;

use ocentra_schema::account_identity_parent_local_bridge::{
    AccountIdentityParentLocalBridgeHandshake,
    ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_CAPABILITY_PREFIX,
    ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_DIGEST_HEX_BYTES,
    ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_NONCE_PREFIX,
};
use sha2::{Digest, Sha256};

pub(crate) const CAPABILITY_DIGEST_DOMAIN: &str =
    "ocentra-account-parent-local-bridge-capability-v1";
const CONNECTION_NONCE_DIGEST_DOMAIN: &str = "ocentra-account-parent-local-bridge-nonce-v1";

/// An Account-issued opaque parent-local bridge bearer. It is intentionally
/// not `Clone`, `Serialize`, or `Deserialize`; the typed handshake is the only
/// transport presentation and is still verified against durable custody.
pub struct ParentLocalBridgeSessionCapability(String);

impl fmt::Debug for ParentLocalBridgeSessionCapability {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("ParentLocalBridgeSessionCapability")
            .field("secret", &"<redacted>")
            .finish()
    }
}

impl ParentLocalBridgeSessionCapability {
    pub fn expose_secret(&self) -> &str {
        &self.0
    }

    pub(crate) fn issue() -> Result<Self, getrandom::Error> {
        super::storage_values::generate_opaque_identifier(
            ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_CAPABILITY_PREFIX,
        )
        .map(Self)
    }

    pub(crate) fn digest(&self) -> String {
        digest(CAPABILITY_DIGEST_DOMAIN, self.0.as_bytes())
    }

    pub(crate) fn digest_presented(value: &str) -> Option<String> {
        opaque_value_is_valid(
            value,
            ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_CAPABILITY_PREFIX,
        )
        .then(|| digest(CAPABILITY_DIGEST_DOMAIN, value.as_bytes()))
    }
}

pub(crate) fn issue_connection_nonce() -> Result<String, getrandom::Error> {
    super::storage_values::generate_opaque_identifier(
        ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_NONCE_PREFIX,
    )
}

pub(crate) fn connection_nonce_digest(value: &str) -> Option<String> {
    opaque_value_is_valid(value, ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_NONCE_PREFIX)
        .then(|| digest(CONNECTION_NONCE_DIGEST_DOMAIN, value.as_bytes()))
}

/// Account-owned output containing the bearer and its one-connection typed
/// handshake. The raw bearer is never stored in the repository; only its
/// domain-separated digest is persisted.
pub struct IssuedParentLocalBridgeSession {
    capability: ParentLocalBridgeSessionCapability,
    handshake: AccountIdentityParentLocalBridgeHandshake,
    expires_at_epoch_millis: i64,
}

impl fmt::Debug for IssuedParentLocalBridgeSession {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("IssuedParentLocalBridgeSession")
            .field("capability", &self.capability)
            .field("handshake", &self.handshake)
            .field("expires_at_epoch_millis", &self.expires_at_epoch_millis)
            .finish()
    }
}

impl IssuedParentLocalBridgeSession {
    pub fn capability(&self) -> &ParentLocalBridgeSessionCapability {
        &self.capability
    }

    pub fn handshake(&self) -> &AccountIdentityParentLocalBridgeHandshake {
        &self.handshake
    }

    pub fn expires_at_epoch_millis(&self) -> i64 {
        self.expires_at_epoch_millis
    }

    pub(crate) fn new(
        capability: ParentLocalBridgeSessionCapability,
        handshake: AccountIdentityParentLocalBridgeHandshake,
        expires_at_epoch_millis: i64,
    ) -> Self {
        Self {
            capability,
            handshake,
            expires_at_epoch_millis,
        }
    }
}

fn opaque_value_is_valid(value: &str, prefix: &str) -> bool {
    let Some(suffix) = value.strip_prefix(prefix) else {
        return false;
    };
    suffix.len() == ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_DIGEST_HEX_BYTES
        && suffix
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn digest(domain: &str, value: &[u8]) -> String {
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
