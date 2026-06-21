use ocentra_eventing::envelope::{DomainEvent, EventContract};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, EventCustody, EventType, IdempotencyKey, RuntimeRole, SchemaVersion,
};
use serde::{Deserialize, Serialize};

use crate::{
    parent_controller_events::{
        ParentActionReceivedEvent, ParentChildCommandForwardRequestedEvent,
        ParentChildCommandForwardedEvent, ParentCommandValidatedEvent,
        ParentReadModelProjectedEvent,
    },
    AgentLogSnapshot, ChildCapabilityStateUpdatedEvent, ChildCommandAcceptedEvent,
    ChildCommandReceivedEvent, ChildRuntimeHealthUpdatedEvent, LogFields,
    PolicyAssistantConfirmationState, PolicyRequestOrigin, PolicyRequestStatus,
};

pub mod parent_child_runtime_input;

#[derive(Clone, Debug)]
pub struct ParentChildRuntimeReport {
    pub publish_reports: Vec<ocentra_eventing::bus::reports::PublishReport>,
    pub stored_events: Vec<ocentra_eventing::envelope::StoredEventEnvelope>,
    pub dead_letters: Vec<ocentra_eventing::bus::reports::DeadLetter>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentPeerRole {
    #[serde(rename = "portal")]
    Portal,
    #[serde(rename = "agent-service")]
    AgentService,
    #[serde(rename = "cloud-relay")]
    CloudRelay,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPeer {
    pub peer_id: String,
    pub role: AgentPeerRole,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRoute {
    #[serde(rename = "localhost")]
    Localhost,
    #[serde(rename = "local-network")]
    LocalNetwork,
    #[serde(rename = "cloud-relay")]
    CloudRelay,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentPairingState {
    #[serde(rename = "unauthenticated")]
    Unauthenticated,
    #[serde(rename = "unpaired")]
    Unpaired,
    #[serde(rename = "pairing")]
    Pairing,
    #[serde(rename = "paired")]
    Paired,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentMessageTarget {
    pub device_id: String,
    pub platform: String,
    pub route: AgentRoute,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentPairingProof {
    pub pairing_id: String,
    pub device_id: String,
    pub parent_peer_id: String,
    pub issued_at: String,
    pub expires_at: String,
    pub token_hash: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentRouteSecurityPolicy {
    pub route: AgentRoute,
    pub requires_pairing: bool,
    pub allows_anonymous_control: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentCommandName {
    #[serde(rename = "agent.health.check")]
    AgentHealthCheck,
    #[serde(rename = "agent.log.snapshot.get")]
    AgentLogSnapshotGet,
    #[serde(rename = "agent.dev.echo")]
    AgentDevEcho,
    #[serde(rename = "agent.watch.status.get")]
    AgentWatchStatusGet,
    #[serde(rename = "agent.activity.ingest.status.get")]
    AgentActivityIngestStatusGet,
    #[serde(rename = "agent.activity.recent.summary.get")]
    AgentActivityRecentSummaryGet,
    #[serde(rename = "agent.activity.memory-graph.get")]
    AgentActivityMemoryGraphGet,
    #[serde(rename = "agent.activity.report.daily.generate")]
    AgentActivityReportDailyGenerate,
    #[serde(rename = "agent.activity.report.weekly.generate")]
    AgentActivityReportWeeklyGenerate,
    #[serde(rename = "agent.activity.report.monthly.generate")]
    AgentActivityReportMonthlyGenerate,
    #[serde(rename = "agent.activity.report.save")]
    AgentActivityReportSave,
    #[serde(rename = "agent.activity.report.history.list")]
    AgentActivityReportHistoryList,
    #[serde(rename = "agent.activity.screen.read-model.get")]
    AgentActivityScreenReadModelGet,
    #[serde(rename = "agent.activity.app-use.read-model.get")]
    AgentActivityAppUseReadModelGet,
    #[serde(rename = "agent.activity.browser.read-model.get")]
    AgentActivityBrowserReadModelGet,
    #[serde(rename = "agent.activity.games.read-model.get")]
    AgentActivityGamesReadModelGet,
    #[serde(rename = "agent.activity.app-game.boundary.read-model.get")]
    AgentActivityAppGameBoundaryReadModelGet,
    #[serde(rename = "agent.activity.app-game.policy-readiness.read-model.get")]
    AgentActivityAppGamePolicyReadinessReadModelGet,
    #[serde(rename = "agent.activity.app-game.notification-readiness.read-model.get")]
    AgentActivityAppGameNotificationReadinessReadModelGet,
    #[serde(rename = "agent.activity.app-game.adapter-execution-readiness.read-model.get")]
    AgentActivityAppGameAdapterExecutionReadinessReadModelGet,
    #[serde(rename = "agent.activity.app-game.platform-proof-status.read-model.get")]
    AgentActivityAppGamePlatformProofStatusReadModelGet,
    #[serde(rename = "agent.activity.app-game.child-runtime-transport-receipt.read-model.get")]
    AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet,
    #[serde(rename = "agent.activity.app-game.adapter-dispatch-preflight.read-model.get")]
    AgentActivityAppGameAdapterDispatchPreflightReadModelGet,
    #[serde(rename = "agent.activity.app-game.adapter-dispatch-result.read-model.get")]
    AgentActivityAppGameAdapterDispatchResultReadModelGet,
    #[serde(rename = "agent.activity.app-game.adapter-dispatch.execute")]
    AgentActivityAppGameAdapterDispatchExecute,
    #[serde(rename = "agent.activity.app-game.timer-parent-surface.read-model.get")]
    AgentActivityAppGameTimerParentSurfaceReadModelGet,
    #[serde(
        rename = "agent.activity.app-game.timer-parent-surface.parent-preference-setup.request"
    )]
    AgentActivityAppGameTimerParentPreferenceSetupRequest,
    #[serde(rename = "agent.browser.social-dashboard.read-model.get")]
    AgentBrowserSocialDashboardReadModelGet,
    #[serde(rename = "agent.browser.social-audit-explanation.read-model.get")]
    AgentBrowserSocialAuditExplanationReadModelGet,
    #[serde(rename = "agent.browser.social-alert-report.read-model.get")]
    AgentBrowserSocialAlertReportReadModelGet,
    #[serde(rename = "agent.browser.social-alert-report.parent-surface.read-model.get")]
    AgentBrowserSocialAlertReportParentSurfaceReadModelGet,
    #[serde(rename = "agent.browser.social-parent-notification-delivery.read-model.get")]
    AgentBrowserSocialParentNotificationDeliveryReadModelGet,
    #[serde(rename = "agent.browser.social-source-custody.mutation.apply")]
    AgentBrowserSocialSourceCustodyMutationApply,
    #[serde(rename = "agent.activity.network.read-model.get")]
    AgentActivityNetworkReadModelGet,
    #[serde(rename = "agent.activity.tracking.read-model.get")]
    AgentActivityTrackingReadModelGet,
    #[serde(rename = "agent.activity.tracking.retention-settings.write")]
    AgentActivityTrackingRetentionSettingsWrite,
    #[serde(rename = "agent.browser.inventory.read-model.get")]
    AgentBrowserInventoryReadModelGet,
    #[serde(rename = "agent.browser.evidence.recent.get")]
    AgentBrowserEvidenceRecentGet,
    #[serde(rename = "agent.browser.managed.bridge.poll")]
    AgentBrowserManagedBridgePoll,
    #[serde(rename = "agent.browser.intervention.read-model.get")]
    AgentBrowserInterventionReadModelGet,
    #[serde(rename = "agent.browser.runtime.event-chain.stream.get")]
    AgentBrowserRuntimeEventChainStreamGet,
    #[serde(rename = "agent.network.flow.read-model.get")]
    AgentNetworkFlowReadModelGet,
    #[serde(rename = "agent.network.runtime.event-chain.stream.get")]
    AgentNetworkRuntimeEventChainStreamGet,
    #[serde(rename = "agent.network.remote-delivery.status.get")]
    AgentNetworkRemoteDeliveryStatusGet,
    #[serde(rename = "agent.network.live-capture.status.get")]
    AgentNetworkLiveCaptureStatusGet,
    #[serde(rename = "agent.network.linux-nftables-lab.status.get")]
    AgentNetworkLinuxNftablesLabStatusGet,
    #[serde(rename = "agent.network.windows-firewall-lab.status.get")]
    AgentNetworkWindowsFirewallLabStatusGet,
    #[serde(rename = "agent.network.windows-wfp-gate.status.get")]
    AgentNetworkWindowsWfpGateStatusGet,
    #[serde(rename = "agent.network.android-vpn-service-gate.status.get")]
    AgentNetworkAndroidVpnServiceGateStatusGet,
    #[serde(rename = "agent.network.apple-network-extension-gate.status.get")]
    AgentNetworkAppleNetworkExtensionGateStatusGet,
    #[serde(rename = "agent.local-ai.runtime.status.get")]
    AgentLocalAiRuntimeStatusGet,
    #[serde(rename = "agent.local-ai.chat.generate")]
    AgentLocalAiChatGenerate,
    #[serde(rename = "agent.parent-assistant.answer.generate")]
    AgentParentAssistantAnswerGenerate,
    #[serde(rename = "agent.policy.preview.read-model.get")]
    AgentPolicyPreviewReadModelGet,
    #[serde(rename = "agent.policy.request.assistant-preview.confirm")]
    AgentPolicyRequestAssistantPreviewConfirm,
    #[serde(rename = "agent.browser-policy.get")]
    AgentBrowserPolicyGet,
    #[serde(rename = "agent.browser-policy.preview")]
    AgentBrowserPolicyPreview,
    #[serde(rename = "agent.browser-policy.patch")]
    AgentBrowserPolicyPatch,
    #[serde(rename = "agent.browser-policy.replace")]
    AgentBrowserPolicyReplace,
    #[serde(rename = "agent.browser-policy.rollback")]
    AgentBrowserPolicyRollback,
    #[serde(rename = "agent.screen-settings.get")]
    AgentScreenSettingsGet,
    #[serde(rename = "agent.screen-settings.replace")]
    AgentScreenSettingsReplace,
    #[serde(rename = "agent.enforcement.execute")]
    AgentEnforcementExecute,
    #[serde(rename = "agent.enforcement.timer.recover")]
    AgentEnforcementTimerRecover,
    #[serde(rename = "agent.enforcement.timer.expire")]
    AgentEnforcementTimerExpire,
    #[serde(rename = "agent.enforcement.override.cancel")]
    AgentEnforcementOverrideCancel,
    #[serde(rename = "agent.enforcement.product-control-spine.get")]
    AgentEnforcementProductControlSpineGet,
    #[serde(rename = "agent.enforcement.policy-dispatch.get")]
    AgentEnforcementPolicyDispatchGet,
    #[serde(rename = "agent.enforcement.broad-adapter-proof.get")]
    AgentEnforcementBroadAdapterProofGet,
    #[serde(rename = "agent.enforcement.supported-adapter-runtime-proof.get")]
    AgentEnforcementSupportedAdapterRuntimeProofGet,
    #[serde(rename = "agent.parent-assistant.thread.list")]
    AgentParentAssistantThreadList,
    #[serde(rename = "agent.parent-assistant.thread.create")]
    AgentParentAssistantThreadCreate,
    #[serde(rename = "agent.parent-assistant.thread.open")]
    AgentParentAssistantThreadOpen,
    #[serde(rename = "agent.parent-assistant.thread.archive")]
    AgentParentAssistantThreadArchive,
    #[serde(rename = "agent.parent-assistant.message.send")]
    AgentParentAssistantMessageSend,
    #[serde(rename = "agent.parent-assistant.run.cancel")]
    AgentParentAssistantRunCancel,
    #[serde(rename = "agent.parent-assistant.quick-action.start")]
    AgentParentAssistantQuickActionStart,
    #[serde(rename = "agent.parent-assistant.action.preview")]
    AgentParentAssistantActionPreview,
    #[serde(rename = "agent.parent-assistant.action.confirm")]
    AgentParentAssistantActionConfirm,
    #[serde(rename = "agent.parent-assistant.provider.status.get")]
    AgentParentAssistantProviderStatusGet,
    #[serde(rename = "agent.lan-pairing.proof.submit")]
    AgentLanPairingProofSubmit,
    #[serde(rename = "agent.lan-pairing.route.select")]
    AgentLanPairingRouteSelect,
    #[serde(rename = "agent.lan-pairing.route.revoke")]
    AgentLanPairingRouteRevoke,
    #[serde(rename = "agent.lan-pairing.status.get")]
    AgentLanPairingStatusGet,
    #[serde(rename = "agent.lan-pairing.browser-discovery.scan")]
    AgentLanPairingBrowserDiscoveryScan,
    #[serde(rename = "agent.lan-pairing.add-device.request")]
    AgentLanPairingAddDeviceRequest,
    #[serde(rename = "agent.lan-pairing.controller-lease.renew")]
    AgentLanPairingControllerLeaseRenew,
    #[serde(rename = "agent.lan-pairing.controller-lease.release")]
    AgentLanPairingControllerLeaseRelease,
    #[serde(rename = "agent.lan-pairing.controller-lease.takeover")]
    AgentLanPairingControllerLeaseTakeover,
    #[serde(rename = "agent.lan-ai.provider.status.get")]
    AgentLanAiProviderStatusGet,
    #[serde(rename = "agent.lan-ai.job.submit")]
    AgentLanAiJobSubmit,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentEventName {
    #[serde(rename = "agent.connection.ready")]
    AgentConnectionReady,
    #[serde(rename = "agent.command.rejected")]
    AgentCommandRejected,
    #[serde(rename = "agent.health.reported")]
    AgentHealthReported,
    #[serde(rename = "agent.log.snapshot.reported")]
    AgentLogSnapshotReported,
    #[serde(rename = "agent.dev.echoed")]
    AgentDevEchoed,
    #[serde(rename = "agent.watch.status.reported")]
    AgentWatchStatusReported,
    #[serde(rename = "agent.activity.ingest.status.reported")]
    AgentActivityIngestStatusReported,
    #[serde(rename = "agent.activity.recent.summary.reported")]
    AgentActivityRecentSummaryReported,
    #[serde(rename = "agent.activity.memory-graph.reported")]
    AgentActivityMemoryGraphReported,
    #[serde(rename = "agent.activity.report.generated")]
    AgentActivityReportGenerated,
    #[serde(rename = "agent.activity.report.saved")]
    AgentActivityReportSaved,
    #[serde(rename = "agent.activity.report.history.reported")]
    AgentActivityReportHistoryReported,
    #[serde(rename = "agent.activity.screen.read-model.reported")]
    AgentActivityScreenReadModelReported,
    #[serde(rename = "agent.activity.app-use.read-model.reported")]
    AgentActivityAppUseReadModelReported,
    #[serde(rename = "agent.activity.browser.read-model.reported")]
    AgentActivityBrowserReadModelReported,
    #[serde(rename = "agent.activity.games.read-model.reported")]
    AgentActivityGamesReadModelReported,
    #[serde(rename = "agent.activity.app-game.boundary.read-model.reported")]
    AgentActivityAppGameBoundaryReadModelReported,
    #[serde(rename = "agent.activity.app-game.policy-readiness.read-model.reported")]
    AgentActivityAppGamePolicyReadinessReadModelReported,
    #[serde(rename = "agent.activity.app-game.notification-readiness.read-model.reported")]
    AgentActivityAppGameNotificationReadinessReadModelReported,
    #[serde(rename = "agent.activity.app-game.adapter-execution-readiness.read-model.reported")]
    AgentActivityAppGameAdapterExecutionReadinessReadModelReported,
    #[serde(rename = "agent.activity.app-game.platform-proof-status.read-model.reported")]
    AgentActivityAppGamePlatformProofStatusReadModelReported,
    #[serde(
        rename = "agent.activity.app-game.child-runtime-transport-receipt.read-model.reported"
    )]
    AgentActivityAppGameChildRuntimeTransportReceiptReadModelReported,
    #[serde(rename = "agent.activity.app-game.adapter-dispatch-preflight.read-model.reported")]
    AgentActivityAppGameAdapterDispatchPreflightReadModelReported,
    #[serde(rename = "agent.activity.app-game.adapter-dispatch-result.read-model.reported")]
    AgentActivityAppGameAdapterDispatchResultReadModelReported,
    #[serde(rename = "agent.activity.app-game.adapter-dispatch.executed")]
    AgentActivityAppGameAdapterDispatchExecuted,
    #[serde(rename = "agent.activity.app-game.timer-parent-surface.read-model.reported")]
    AgentActivityAppGameTimerParentSurfaceReadModelReported,
    #[serde(
        rename = "agent.activity.app-game.timer-parent-surface.parent-preference-setup.requested"
    )]
    AgentActivityAppGameTimerParentPreferenceSetupRequested,
    #[serde(rename = "agent.browser.social-dashboard.read-model.reported")]
    AgentBrowserSocialDashboardReadModelReported,
    #[serde(rename = "agent.browser.social-audit-explanation.read-model.reported")]
    AgentBrowserSocialAuditExplanationReadModelReported,
    #[serde(rename = "agent.browser.social-alert-report.read-model.reported")]
    AgentBrowserSocialAlertReportReadModelReported,
    #[serde(rename = "agent.browser.social-alert-report.parent-surface.read-model.reported")]
    AgentBrowserSocialAlertReportParentSurfaceReadModelReported,
    #[serde(rename = "agent.browser.social-parent-notification-delivery.read-model.reported")]
    AgentBrowserSocialParentNotificationDeliveryReadModelReported,
    #[serde(rename = "agent.browser.social-source-custody.mutation.applied")]
    AgentBrowserSocialSourceCustodyMutationApplied,
    #[serde(rename = "agent.activity.network.read-model.reported")]
    AgentActivityNetworkReadModelReported,
    #[serde(rename = "agent.activity.tracking.read-model.reported")]
    AgentActivityTrackingReadModelReported,
    #[serde(rename = "agent.activity.tracking.retention-settings.write.reported")]
    AgentActivityTrackingRetentionSettingsWriteReported,
    #[serde(rename = "agent.browser.inventory.read-model.reported")]
    AgentBrowserInventoryReadModelReported,
    #[serde(rename = "agent.browser.evidence.recent.reported")]
    AgentBrowserEvidenceRecentReported,
    #[serde(rename = "agent.browser.managed.status.reported")]
    AgentBrowserManagedStatusReported,
    #[serde(rename = "agent.browser.intervention.read-model.reported")]
    AgentBrowserInterventionReadModelReported,
    #[serde(rename = "agent.browser.runtime.event-chain.stream.reported")]
    AgentBrowserRuntimeEventChainStreamReported,
    #[serde(rename = "agent.network.flow.read-model.reported")]
    AgentNetworkFlowReadModelReported,
    #[serde(rename = "agent.network.runtime.event-chain.stream.reported")]
    AgentNetworkRuntimeEventChainStreamReported,
    #[serde(rename = "agent.network.remote-delivery.status.reported")]
    AgentNetworkRemoteDeliveryStatusReported,
    #[serde(rename = "agent.network.live-capture.status.reported")]
    AgentNetworkLiveCaptureStatusReported,
    #[serde(rename = "agent.network.linux-nftables-lab.status.reported")]
    AgentNetworkLinuxNftablesLabStatusReported,
    #[serde(rename = "agent.network.windows-firewall-lab.status.reported")]
    AgentNetworkWindowsFirewallLabStatusReported,
    #[serde(rename = "agent.network.windows-wfp-gate.status.reported")]
    AgentNetworkWindowsWfpGateStatusReported,
    #[serde(rename = "agent.network.android-vpn-service-gate.status.reported")]
    AgentNetworkAndroidVpnServiceGateStatusReported,
    #[serde(rename = "agent.network.apple-network-extension-gate.status.reported")]
    AgentNetworkAppleNetworkExtensionGateStatusReported,
    #[serde(rename = "agent.local-ai.runtime.status.reported")]
    AgentLocalAiRuntimeStatusReported,
    #[serde(rename = "agent.local-ai.chat.generation.reported")]
    AgentLocalAiChatGenerationReported,
    #[serde(rename = "agent.parent-assistant.answer.reported")]
    AgentParentAssistantAnswerReported,
    #[serde(rename = "agent.policy.preview.read-model.reported")]
    AgentPolicyPreviewReadModelReported,
    #[serde(rename = "agent.policy.request.assistant-preview.confirm.reported")]
    AgentPolicyRequestAssistantPreviewConfirmReported,
    #[serde(rename = "agent.browser-policy.reported")]
    AgentBrowserPolicyReported,
    #[serde(rename = "agent.browser-policy.previewed")]
    AgentBrowserPolicyPreviewed,
    #[serde(rename = "agent.browser-policy.patch.accepted")]
    AgentBrowserPolicyPatchAccepted,
    #[serde(rename = "agent.browser-policy.patch.rejected")]
    AgentBrowserPolicyPatchRejected,
    #[serde(rename = "agent.browser-policy.replace.accepted")]
    AgentBrowserPolicyReplaceAccepted,
    #[serde(rename = "agent.browser-policy.replace.rejected")]
    AgentBrowserPolicyReplaceRejected,
    #[serde(rename = "agent.browser-policy.rollback.accepted")]
    AgentBrowserPolicyRollbackAccepted,
    #[serde(rename = "agent.browser-policy.rollback.rejected")]
    AgentBrowserPolicyRollbackRejected,
    #[serde(rename = "agent.screen-settings.reported")]
    AgentScreenSettingsReported,
    #[serde(rename = "agent.screen-settings.replace.accepted")]
    AgentScreenSettingsReplaceAccepted,
    #[serde(rename = "agent.screen-settings.replace.rejected")]
    AgentScreenSettingsReplaceRejected,
    #[serde(rename = "agent.enforcement.audit.reported")]
    AgentEnforcementAuditReported,
    #[serde(rename = "agent.enforcement.timer.reported")]
    AgentEnforcementTimerReported,
    #[serde(rename = "agent.enforcement.product-control-spine.reported")]
    AgentEnforcementProductControlSpineReported,
    #[serde(rename = "agent.enforcement.policy-dispatch.reported")]
    AgentEnforcementPolicyDispatchReported,
    #[serde(rename = "agent.enforcement.broad-adapter-proof.reported")]
    AgentEnforcementBroadAdapterProofReported,
    #[serde(rename = "agent.enforcement.supported-adapter-runtime-proof.reported")]
    AgentEnforcementSupportedAdapterRuntimeProofReported,
    #[serde(rename = "agent.parent-assistant.thread.updated")]
    AgentParentAssistantThreadUpdated,
    #[serde(rename = "agent.parent-assistant.message.accepted")]
    AgentParentAssistantMessageAccepted,
    #[serde(rename = "agent.parent-assistant.run.started")]
    AgentParentAssistantRunStarted,
    #[serde(rename = "agent.parent-assistant.message.delta")]
    AgentParentAssistantMessageDelta,
    #[serde(rename = "agent.parent-assistant.message.completed")]
    AgentParentAssistantMessageCompleted,
    #[serde(rename = "agent.parent-assistant.action.previewed")]
    AgentParentAssistantActionPreviewed,
    #[serde(rename = "agent.parent-assistant.action.confirmed")]
    AgentParentAssistantActionConfirmed,
    #[serde(rename = "agent.parent-assistant.provider.degraded")]
    AgentParentAssistantProviderDegraded,
    #[serde(rename = "agent.parent-assistant.error.reported")]
    AgentParentAssistantErrorReported,
    #[serde(rename = "agent.lan-pairing.status.reported")]
    AgentLanPairingStatusReported,
    #[serde(rename = "agent.lan-pairing.browser-discovery.reported")]
    AgentLanPairingBrowserDiscoveryReported,
    #[serde(rename = "agent.lan-pairing.add-device.reported")]
    AgentLanPairingAddDeviceReported,
    #[serde(rename = "agent.lan-pairing.audit.reported")]
    AgentLanPairingAuditReported,
    #[serde(rename = "agent.lan-ai.job.reported")]
    AgentLanAiJobReported,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestAssistantPreviewConfirmRequestKind {
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "bonus-time")]
    BonusTime,
    #[serde(rename = "temporary-override")]
    TemporaryOverride,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestAssistantPreviewConfirmTargetKind {
    #[serde(rename = "child-profile")]
    ChildProfile,
    #[serde(rename = "device")]
    Device,
    #[serde(rename = "app")]
    App,
    #[serde(rename = "site")]
    Site,
    #[serde(rename = "category")]
    Category,
    #[serde(rename = "resource")]
    Resource,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestAssistantPreviewConfirmAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "time-limit")]
    TimeLimit,
    #[serde(rename = "block")]
    Block,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestAssistantPreviewConfirmActorRole {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "co-parent")]
    CoParent,
    #[serde(rename = "observer")]
    Observer,
    #[serde(rename = "child")]
    Child,
    #[serde(rename = "support")]
    Support,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestAssistantPreviewConfirmActorState {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "revoked")]
    Revoked,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestAssistantPreviewConfirmResultState {
    #[serde(rename = "confirmed")]
    Confirmed,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyRequestAssistantPreviewConfirmClaimState {
    #[serde(rename = "claimed")]
    Claimed,
    #[serde(rename = "unclaimed")]
    Unclaimed,
}

pub type PolicyPreviewRequestStatusValue = PolicyRequestStatus;
pub type PolicyPreviewAssistantConfirmationStateValue = PolicyAssistantConfirmationState;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PolicyRequestAssistantPreviewConfirmRequest {
    pub schema_version: u16,
    pub command_id: String,
    pub request_id: String,
    pub submission_key: String,
    pub household_id: String,
    pub child_profile_id: String,
    pub device_id: Option<String>,
    pub source_document_id: String,
    pub policy_version: u64,
    pub request_kind: PolicyRequestAssistantPreviewConfirmRequestKind,
    pub target_kind: PolicyRequestAssistantPreviewConfirmTargetKind,
    pub target_reference_id: String,
    pub requested_action: PolicyRequestAssistantPreviewConfirmAction,
    pub rule_id: Option<String>,
    pub requested_bonus_minutes: Option<u16>,
    pub requested_at: String,
    pub expires_at: String,
    pub origin: PolicyRequestOrigin,
    pub assistant_preview_id: String,
    pub assistant_confirmation_state: PolicyPreviewAssistantConfirmationStateValue,
    pub request_status: PolicyPreviewRequestStatusValue,
    pub audit_reference_ids: Vec<String>,
    pub confirmation_actor_id: String,
    pub confirmation_actor_role: PolicyRequestAssistantPreviewConfirmActorRole,
    pub confirmation_actor_state: PolicyRequestAssistantPreviewConfirmActorState,
    pub confirmation_audit_reference_id: String,
    pub confirmed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PolicyRequestAssistantPreviewConfirmResult {
    pub schema_version: u16,
    pub command_id: String,
    pub request_id: String,
    pub assistant_preview_id: Option<String>,
    pub result_state: PolicyRequestAssistantPreviewConfirmResultState,
    pub policy_request_status: PolicyPreviewRequestStatusValue,
    pub policy_assistant_confirmation_state: PolicyPreviewAssistantConfirmationStateValue,
    pub policy_audit_reference_id: Option<String>,
    pub confirmed_at: Option<String>,
    pub rejection_reason: Option<String>,
    pub command_transport_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    pub service_validation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    pub activity_store_mutation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    pub upstream_writer_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    pub read_model_projection_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    pub portal_writable_ui_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    pub child_device_delivery_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    pub provider_delivery_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    pub platform_enforcement_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    pub product_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCommandEnvelope {
    pub schema_version: u16,
    pub message_id: String,
    pub sent_at: String,
    pub source: AgentPeer,
    pub target: AgentMessageTarget,
    pub command: AgentCommandName,
    pub payload: LogFields,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentEventEnvelope {
    pub schema_version: u16,
    pub event_id: String,
    pub correlation_id: String,
    pub sent_at: String,
    pub source: AgentPeer,
    pub target: AgentPeer,
    pub event: AgentEventName,
    pub severity: crate::LogLevel,
    pub payload: LogFields,
    pub snapshot: Option<AgentLogSnapshot>,
}

const PARENT_CHILD_RUNTIME_PHASES: [ParentChildRuntimePhase; 9] = [
    ParentChildRuntimePhase::ParentActionReceived,
    ParentChildRuntimePhase::ParentCommandValidated,
    ParentChildRuntimePhase::ParentChildCommandForwardRequested,
    ParentChildRuntimePhase::ParentChildCommandForwarded,
    ParentChildRuntimePhase::ChildCommandReceived,
    ParentChildRuntimePhase::ChildCommandAccepted,
    ParentChildRuntimePhase::ChildCapabilityStateUpdated,
    ParentChildRuntimePhase::ChildRuntimeHealthUpdated,
    ParentChildRuntimePhase::ParentReadModelProjected,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParentChildRuntimePhase {
    ParentActionReceived,
    ParentCommandValidated,
    ParentChildCommandForwardRequested,
    ParentChildCommandForwarded,
    ChildCommandReceived,
    ChildCommandAccepted,
    ChildCapabilityStateUpdated,
    ChildRuntimeHealthUpdated,
    ParentReadModelProjected,
}

impl ParentChildRuntimePhase {
    pub fn ordered_chain() -> &'static [Self] {
        &PARENT_CHILD_RUNTIME_PHASES
    }

    pub fn event_type(self) -> &'static str {
        match self {
            Self::ParentActionReceived => {
                crate::constants::parent_controller::EVENT_PARENT_ACTION_RECEIVED
            }
            Self::ParentCommandValidated => {
                crate::constants::parent_controller::EVENT_COMMAND_VALIDATED
            }
            Self::ParentChildCommandForwardRequested => {
                crate::constants::parent_controller::EVENT_CHILD_COMMAND_FORWARD_REQUESTED
            }
            Self::ParentChildCommandForwarded => {
                crate::constants::parent_controller::EVENT_CHILD_COMMAND_FORWARDED
            }
            Self::ChildCommandReceived => crate::constants::child_agent::EVENT_COMMAND_RECEIVED,
            Self::ChildCommandAccepted => crate::constants::child_agent::EVENT_COMMAND_ACCEPTED,
            Self::ChildCapabilityStateUpdated => {
                crate::constants::child_agent::EVENT_CAPABILITY_STATE_UPDATED
            }
            Self::ChildRuntimeHealthUpdated => {
                crate::constants::child_agent::EVENT_RUNTIME_HEALTH_UPDATED
            }
            Self::ParentReadModelProjected => {
                crate::constants::parent_controller::EVENT_READ_MODEL_PROJECTED
            }
        }
    }

    pub fn schema_version(self) -> u16 {
        if self.is_child_agent_phase() {
            crate::constants::child_agent::EVENT_SCHEMA_VERSION
        } else {
            crate::constants::parent_controller::EVENT_SCHEMA_VERSION
        }
    }

    pub fn subscriber_id(self) -> &'static str {
        match self {
            Self::ParentActionReceived => {
                crate::constants::parent_controller::SUBSCRIBER_PARENT_ACTION_VALIDATOR
            }
            Self::ParentCommandValidated => {
                crate::constants::parent_controller::SUBSCRIBER_PARENT_COMMAND_VALIDATOR
            }
            Self::ParentChildCommandForwardRequested | Self::ParentChildCommandForwarded => {
                crate::constants::parent_controller::SUBSCRIBER_PARENT_CHILD_TRANSPORT
            }
            Self::ChildCommandReceived => {
                crate::constants::child_agent::SUBSCRIBER_CHILD_COMMAND_RECEIVER
            }
            Self::ChildCommandAccepted => {
                crate::constants::child_agent::SUBSCRIBER_CHILD_COMMAND_DECIDER
            }
            Self::ChildCapabilityStateUpdated => {
                crate::constants::child_agent::SUBSCRIBER_CHILD_CAPABILITY_PROJECTOR
            }
            Self::ChildRuntimeHealthUpdated => {
                crate::constants::child_agent::SUBSCRIBER_CHILD_HEALTH_PROJECTOR
            }
            Self::ParentReadModelProjected => {
                crate::constants::parent_controller::SUBSCRIBER_PARENT_READ_MODEL_PROJECTOR
            }
        }
    }

    pub fn target_handler(self) -> &'static str {
        match self {
            Self::ParentActionReceived => {
                crate::constants::parent_controller::TARGET_PARENT_ACTION_VALIDATOR
            }
            Self::ParentCommandValidated => {
                crate::constants::parent_controller::TARGET_PARENT_COMMAND_VALIDATOR
            }
            Self::ParentChildCommandForwardRequested | Self::ParentChildCommandForwarded => {
                crate::constants::parent_controller::TARGET_PARENT_CHILD_TRANSPORT
            }
            Self::ChildCommandReceived => {
                crate::constants::child_agent::TARGET_CHILD_COMMAND_RECEIVER
            }
            Self::ChildCommandAccepted => {
                crate::constants::child_agent::TARGET_CHILD_COMMAND_DECIDER
            }
            Self::ChildCapabilityStateUpdated => {
                crate::constants::child_agent::TARGET_CHILD_CAPABILITY_PROJECTOR
            }
            Self::ChildRuntimeHealthUpdated => {
                crate::constants::child_agent::TARGET_CHILD_HEALTH_PROJECTOR
            }
            Self::ParentReadModelProjected => {
                crate::constants::parent_controller::TARGET_PARENT_READ_MODEL_PROJECTOR
            }
        }
    }

    pub fn runtime_role(self) -> RuntimeRole {
        let value = if self.is_child_agent_phase() {
            crate::constants::eventing_source::ROLE_AGENT
        } else if self == Self::ParentReadModelProjected {
            crate::constants::eventing_source::ROLE_READ_MODEL
        } else {
            crate::constants::eventing_source::ROLE_CONTROLLER
        };
        match RuntimeRole::parse(value) {
            Ok(role) => role,
            Err(_) => std::process::abort(),
        }
    }

    pub fn custody(self) -> EventCustody {
        let value = if self.is_child_agent_phase() {
            crate::constants::eventing_source::CUSTODY_LOCAL_JOURNAL
        } else {
            crate::constants::eventing_source::CUSTODY_COORDINATOR_CACHE
        };
        match EventCustody::parse(value) {
            Ok(custody) => custody,
            Err(_) => std::process::abort(),
        }
    }

    pub fn is_child_agent_phase(self) -> bool {
        matches!(
            self,
            Self::ChildCommandReceived
                | Self::ChildCommandAccepted
                | Self::ChildCapabilityStateUpdated
                | Self::ChildRuntimeHealthUpdated
        )
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ParentChildRuntimeEventPayload {
    ParentActionReceived(ParentActionReceivedEvent),
    ParentCommandValidated(ParentCommandValidatedEvent),
    ParentChildCommandForwardRequested(ParentChildCommandForwardRequestedEvent),
    ParentChildCommandForwarded(ParentChildCommandForwardedEvent),
    ChildCommandReceived(ChildCommandReceivedEvent),
    ChildCommandAccepted(ChildCommandAcceptedEvent),
    ChildCapabilityStateUpdated(ChildCapabilityStateUpdatedEvent),
    ChildRuntimeHealthUpdated(ChildRuntimeHealthUpdatedEvent),
    ParentReadModelProjected(ParentReadModelProjectedEvent),
}

impl ParentChildRuntimeEventPayload {
    pub fn phase(&self) -> ParentChildRuntimePhase {
        match self {
            Self::ParentActionReceived(_) => ParentChildRuntimePhase::ParentActionReceived,
            Self::ParentCommandValidated(_) => ParentChildRuntimePhase::ParentCommandValidated,
            Self::ParentChildCommandForwardRequested(_) => {
                ParentChildRuntimePhase::ParentChildCommandForwardRequested
            }
            Self::ParentChildCommandForwarded(_) => {
                ParentChildRuntimePhase::ParentChildCommandForwarded
            }
            Self::ChildCommandReceived(_) => ParentChildRuntimePhase::ChildCommandReceived,
            Self::ChildCommandAccepted(_) => ParentChildRuntimePhase::ChildCommandAccepted,
            Self::ChildCapabilityStateUpdated(_) => {
                ParentChildRuntimePhase::ChildCapabilityStateUpdated
            }
            Self::ChildRuntimeHealthUpdated(_) => {
                ParentChildRuntimePhase::ChildRuntimeHealthUpdated
            }
            Self::ParentReadModelProjected(_) => ParentChildRuntimePhase::ParentReadModelProjected,
        }
    }

    pub fn event_ref(&self) -> &str {
        match self {
            Self::ParentActionReceived(event) => &event.parent_action_event_ref,
            Self::ParentCommandValidated(event) => &event.command_validated_event_ref,
            Self::ParentChildCommandForwardRequested(event) => &event.forward_requested_event_ref,
            Self::ParentChildCommandForwarded(event) => &event.forwarded_event_ref,
            Self::ChildCommandReceived(event) => &event.command_received_event_ref,
            Self::ChildCommandAccepted(event) => &event.command_accepted_event_ref,
            Self::ChildCapabilityStateUpdated(event) => &event.capability_state_event_ref,
            Self::ChildRuntimeHealthUpdated(event) => &event.runtime_health_event_ref,
            Self::ParentReadModelProjected(event) => &event.read_model_projected_event_ref,
        }
    }
}

impl DomainEvent for ParentChildRuntimeEventPayload {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(self.phase().event_type())?,
            SchemaVersion::new(self.phase().schema_version())?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(parent_child_aggregate_key(self.event_ref()))
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(parent_child_idempotency_key(self.event_ref()))
    }
}

fn parent_child_aggregate_key(event_ref: &str) -> String {
    let mut value =
        String::from(crate::constants::parent_controller::AGGREGATE_PARENT_CHILD_RUNTIME_PREFIX);
    value.push_str(event_ref);
    value
}

fn parent_child_idempotency_key(event_ref: &str) -> String {
    let mut value =
        String::from(crate::constants::parent_controller::IDEMPOTENCY_PARENT_CHILD_RUNTIME_PREFIX);
    value.push_str(event_ref);
    value
}

#[cfg(test)]
mod policy_request_assistant_preview_confirm_tests {
    use super::*;

    #[test]
    fn policy_request_assistant_preview_confirm_command_and_event_names_serialize_to_contract_shape(
    ) {
        let command =
            serde_json::to_value(AgentCommandName::AgentPolicyRequestAssistantPreviewConfirm)
                .expect("policy request confirm command serializes");
        let event =
            serde_json::to_value(AgentEventName::AgentPolicyRequestAssistantPreviewConfirmReported)
                .expect("policy request confirm event serializes");

        assert_eq!(command, "agent.policy.request.assistant-preview.confirm");
        assert_eq!(
            event,
            "agent.policy.request.assistant-preview.confirm.reported"
        );
    }

    #[test]
    fn policy_request_assistant_preview_confirm_result_serializes_without_product_overclaims() {
        let result = PolicyRequestAssistantPreviewConfirmResult {
            schema_version: 1,
            command_id: "policy-request-assistant-preview-confirm-command".to_string(),
            request_id: "policy-request-1".to_string(),
            assistant_preview_id: Some("assistant-preview-1".to_string()),
            result_state: PolicyRequestAssistantPreviewConfirmResultState::Confirmed,
            policy_request_status: PolicyPreviewRequestStatusValue::PendingParentReview,
            policy_assistant_confirmation_state:
                PolicyPreviewAssistantConfirmationStateValue::ParentConfirmed,
            policy_audit_reference_id: Some("audit.policy-request.confirmed".to_string()),
            confirmed_at: Some("2026-06-18T00:05:00Z".to_string()),
            rejection_reason: None,
            command_transport_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
            service_validation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
            activity_store_mutation_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
            upstream_writer_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
            read_model_projection_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
            portal_writable_ui_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
            child_device_delivery_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
            provider_delivery_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
            platform_enforcement_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
            product_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        };

        let serialized =
            serde_json::to_value(result).expect("policy request confirm result serializes");

        assert_eq!(serialized["resultState"], "confirmed");
        assert_eq!(serialized["policyRequestStatus"], "pending-parent-review");
        assert_eq!(
            serialized["policyAssistantConfirmationState"],
            "parent-confirmed"
        );
        assert_eq!(serialized["commandTransportClaimState"], "claimed");
        assert_eq!(serialized["serviceValidationClaimState"], "claimed");
        assert_eq!(serialized["productClaimState"], "unclaimed");
    }
}
