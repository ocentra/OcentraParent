use serde::{Deserialize, Serialize};

pub mod family {
    use super::*;

    pub type ParentActorId = String;
    pub type ParentAccountId = String;
    pub type FamilyId = String;
    pub type ChildProfileId = String;
    pub type ParentDeviceId = String;
    pub type ParentEvidenceReferenceId = String;
    pub type ParentActionReferenceId = String;
    pub type ParentPolicyVersion = String;
    pub type ParentContractSchemaVersion = String;
    pub type ParentTimestamp = String;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum ParentActorRole {
        Parent,
        Caregiver,
        System,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum ParentDevicePlatform {
        Windows,
        Macos,
        Linux,
        Android,
        Ios,
        Web,
        Unknown,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ParentActorReference {
        pub actor_id: ParentActorId,
        pub role: ParentActorRole,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FamilyReference {
        pub family_id: FamilyId,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ChildProfileReference {
        pub child_profile_id: ChildProfileId,
        pub family_id: FamilyId,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ParentDeviceReference {
        pub device_id: ParentDeviceId,
        pub child_profile_id: Option<ChildProfileId>,
        pub label: String,
        pub platform: ParentDevicePlatform,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum ParentEvidenceReferenceKind {
        PolicyDecision,
        ActivityEvent,
        BrowserEvidence,
        NetworkEvidence,
        ScreenEvidence,
        AppGameEvidence,
        AuditEvent,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ParentEvidenceReference {
        pub evidence_reference_id: ParentEvidenceReferenceId,
        pub kind: ParentEvidenceReferenceKind,
        pub observed_at: ParentTimestamp,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ParentActionReference {
        pub action_reference_id: ParentActionReferenceId,
        pub actor: ParentActorReference,
        pub policy_version: ParentPolicyVersion,
        pub created_at: ParentTimestamp,
    }
}

pub mod policy {
    use super::family::*;
    use super::*;

    pub type PolicyTimestamp = String;
    pub type PolicyRuleId = String;
    pub type PolicyScheduleId = String;
    pub type PolicyTargetId = String;
    pub type PermissionRequestId = String;
    pub type PolicyDecisionId = String;
    pub type PolicyReasonCode = String;
    pub type PolicyLocalTime = String;
    pub type PolicyTimeZone = String;
    pub type LocalAiResultReferenceId = String;
    pub type PolicyScheduleExceptionId = String;
    pub type PolicyPreviewId = String;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyAction {
        Allow,
        Warn,
        Block,
        TimeLimit,
        AskParent,
        Unknown,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyTargetType {
        App,
        Process,
        Window,
        Domain,
        Site,
        Category,
        Video,
        Channel,
        ActivityType,
        Device,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyScheduleDay {
        Monday,
        Tuesday,
        Wednesday,
        Thursday,
        Friday,
        Saturday,
        Sunday,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyDecisionHandoffState {
        NotRequested,
        Disabled,
        Pending,
        HandedOff,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PermissionRequestState {
        Open,
        Approved,
        Denied,
        Expired,
        Cancelled,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyScheduleBoundaryState {
        WithinWindow,
        OutsideWindow,
        DstGap,
        DstOverlap,
        ClockSkew,
        ExceptionActive,
        Expired,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyScheduleDstTransition {
        SpringForward,
        FallBack,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyScheduleDstResolution {
        SkipForward,
        FirstOccurrence,
        SecondOccurrence,
        ManualRequired,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyScheduleClockSource {
        ChildDevice,
        TrustedService,
        ManualRequired,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyScheduleBudgetResetKind {
        Daily,
        Weekly,
        Monthly,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyScheduleBudgetCarryoverMode {
        DiscardUnused,
        CarryForward,
        CapCarryover,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyScheduleOfflineRecovery {
        ResumeRemaining,
        RecomputeFromJournal,
        ManualRequired,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyScheduleOfflineRecoveryState {
        NotNeeded,
        RecoveredFromDevice,
        RecomputedFromJournal,
        ManualRequired,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyPreviewOrigin {
        ParentPreview,
        AssistantPreview,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyPreviewConfirmationState {
        ConfirmationRequired,
        Confirmed,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PolicyPreviewBudgetBoundaryState {
        WithinBudget,
        BonusTimeActive,
        BonusTimeExpiring,
        ManualRequired,
        Expired,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PolicyTarget {
        pub target_id: PolicyTargetId,
        pub target_type: PolicyTargetType,
        pub target_value: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PolicyScheduleWindow {
        pub days: Vec<PolicyScheduleDay>,
        pub start_local_time: PolicyLocalTime,
        pub end_local_time: PolicyLocalTime,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PolicyScheduleBudgetReset {
        pub kind: PolicyScheduleBudgetResetKind,
        pub local_time: PolicyLocalTime,
        pub day: Option<PolicyScheduleDay>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PolicyScheduleBudgetCarryover {
        pub mode: PolicyScheduleBudgetCarryoverMode,
        pub max_minutes: Option<f64>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PolicyScheduleTimeBudget {
        pub budget_window_minutes: f64,
        pub reset: PolicyScheduleBudgetReset,
        pub carryover: PolicyScheduleBudgetCarryover,
        pub grace_period_minutes: f64,
        pub effective_from: PolicyTimestamp,
        pub effective_until: Option<PolicyTimestamp>,
        pub clock_source: PolicyScheduleClockSource,
        pub offline_recovery: PolicyScheduleOfflineRecovery,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PolicySchedule {
        pub schedule_id: PolicyScheduleId,
        pub time_zone: PolicyTimeZone,
        pub windows: Vec<PolicyScheduleWindow>,
        pub time_budget: PolicyScheduleTimeBudget,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PolicyRule {
        pub rule_id: PolicyRuleId,
        pub target: PolicyTarget,
        pub action: PolicyAction,
        pub schedule_id: Option<PolicyScheduleId>,
        pub priority: f64,
        pub reason_code: PolicyReasonCode,
        pub created_by: ParentActorReference,
        pub enabled: bool,
        pub effective_from: Option<PolicyTimestamp>,
        pub effective_until: Option<PolicyTimestamp>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FamilyPolicySet {
        pub schema_version: ParentContractSchemaVersion,
        pub family: FamilyReference,
        pub child_profiles: Vec<ChildProfileReference>,
        pub devices: Vec<ParentDeviceReference>,
        pub policy_version: ParentPolicyVersion,
        pub rules: Vec<PolicyRule>,
        pub schedules: Vec<PolicySchedule>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PermissionRequest {
        pub permission_request_id: PermissionRequestId,
        pub child_profile: ChildProfileReference,
        pub device: ParentDeviceReference,
        pub evidence_references: Vec<ParentEvidenceReference>,
        pub requested_action: PolicyAction,
        pub requested_target: PolicyTarget,
        pub state: PermissionRequestState,
        pub parent_action: Option<ParentActionReference>,
        pub expires_at: Option<PolicyTimestamp>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct PolicyDecision {
        pub schema_version: ParentContractSchemaVersion,
        pub decision_id: PolicyDecisionId,
        pub action: PolicyAction,
        pub reason_codes: Vec<PolicyReasonCode>,
        pub evidence_references: Vec<ParentEvidenceReference>,
        pub rule_ids: Vec<PolicyRuleId>,
        pub local_ai_result_id: Option<LocalAiResultReferenceId>,
        pub dry_run: bool,
        pub enforcement_handoff_state: PolicyDecisionHandoffState,
        pub expires_at: Option<PolicyTimestamp>,
    }
}

pub mod ai {
    use super::family::*;
    use super::policy::*;
    use super::*;

    pub type LocalAiEvaluationRequestId = String;
    pub type LocalAiResultId = String;
    pub type LocalAiPromptVersion = String;
    pub type LocalAiModelId = String;
    pub type LocalAiProviderId = String;
    pub type LocalAiModelReference = String;
    pub type LocalAiExplanationReference = String;
    pub type LocalAiMemoryReferenceId = String;
    pub type LocalAiGraphReferenceId = String;
    pub type LocalAiRuntimeReferenceId = String;
    pub type LocalAiTimestamp = String;
    pub type LocalAiDerivedIndexVersion = String;
    pub type LocalAiUnavailableReason = String;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum LocalAiContextKind {
        App,
        Process,
        Window,
        Url,
        Page,
        Video,
        Domain,
        Network,
        RecentActivity,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum LocalAiModelLoadState {
        Unavailable,
        Loading,
        Loaded,
        Degraded,
        Failed,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum LocalAiCapabilityFlag {
        Classification,
        Summarization,
        Embedding,
        SafetyDecision,
        ChatCompletion,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum LocalAiResourceClass {
        Cpu,
        Gpu,
        Npu,
        RemoteUnavailable,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum LocalAiDegradedState {
        None,
        ProviderUnavailable,
        ModelLoadFailed,
        Overloaded,
        InvalidOutput,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum LocalAiUnknownState {
        None,
        MissingEvidence,
        LowConfidence,
        ModelUnavailable,
        PolicyConflict,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum LocalAiEvidenceContextKind {
        Browser,
        AppGame,
        NetworkFlow,
        ScreenSummary,
        PolicyDecision,
        ParentAction,
        RecentActivity,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum LocalAiEvidenceCustody {
        LiveLocalChildAgent,
        LiveLanChildAgent,
        ChildDeviceJournal,
        ChildDeviceQueryStore,
        ParentDeviceCache,
        ParentOwnedExport,
        OcentraHostedNonActivity,
        Unavailable,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum LocalAiContextBuildState {
        Ready,
        Partial,
        Insufficient,
        Unavailable,
        Rejected,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LocalModelRuntimeStatus {
        pub runtime_reference_id: LocalAiRuntimeReferenceId,
        pub provider_id: LocalAiProviderId,
        pub model_id: LocalAiModelId,
        pub model_reference: LocalAiModelReference,
        pub load_state: LocalAiModelLoadState,
        pub capability_flags: Vec<LocalAiCapabilityFlag>,
        pub resource_class: LocalAiResourceClass,
        pub degraded_state: LocalAiDegradedState,
        pub last_checked_at: LocalAiTimestamp,
        pub unavailable_reason: Option<LocalAiUnavailableReason>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LocalAiMemoryReference {
        pub memory_reference_id: LocalAiMemoryReferenceId,
        pub source_evidence_references: Vec<ParentEvidenceReference>,
        pub source_policy_version: Option<ParentPolicyVersion>,
        pub generated_at: LocalAiTimestamp,
        pub confidence: f64,
        pub derived_index_version: LocalAiDerivedIndexVersion,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LocalAiGraphReference {
        pub graph_reference_id: LocalAiGraphReferenceId,
        pub source_evidence_references: Vec<ParentEvidenceReference>,
        pub source_policy_version: Option<ParentPolicyVersion>,
        pub generated_at: LocalAiTimestamp,
        pub confidence: f64,
        pub derived_index_version: LocalAiDerivedIndexVersion,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LocalAiEvidenceContextSourceRef {
        pub evidence_ref_id: String,
        pub evidence: ParentEvidenceReference,
        pub evidence_kind: LocalAiEvidenceContextKind,
        pub source_schema_version: ParentContractSchemaVersion,
        pub observed_at: LocalAiTimestamp,
        pub ingested_at: Option<LocalAiTimestamp>,
        pub fresh_until: Option<LocalAiTimestamp>,
        pub source_id: String,
        pub adapter_id: String,
        pub device: ParentDeviceReference,
        pub child_profile: ChildProfileReference,
        pub custody: LocalAiEvidenceCustody,
        pub source_evidence_references: Vec<ParentEvidenceReference>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LocalAiParentRuleContextRef {
        pub parent_rule_ref_id: String,
        pub policy_version: ParentPolicyVersion,
        pub family: FamilyReference,
        pub child_profile: ChildProfileReference,
        pub device: ParentDeviceReference,
        pub rule: PolicyRule,
        pub target_evidence_refs: Vec<String>,
        pub custody: LocalAiEvidenceCustody,
        pub updated_at: LocalAiTimestamp,
        pub expires_at: Option<LocalAiTimestamp>,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LocalAiEvidenceContext {
        pub schema_version: ParentContractSchemaVersion,
        pub context_id: String,
        pub request_id: LocalAiEvaluationRequestId,
        pub child_profile: ChildProfileReference,
        pub device: ParentDeviceReference,
        pub evidence_references: Vec<LocalAiEvidenceContextSourceRef>,
        pub parent_rule_references: Vec<PolicyRuleId>,
        pub parent_rule_context_references: Vec<LocalAiParentRuleContextRef>,
        pub memory_references: Vec<LocalAiMemoryReference>,
        pub graph_references: Vec<LocalAiGraphReference>,
        pub local_model_runtime_refs: Vec<LocalAiRuntimeReferenceId>,
        pub prompt_version: LocalAiPromptVersion,
        pub custody_labels: Vec<LocalAiEvidenceCustody>,
    }
}

pub mod notification {
    use super::family::*;
    use super::*;

    pub type NotificationLocalOutboxReadModelId = String;
    pub type NotificationLocalOutboxEntryId = String;
    pub type NotificationLocalOutboxReference = String;
    pub type NotificationLocalOutboxPayloadPreview = String;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum V3NotificationRuleReasonCode {
        PolicyViolation,
        ParentRequest,
        SuspiciousUnknown,
        DeviceOffline,
        SyncFailure,
        ProviderFailure,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum V3NotificationProviderChannel {
        Push,
        Email,
        Sms,
        Whatsapp,
        InApp,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum NotificationLocalOutboxState {
        QueuedLocal,
        DeferredQuietHours,
        RetryScheduled,
        DeadLettered,
        ReceiptRequired,
        ManualRequired,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum NotificationLocalOutboxSeverity {
        Info,
        Attention,
        Urgent,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum NotificationLocalOutboxDeliveryClaimState {
        LocalOutboxOnly,
        ProviderReceiptRequired,
        ManualRequired,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NotificationLocalOutboxMinimalAlertEnvelope {
        pub alert_ref: NotificationLocalOutboxReference,
        pub family: FamilyReference,
        pub device: ParentDeviceReference,
        pub parent_action: ParentActionReference,
        pub severity: NotificationLocalOutboxSeverity,
        pub reason_code: V3NotificationRuleReasonCode,
        pub provider_channel: V3NotificationProviderChannel,
        pub evidence_refs: Vec<ParentEvidenceReference>,
        pub policy_refs: Vec<NotificationLocalOutboxReference>,
        pub audit_refs: Vec<NotificationLocalOutboxReference>,
        pub payload_template_ref: NotificationLocalOutboxReference,
        pub provider_payload_preview: NotificationLocalOutboxPayloadPreview,
        pub sensitive_detail_minimized: bool,
        pub raw_child_evidence_included: bool,
        pub raw_url_or_title_included: bool,
        pub raw_message_text_included: bool,
        pub screenshot_or_report_included: bool,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NotificationLocalOutboxRecord {
        pub entry_id: NotificationLocalOutboxEntryId,
        pub state: NotificationLocalOutboxState,
        pub envelope: NotificationLocalOutboxMinimalAlertEnvelope,
        pub outbox_file_ref: NotificationLocalOutboxReference,
        pub local_data_path_ref: NotificationLocalOutboxReference,
        pub delivery_claim_state: NotificationLocalOutboxDeliveryClaimState,
        pub visible_after_at: Option<ParentTimestamp>,
        pub retry_attempt_count: u64,
        pub quiet_hours_ref: Option<NotificationLocalOutboxReference>,
        pub retry_policy_ref: Option<NotificationLocalOutboxReference>,
        pub dead_letter_ref: Option<NotificationLocalOutboxReference>,
        pub provider_receipt_ref: Option<NotificationLocalOutboxReference>,
        pub manual_proof_requirements: Vec<NotificationLocalOutboxReference>,
        pub manual_action_required: bool,
        pub provider_delivery_attempted: bool,
        pub provider_delivery_observed: bool,
        pub provider_receipt_ingested: bool,
        pub provider_credentials_stored: bool,
        pub cloud_routing_claimed: bool,
        pub parent_notification_ui_claimed: bool,
        pub sensitive_provider_metadata_stored: bool,
    }

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NotificationLocalOutboxAdapterProof {
        pub schema_version: String,
        pub contract_version: ParentContractSchemaVersion,
        pub read_model_id: NotificationLocalOutboxReadModelId,
        pub generated_at: ParentTimestamp,
        pub outbox_root_ref: NotificationLocalOutboxReference,
        pub records: Vec<NotificationLocalOutboxRecord>,
        pub non_claims: Vec<String>,
        pub provider_delivery_runtime_claimed: bool,
        pub provider_receipt_ingestion_claimed: bool,
        pub provider_credentials_claimed: bool,
        pub cloud_routing_claimed: bool,
        pub parent_notification_ui_claimed: bool,
    }
}

pub mod portal {
    use super::*;

    pub type PortalDevToolUrl = String;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PortalRoute {
        Overview,
        Assistant,
        Start,
        Activity,
        Browser,
        BrowserSettings,
        Policy,
        PolicyApps,
        PolicyGames,
        PolicyScreen,
        PolicyNetwork,
        PolicyTracking,
        PolicyRemoteScreen,
        RuleManagement,
        Schedules,
        Approvals,
        Enforcement,
        PrivacyDesign,
        Memory,
        MemorySettings,
        AiGuide,
        AiRuntime,
        ApiProviders,
        ReportsGuide,
        ScreenAnalysis,
        AppGameSessions,
        NetworkActivity,
        Devices,
        LanPairing,
        CapabilityStatus,
        Notifications,
        NotificationChannels,
        DriveConnections,
        ExportRetention,
        RemoteAccess,
        ReportCompiler,
        AuditHistory,
        Subscription,
        Entitlements,
        PlatformsInstall,
        InstallUpdates,
        Diagnostics,
        SettingsRules,
        #[serde(rename = "app-layout")]
        FrameTuner,
        Commands,
        Events,
        Logs,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "kebab-case")]
    pub enum PortalConnectionState {
        Disconnected,
        Connecting,
        Connected,
        Error,
    }
}
