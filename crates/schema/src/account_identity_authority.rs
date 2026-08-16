//! Rust-owned account identity to family-authority handoff shapes.
//!
//! These types describe the encoded boundary consumed by storage and edge
//! adapters. They do not verify an external provider, mint a session, or
//! authorize a household action.

use serde::{Deserialize, Serialize};

pub const ACCOUNT_IDENTITY_AUTHORITY_SCHEMA_VERSION: &str = "v0.6";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountIdentityProvider {
    Authjs,
    Firebase,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountIdentityMappingStatus {
    Active,
    Revoked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountIdentityAccountState {
    Active,
    Suspended,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccountIdentityMembershipState {
    Invited,
    Pending,
    Active,
    Revoked,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountIdentityRole {
    ParentOwner,
    CoParentGuardian,
    Observer,
    ChildProfile,
    ChildDeviceAgent,
    SupportAdmin,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountIdentityDeviceTrustState {
    Pending,
    Trusted,
    Revoked,
    ResetRequired,
    Disabled,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountIdentitySessionFreshnessState {
    Fresh,
    Stale,
    Expired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentityProviderSubjectMapping {
    pub account_id: String,
    pub provider: AccountIdentityProvider,
    pub provider_subject: String,
    pub status: AccountIdentityMappingStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentityAuthoritySnapshot {
    pub account_id: String,
    pub account_state: AccountIdentityAccountState,
    pub household_id: Option<String>,
    pub member_id: Option<String>,
    pub membership_state: Option<AccountIdentityMembershipState>,
    pub role: Option<AccountIdentityRole>,
    pub child_profile_id: Option<String>,
    pub device_id: Option<String>,
    pub device_trust_state: Option<AccountIdentityDeviceTrustState>,
    pub session_id: Option<String>,
    pub session_freshness_state: Option<AccountIdentitySessionFreshnessState>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentityAuthorityHandoff {
    pub schema_version: String,
    pub mapping: AccountIdentityProviderSubjectMapping,
    pub authority: Option<AccountIdentityAuthoritySnapshot>,
}
