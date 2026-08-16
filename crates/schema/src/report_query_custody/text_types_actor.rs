use std::fmt::{Display, Formatter};

use super::text_parse::parse_text_identifier;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParentActorId(String);

impl ParentActorId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for ParentActorId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParentPolicyVersion(String);

impl ParentPolicyVersion {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for ParentPolicyVersion {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParentEvidenceReferenceId(String);

impl ParentEvidenceReferenceId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for ParentEvidenceReferenceId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParentActionReferenceId(String);

impl ParentActionReferenceId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for ParentActionReferenceId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ParentTimestamp(String);

impl ParentTimestamp {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for ParentTimestamp {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReportQueryCustodyRequestId(String);

impl ReportQueryCustodyRequestId {
    pub fn parse(value: impl Into<String>) -> Option<Self> {
        parse_text_identifier(value).map(Self)
    }
}

impl Display for ReportQueryCustodyRequestId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}
