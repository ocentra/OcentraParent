//! Account-owned contract for the parent-local authenticated bridge handshake.
//!
//! The handshake is an untrusted transport presentation. It contains only an
//! opaque Account-issued capability, a fixed audience, and an Account-issued
//! connection nonce. It carries no actor, account, household, device, role,
//! session, or authority-generation claims. The Account repository must
//! resolve and validate those bindings before a caller treats the handshake as
//! authenticated.

use std::fmt;

use serde::{Deserialize, Serialize};

pub const ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_SCHEMA_VERSION: u16 = 1;
pub const ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_CAPABILITY_PREFIX: &str = "ocentra_parent_bridge_";
pub const ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_NONCE_PREFIX: &str = "ocentra_parent_nonce_";
pub const ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_DIGEST_HEX_BYTES: usize = 64;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountIdentityParentLocalBridgeAudience {
    ParentDesktopAgentService,
}

impl AccountIdentityParentLocalBridgeAudience {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParentDesktopAgentService => "parent-desktop-agent-service",
        }
    }
}

/// Untrusted wire presentation for one parent-local bridge authentication
/// attempt. The capability and nonce are validated as opaque values, then
/// matched against Account-owned durable custody; they are never identity
/// authority by themselves.
#[derive(Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AccountIdentityParentLocalBridgeHandshake {
    pub schema_version: u16,
    pub capability: String,
    pub audience: AccountIdentityParentLocalBridgeAudience,
    pub connection_nonce: String,
}

impl fmt::Debug for AccountIdentityParentLocalBridgeHandshake {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("AccountIdentityParentLocalBridgeHandshake")
            .field("schema_version", &self.schema_version)
            .field("capability", &"<redacted>")
            .field("audience", &self.audience)
            .field("connection_nonce", &"<redacted>")
            .finish()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountIdentityParentLocalBridgeHandshakeValidationError {
    UnsupportedSchemaVersion,
    InvalidCapability,
    InvalidConnectionNonce,
}

impl AccountIdentityParentLocalBridgeHandshake {
    pub fn validate_shape(
        &self,
    ) -> Result<(), AccountIdentityParentLocalBridgeHandshakeValidationError> {
        if self.schema_version != ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_SCHEMA_VERSION {
            return Err(
                AccountIdentityParentLocalBridgeHandshakeValidationError::UnsupportedSchemaVersion,
            );
        }
        if !opaque_value_is_valid(
            &self.capability,
            ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_CAPABILITY_PREFIX,
        ) {
            return Err(
                AccountIdentityParentLocalBridgeHandshakeValidationError::InvalidCapability,
            );
        }
        if !opaque_value_is_valid(
            &self.connection_nonce,
            ACCOUNT_IDENTITY_PARENT_LOCAL_BRIDGE_NONCE_PREFIX,
        ) {
            return Err(
                AccountIdentityParentLocalBridgeHandshakeValidationError::InvalidConnectionNonce,
            );
        }
        Ok(())
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
