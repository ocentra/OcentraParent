use serde::{Deserialize, Serialize};

use crate::{AgentLogSnapshot, LogFields};

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
    #[serde(rename = "agent.activity.network.read-model.get")]
    AgentActivityNetworkReadModelGet,
    #[serde(rename = "agent.activity.tracking.read-model.get")]
    AgentActivityTrackingReadModelGet,
    #[serde(rename = "agent.browser.inventory.read-model.get")]
    AgentBrowserInventoryReadModelGet,
    #[serde(rename = "agent.browser.evidence.recent.get")]
    AgentBrowserEvidenceRecentGet,
    #[serde(rename = "agent.browser.managed.bridge.poll")]
    AgentBrowserManagedBridgePoll,
    #[serde(rename = "agent.browser.intervention.read-model.get")]
    AgentBrowserInterventionReadModelGet,
    #[serde(rename = "agent.network.flow.read-model.get")]
    AgentNetworkFlowReadModelGet,
    #[serde(rename = "agent.local-ai.runtime.status.get")]
    AgentLocalAiRuntimeStatusGet,
    #[serde(rename = "agent.local-ai.chat.generate")]
    AgentLocalAiChatGenerate,
    #[serde(rename = "agent.parent-assistant.answer.generate")]
    AgentParentAssistantAnswerGenerate,
    #[serde(rename = "agent.policy.preview.read-model.get")]
    AgentPolicyPreviewReadModelGet,
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
    #[serde(rename = "agent.activity.network.read-model.reported")]
    AgentActivityNetworkReadModelReported,
    #[serde(rename = "agent.activity.tracking.read-model.reported")]
    AgentActivityTrackingReadModelReported,
    #[serde(rename = "agent.browser.inventory.read-model.reported")]
    AgentBrowserInventoryReadModelReported,
    #[serde(rename = "agent.browser.evidence.recent.reported")]
    AgentBrowserEvidenceRecentReported,
    #[serde(rename = "agent.browser.managed.status.reported")]
    AgentBrowserManagedStatusReported,
    #[serde(rename = "agent.browser.intervention.read-model.reported")]
    AgentBrowserInterventionReadModelReported,
    #[serde(rename = "agent.network.flow.read-model.reported")]
    AgentNetworkFlowReadModelReported,
    #[serde(rename = "agent.local-ai.runtime.status.reported")]
    AgentLocalAiRuntimeStatusReported,
    #[serde(rename = "agent.local-ai.chat.generation.reported")]
    AgentLocalAiChatGenerationReported,
    #[serde(rename = "agent.parent-assistant.answer.reported")]
    AgentParentAssistantAnswerReported,
    #[serde(rename = "agent.policy.preview.read-model.reported")]
    AgentPolicyPreviewReadModelReported,
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
