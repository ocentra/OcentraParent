use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

fn parse_text_identifier(value: impl Into<String>) -> Option<String> {
    let value = value.into();
    (!value.trim().is_empty()).then_some(value)
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RetentionDeleteContractVersion(String);

impl RetentionDeleteContractVersion {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for RetentionDeleteContractVersion {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RetentionDeleteRequestId(String);

impl RetentionDeleteRequestId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for RetentionDeleteRequestId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RetentionDeleteRowId(String);

impl RetentionDeleteRowId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for RetentionDeleteRowId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RetentionDeleteFamilyId(String);

impl RetentionDeleteFamilyId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for RetentionDeleteFamilyId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RetentionDeleteActorId(String);

impl RetentionDeleteActorId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for RetentionDeleteActorId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.as_str())
    }
}
