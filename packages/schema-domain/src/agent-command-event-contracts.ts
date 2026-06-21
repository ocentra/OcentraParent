import { type Infer, Schema, withParser } from './effect';
import {
  AgentCorrelationIdSchema,
  AgentDeviceIdSchema,
  AgentEventIdSchema,
  AgentMessageIdSchema,
  AgentMessageTargetSchema,
  AgentPeerSchema,
  AgentProtocolSchemaVersion,
  AgentTimestampSchema,
  AgentWebSocketUrlSchema,
  SerializedAgentMessageSchema,
} from './event-primitives';
import {
  AgentLogSnapshotSchema,
  LogFieldsSchema,
  LogLevelSchema,
  type AgentLogSnapshot,
  type LogFields,
  type LogLevel,
} from './logging-contracts';
import {
  AgentLanBrowserRuntimeCommandNameLiteral,
  AgentLanBrowserRuntimeEventNameLiteral,
} from './lan-pairing-browser-runtime';

export const AgentCommandNameLiteral = {
  HealthCheck: 'agent.health.check',
  LogSnapshotGet: 'agent.log.snapshot.get',
  DevEcho: 'agent.dev.echo',
  WatchStatusGet: 'agent.watch.status.get',
  ActivityIngestStatusGet: 'agent.activity.ingest.status.get',
  ActivityRecentSummaryGet: 'agent.activity.recent.summary.get',
  ActivityMemoryGraphGet: 'agent.activity.memory-graph.get',
  ActivityReportDailyGenerate: 'agent.activity.report.daily.generate',
  ActivityReportWeeklyGenerate: 'agent.activity.report.weekly.generate',
  ActivityReportMonthlyGenerate: 'agent.activity.report.monthly.generate',
  ActivityReportSave: 'agent.activity.report.save',
  ActivityReportHistoryList: 'agent.activity.report.history.list',
  ActivityScreenReadModelGet: 'agent.activity.screen.read-model.get',
  ActivityAppUseReadModelGet: 'agent.activity.app-use.read-model.get',
  ActivityBrowserReadModelGet: 'agent.activity.browser.read-model.get',
  ActivityGamesReadModelGet: 'agent.activity.games.read-model.get',
  ActivityAppGameBoundaryReadModelGet: 'agent.activity.app-game.boundary.read-model.get',
  BrowserSocialDashboardReadModelGet: 'agent.browser.social-dashboard.read-model.get',
  ActivityNetworkReadModelGet: 'agent.activity.network.read-model.get',
  ActivityTrackingReadModelGet: 'agent.activity.tracking.read-model.get',
  BrowserInventoryReadModelGet: 'agent.browser.inventory.read-model.get',
  BrowserEvidenceRecentGet: 'agent.browser.evidence.recent.get',
  BrowserManagedBridgePoll: 'agent.browser.managed.bridge.poll',
  BrowserInterventionReadModelGet: 'agent.browser.intervention.read-model.get',
  BrowserRuntimeEventChainStreamGet: 'agent.browser.runtime.event-chain.stream.get',
  NetworkFlowReadModelGet: 'agent.network.flow.read-model.get',
  NetworkRuntimeEventChainStreamGet: 'agent.network.runtime.event-chain.stream.get',
  NetworkRemoteDeliveryStatusGet: 'agent.network.remote-delivery.status.get',
  NetworkLiveCaptureStatusGet: 'agent.network.live-capture.status.get',
  NetworkLinuxNftablesLabStatusGet: 'agent.network.linux-nftables-lab.status.get',
  NetworkWindowsFirewallLabStatusGet: 'agent.network.windows-firewall-lab.status.get',
  NetworkWindowsWfpGateStatusGet: 'agent.network.windows-wfp-gate.status.get',
  NetworkAndroidVpnServiceGateStatusGet: 'agent.network.android-vpn-service-gate.status.get',
  NetworkAppleNetworkExtensionGateStatusGet: 'agent.network.apple-network-extension-gate.status.get',
  LocalAiRuntimeStatusGet: 'agent.local-ai.runtime.status.get',
  LocalAiChatGenerate: 'agent.local-ai.chat.generate',
  ParentAssistantAnswerGenerate: 'agent.parent-assistant.answer.generate',
  PolicyPreviewReadModelGet: 'agent.policy.preview.read-model.get',
  PolicyRequestAssistantPreviewConfirm: 'agent.policy.request.assistant-preview.confirm',
  BrowserPolicyGet: 'agent.browser-policy.get',
  BrowserPolicyPreview: 'agent.browser-policy.preview',
  BrowserPolicyPatch: 'agent.browser-policy.patch',
  BrowserPolicyReplace: 'agent.browser-policy.replace',
  BrowserPolicyRollback: 'agent.browser-policy.rollback',
  ScreenSettingsGet: 'agent.screen-settings.get',
  ScreenSettingsReplace: 'agent.screen-settings.replace',
  EnforcementExecute: 'agent.enforcement.execute',
  EnforcementTimerRecover: 'agent.enforcement.timer.recover',
  EnforcementTimerExpire: 'agent.enforcement.timer.expire',
  EnforcementOverrideCancel: 'agent.enforcement.override.cancel',
  EnforcementProductControlSpineGet: 'agent.enforcement.product-control-spine.get',
  EnforcementPolicyDispatchGet: 'agent.enforcement.policy-dispatch.get',
  EnforcementBroadAdapterProofGet: 'agent.enforcement.broad-adapter-proof.get',
  ParentAssistantThreadList: 'agent.parent-assistant.thread.list',
  ParentAssistantThreadCreate: 'agent.parent-assistant.thread.create',
  ParentAssistantThreadOpen: 'agent.parent-assistant.thread.open',
  ParentAssistantThreadArchive: 'agent.parent-assistant.thread.archive',
  ParentAssistantMessageSend: 'agent.parent-assistant.message.send',
  ParentAssistantRunCancel: 'agent.parent-assistant.run.cancel',
  ParentAssistantQuickActionStart: 'agent.parent-assistant.quick-action.start',
  ParentAssistantActionPreview: 'agent.parent-assistant.action.preview',
  ParentAssistantActionConfirm: 'agent.parent-assistant.action.confirm',
  ParentAssistantProviderStatusGet: 'agent.parent-assistant.provider.status.get',
  LanPairingProofSubmit: 'agent.lan-pairing.proof.submit',
  LanPairingRouteSelect: 'agent.lan-pairing.route.select',
  LanPairingRouteRevoke: 'agent.lan-pairing.route.revoke',
  LanPairingStatusGet: 'agent.lan-pairing.status.get',
  LanPairingControllerLeaseRenew: 'agent.lan-pairing.controller-lease.renew',
  LanPairingControllerLeaseRelease: 'agent.lan-pairing.controller-lease.release',
  LanPairingControllerLeaseTakeover: 'agent.lan-pairing.controller-lease.takeover',
  LanAiProviderStatusGet: 'agent.lan-ai.provider.status.get',
  LanAiJobSubmit: 'agent.lan-ai.job.submit',
  ActivityAppGameTimerParentSurfaceParentPreferenceSetupRequest:
    'agent.activity.app-game.timer-parent-surface.parent-preference-setup.request',
  ActivityAppGameAdapterDispatchExecute: 'agent.activity.app-game.adapter-dispatch.execute',
  ActivityAppGameAdapterDispatchPreflightReadModelGet:
    'agent.activity.app-game.adapter-dispatch-preflight.read-model.get',
  ActivityAppGameAdapterDispatchResultReadModelGet:
    'agent.activity.app-game.adapter-dispatch-result.read-model.get',
  ActivityAppGameAdapterExecutionReadinessReadModelGet:
    'agent.activity.app-game.adapter-execution-readiness.read-model.get',
  ActivityAppGameChildRuntimeTransportReceiptReadModelGet:
    'agent.activity.app-game.child-runtime-transport-receipt.read-model.get',
  ActivityAppGameNotificationReadinessReadModelGet:
    'agent.activity.app-game.notification-readiness.read-model.get',
  ActivityAppGamePlatformProofStatusReadModelGet:
    'agent.activity.app-game.platform-proof-status.read-model.get',
  ActivityAppGamePolicyReadinessReadModelGet:
    'agent.activity.app-game.policy-readiness.read-model.get',
  ActivityAppGameTimerParentSurfaceReadModelGet:
    'agent.activity.app-game.timer-parent-surface.read-model.get',
  ActivityTrackingRetentionSettingsWrite: 'agent.activity.tracking.retention-settings.write',
  BrowserSocialAlertReportParentSurfaceReadModelGet:
    'agent.browser.social-alert-report.parent-surface.read-model.get',
  BrowserSocialAlertReportReadModelGet: 'agent.browser.social-alert-report.read-model.get',
  BrowserSocialAuditExplanationReadModelGet:
    'agent.browser.social-audit-explanation.read-model.get',
  BrowserSocialParentNotificationDeliveryReadModelGet:
    'agent.browser.social-parent-notification-delivery.read-model.get',
  BrowserSocialSourceCustodyMutationApply:
    'agent.browser.social-source-custody.mutation.apply',
  EnforcementSupportedAdapterRuntimeProofGet:
    'agent.enforcement.supported-adapter-runtime-proof.get',
} as const;

export const AgentEventNameLiteral = {
  ConnectionReady: 'agent.connection.ready',
  CommandRejected: 'agent.command.rejected',
  HealthReported: 'agent.health.reported',
  LogSnapshotReported: 'agent.log.snapshot.reported',
  DevEchoed: 'agent.dev.echoed',
  WatchStatusReported: 'agent.watch.status.reported',
  ActivityIngestStatusReported: 'agent.activity.ingest.status.reported',
  ActivityRecentSummaryReported: 'agent.activity.recent.summary.reported',
  ActivityMemoryGraphReported: 'agent.activity.memory-graph.reported',
  ActivityReportGenerated: 'agent.activity.report.generated',
  ActivityReportSaved: 'agent.activity.report.saved',
  ActivityReportHistoryReported: 'agent.activity.report.history.reported',
  ActivityScreenReadModelReported: 'agent.activity.screen.read-model.reported',
  ActivityAppUseReadModelReported: 'agent.activity.app-use.read-model.reported',
  ActivityBrowserReadModelReported: 'agent.activity.browser.read-model.reported',
  ActivityGamesReadModelReported: 'agent.activity.games.read-model.reported',
  ActivityNetworkReadModelReported: 'agent.activity.network.read-model.reported',
  ActivityTrackingReadModelReported: 'agent.activity.tracking.read-model.reported',
  BrowserInventoryReadModelReported: 'agent.browser.inventory.read-model.reported',
  BrowserEvidenceRecentReported: 'agent.browser.evidence.recent.reported',
  BrowserManagedStatusReported: 'agent.browser.managed.status.reported',
  BrowserInterventionReadModelReported: 'agent.browser.intervention.read-model.reported',
  NetworkFlowReadModelReported: 'agent.network.flow.read-model.reported',
  NetworkRemoteDeliveryStatusReported: 'agent.network.remote-delivery.status.reported',
  NetworkLiveCaptureStatusReported: 'agent.network.live-capture.status.reported',
  NetworkLinuxNftablesLabStatusReported:
    'agent.network.linux-nftables-lab.status.reported',
  NetworkWindowsWfpGateStatusReported:
    'agent.network.windows-wfp-gate.status.reported',
  NetworkAndroidVpnServiceGateStatusReported:
    'agent.network.android-vpn-service-gate.status.reported',
  NetworkAppleNetworkExtensionGateStatusReported:
    'agent.network.apple-network-extension-gate.status.reported',
  LocalAiRuntimeStatusReported: 'agent.local-ai.runtime.status.reported',
  LocalAiChatGenerationReported: 'agent.local-ai.chat.generation.reported',
  ParentAssistantAnswerReported: 'agent.parent-assistant.answer.reported',
  PolicyPreviewReadModelReported: 'agent.policy.preview.read-model.reported',
  PolicyRequestAssistantPreviewConfirmReported:
    'agent.policy.request.assistant-preview.confirm.reported',
  BrowserPolicyReported: 'agent.browser-policy.reported',
  BrowserPolicyPreviewed: 'agent.browser-policy.previewed',
  BrowserPolicyPatchAccepted: 'agent.browser-policy.patch.accepted',
  BrowserPolicyPatchRejected: 'agent.browser-policy.patch.rejected',
  BrowserPolicyReplaceAccepted: 'agent.browser-policy.replace.accepted',
  BrowserPolicyReplaceRejected: 'agent.browser-policy.replace.rejected',
  BrowserPolicyRollbackAccepted: 'agent.browser-policy.rollback.accepted',
  BrowserPolicyRollbackRejected: 'agent.browser-policy.rollback.rejected',
  ScreenSettingsReported: 'agent.screen-settings.reported',
  ScreenSettingsReplaceAccepted: 'agent.screen-settings.replace.accepted',
  ScreenSettingsReplaceRejected: 'agent.screen-settings.replace.rejected',
  EnforcementAuditReported: 'agent.enforcement.audit.reported',
  EnforcementTimerReported: 'agent.enforcement.timer.reported',
  EnforcementPolicyDispatchReported: 'agent.enforcement.policy-dispatch.reported',
  EnforcementBroadAdapterProofReported:
    'agent.enforcement.broad-adapter-proof.reported',
  ParentAssistantThreadUpdated: 'agent.parent-assistant.thread.updated',
  ParentAssistantMessageAccepted: 'agent.parent-assistant.message.accepted',
  ParentAssistantRunStarted: 'agent.parent-assistant.run.started',
  ParentAssistantMessageDelta: 'agent.parent-assistant.message.delta',
  ParentAssistantMessageCompleted: 'agent.parent-assistant.message.completed',
  ParentAssistantActionPreviewed: 'agent.parent-assistant.action.previewed',
  ParentAssistantActionConfirmed: 'agent.parent-assistant.action.confirmed',
  ParentAssistantProviderDegraded: 'agent.parent-assistant.provider.degraded',
  ParentAssistantErrorReported: 'agent.parent-assistant.error.reported',
  LanPairingStatusReported: 'agent.lan-pairing.status.reported',
  LanPairingAuditReported: 'agent.lan-pairing.audit.reported',
  LanAiJobReported: 'agent.lan-ai.job.reported',
  ActivityAppGameTimerParentSurfaceParentPreferenceSetupRequested:
    'agent.activity.app-game.timer-parent-surface.parent-preference-setup.requested',
  ActivityAppGameAdapterDispatchExecuted:
    'agent.activity.app-game.adapter-dispatch.executed',
  ActivityAppGameAdapterDispatchPreflightReadModelReported:
    'agent.activity.app-game.adapter-dispatch-preflight.read-model.reported',
  ActivityAppGameAdapterDispatchResultReadModelReported:
    'agent.activity.app-game.adapter-dispatch-result.read-model.reported',
  ActivityAppGameAdapterExecutionReadinessReadModelReported:
    'agent.activity.app-game.adapter-execution-readiness.read-model.reported',
  ActivityAppGameBoundaryReadModelReported:
    'agent.activity.app-game.boundary.read-model.reported',
  ActivityAppGameChildRuntimeTransportReceiptReadModelReported:
    'agent.activity.app-game.child-runtime-transport-receipt.read-model.reported',
  ActivityAppGameNotificationReadinessReadModelReported:
    'agent.activity.app-game.notification-readiness.read-model.reported',
  ActivityAppGamePlatformProofStatusReadModelReported:
    'agent.activity.app-game.platform-proof-status.read-model.reported',
  ActivityAppGamePolicyReadinessReadModelReported:
    'agent.activity.app-game.policy-readiness.read-model.reported',
  ActivityAppGameTimerParentSurfaceReadModelReported:
    'agent.activity.app-game.timer-parent-surface.read-model.reported',
  ActivityTrackingRetentionSettingsWriteReported:
    'agent.activity.tracking.retention-settings.write.reported',
  BrowserRuntimeEventChainStreamReported:
    'agent.browser.runtime.event-chain.stream.reported',
  BrowserSocialAlertReportParentSurfaceReadModelReported:
    'agent.browser.social-alert-report.parent-surface.read-model.reported',
  BrowserSocialAlertReportReadModelReported:
    'agent.browser.social-alert-report.read-model.reported',
  BrowserSocialAuditExplanationReadModelReported:
    'agent.browser.social-audit-explanation.read-model.reported',
  BrowserSocialDashboardReadModelReported:
    'agent.browser.social-dashboard.read-model.reported',
  BrowserSocialParentNotificationDeliveryReadModelReported:
    'agent.browser.social-parent-notification-delivery.read-model.reported',
  BrowserSocialSourceCustodyMutationApplied:
    'agent.browser.social-source-custody.mutation.applied',
  EnforcementProductControlSpineReported:
    'agent.enforcement.product-control-spine.reported',
  EnforcementSupportedAdapterRuntimeProofReported:
    'agent.enforcement.supported-adapter-runtime-proof.reported',
  NetworkRuntimeEventChainStreamReported:
    'agent.network.runtime.event-chain.stream.reported',
  NetworkWindowsFirewallLabStatusReported:
    'agent.network.windows-firewall-lab.status.reported',
} as const;

export const AgentCommandNameSchema = withParser(
  Schema.Literal(
    AgentCommandNameLiteral.HealthCheck,
    AgentCommandNameLiteral.LogSnapshotGet,
    AgentCommandNameLiteral.DevEcho,
    AgentCommandNameLiteral.WatchStatusGet,
    AgentCommandNameLiteral.ActivityIngestStatusGet,
    AgentCommandNameLiteral.ActivityRecentSummaryGet,
    AgentCommandNameLiteral.ActivityMemoryGraphGet,
    AgentCommandNameLiteral.ActivityReportDailyGenerate,
    AgentCommandNameLiteral.ActivityReportWeeklyGenerate,
    AgentCommandNameLiteral.ActivityReportMonthlyGenerate,
    AgentCommandNameLiteral.ActivityReportSave,
    AgentCommandNameLiteral.ActivityReportHistoryList,
    AgentCommandNameLiteral.ActivityScreenReadModelGet,
    AgentCommandNameLiteral.ActivityAppUseReadModelGet,
    AgentCommandNameLiteral.ActivityBrowserReadModelGet,
    AgentCommandNameLiteral.ActivityGamesReadModelGet,
    AgentCommandNameLiteral.ActivityAppGameBoundaryReadModelGet,
    AgentCommandNameLiteral.ActivityAppGamePolicyReadinessReadModelGet,
    AgentCommandNameLiteral.ActivityAppGameNotificationReadinessReadModelGet,
    AgentCommandNameLiteral.ActivityAppGameAdapterExecutionReadinessReadModelGet,
    AgentCommandNameLiteral.ActivityAppGamePlatformProofStatusReadModelGet,
    AgentCommandNameLiteral.ActivityAppGameChildRuntimeTransportReceiptReadModelGet,
    AgentCommandNameLiteral.ActivityAppGameAdapterDispatchPreflightReadModelGet,
    AgentCommandNameLiteral.ActivityAppGameAdapterDispatchResultReadModelGet,
    AgentCommandNameLiteral.ActivityAppGameAdapterDispatchExecute,
    AgentCommandNameLiteral.ActivityAppGameTimerParentSurfaceReadModelGet,
    AgentCommandNameLiteral.ActivityAppGameTimerParentSurfaceParentPreferenceSetupRequest,
    AgentCommandNameLiteral.BrowserSocialDashboardReadModelGet,
    AgentCommandNameLiteral.BrowserSocialAuditExplanationReadModelGet,
    AgentCommandNameLiteral.BrowserSocialAlertReportReadModelGet,
    AgentCommandNameLiteral.BrowserSocialAlertReportParentSurfaceReadModelGet,
    AgentCommandNameLiteral.BrowserSocialParentNotificationDeliveryReadModelGet,
    AgentCommandNameLiteral.BrowserSocialSourceCustodyMutationApply,
    AgentCommandNameLiteral.ActivityNetworkReadModelGet,
    AgentCommandNameLiteral.ActivityTrackingReadModelGet,
    AgentCommandNameLiteral.ActivityTrackingRetentionSettingsWrite,
    AgentCommandNameLiteral.BrowserInventoryReadModelGet,
    AgentCommandNameLiteral.BrowserEvidenceRecentGet,
    AgentCommandNameLiteral.BrowserManagedBridgePoll,
    AgentCommandNameLiteral.BrowserInterventionReadModelGet,
    AgentCommandNameLiteral.BrowserRuntimeEventChainStreamGet,
    AgentCommandNameLiteral.NetworkFlowReadModelGet,
    AgentCommandNameLiteral.NetworkRuntimeEventChainStreamGet,
    AgentCommandNameLiteral.NetworkRemoteDeliveryStatusGet,
    AgentCommandNameLiteral.NetworkLiveCaptureStatusGet,
    AgentCommandNameLiteral.NetworkLinuxNftablesLabStatusGet,
    AgentCommandNameLiteral.NetworkWindowsFirewallLabStatusGet,
    AgentCommandNameLiteral.NetworkWindowsWfpGateStatusGet,
    AgentCommandNameLiteral.NetworkAndroidVpnServiceGateStatusGet,
    AgentCommandNameLiteral.NetworkAppleNetworkExtensionGateStatusGet,
    AgentCommandNameLiteral.LocalAiRuntimeStatusGet,
    AgentCommandNameLiteral.LocalAiChatGenerate,
    AgentCommandNameLiteral.ParentAssistantAnswerGenerate,
    AgentCommandNameLiteral.PolicyPreviewReadModelGet,
    AgentCommandNameLiteral.PolicyRequestAssistantPreviewConfirm,
    AgentCommandNameLiteral.BrowserPolicyGet,
    AgentCommandNameLiteral.BrowserPolicyPreview,
    AgentCommandNameLiteral.BrowserPolicyPatch,
    AgentCommandNameLiteral.BrowserPolicyReplace,
    AgentCommandNameLiteral.BrowserPolicyRollback,
    AgentCommandNameLiteral.ScreenSettingsGet,
    AgentCommandNameLiteral.ScreenSettingsReplace,
    AgentCommandNameLiteral.EnforcementExecute,
    AgentCommandNameLiteral.EnforcementTimerRecover,
    AgentCommandNameLiteral.EnforcementTimerExpire,
    AgentCommandNameLiteral.EnforcementOverrideCancel,
    AgentCommandNameLiteral.EnforcementProductControlSpineGet,
    AgentCommandNameLiteral.EnforcementPolicyDispatchGet,
    AgentCommandNameLiteral.EnforcementBroadAdapterProofGet,
    AgentCommandNameLiteral.EnforcementSupportedAdapterRuntimeProofGet,
    AgentCommandNameLiteral.ParentAssistantThreadList,
    AgentCommandNameLiteral.ParentAssistantThreadCreate,
    AgentCommandNameLiteral.ParentAssistantThreadOpen,
    AgentCommandNameLiteral.ParentAssistantThreadArchive,
    AgentCommandNameLiteral.ParentAssistantMessageSend,
    AgentCommandNameLiteral.ParentAssistantRunCancel,
    AgentCommandNameLiteral.ParentAssistantQuickActionStart,
    AgentCommandNameLiteral.ParentAssistantActionPreview,
    AgentCommandNameLiteral.ParentAssistantActionConfirm,
    AgentCommandNameLiteral.ParentAssistantProviderStatusGet,
    AgentCommandNameLiteral.LanPairingProofSubmit,
    AgentCommandNameLiteral.LanPairingRouteSelect,
    AgentCommandNameLiteral.LanPairingRouteRevoke,
    AgentCommandNameLiteral.LanPairingStatusGet,
    AgentLanBrowserRuntimeCommandNameLiteral.BrowserDiscoveryScan,
    AgentLanBrowserRuntimeCommandNameLiteral.AddDeviceRequest,
    AgentCommandNameLiteral.LanPairingControllerLeaseRenew,
    AgentCommandNameLiteral.LanPairingControllerLeaseRelease,
    AgentCommandNameLiteral.LanPairingControllerLeaseTakeover,
    AgentCommandNameLiteral.LanAiProviderStatusGet,
    AgentCommandNameLiteral.LanAiJobSubmit
  )
);

export const AgentEventNameSchema = withParser(
  Schema.Literal(
    AgentEventNameLiteral.ConnectionReady,
    AgentEventNameLiteral.CommandRejected,
    AgentEventNameLiteral.HealthReported,
    AgentEventNameLiteral.LogSnapshotReported,
    AgentEventNameLiteral.DevEchoed,
    AgentEventNameLiteral.WatchStatusReported,
    AgentEventNameLiteral.ActivityIngestStatusReported,
    AgentEventNameLiteral.ActivityRecentSummaryReported,
    AgentEventNameLiteral.ActivityMemoryGraphReported,
    AgentEventNameLiteral.ActivityReportGenerated,
    AgentEventNameLiteral.ActivityReportSaved,
    AgentEventNameLiteral.ActivityReportHistoryReported,
    AgentEventNameLiteral.ActivityScreenReadModelReported,
    AgentEventNameLiteral.ActivityAppUseReadModelReported,
    AgentEventNameLiteral.ActivityBrowserReadModelReported,
    AgentEventNameLiteral.ActivityGamesReadModelReported,
    AgentEventNameLiteral.ActivityAppGameBoundaryReadModelReported,
    AgentEventNameLiteral.ActivityAppGamePolicyReadinessReadModelReported,
    AgentEventNameLiteral.ActivityAppGameNotificationReadinessReadModelReported,
    AgentEventNameLiteral.ActivityAppGameAdapterExecutionReadinessReadModelReported,
    AgentEventNameLiteral.ActivityAppGamePlatformProofStatusReadModelReported,
    AgentEventNameLiteral.ActivityAppGameChildRuntimeTransportReceiptReadModelReported,
    AgentEventNameLiteral.ActivityAppGameAdapterDispatchPreflightReadModelReported,
    AgentEventNameLiteral.ActivityAppGameAdapterDispatchResultReadModelReported,
    AgentEventNameLiteral.ActivityAppGameAdapterDispatchExecuted,
    AgentEventNameLiteral.ActivityAppGameTimerParentSurfaceReadModelReported,
    AgentEventNameLiteral.ActivityAppGameTimerParentSurfaceParentPreferenceSetupRequested,
    AgentEventNameLiteral.BrowserSocialDashboardReadModelReported,
    AgentEventNameLiteral.BrowserSocialAuditExplanationReadModelReported,
    AgentEventNameLiteral.BrowserSocialAlertReportReadModelReported,
    AgentEventNameLiteral.BrowserSocialAlertReportParentSurfaceReadModelReported,
    AgentEventNameLiteral.BrowserSocialParentNotificationDeliveryReadModelReported,
    AgentEventNameLiteral.BrowserSocialSourceCustodyMutationApplied,
    AgentEventNameLiteral.ActivityNetworkReadModelReported,
    AgentEventNameLiteral.ActivityTrackingReadModelReported,
    AgentEventNameLiteral.ActivityTrackingRetentionSettingsWriteReported,
    AgentEventNameLiteral.BrowserInventoryReadModelReported,
    AgentEventNameLiteral.BrowserEvidenceRecentReported,
    AgentEventNameLiteral.BrowserManagedStatusReported,
    AgentEventNameLiteral.BrowserInterventionReadModelReported,
    AgentEventNameLiteral.BrowserRuntimeEventChainStreamReported,
    AgentEventNameLiteral.NetworkFlowReadModelReported,
    AgentEventNameLiteral.NetworkRuntimeEventChainStreamReported,
    AgentEventNameLiteral.NetworkRemoteDeliveryStatusReported,
    AgentEventNameLiteral.NetworkLiveCaptureStatusReported,
    AgentEventNameLiteral.NetworkLinuxNftablesLabStatusReported,
    AgentEventNameLiteral.NetworkWindowsFirewallLabStatusReported,
    AgentEventNameLiteral.NetworkWindowsWfpGateStatusReported,
    AgentEventNameLiteral.NetworkAndroidVpnServiceGateStatusReported,
    AgentEventNameLiteral.NetworkAppleNetworkExtensionGateStatusReported,
    AgentEventNameLiteral.LocalAiRuntimeStatusReported,
    AgentEventNameLiteral.LocalAiChatGenerationReported,
    AgentEventNameLiteral.ParentAssistantAnswerReported,
    AgentEventNameLiteral.PolicyPreviewReadModelReported,
    AgentEventNameLiteral.PolicyRequestAssistantPreviewConfirmReported,
    AgentEventNameLiteral.BrowserPolicyReported,
    AgentEventNameLiteral.BrowserPolicyPreviewed,
    AgentEventNameLiteral.BrowserPolicyPatchAccepted,
    AgentEventNameLiteral.BrowserPolicyPatchRejected,
    AgentEventNameLiteral.BrowserPolicyReplaceAccepted,
    AgentEventNameLiteral.BrowserPolicyReplaceRejected,
    AgentEventNameLiteral.BrowserPolicyRollbackAccepted,
    AgentEventNameLiteral.BrowserPolicyRollbackRejected,
    AgentEventNameLiteral.ScreenSettingsReported,
    AgentEventNameLiteral.ScreenSettingsReplaceAccepted,
    AgentEventNameLiteral.ScreenSettingsReplaceRejected,
    AgentEventNameLiteral.EnforcementAuditReported,
    AgentEventNameLiteral.EnforcementTimerReported,
    AgentEventNameLiteral.EnforcementProductControlSpineReported,
    AgentEventNameLiteral.EnforcementPolicyDispatchReported,
    AgentEventNameLiteral.EnforcementBroadAdapterProofReported,
    AgentEventNameLiteral.EnforcementSupportedAdapterRuntimeProofReported,
    AgentEventNameLiteral.ParentAssistantThreadUpdated,
    AgentEventNameLiteral.ParentAssistantMessageAccepted,
    AgentEventNameLiteral.ParentAssistantRunStarted,
    AgentEventNameLiteral.ParentAssistantMessageDelta,
    AgentEventNameLiteral.ParentAssistantMessageCompleted,
    AgentEventNameLiteral.ParentAssistantActionPreviewed,
    AgentEventNameLiteral.ParentAssistantActionConfirmed,
    AgentEventNameLiteral.ParentAssistantProviderDegraded,
    AgentEventNameLiteral.ParentAssistantErrorReported,
    AgentEventNameLiteral.LanPairingStatusReported,
    AgentLanBrowserRuntimeEventNameLiteral.BrowserDiscoveryReported,
    AgentLanBrowserRuntimeEventNameLiteral.AddDeviceReported,
    AgentEventNameLiteral.LanPairingAuditReported,
    AgentEventNameLiteral.LanAiJobReported
  )
);

export const AgentCommandEnvelopeSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    messageId: AgentMessageIdSchema,
    sentAt: AgentTimestampSchema,
    source: AgentPeerSchema,
    target: AgentMessageTargetSchema,
    command: AgentCommandNameSchema,
    payload: LogFieldsSchema,
  })
);

export const AgentEventEnvelopeSchema = withParser(
  Schema.Struct({
    schemaVersion: Schema.Literal(AgentProtocolSchemaVersion),
    eventId: AgentEventIdSchema,
    correlationId: AgentCorrelationIdSchema,
    sentAt: AgentTimestampSchema,
    source: AgentPeerSchema,
    target: AgentPeerSchema,
    event: AgentEventNameSchema,
    severity: LogLevelSchema,
    payload: LogFieldsSchema,
    snapshot: Schema.Union(AgentLogSnapshotSchema, Schema.Null),
  })
);

export type AgentCommandName = Infer<typeof AgentCommandNameSchema>;
export type AgentEventName = Infer<typeof AgentEventNameSchema>;
export type AgentCommandEnvelope = Infer<typeof AgentCommandEnvelopeSchema>;
export type AgentEventEnvelope = Infer<typeof AgentEventEnvelopeSchema>;
export type AgentProtocolLogFields = LogFields;
export type AgentProtocolLogLevel = LogLevel;
export type AgentProtocolSnapshot = AgentLogSnapshot;

function parseLiteralRecord<T extends Record<string, string>, TParsed extends string>(
  literalMap: T,
  parse: (value: string) => TParsed
): { [K in keyof T]: TParsed } {
  return Object.fromEntries(
    Object.entries(literalMap).map(([key, value]) => [key, parse(value)])
  ) as { [K in keyof T]: TParsed };
}

export const decodeAgentDeviceId = Schema.decodeUnknownSync(AgentDeviceIdSchema);
export const decodeAgentMessageId = Schema.decodeUnknownSync(AgentMessageIdSchema);
export const decodeAgentTimestamp = Schema.decodeUnknownSync(AgentTimestampSchema);
export const decodeAgentWebSocketUrl = Schema.decodeUnknownSync(AgentWebSocketUrlSchema);
export const decodeSerializedAgentMessage = Schema.decodeUnknownSync(SerializedAgentMessageSchema);

export function isAgentProtocolLogText(value: unknown): value is string {
  return typeof value === 'string';
}

const agentCommandLiteralMap = (() => {
  const {
    ActivityAppGameTimerParentSurfaceParentPreferenceSetupRequest:
      activityAppGameTimerParentPreferenceSetupRequest,
    ...base
  } = AgentCommandNameLiteral;

  return {
    ...base,
    ActivityAppGameTimerParentPreferenceSetupRequest:
      activityAppGameTimerParentPreferenceSetupRequest,
    LanPairingBrowserDiscoveryScan:
      AgentLanBrowserRuntimeCommandNameLiteral.BrowserDiscoveryScan,
    LanPairingAddDeviceRequest: AgentLanBrowserRuntimeCommandNameLiteral.AddDeviceRequest,
  } as const;
})();

export const AgentCommand = parseLiteralRecord(agentCommandLiteralMap, (value) =>
  AgentCommandNameSchema.parse(value)
);

export const AgentLanPairingSupportedWebSocketCommand = {
  ProofSubmit: AgentCommand.LanPairingProofSubmit,
  RouteSelect: AgentCommand.LanPairingRouteSelect,
  RouteRevoke: AgentCommand.LanPairingRouteRevoke,
  StatusGet: AgentCommand.LanPairingStatusGet,
  BrowserDiscoveryScan: AgentCommand.LanPairingBrowserDiscoveryScan,
  AddDeviceRequest: AgentCommand.LanPairingAddDeviceRequest,
  ControllerLeaseRenew: AgentCommand.LanPairingControllerLeaseRenew,
  ControllerLeaseRelease: AgentCommand.LanPairingControllerLeaseRelease,
  ControllerLeaseTakeover: AgentCommand.LanPairingControllerLeaseTakeover,
  LanAiProviderStatusGet: AgentCommand.LanAiProviderStatusGet,
  LanAiJobSubmit: AgentCommand.LanAiJobSubmit,
} as const;

const agentEventLiteralMap = (() => {
  const {
    ActivityAppGameTimerParentSurfaceParentPreferenceSetupRequested:
      activityAppGameTimerParentPreferenceSetupRequested,
    ...base
  } = AgentEventNameLiteral;

  return {
    ...base,
    ActivityAppGameTimerParentPreferenceSetupRequested:
      activityAppGameTimerParentPreferenceSetupRequested,
    LanPairingBrowserDiscoveryReported:
      AgentLanBrowserRuntimeEventNameLiteral.BrowserDiscoveryReported,
    LanPairingAddDeviceReported: AgentLanBrowserRuntimeEventNameLiteral.AddDeviceReported,
  } as const;
})();

export const AgentEvent = parseLiteralRecord(agentEventLiteralMap, (value) =>
  AgentEventNameSchema.parse(value)
);
