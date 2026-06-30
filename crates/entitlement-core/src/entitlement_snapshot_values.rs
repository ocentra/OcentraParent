#![forbid(unsafe_code)]

//! Shared signed-snapshot value types owned by entitlement-core.

use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementSnapshotSignatureState {
    #[serde(rename = "trusted")]
    Trusted,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "invalid")]
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementSnapshotFreshnessState {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementSnapshotBindingState {
    #[serde(rename = "matched")]
    Matched,
    #[serde(rename = "mismatched")]
    Mismatched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementDeviceTrustRequirementState {
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "not-required")]
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementDeviceTrustState {
    #[serde(rename = "present")]
    Present,
    #[serde(rename = "missing")]
    Missing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementPackageBuildState {
    #[serde(rename = "valid")]
    Valid,
    #[serde(rename = "invalid")]
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementSnapshotPlanTier {
    #[serde(rename = "starter")]
    Starter,
    #[serde(rename = "paid")]
    Paid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementProviderStateBoundary {
    #[serde(rename = "input-only")]
    InputOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementAccountAuthorityState {
    #[serde(rename = "verified-account-handoff")]
    VerifiedAccountHandoff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntitlementSafetyFeatureState {
    #[serde(rename = "preserved-outside-paid-gates")]
    PreservedOutsidePaidGates,
}

macro_rules! entitlement_snapshot_text_id {
    ($name:ident, $field:expr) => {
        #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
        #[serde(try_from = "String", into = "String")]
        pub struct $name(String);

        impl $name {
            pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(EventingError::EmptyValue { field: $field });
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $name {
            type Error = EventingError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                value.0
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str(self.as_str())
            }
        }
    };
}

entitlement_snapshot_text_id!(EntitlementSnapshotId, "entitlement.snapshot_id");
entitlement_snapshot_text_id!(EntitlementAccountRef, "entitlement.account_ref");
entitlement_snapshot_text_id!(EntitlementHouseholdRef, "entitlement.household_ref");
entitlement_snapshot_text_id!(
    EntitlementTrustedDeviceRef,
    "entitlement.trusted_device_ref"
);
entitlement_snapshot_text_id!(EntitlementRevocationCursor, "entitlement.revocation_cursor");
entitlement_snapshot_text_id!(EntitlementPackageBuildRef, "entitlement.package_build_ref");
entitlement_snapshot_text_id!(EntitlementSignatureKeyId, "entitlement.signature_key_id");
