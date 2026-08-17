//! Rust-owned account identity to family-authority handoff shapes.
//!
//! These types describe the encoded boundary consumed by storage and edge
//! adapters. They do not verify an external provider, mint a session, or
//! authorize a household action.

use serde::{Deserialize, Deserializer, Serialize};

use crate::report_query_custody::{ChildProfileId, FamilyId, ParentAccountId};

pub const ACCOUNT_IDENTITY_AUTHORITY_SCHEMA_VERSION: &str = "v0.7";
pub const ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION: u64 = 9_007_199_254_740_991;
pub const ACCOUNT_IDENTITY_MEMBER_AUTHORITY_SCHEMA_VERSION: &str = "v0.1";

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
account_identity_text_id!(AccountIdentityMemberId);
account_identity_text_id!(AccountIdentityDeviceId);

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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename = "v0.1")]
pub enum AccountIdentityMemberAuthoritySchemaVersion {
    V0_1,
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentityCurrentMemberDeviceAuthority {
    pub account_id: ParentAccountId,
    pub household_id: FamilyId,
    pub member_id: AccountIdentityMemberId,
    pub role: AccountIdentityRole,
    pub account_state: AccountIdentityAccountState,
    pub membership_state: AccountIdentityMembershipState,
    pub device_id: AccountIdentityDeviceId,
    pub device_trust_state: AccountIdentityDeviceTrustState,
    pub session_freshness_state: AccountIdentitySessionFreshnessState,
    pub authority_generation: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountIdentityCurrentMemberDeviceAuthorityHandoff {
    pub schema_version: AccountIdentityMemberAuthoritySchemaVersion,
    pub mapping: AccountIdentityProviderSubjectMapping,
    pub member: AccountIdentityCurrentMemberDeviceAuthority,
    pub binding: AccountIdentityHouseholdChildDeviceBinding,
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountIdentityMemberAuthorityValidationError {
    SchemaVersionMismatch,
    InactiveProviderMapping,
    MappingAccountMismatch,
    MemberAccountMismatch,
    MemberHouseholdMismatch,
    BindingAccountMismatch,
    BindingHouseholdMismatch,
    InactiveAccount,
    InactiveMembership,
    UntrustedDevice,
    StaleSession,
    PairingNotComplete,
    InstallNotComplete,
    LifecycleNotActive,
    Revoked,
    ZeroAuthorityGeneration,
    AuthorityGenerationExceedsSafeInteger,
    AuthorityGenerationMismatch,
}

impl AccountIdentityCurrentMemberDeviceAuthorityHandoff {
    /// Validate the encoded current-authority shape and identity consistency.
    ///
    /// This is a fail-closed handoff contract. It does not replace the durable
    /// repository's compare-and-swap/currentness check or mint authority from
    /// caller-provided headers.
    pub fn validate_shape(&self) -> Result<(), AccountIdentityMemberAuthorityValidationError> {
        (self.schema_version == AccountIdentityMemberAuthoritySchemaVersion::V0_1)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::SchemaVersionMismatch)?;
        (self.mapping.status == AccountIdentityMappingStatus::Active)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::InactiveProviderMapping)?;
        (self.mapping.account_id == self.member.account_id)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::MappingAccountMismatch)?;
        (self.member.account_id == self.binding.account_id)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::MemberAccountMismatch)?;
        (self.member.household_id == self.binding.household_id)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::MemberHouseholdMismatch)?;
        (self.binding.account_id == self.member.account_id)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::BindingAccountMismatch)?;
        (self.binding.household_id == self.member.household_id)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::BindingHouseholdMismatch)?;
        (self.member.account_state == AccountIdentityAccountState::Active)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::InactiveAccount)?;
        (self.member.membership_state == AccountIdentityMembershipState::Active)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::InactiveMembership)?;
        (self.member.device_trust_state == AccountIdentityDeviceTrustState::Trusted)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::UntrustedDevice)?;
        (self.member.session_freshness_state == AccountIdentitySessionFreshnessState::Fresh)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::StaleSession)?;
        (self.binding.pairing_state == AccountIdentityPairingState::Paired)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::PairingNotComplete)?;
        (self.binding.install_state == AccountIdentityInstallState::Installed)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::InstallNotComplete)?;
        (self.binding.lifecycle_state == AccountIdentityBindingLifecycleState::Active)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::LifecycleNotActive)?;
        (self.binding.revocation_state == AccountIdentityBindingRevocationState::Active)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::Revoked)?;
        (self.member.authority_generation > 0)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::ZeroAuthorityGeneration)?;
        (self.member.authority_generation <= ACCOUNT_IDENTITY_AUTHORITY_MAX_GENERATION)
            .then_some(())
            .ok_or(
                AccountIdentityMemberAuthorityValidationError::AuthorityGenerationExceedsSafeInteger,
            )?;
        (self.member.authority_generation == self.binding.authority_generation)
            .then_some(())
            .ok_or(AccountIdentityMemberAuthorityValidationError::AuthorityGenerationMismatch)?;
        self.binding.validate_shape().map_err(|error| match error {
            AccountIdentityBindingValidationError::SchemaVersionMismatch => {
                AccountIdentityMemberAuthorityValidationError::SchemaVersionMismatch
            }
            AccountIdentityBindingValidationError::InactiveProviderMapping => {
                AccountIdentityMemberAuthorityValidationError::InactiveProviderMapping
            }
            AccountIdentityBindingValidationError::MappingAccountMismatch => {
                AccountIdentityMemberAuthorityValidationError::MappingAccountMismatch
            }
            AccountIdentityBindingValidationError::ZeroAuthorityGeneration => {
                AccountIdentityMemberAuthorityValidationError::ZeroAuthorityGeneration
            }
            AccountIdentityBindingValidationError::AuthorityGenerationExceedsSafeInteger => {
                AccountIdentityMemberAuthorityValidationError::AuthorityGenerationExceedsSafeInteger
            }
        })
    }
}
