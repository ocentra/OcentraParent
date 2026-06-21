use serde::{Deserialize, Serialize};

use super::policy::PolicyRule;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FamilyReference {
    pub family_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChildProfileReference {
    pub child_profile_id: String,
    pub display_name: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParentDeviceReference {
    pub device_id: String,
    pub child_profile_id: Option<String>,
    pub label: String,
    pub platform: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalAiParentRuleContextRef {
    pub parent_rule_ref_id: String,
    pub policy_version: String,
    pub family: FamilyReference,
    pub child_profile: ChildProfileReference,
    pub device: ParentDeviceReference,
    pub rule: PolicyRule,
    pub target_evidence_refs: Vec<String>,
    pub custody: String,
    pub updated_at: String,
    pub expires_at: Option<String>,
}
