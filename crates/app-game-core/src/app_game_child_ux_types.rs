use ocentra_eventing::error::EventingError;
use serde::{Deserialize, Serialize};

use crate::app_game_policy_evaluator_runtime::types::AppGamePolicyRuntimeDecision;
use crate::app_game_policy_target_compiler::references::{
    AppGamePolicyEvidenceRef, AppGamePolicyRuleRef,
};

const CHILD_UX_REFERENCE_FIELDS: [&str; 3] = [
    "app_game.child_ux.child_reason_ref",
    "app_game.child_ux.child_status_ref",
    "app_game.child_ux.adapter_action_ref",
];

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct AppGameChildUxReference<const KIND: usize>(String);

impl<const KIND: usize> AppGameChildUxReference<KIND> {
    pub fn parse(value: impl Into<String>) -> Result<Self, EventingError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(EventingError::EmptyValue {
                field: CHILD_UX_REFERENCE_FIELDS[KIND],
            });
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<const KIND: usize> TryFrom<String> for AppGameChildUxReference<KIND> {
    type Error = EventingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl<const KIND: usize> From<AppGameChildUxReference<KIND>> for String {
    fn from(value: AppGameChildUxReference<KIND>) -> Self {
        value.0
    }
}

pub type AppGameChildReasonRef = AppGameChildUxReference<0>;
pub type AppGameChildStatusRef = AppGameChildUxReference<1>;
pub type AppGameChildAdapterActionRef = AppGameChildUxReference<2>;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameChildUxSubjectKind {
    App,
    Game,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameChildUxRequestState {
    None,
    ApprovalNeeded,
    Submitted,
    Approved,
    Denied,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameChildUxCapabilityState {
    Available,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameChildUxNoticeState {
    NoNotice,
    AppLimited,
    NewAppNeedsApproval,
    NewGameNeedsApproval,
    GameTimeAlmostFinished,
    RequestSubmitted,
    RequestApproved,
    RequestDenied,
    ManualRequired,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameChildUxTextToken {
    NoNotice,
    FamilyRuleAppLimit,
    FamilyRuleNewAppApproval,
    FamilyRuleNewGameApproval,
    FamilyRuleGameTimeAlmostFinished,
    FamilyRuleRequestSubmitted,
    FamilyRuleRequestApproved,
    FamilyRuleRequestDenied,
    FamilyRuleNeedsHelp,
    FamilyRuleUnavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGameChildUxAction {
    None,
    AskParent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameChildUxInput {
    pub subject_kind: AppGameChildUxSubjectKind,
    pub runtime_decision: AppGamePolicyRuntimeDecision,
    pub request_state: AppGameChildUxRequestState,
    pub capability_state: AppGameChildUxCapabilityState,
    pub policy_rule_ref: AppGamePolicyRuleRef,
    pub evidence_refs: Vec<AppGamePolicyEvidenceRef>,
    pub child_reason_refs: Vec<AppGameChildReasonRef>,
    pub child_status_refs: Vec<AppGameChildStatusRef>,
    pub adapter_action_ref: Option<AppGameChildAdapterActionRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGameChildUxNotice {
    pub state: AppGameChildUxNoticeState,
    pub text_token: AppGameChildUxTextToken,
    pub action: AppGameChildUxAction,
    pub policy_rule_ref: AppGamePolicyRuleRef,
    pub evidence_refs: Vec<AppGamePolicyEvidenceRef>,
    pub child_reason_refs: Vec<AppGameChildReasonRef>,
    pub child_status_refs: Vec<AppGameChildStatusRef>,
    pub remaining_seconds: u64,
    pub adapter_dispatch_claimed: bool,
}
