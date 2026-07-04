use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

fn parse_text_identifier(value: impl Into<String>) -> Option<String> {
    let value = value.into();
    (!value.trim().is_empty()).then_some(value)
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SetupDeviceTrustHandoffChildPackageTargetRef(String);

impl SetupDeviceTrustHandoffChildPackageTargetRef {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SetupDeviceTrustHandoffChildPackageTargetRef {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SetupDeviceTrustHandoffArtifactRequirementRef(String);

impl SetupDeviceTrustHandoffArtifactRequirementRef {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SetupDeviceTrustHandoffArtifactRequirementRef {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SetupDeviceTrustHandoffClaimBoundary(String);

impl SetupDeviceTrustHandoffClaimBoundary {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for SetupDeviceTrustHandoffClaimBoundary {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
