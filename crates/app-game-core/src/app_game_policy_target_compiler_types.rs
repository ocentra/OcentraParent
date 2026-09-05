use serde::{Deserialize, Serialize};

use super::references::{
    AppGamePolicyAuditRef, AppGamePolicyAuthorityRef, AppGamePolicyCapabilityRef,
    AppGamePolicyCompileRequestId, AppGamePolicyCompiledDecisionId, AppGamePolicyDeviceId,
    AppGamePolicyEvidenceRef, AppGamePolicyLocalUserRef, AppGamePolicyRuleRef,
    AppGamePolicyScheduleRef, AppGamePolicyTargetRef,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyTargetKind {
    SpecificApp,
    PackageId,
    BundleId,
    AppUserModelId,
    DesktopEntryId,
    ExecutableHash,
    Publisher,
    AppCategory,
    UnknownApp,
    NewApp,
    PortableApp,
    RiskApp,
    AllNonSystemApps,
    SpecificGame,
    LauncherGameId,
    StoreGameId,
    GameCategory,
    UnknownGame,
    NewGame,
    LauncherGameCandidate,
    MultiplayerGame,
    UgcGame,
    PurchaseCapableGame,
    MatureGame,
    AllGames,
}

impl AppGamePolicyTargetKind {
    pub(super) fn permits_missing_target_ref(self) -> bool {
        matches!(self, Self::AllNonSystemApps | Self::AllGames)
    }

    pub(super) fn requires_identity_proof(self) -> bool {
        matches!(
            self,
            Self::SpecificApp
                | Self::PackageId
                | Self::BundleId
                | Self::AppUserModelId
                | Self::DesktopEntryId
                | Self::ExecutableHash
                | Self::Publisher
                | Self::SpecificGame
                | Self::LauncherGameId
                | Self::StoreGameId
        )
    }

    pub(super) fn requires_unknown_state_proof(self) -> bool {
        matches!(
            self,
            Self::UnknownApp
                | Self::NewApp
                | Self::PortableApp
                | Self::UnknownGame
                | Self::NewGame
                | Self::LauncherGameCandidate
        )
    }

    pub(super) fn requires_category_proof(self) -> bool {
        matches!(
            self,
            Self::AppCategory
                | Self::RiskApp
                | Self::GameCategory
                | Self::LauncherGameCandidate
                | Self::MultiplayerGame
                | Self::UgcGame
                | Self::PurchaseCapableGame
                | Self::MatureGame
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyCompilerProofKind {
    IdentityProof,
    CategoryProof,
    UnknownStateProof,
    ScheduleProof,
    ApprovalProof,
    AuthorityProof,
    CapabilityProof,
    SessionSummaryProof,
    CurrentProcessProof,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyCompilerEvidenceState {
    Active,
    Stale,
    WrongDevice,
    WrongLocalUser,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyCompilerCapabilityState {
    Supported,
    ManualRequired,
    Unsupported,
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyCompilerAuthorityState {
    Proved,
    ManualRequired,
    Unavailable,
    Unproved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyCompilerRequestedAction {
    Observe,
    Warn,
    AskParent,
    TimeLimit,
    TerminateRunning,
    BlockLaunch,
    HideApp,
    SuspendApp,
    ShieldApp,
    ManualRequired,
}

impl AppGamePolicyCompilerRequestedAction {
    pub(super) fn is_hard_action(self) -> bool {
        matches!(
            self,
            Self::TerminateRunning
                | Self::BlockLaunch
                | Self::HideApp
                | Self::SuspendApp
                | Self::ShieldApp
        )
    }

    pub(super) fn requires_authority(self) -> bool {
        self.is_hard_action() || matches!(self, Self::TimeLimit)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyCompilerOutcomeState {
    DryRunReady,
    ManualRequired,
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyCompilerRejectionReason {
    None,
    MissingTargetReference,
    MissingEvidence,
    StaleEvidence,
    WrongDevice,
    WrongLocalUser,
    MissingIdentityProof,
    MissingUnknownStateProof,
    MissingCategoryProof,
    MissingScheduleProof,
    MissingApprovalProof,
    MissingCapabilityProof,
    MissingAuthorityProof,
    UnboundCapabilityEvidence,
    UnboundAuthorityEvidence,
    BlockLaunchManualRequired,
    HardActionManualRequired,
    RequestedManualRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyEnforcementHandoffState {
    Disabled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyCompilerTraceOwner {
    AppGameCore,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyCompilerTraceBoundary {
    PolicyTargetCompiler,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AppGamePolicyCompilerRedactionState {
    OpaqueReferencesOnly,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyCompilerTarget {
    pub target_kind: AppGamePolicyTargetKind,
    pub target_ref: Option<AppGamePolicyTargetRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyCompilerEvidence {
    pub evidence_ref: AppGamePolicyEvidenceRef,
    pub proof_kind: AppGamePolicyCompilerProofKind,
    pub evidence_state: AppGamePolicyCompilerEvidenceState,
    pub device_id: AppGamePolicyDeviceId,
    pub local_user_ref: AppGamePolicyLocalUserRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyCompilerCapabilityEvidence {
    pub capability_ref: AppGamePolicyCapabilityRef,
    pub capability_state: AppGamePolicyCompilerCapabilityState,
    pub evidence_refs: Vec<AppGamePolicyEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyCompilerAuthorityEvidence {
    pub authority_ref: AppGamePolicyAuthorityRef,
    pub authority_state: AppGamePolicyCompilerAuthorityState,
    pub evidence_refs: Vec<AppGamePolicyEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyCompileRequest {
    pub compile_request_id: AppGamePolicyCompileRequestId,
    pub rule_ref: AppGamePolicyRuleRef,
    pub device_id: AppGamePolicyDeviceId,
    pub local_user_ref: AppGamePolicyLocalUserRef,
    pub target: AppGamePolicyCompilerTarget,
    pub requested_action: AppGamePolicyCompilerRequestedAction,
    pub schedule_ref: Option<AppGamePolicyScheduleRef>,
    pub evidence: Vec<AppGamePolicyCompilerEvidence>,
    pub capability_refs: Vec<AppGamePolicyCompilerCapabilityEvidence>,
    pub authority_refs: Vec<AppGamePolicyCompilerAuthorityEvidence>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyCompilerContext {
    pub compiled_decision_id: AppGamePolicyCompiledDecisionId,
    pub audit_ref: AppGamePolicyAuditRef,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyCompiledDecision {
    pub compiled_decision_id: AppGamePolicyCompiledDecisionId,
    pub request: AppGamePolicyCompileRequest,
    pub outcome_state: AppGamePolicyCompilerOutcomeState,
    pub rejection_reason: AppGamePolicyCompilerRejectionReason,
    pub dry_run: bool,
    pub enforcement_handoff_state: AppGamePolicyEnforcementHandoffState,
    pub evidence_refs: Vec<AppGamePolicyEvidenceRef>,
    pub rule_refs: Vec<AppGamePolicyRuleRef>,
    pub capability_refs: Vec<AppGamePolicyCapabilityRef>,
    pub authority_refs: Vec<AppGamePolicyAuthorityRef>,
    pub audit_refs: Vec<AppGamePolicyAuditRef>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyCompilerTrace {
    pub run_id: AppGamePolicyAuditRef,
    pub correlation_id: AppGamePolicyCompileRequestId,
    pub owner: AppGamePolicyCompilerTraceOwner,
    pub boundary: AppGamePolicyCompilerTraceBoundary,
    pub result: AppGamePolicyCompilerOutcomeState,
    pub no_claim_reason: Option<AppGamePolicyCompilerRejectionReason>,
    pub redaction_state: AppGamePolicyCompilerRedactionState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppGamePolicyCompilation {
    pub decision: AppGamePolicyCompiledDecision,
    pub trace: AppGamePolicyCompilerTrace,
}
