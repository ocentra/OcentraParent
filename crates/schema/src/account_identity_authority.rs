//! Rust-owned account identity to family-authority handoff shapes.
//!
//! These types describe the encoded boundary consumed by storage and edge
//! adapters. They do not verify an external provider, mint a session, or
//! authorize a household action.

use serde::{Deserialize, Deserializer, Serialize};

use crate::report_query_custody::{ChildProfileId, FamilyId, ParentAccountId};

pub const ACCOUNT_IDENTITY_AUTHORITY_SCHEMA_VERSION: &str = "v0.7";
pub const ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION: u64 = 9_007_199_254_740_991;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "v0.7")]
pub enum AccountIdentityAuthoritySchemaVersion {
    V0_7,
}

macro_rules! account_identity_text_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value = String::deserialize(deserializer)?;
                Self::parse(value)
                    .ok_or("account identity identifier must be non-empty")
                    .map_err(serde::de::Error::custom)
            }
        }

        impl $name {
            pub fn parse(value: impl Into<String>) -> Option<Self> {
                let value = value.into();
                (!value.trim().is_empty()).then_some(Self(value))
            }
        }
    };
}

account_identity_text_id!(AccountIdentityChildDeviceId);
account_identity_text_id!(AccountIdentityPairingId);
account_identity_text_id!(AccountIdentityInstallationId);
account_identity_text_id!(AccountIdentityRouteId);
account_identity_text_id!(AccountIdentityProviderSubject);

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountIdentityPairingState {
    Pending,
    Paired,
    Unpaired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountIdentityInstallState {
    Pending,
    Installed,
    Failed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountIdentitySelectedRouteKind {
    Local,
    Lan,
    Remote,
    ManualRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountIdentityBindingLifecycleState {
    Pending,
    Active,
    Suspended,
    Removed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AccountIdentityBindingRevocationState {
    Active,
    Revoked,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentityProviderSubjectMapping {
    pub account_id: ParentAccountId,
    pub provider: AccountIdentityProvider,
    pub provider_subject: AccountIdentityProviderSubject,
    pub status: AccountIdentityMappingStatus,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentityHouseholdChildDeviceBinding {
    pub account_id: ParentAccountId,
    pub household_id: FamilyId,
    pub child_profile_id: ChildProfileId,
    pub child_device_id: AccountIdentityChildDeviceId,
    pub pairing_id: AccountIdentityPairingId,
    pub installation_id: AccountIdentityInstallationId,
    pub selected_route_id: AccountIdentityRouteId,
    pub pairing_state: AccountIdentityPairingState,
    pub install_state: AccountIdentityInstallState,
    pub selected_route: AccountIdentitySelectedRouteKind,
    pub lifecycle_state: AccountIdentityBindingLifecycleState,
    pub revocation_state: AccountIdentityBindingRevocationState,
    pub authority_generation: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountIdentityBindingValidationError {
    SchemaVersionMismatch,
    InactiveProviderMapping,
    MappingAccountMismatch,
    ZeroAuthorityGeneration,
    AuthorityGenerationExceedsSafeInteger,
}

impl AccountIdentityHouseholdChildDeviceBinding {
    /// Validate only the encoded binding shape.
    ///
    /// This does not establish current authority. Durable account storage must
    /// atomically resolve the provider mapping and binding, then own lifecycle,
    /// revocation, and generation-currentness checks.
    pub fn validate_shape(&self) -> Result<(), AccountIdentityBindingValidationError> {
        if self.authority_generation == 0 {
            return Err(AccountIdentityBindingValidationError::ZeroAuthorityGeneration);
        }
        (self.authority_generation <= ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION)
            .then_some(())
            .ok_or(AccountIdentityBindingValidationError::AuthorityGenerationExceedsSafeInteger)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentityAuthorityHandoff {
    pub schema_version: AccountIdentityAuthoritySchemaVersion,
    pub mapping: AccountIdentityProviderSubjectMapping,
    pub binding: AccountIdentityHouseholdChildDeviceBinding,
}

impl AccountIdentityAuthorityHandoff {
    /// Validate only the encoded handoff shape and mapping consistency.
    ///
    /// This is an identity lookup envelope, not a role, session, device-trust,
    /// or action authority. Currentness remains repository-owned.
    pub fn validate_shape(&self) -> Result<(), AccountIdentityBindingValidationError> {
        (self.schema_version == AccountIdentityAuthoritySchemaVersion::V0_7)
            .then_some(())
            .ok_or(AccountIdentityBindingValidationError::SchemaVersionMismatch)?;
        (self.mapping.status == AccountIdentityMappingStatus::Active)
            .then_some(())
            .ok_or(AccountIdentityBindingValidationError::InactiveProviderMapping)?;
        (self.mapping.account_id == self.binding.account_id)
            .then_some(())
            .ok_or(AccountIdentityBindingValidationError::MappingAccountMismatch)?;
        self.binding.validate_shape()
    }
}
