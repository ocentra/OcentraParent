#![forbid(unsafe_code)]

//! Cross-domain policy-control ownership.
//!
//! This crate owns generic policy decision gating that feature crates can
//! consume before child-side enforcement. Feature domains still own their
//! evidence interpretation; enforcement crates own adapter execution.

pub use ocentra_evidence::EvidenceReferenceState;

pub const CRATE_NAME: &str = "ocentra-policy-control-core";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecisionMode {
    ObserveOnly,
    Preview,
    Enforce,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParentAuthorityState {
    Authorized,
    Unauthorized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiResultAuthorityState {
    EvidenceOnly,
    ClaimsAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyActionAuthorizationState {
    Authorized,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyEnforcementExecutionState {
    MayExecute,
    MustNotExecute,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyManualReviewState {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyControlInput {
    pub mode: PolicyDecisionMode,
    pub parent_authority_state: ParentAuthorityState,
    pub evidence_reference_state: EvidenceReferenceState,
    pub ai_result_authority_state: AiResultAuthorityState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PolicyControlDecision {
    pub action_authorization_state: PolicyActionAuthorizationState,
    pub enforcement_execution_state: PolicyEnforcementExecutionState,
    pub manual_review_state: PolicyManualReviewState,
}

pub fn evaluate_policy_control(input: PolicyControlInput) -> PolicyControlDecision {
    let policy_can_authorize_action = input.parent_authority_state == ParentAuthorityState::Authorized
        && input.evidence_reference_state == EvidenceReferenceState::Stable
        && input.ai_result_authority_state == AiResultAuthorityState::EvidenceOnly;
    let enforcement_may_execute =
        policy_can_authorize_action && input.mode == PolicyDecisionMode::Enforce;

    PolicyControlDecision {
        action_authorization_state: if policy_can_authorize_action {
            PolicyActionAuthorizationState::Authorized
        } else {
            PolicyActionAuthorizationState::Blocked
        },
        enforcement_execution_state: if enforcement_may_execute {
            PolicyEnforcementExecutionState::MayExecute
        } else {
            PolicyEnforcementExecutionState::MustNotExecute
        },
        manual_review_state: if policy_can_authorize_action {
            PolicyManualReviewState::NotRequired
        } else {
            PolicyManualReviewState::Required
        },
    }
}
