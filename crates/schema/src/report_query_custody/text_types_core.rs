use std::fmt::{Display, Formatter};

use super::text_parse::parse_text_identifier;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParentContractSchemaVersion(String);

impl ParentContractSchemaVersion {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for ParentContractSchemaVersion {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParentAccountId(String);

impl ParentAccountId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for ParentAccountId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FamilyId(String);

impl FamilyId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for FamilyId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChildProfileId(String);

impl ChildProfileId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for ChildProfileId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParentDeviceId(String);

impl ParentDeviceId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for ParentDeviceId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParentDeviceLabel(String);

impl ParentDeviceLabel {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for ParentDeviceLabel {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}
