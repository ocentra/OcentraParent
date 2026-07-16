use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

pub mod constants;
pub mod enums_capability;
pub mod enums_runtime;
pub mod identifiers;
pub mod lifecycle_proofs;
pub mod proof_types;
pub mod sample;
pub mod surface_proofs;

fn parse_text_identifier(value: impl Into<String>) -> Option<String> {
    let value = value.into();
    (!value.trim().is_empty()).then_some(value)
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChildIosEntitlementBundleId(String);

impl ChildIosEntitlementBundleId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ChildIosEntitlementBundleId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChildIosEntitlementClassName(String);

impl ChildIosEntitlementClassName {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ChildIosEntitlementClassName {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChildIosEntitlementRequirement(String);

impl ChildIosEntitlementRequirement {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ChildIosEntitlementRequirement {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChildIosEntitlementBoundary(String);

impl ChildIosEntitlementBoundary {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ChildIosEntitlementBoundary {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChildIosEntitlementTimestamp(String);

impl ChildIosEntitlementTimestamp {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ChildIosEntitlementTimestamp {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

pub type ChildIosEntitlementParentCapability =
    enums_capability::ChildIosEntitlementParentCapability;
pub type ChildIosEntitlementParentCapabilityStatus =
    enums_capability::ChildIosEntitlementParentCapabilityStatus;
pub type ChildIosEntitlementSurfaceName = enums_capability::ChildIosEntitlementSurfaceName;
pub type ChildIosEntitlementProofState = enums_capability::ChildIosEntitlementProofState;

pub type ChildIosEntitlementRuntimeOwner = enums_runtime::ChildIosEntitlementRuntimeOwner;
pub type ChildIosEntitlementDeclarationState = enums_runtime::ChildIosEntitlementDeclarationState;
pub type ChildIosEntitlementPackagePhase = enums_runtime::ChildIosEntitlementPackagePhase;
pub type ChildIosEntitlementProtocolCommand = enums_runtime::ChildIosEntitlementProtocolCommand;
pub type ChildIosEntitlementProtocolEvent = enums_runtime::ChildIosEntitlementProtocolEvent;
pub type ChildIosEntitlementBridgeState = enums_runtime::ChildIosEntitlementBridgeState;

pub type ChildIosEntitlementSurfaceProof = proof_types::ChildIosEntitlementSurfaceProof;
pub type ChildIosEntitlementPackageLifecycleProof =
    proof_types::ChildIosEntitlementPackageLifecycleProof;
pub type ChildIosEntitlementProtocolBridgeProof =
    proof_types::ChildIosEntitlementProtocolBridgeProof;
pub type ChildIosEntitlementClaimBoundaries = proof_types::ChildIosEntitlementClaimBoundaries;
pub type ChildIosEntitlementCapabilityReadModel =
    proof_types::ChildIosEntitlementCapabilityReadModel;

pub const CHILD_IOS_ENTITLEMENT_CAPABILITY_PROOF_SCHEMA_VERSION: &str =
    constants::CHILD_IOS_ENTITLEMENT_CAPABILITY_PROOF_SCHEMA_VERSION;

pub fn sample_child_ios_entitlement_capability_read_model() -> ChildIosEntitlementCapabilityReadModel
{
    sample::sample_child_ios_entitlement_capability_read_model()
}
