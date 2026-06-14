import { expect, it } from 'vitest';
import {
  AgentCommandEnvelopeSchema,
  AgentCommand,
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentLanPairingSupportedWebSocketCommand,
  AgentPairingProofSchema,
  AgentProtocolDefaults,
} from '../../src/contracts';
import {
  AgentLanBrowserRuntimeCommandNameLiteral,
  AgentLanBrowserRuntimeEventNameLiteral,
} from '../../src/lan-pairing-browser-runtime';

const EXPECTED_AGENT_COMMAND_ENTRIES = [
  ['ActivityIngestStatusGet', 'agent.activity.ingest.status.get'],
  ['ActivityRecentSummaryGet', 'agent.activity.recent.summary.get'],
  ['ActivityMemoryGraphGet', 'agent.activity.memory-graph.get'],
  ['ActivityReportDailyGenerate', 'agent.activity.report.daily.generate'],
  ['ActivityReportWeeklyGenerate', 'agent.activity.report.weekly.generate'],
  ['ActivityReportMonthlyGenerate', 'agent.activity.report.monthly.generate'],
  ['ActivityReportSave', 'agent.activity.report.save'],
  ['ActivityReportHistoryList', 'agent.activity.report.history.list'],
  ['ActivityScreenReadModelGet', 'agent.activity.screen.read-model.get'],
  ['ActivityAppUseReadModelGet', 'agent.activity.app-use.read-model.get'],
  ['ActivityBrowserReadModelGet', 'agent.activity.browser.read-model.get'],
  ['ActivityGamesReadModelGet', 'agent.activity.games.read-model.get'],
  ['ActivityAppGameBoundaryReadModelGet', 'agent.activity.app-game.boundary.read-model.get'],
  ['ActivityAppGamePolicyReadinessReadModelGet', 'agent.activity.app-game.policy-readiness.read-model.get'],
  ['ActivityAppGameNotificationReadinessReadModelGet', 'agent.activity.app-game.notification-readiness.read-model.get'],
  ['ActivityAppGameTimerParentSurfaceReadModelGet', 'agent.activity.app-game.timer-parent-surface.read-model.get'],
  [
    'ActivityAppGameTimerParentPreferenceSetupRequest',
    'agent.activity.app-game.timer-parent-surface.parent-preference-setup.request',
  ],
  ['BrowserSocialDashboardReadModelGet', 'agent.browser.social-dashboard.read-model.get'],
  ['BrowserSocialAuditExplanationReadModelGet', 'agent.browser.social-audit-explanation.read-model.get'],
  ['ActivityNetworkReadModelGet', 'agent.activity.network.read-model.get'],
  ['ActivityTrackingReadModelGet', 'agent.activity.tracking.read-model.get'],
  ['BrowserEvidenceRecentGet', 'agent.browser.evidence.recent.get'],
  ['BrowserManagedBridgePoll', 'agent.browser.managed.bridge.poll'],
  ['BrowserInterventionReadModelGet', 'agent.browser.intervention.read-model.get'],
  ['BrowserRuntimeEventChainStreamGet', 'agent.browser.runtime.event-chain.stream.get'],
  ['NetworkFlowReadModelGet', 'agent.network.flow.read-model.get'],
  ['NetworkRuntimeEventChainStreamGet', 'agent.network.runtime.event-chain.stream.get'],
  ['NetworkRemoteDeliveryStatusGet', 'agent.network.remote-delivery.status.get'],
  ['NetworkLiveCaptureStatusGet', 'agent.network.live-capture.status.get'],
  ['NetworkLinuxNftablesLabStatusGet', 'agent.network.linux-nftables-lab.status.get'],
  ['NetworkWindowsFirewallLabStatusGet', 'agent.network.windows-firewall-lab.status.get'],
  ['NetworkWindowsWfpGateStatusGet', 'agent.network.windows-wfp-gate.status.get'],
  ['LocalAiRuntimeStatusGet', 'agent.local-ai.runtime.status.get'],
  ['LocalAiChatGenerate', 'agent.local-ai.chat.generate'],
  ['ParentAssistantAnswerGenerate', 'agent.parent-assistant.answer.generate'],
  ['PolicyPreviewReadModelGet', 'agent.policy.preview.read-model.get'],
  ['BrowserPolicyGet', 'agent.browser-policy.get'],
  ['BrowserPolicyPreview', 'agent.browser-policy.preview'],
  ['BrowserPolicyPatch', 'agent.browser-policy.patch'],
  ['BrowserPolicyReplace', 'agent.browser-policy.replace'],
  ['BrowserPolicyRollback', 'agent.browser-policy.rollback'],
  ['EnforcementExecute', 'agent.enforcement.execute'],
  ['EnforcementTimerRecover', 'agent.enforcement.timer.recover'],
  ['EnforcementTimerExpire', 'agent.enforcement.timer.expire'],
  ['EnforcementOverrideCancel', 'agent.enforcement.override.cancel'],
  ['EnforcementProductControlSpineGet', 'agent.enforcement.product-control-spine.get'],
  ['ParentAssistantThreadList', 'agent.parent-assistant.thread.list'],
  ['ParentAssistantThreadCreate', 'agent.parent-assistant.thread.create'],
  ['ParentAssistantThreadOpen', 'agent.parent-assistant.thread.open'],
  ['ParentAssistantThreadArchive', 'agent.parent-assistant.thread.archive'],
  ['ParentAssistantMessageSend', 'agent.parent-assistant.message.send'],
  ['ParentAssistantRunCancel', 'agent.parent-assistant.run.cancel'],
  ['ParentAssistantQuickActionStart', 'agent.parent-assistant.quick-action.start'],
  ['ParentAssistantActionPreview', 'agent.parent-assistant.action.preview'],
  ['ParentAssistantActionConfirm', 'agent.parent-assistant.action.confirm'],
  ['ParentAssistantProviderStatusGet', 'agent.parent-assistant.provider.status.get'],
  ['LanPairingProofSubmit', 'agent.lan-pairing.proof.submit'],
  ['LanPairingRouteSelect', 'agent.lan-pairing.route.select'],
  ['LanPairingRouteRevoke', 'agent.lan-pairing.route.revoke'],
  ['LanPairingStatusGet', 'agent.lan-pairing.status.get'],
  ['LanPairingBrowserDiscoveryScan', AgentLanBrowserRuntimeCommandNameLiteral.BrowserDiscoveryScan],
  ['LanPairingAddDeviceRequest', AgentLanBrowserRuntimeCommandNameLiteral.AddDeviceRequest],
  ['LanPairingControllerLeaseRenew', 'agent.lan-pairing.controller-lease.renew'],
  ['LanPairingControllerLeaseRelease', 'agent.lan-pairing.controller-lease.release'],
  ['LanPairingControllerLeaseTakeover', 'agent.lan-pairing.controller-lease.takeover'],
  ['LanAiProviderStatusGet', 'agent.lan-ai.provider.status.get'],
  ['LanAiJobSubmit', 'agent.lan-ai.job.submit'],
] as const satisfies ReadonlyArray<readonly [keyof typeof AgentCommand, unknown]>;

const EXPECTED_AGENT_EVENT_ENTRIES = [
  ['HealthReported', 'agent.health.reported'],
  ['LogSnapshotReported', 'agent.log.snapshot.reported'],
  ['DevEchoed', 'agent.dev.echoed'],
  ['WatchStatusReported', 'agent.watch.status.reported'],
  ['ActivityIngestStatusReported', 'agent.activity.ingest.status.reported'],
  ['ActivityRecentSummaryReported', 'agent.activity.recent.summary.reported'],
  ['ActivityMemoryGraphReported', 'agent.activity.memory-graph.reported'],
  ['ActivityReportGenerated', 'agent.activity.report.generated'],
  ['ActivityReportSaved', 'agent.activity.report.saved'],
  ['ActivityReportHistoryReported', 'agent.activity.report.history.reported'],
  ['ActivityScreenReadModelReported', 'agent.activity.screen.read-model.reported'],
  ['ActivityAppUseReadModelReported', 'agent.activity.app-use.read-model.reported'],
  ['ActivityBrowserReadModelReported', 'agent.activity.browser.read-model.reported'],
  ['ActivityGamesReadModelReported', 'agent.activity.games.read-model.reported'],
  ['ActivityAppGameBoundaryReadModelReported', 'agent.activity.app-game.boundary.read-model.reported'],
  ['ActivityAppGamePolicyReadinessReadModelReported', 'agent.activity.app-game.policy-readiness.read-model.reported'],
  [
    'ActivityAppGameNotificationReadinessReadModelReported',
    'agent.activity.app-game.notification-readiness.read-model.reported',
  ],
  [
    'ActivityAppGameTimerParentSurfaceReadModelReported',
    'agent.activity.app-game.timer-parent-surface.read-model.reported',
  ],
  [
    'ActivityAppGameTimerParentPreferenceSetupRequested',
    'agent.activity.app-game.timer-parent-surface.parent-preference-setup.requested',
  ],
  ['BrowserSocialDashboardReadModelReported', 'agent.browser.social-dashboard.read-model.reported'],
  ['BrowserSocialAuditExplanationReadModelReported', 'agent.browser.social-audit-explanation.read-model.reported'],
  ['ActivityNetworkReadModelReported', 'agent.activity.network.read-model.reported'],
  ['ActivityTrackingReadModelReported', 'agent.activity.tracking.read-model.reported'],
  ['BrowserEvidenceRecentReported', 'agent.browser.evidence.recent.reported'],
  ['BrowserManagedStatusReported', 'agent.browser.managed.status.reported'],
  ['BrowserInterventionReadModelReported', 'agent.browser.intervention.read-model.reported'],
  ['BrowserRuntimeEventChainStreamReported', 'agent.browser.runtime.event-chain.stream.reported'],
  ['NetworkFlowReadModelReported', 'agent.network.flow.read-model.reported'],
  ['NetworkRuntimeEventChainStreamReported', 'agent.network.runtime.event-chain.stream.reported'],
  ['NetworkRemoteDeliveryStatusReported', 'agent.network.remote-delivery.status.reported'],
  ['NetworkLiveCaptureStatusReported', 'agent.network.live-capture.status.reported'],
  ['NetworkLinuxNftablesLabStatusReported', 'agent.network.linux-nftables-lab.status.reported'],
  ['NetworkWindowsFirewallLabStatusReported', 'agent.network.windows-firewall-lab.status.reported'],
  ['NetworkWindowsWfpGateStatusReported', 'agent.network.windows-wfp-gate.status.reported'],
  ['LocalAiRuntimeStatusReported', 'agent.local-ai.runtime.status.reported'],
  ['LocalAiChatGenerationReported', 'agent.local-ai.chat.generation.reported'],
  ['ParentAssistantAnswerReported', 'agent.parent-assistant.answer.reported'],
  ['PolicyPreviewReadModelReported', 'agent.policy.preview.read-model.reported'],
  ['BrowserPolicyReported', 'agent.browser-policy.reported'],
  ['BrowserPolicyPreviewed', 'agent.browser-policy.previewed'],
  ['BrowserPolicyPatchAccepted', 'agent.browser-policy.patch.accepted'],
  ['BrowserPolicyPatchRejected', 'agent.browser-policy.patch.rejected'],
  ['BrowserPolicyReplaceAccepted', 'agent.browser-policy.replace.accepted'],
  ['BrowserPolicyReplaceRejected', 'agent.browser-policy.replace.rejected'],
  ['BrowserPolicyRollbackAccepted', 'agent.browser-policy.rollback.accepted'],
  ['BrowserPolicyRollbackRejected', 'agent.browser-policy.rollback.rejected'],
  ['EnforcementAuditReported', 'agent.enforcement.audit.reported'],
  ['EnforcementTimerReported', 'agent.enforcement.timer.reported'],
  ['EnforcementProductControlSpineReported', 'agent.enforcement.product-control-spine.reported'],
  ['ParentAssistantThreadUpdated', 'agent.parent-assistant.thread.updated'],
  ['ParentAssistantMessageAccepted', 'agent.parent-assistant.message.accepted'],
  ['ParentAssistantRunStarted', 'agent.parent-assistant.run.started'],
  ['ParentAssistantMessageDelta', 'agent.parent-assistant.message.delta'],
  ['ParentAssistantMessageCompleted', 'agent.parent-assistant.message.completed'],
  ['ParentAssistantActionPreviewed', 'agent.parent-assistant.action.previewed'],
  ['ParentAssistantActionConfirmed', 'agent.parent-assistant.action.confirmed'],
  ['ParentAssistantProviderDegraded', 'agent.parent-assistant.provider.degraded'],
  ['ParentAssistantErrorReported', 'agent.parent-assistant.error.reported'],
  ['LanPairingStatusReported', 'agent.lan-pairing.status.reported'],
  ['LanPairingBrowserDiscoveryReported', AgentLanBrowserRuntimeEventNameLiteral.BrowserDiscoveryReported],
  ['LanPairingAddDeviceReported', AgentLanBrowserRuntimeEventNameLiteral.AddDeviceReported],
  ['LanPairingAuditReported', 'agent.lan-pairing.audit.reported'],
  ['LanAiJobReported', 'agent.lan-ai.job.reported'],
] as const satisfies ReadonlyArray<readonly [keyof typeof AgentEvent, unknown]>;

const EXPECTED_LAN_HOUSEHOLD_ACTION_FIELD_ENTRIES = [
  ['LanHouseholdActionId', 'householdActionId'],
  ['LanHouseholdActionKind', 'householdActionKind'],
  ['LanHouseholdActionChildProfileId', 'childProfileId'],
  ['LanHouseholdActionDisplayName', 'displayName'],
  ['LanHouseholdActionRevokedAt', 'revokedAt'],
] as const satisfies ReadonlyArray<readonly [keyof typeof AgentProtocolDefaults.Field, unknown]>;

const EXPECTED_LAN_HOUSEHOLD_ACTION_KIND_ENTRIES = [
  ['Assign', 'assign'],
  ['Rename', 'rename'],
  ['Ignore', 'ignore'],
  ['Restore', 'restore'],
  ['Trust', 'trust'],
] as const satisfies ReadonlyArray<readonly [keyof typeof AgentProtocolDefaults.LanHouseholdActionKind, unknown]>;

const EXPECTED_LAN_PARENT_AUTHORITY_ENTRIES = [
  ['ActiveController', 'active-controller'],
  ['Observer', 'observer'],
] as const satisfies ReadonlyArray<readonly [keyof typeof AgentProtocolDefaults.LanParentAuthority, unknown]>;

function expectConstantEntries<TConstants extends object>(
  constants: TConstants,
  entries: ReadonlyArray<readonly [keyof TConstants, unknown]>
) {
  for (const [key, value] of entries) {
    expect(constants[key]).toBe(value);
  }
}

it('AgentCommandEnvelopeSchema: accepts a portal command for a Windows localhost agent', () => {
  const parsed = AgentCommandEnvelopeSchema.safeParse({
    schemaVersion: 1,
    messageId: 'cmd-1',
    sentAt: '2026-05-19T00:00:00Z',
    source: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    target: {
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    },
    command: 'agent.health.check',
    payload: {},
  });

  expect(parsed.success).toBe(true);
});

it('AgentMessageTargetSchema: accepts a Windows local network agent route', () => {
  const target = AgentProtocolDefaults.Target.LocalNetworkWindowsAgent;

  expect(target.route).toBe('local-network');
});

it('AgentEventEnvelopeSchema: accepts a Rust response event with an optional snapshot', () => {
  const parsed = AgentEventEnvelopeSchema.safeParse({
    schemaVersion: 1,
    eventId: 'evt-1',
    correlationId: 'cmd-1',
    sentAt: '2026-05-19T00:00:01Z',
    source: {
      peerId: 'local-dev-agent',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: 'agent.health.reported',
    severity: 'info',
    payload: {
      online: true,
    },
    snapshot: null,
  });

  expect(parsed.success).toBe(true);
});

it('AgentCommandEnvelopeSchema: accepts parent assistant message commands over the agent protocol', () => {
  const parsed = AgentCommandEnvelopeSchema.safeParse({
    schemaVersion: 1,
    messageId: 'cmd-assistant-message-1',
    sentAt: '2026-05-25T14:00:00Z',
    source: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    target: {
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    },
    command: 'agent.parent-assistant.message.send',
    payload: {
      assistantThreadId: 'assistant-thread-1',
      inputText: 'Give me today report.',
      inputSource: 'typed',
    },
  });

  expect(parsed.success).toBe(true);
});

it('AgentCommand: exposes typed command constants for portal requests', () => {
  expectConstantEntries(AgentCommand, EXPECTED_AGENT_COMMAND_ENTRIES);
});

it('AgentLanPairingSupportedWebSocketCommand: keeps V0.9 LAN pairing support limited to WebSocket commands', () => {
  expect(Object.values(AgentLanPairingSupportedWebSocketCommand)).toEqual([
    'agent.lan-pairing.proof.submit',
    'agent.lan-pairing.route.select',
    'agent.lan-pairing.route.revoke',
    'agent.lan-pairing.status.get',
    AgentLanBrowserRuntimeCommandNameLiteral.BrowserDiscoveryScan,
    AgentLanBrowserRuntimeCommandNameLiteral.AddDeviceRequest,
    'agent.lan-pairing.controller-lease.renew',
    'agent.lan-pairing.controller-lease.release',
    'agent.lan-pairing.controller-lease.takeover',
    'agent.lan-ai.provider.status.get',
    'agent.lan-ai.job.submit',
  ]);
  expect(Object.values(AgentLanPairingSupportedWebSocketCommand)).not.toContain('agent.lan-pairing.discovery.http');
});

it('AgentProtocolDefaults: exposes LAN household action fields for add-device decisions', () => {
  expectConstantEntries(AgentProtocolDefaults.Field, EXPECTED_LAN_HOUSEHOLD_ACTION_FIELD_ENTRIES);
  expectConstantEntries(AgentProtocolDefaults.LanHouseholdActionKind, EXPECTED_LAN_HOUSEHOLD_ACTION_KIND_ENTRIES);
  expectConstantEntries(AgentProtocolDefaults.LanParentAuthority, EXPECTED_LAN_PARENT_AUTHORITY_ENTRIES);
});

it('AgentProtocolDefaults.Field: exposes local AI model cache payload fields', () => {
  expect(AgentProtocolDefaults.Field.LocalAiModelArtifactRef).toBe('artifactRef');
  expect(AgentProtocolDefaults.Field.LocalAiModelManifestRef).toBe('manifestRef');
  expect(AgentProtocolDefaults.Field.LocalAiModelSourcePolicy).toBe('sourcePolicy');
  expect(AgentProtocolDefaults.Field.LocalAiModelCacheState).toBe('cacheState');
  expect(AgentProtocolDefaults.Field.LocalAiModelCacheHealth).toBe('cacheHealth');
  expect(AgentProtocolDefaults.Field.LocalAiModelManifestIntegrity).toBe('manifestIntegrity');
  expect(AgentProtocolDefaults.Field.LocalAiModelDownloadEnabled).toBe('downloadEnabled');
  expect(AgentProtocolDefaults.Field.LocalAiModelDownloadStatus).toBe('downloadStatus');
  expect(AgentProtocolDefaults.Field.LocalAiModelCacheByteSize).toBe('cacheByteSize');
  expect(AgentProtocolDefaults.Field.LocalAiModelCacheUnavailableReason).toBe('cacheUnavailableReason');
  expect(AgentProtocolDefaults.Field.LocalAiModelCacheStorageError).toBe('storageError');
  expect(AgentProtocolDefaults.Field.LocalAiModelCacheCorruptionReason).toBe('corruptionReason');
});

it('AgentProtocolDefaults.Field: exposes local AI runtime payload fields', () => {
  expect(AgentProtocolDefaults.Field.LocalAiRuntimeReferenceId).toBe('runtimeReferenceId');
  expect(AgentProtocolDefaults.Field.LocalAiUnavailableReason).toBe('unavailableReason');
  expect(AgentProtocolDefaults.Field.LocalAiPrivacyMode).toBe('privacyMode');
  expect(AgentProtocolDefaults.Field.LocalAiAdapterBoundary).toBe('adapterBoundary');
  expect(AgentProtocolDefaults.Field.LocalAiExecutionState).toBe('executionState');
  expect(AgentProtocolDefaults.Field.LocalAiProviderSource).toBe('providerSource');
  expect(AgentProtocolDefaults.Field.LocalAiAdapterProbeState).toBe('probeState');
  expect(AgentProtocolDefaults.Field.LocalAiProviderConfigurationState).toBe('configurationState');
  expect(AgentProtocolDefaults.Field.LocalAiAdapterReadinessState).toBe('readinessState');
  expect(AgentProtocolDefaults.Field.LocalAiExecutionAllowed).toBe('executionAllowed');
  expect(AgentProtocolDefaults.Field.LocalAiGenerationState).toBe('generationState');
  expect(AgentProtocolDefaults.Field.LocalAiOutputText).toBe('outputText');
  expect(AgentProtocolDefaults.Field.LocalAiPrompt).toBe('prompt');
  expect(AgentProtocolDefaults.Field.LocalAiPromptCharCount).toBe('promptCharCount');
  expect(AgentProtocolDefaults.Field.LocalAiMaxOutputTokens).toBe('maxOutputTokens');
  expect(AgentProtocolDefaults.Field.LocalAiTimeoutMs).toBe('timeoutMs');
  expect(AgentProtocolDefaults.Field.LocalAiDurationMs).toBe('durationMs');
  expect(AgentProtocolDefaults.Field.LocalAiExitCode).toBe('exitCode');
  expect(AgentProtocolDefaults.Field.LocalAiStderrByteSize).toBe('stderrByteSize');
});

it('AgentProtocolDefaults.Field: exposes parent assistant payload fields', () => {
  expect(AgentProtocolDefaults.Field.ParentAssistantAnswer).toBe('parentAssistantAnswer');
  expect(AgentProtocolDefaults.Field.ParentAssistantActionPreview).toBe('parentAssistantActionPreview');
  expect(AgentProtocolDefaults.Field.ParentAssistantAnswerState).toBe('parentAssistantAnswerState');
  expect(AgentProtocolDefaults.Field.ParentAssistantAnswerText).toBe('parentAssistantAnswerText');
  expect(AgentProtocolDefaults.Field.ParentAssistantApiAuthorizationState).toBe('parentAssistantApiAuthorizationState');
  expect(AgentProtocolDefaults.Field.ParentAssistantApiCustodyLabel).toBe('parentAssistantApiCustodyLabel');
  expect(AgentProtocolDefaults.Field.ParentAssistantApiDeletionState).toBe('parentAssistantApiDeletionState');
  expect(AgentProtocolDefaults.Field.ParentAssistantApiProviderBoundary).toBe('parentAssistantApiProviderBoundary');
  expect(AgentProtocolDefaults.Field.ParentAssistantApiRetentionState).toBe('parentAssistantApiRetentionState');
  expect(AgentProtocolDefaults.Field.ParentAssistantCitationCount).toBe('parentAssistantCitationCount');
  expect(AgentProtocolDefaults.Field.ParentAssistantEvidenceSummary).toBe('parentAssistantEvidenceSummary');
  expect(AgentProtocolDefaults.Field.ParentAssistantProviderState).toBe('parentAssistantProviderState');
  expect(AgentProtocolDefaults.Field.ParentAssistantProviderRoute).toBe('parentAssistantProviderRoute');
  expect(AgentProtocolDefaults.Field.ParentAssistantQuestion).toBe('parentAssistantQuestion');
  expect(AgentProtocolDefaults.Field.ParentAssistantRequestId).toBe('parentAssistantRequestId');
  expect(AgentProtocolDefaults.Field.ParentAssistantThreadId).toBe('assistantThreadId');
  expect(AgentProtocolDefaults.Field.ParentAssistantMessageId).toBe('assistantMessageId');
  expect(AgentProtocolDefaults.Field.ParentAssistantRunId).toBe('assistantRunId');
  expect(AgentProtocolDefaults.Field.ParentAssistantActionIntentId).toBe('assistantActionIntentId');
  expect(AgentProtocolDefaults.Field.ParentAssistantQuickActionId).toBe('quickActionId');
  expect(AgentProtocolDefaults.Field.ParentAssistantPromptTemplateId).toBe('promptTemplateId');
  expect(AgentProtocolDefaults.Field.ParentAssistantStarterCategory).toBe('starterCategory');
  expect(AgentProtocolDefaults.Field.ParentAssistantInputText).toBe('inputText');
  expect(AgentProtocolDefaults.Field.ParentAssistantInputSource).toBe('inputSource');
  expect(AgentProtocolDefaults.Field.ParentAssistantRequiredChildContracts).toBe('requiredChildContracts');
  expect(AgentProtocolDefaults.Field.ParentAssistantBackendState).toBe('assistantBackendState');
});

it('AgentProtocolDefaults.Field: exposes read-model payload fields', () => {
  expectReadModelBridgeAndActivityFields();
  expectReadModelBrowserInterventionFields();
  expectReadModelEnforcementAndUnmanagedFields();
  expect(AgentProtocolDefaults.Delimiter.List).toBe(',');
});

it('AgentProtocolDefaults.Field: exposes network product path ref fields', () => {
  expect(AgentProtocolDefaults.Field.NetworkProductPathAnalyzerAlertRefs).toBe(
    'networkProductPathAnalyzerAlertRefs'
  );
  expect(AgentProtocolDefaults.Field.NetworkProductPathAiDetectionRefs).toBe(
    'networkProductPathAiDetectionRefs'
  );
  expect(AgentProtocolDefaults.Field.NetworkProductPathRiskBudgetRefs).toBe(
    'networkProductPathRiskBudgetRefs'
  );
});

it('AgentProtocolDefaults.PolicyPreview: exposes portal policy preview parser values', () => {
  expect(AgentProtocolDefaults.Primitive.Number).toBe('number');
  expect(AgentProtocolDefaults.PolicyPreview.TargetType.NetworkDomain).toBe('network-domain');
  expect(AgentProtocolDefaults.PolicyPreview.Action.AskParent).toBe('ask-parent');
  expect(AgentProtocolDefaults.PolicyPreview.Action.ManualReview).toBe('manual-review');
  expect(AgentProtocolDefaults.PolicyPreview.EvidenceGrade.A).toBe('A');
  expect(AgentProtocolDefaults.PolicyPreview.MappingMode.ParentReview).toBe('parent-review');
  expect(AgentProtocolDefaults.PolicyPreview.MappingMode.AdapterUnavailable).toBe('adapter-unavailable');
  expect(AgentProtocolDefaults.PolicyPreview.HandoffState.DisabledPreviewOnly).toBe('disabled-preview-only');
  expect(AgentProtocolDefaults.PolicyPreview.ValidationMessage.DryRunPreviewOnlyHandoffRequired).toContain(
    'preview-only handoff'
  );
});

function expectReadModelBridgeAndActivityFields() {
  expect(AgentProtocolDefaults.Field.BridgeEndpointRef).toBe('bridgeEndpointRef');
  expect(AgentProtocolDefaults.Field.BridgeKind).toBe('bridgeKind');
  expect(AgentProtocolDefaults.Field.ActivityDigest).toBe('activityDigest');
  expect(AgentProtocolDefaults.Field.ActivityTrackingReadModel).toBe('trackingReadModel');
  expect(AgentProtocolDefaults.Field.BrowserSocialDashboardReadModel).toBe('browserSocialDashboardReadModel');
  expect(AgentProtocolDefaults.Field.BrowserSocialAuditExplanationReadModel).toBe(
    'browserSocialAuditExplanationReadModel'
  );
  expect(AgentProtocolDefaults.Field.ActivityAppGameNotificationReadinessReadModel).toBe(
    'appGameNotificationReadinessReadModel'
  );
  expect(AgentProtocolDefaults.Field.ActivityAppGameTimerParentPreferenceSetupRequest).toBe(
    'appGameTimerParentPreferenceSetupRequest'
  );
  expect(AgentProtocolDefaults.Field.DatabaseReady).toBe('databaseReady');
  expect(AgentProtocolDefaults.Field.BrowserEvidenceId).toBe('browserEvidenceId');
  expect(AgentProtocolDefaults.Field.BrowserInterventionId).toBe('browserInterventionId');
  expect(AgentProtocolDefaults.Field.BrowserInterventionActionId).toBe('browserInterventionActionId');
  expect(AgentProtocolDefaults.Field.BrowserInterventionAuditId).toBe('browserInterventionAuditId');
  expect(AgentProtocolDefaults.Field.BrowserBoundaryState).toBe('browserBoundaryState');
  expect(AgentProtocolDefaults.Field.ChildDeliveryState).toBe('childDeliveryState');
  expect(AgentProtocolDefaults.Field.ClaimBoundary).toBe('claimBoundary');
  expect(AgentProtocolDefaults.Field.ExactUrlAvailable).toBe('exactUrlAvailable');
}

function expectReadModelBrowserInterventionFields() {
  expect(AgentProtocolDefaults.Field.DecisionSource).toBe('decisionSource');
  expect(AgentProtocolDefaults.Field.ExactUrlClaimState).toBe('exactUrlClaimState');
  expect(AgentProtocolDefaults.Field.FreshUntil).toBe('freshUntil');
  expect(AgentProtocolDefaults.Field.InterventionAction).toBe('interventionAction');
  expect(AgentProtocolDefaults.Field.InterventionMechanism).toBe('interventionMechanism');
  expect(AgentProtocolDefaults.Field.InterventionOutcome).toBe('interventionOutcome');
  expect(AgentProtocolDefaults.Field.InterventionTargetType).toBe('interventionTargetType');
  expect(AgentProtocolDefaults.Field.InterventionTargetValue).toBe('interventionTargetValue');
  expect(AgentProtocolDefaults.Field.ManagedSessionInterventionCapability).toBe('managedSessionInterventionCapability');
  expect(AgentProtocolDefaults.Field.ManagedState).toBe('managedState');
  expect(AgentProtocolDefaults.Field.ObservedUrl).toBe('observedUrl');
  expect(AgentProtocolDefaults.Field.ProfilePathRef).toBe('profilePathRef');
  expect(AgentProtocolDefaults.Field.QueryVisibility).toBe('queryVisibility');
  expect(AgentProtocolDefaults.Field.RequestedUrl).toBe('requestedUrl');
  expect(AgentProtocolDefaults.Field.StaleAt).toBe('staleAt');
  expect(AgentProtocolDefaults.Field.TabId).toBe('tabId');
  expect(AgentProtocolDefaults.Field.WindowId).toBe('windowId');
}

function expectReadModelEnforcementAndUnmanagedFields() {
  expect(AgentProtocolDefaults.Field.CapabilityStatus).toBe('capabilityStatus');
  expect(AgentProtocolDefaults.Field.DestinationDomain).toBe('destinationDomain');
  expect(AgentProtocolDefaults.Field.EnforcementActionId).toBe('enforcementActionId');
  expect(AgentProtocolDefaults.Field.EnforcementAuditEvent).toBe('enforcementAuditEvent');
  expect(AgentProtocolDefaults.Field.EnforcementStatus).toBe('enforcementStatus');
  expect(AgentProtocolDefaults.Field.EnforcementTimerState).toBe('enforcementTimerState');
  expect(AgentProtocolDefaults.Field.EnforcementTimerStateId).toBe('enforcementTimerStateId');
  expect(AgentProtocolDefaults.Field.EnforcementProductControlSpineReadModel).toBe(
    'enforcementProductControlSpineReadModel'
  );
  expect(AgentProtocolDefaults.Field.AdapterActionExecuted).toBe('adapterActionExecuted');
  expect(AgentProtocolDefaults.Field.EventRef).toBe('eventRef');
  expect(AgentProtocolDefaults.Field.EventType).toBe('eventType');
  expect(AgentProtocolDefaults.Field.EvidenceReferenceIds).toBe('evidenceReferenceIds');
  expect(AgentProtocolDefaults.Field.MostRecentSubjectName).toBe('mostRecentSubjectName');
  expectReadModelBrowserRuntimeStreamFields();
  expectReadModelNetworkStatusFields();
  expect(AgentProtocolDefaults.Field.Payload).toBe('payload');
  expect(AgentProtocolDefaults.Field.VisibleManualRequired).toBe('visibleManualRequired');
  expect(AgentProtocolDefaults.Field.UnmanagedBrowserEnforcement).toBe('unmanagedBrowserEnforcement');
  expect(AgentProtocolDefaults.Field.UnmanagedFallbackAction).toBe('unmanagedFallbackAction');
  expect(AgentProtocolDefaults.Field.UnmanagedDetectionConfidence).toBe('unmanagedDetectionConfidence');
  expect(AgentProtocolDefaults.Field.UnmanagedDetectionReason).toBe('unmanagedDetectionReason');
  expect(AgentProtocolDefaults.Field.UnmanagedDetectionState).toBe('unmanagedDetectionState');
  expect(AgentProtocolDefaults.Field.UnmanagedExecutablePathRef).toBe('unmanagedExecutablePathRef');
  expect(AgentProtocolDefaults.Field.UnmanagedProcessHashRef).toBe('unmanagedProcessHashRef');
  expect(AgentProtocolDefaults.Field.UnmanagedProcessKind).toBe('unmanagedProcessKind');
  expect(AgentProtocolDefaults.Field.UnmanagedProcessName).toBe('unmanagedProcessName');
  expect(AgentProtocolDefaults.Field.UnmanagedSignatureRef).toBe('unmanagedSignatureRef');
}

function expectReadModelBrowserRuntimeStreamFields() {
  expect(AgentProtocolDefaults.Field.BrowserRuntimeEventChainStream).toBe('browserRuntimeEventChainStream');
  expect(AgentProtocolDefaults.Field.BrowserRuntimeObservedRows).toBe('browserRuntimeObservedRows');
  expect(AgentProtocolDefaults.Field.BrowserRuntimeStreamedEvents).toBe('browserRuntimeStreamedEvents');
  expect(AgentProtocolDefaults.Field.BrowserRuntimeFailedRows).toBe('browserRuntimeFailedRows');
  expect(AgentProtocolDefaults.Field.BrowserRuntimeExactUrlRows).toBe('browserRuntimeExactUrlRows');
  expect(AgentProtocolDefaults.Field.BrowserRuntimeManualRequiredRows).toBe('browserRuntimeManualRequiredRows');
  expect(AgentProtocolDefaults.Field.BrowserRuntimeInterventionCommandEvents).toBe(
    'browserRuntimeInterventionCommandEvents'
  );
  expect(AgentProtocolDefaults.Field.BrowserRuntimeReadModelProjectionEvents).toBe(
    'browserRuntimeReadModelProjectionEvents'
  );
}

function expectReadModelNetworkStatusFields() {
  expect(AgentProtocolDefaults.Field.NetworkRuntimeEventChainStream).toBe('networkRuntimeEventChainStream');
  expect(AgentProtocolDefaults.Field.NetworkRuntimeStreamedEvents).toBe('networkRuntimeStreamedEvents');
  expect(AgentProtocolDefaults.Field.NetworkRemoteDeliveryStatus).toBe('networkRemoteDeliveryStatus');
  expect(AgentProtocolDefaults.Field.NetworkLiveCaptureStatus).toBe('networkLiveCaptureStatus');
  expect(AgentProtocolDefaults.Field.NetworkLinuxNftablesLabStatus).toBe('networkLinuxNftablesLabStatus');
  expect(AgentProtocolDefaults.Field.NetworkWindowsFirewallLabStatus).toBe('networkWindowsFirewallLabStatus');
  expect(AgentProtocolDefaults.Field.NetworkWindowsWfpGateStatus).toBe('networkWindowsWfpGateStatus');
}

it('AgentProtocolDefaults.Field: exposes browser policy payload fields', () => {
  expect(AgentProtocolDefaults.Field.BrowserPolicyRequest).toBe('browserPolicyRequest');
  expect(AgentProtocolDefaults.Field.BrowserPolicyResponse).toBe('browserPolicyResponse');
  expect(AgentProtocolDefaults.Field.BrowserPolicyUpdateKind).toBe('browserPolicyUpdateKind');
  expect(AgentProtocolDefaults.Field.BrowserPolicyRejectionReason).toBe('browserPolicyRejectionReason');
  expect(AgentProtocolDefaults.Field.BrowserPolicyEffectivePolicy).toBe('browserPolicyEffectivePolicy');
  expect(AgentProtocolDefaults.Field.BrowserPolicyCapabilityRegistry).toBe('browserPolicyCapabilityRegistry');
});

it('AgentEvent: exposes typed constants for portal result rendering', () => {
  expectConstantEntries(AgentEvent, EXPECTED_AGENT_EVENT_ENTRIES);
});

it('AgentProtocolDefaults.NetworkRemoteDeliveryStatus: tracks row10t status identity', () => {
  expect(AgentProtocolDefaults.NetworkRemoteDeliveryStatus.StatusRef).toBe(
    'network.remote-delivery.external-cross-process-transport-status.10t'
  );
});

it('AgentCommandEnvelopeSchema: rejects unknown commands', () => {
  const parsed = AgentCommandEnvelopeSchema.safeParse({
    schemaVersion: 1,
    messageId: 'cmd-1',
    sentAt: '2026-05-19T00:00:00Z',
    source: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    target: {
      deviceId: 'local-dev-agent',
      platform: 'windows',
      route: 'localhost',
    },
    command: 'agent.process.kill',
    payload: {},
  });

  expect(parsed.success).toBe(false);
});

it('AgentRouteSecurityPolicySchema: forbids anonymous local-network control', () => {
  expect(AgentProtocolDefaults.RouteSecurity.Localhost.allowsAnonymousControl).toBe(true);
  expect(AgentProtocolDefaults.RouteSecurity.LocalNetwork.requiresPairing).toBe(true);
  expect(AgentProtocolDefaults.RouteSecurity.LocalNetwork.allowsAnonymousControl).toBe(false);
  expect(AgentProtocolDefaults.RouteSecurity.CloudRelay.allowsAnonymousControl).toBe(false);
});

it('AgentPairingProofSchema: accepts hashed pairing proof without raw token transport', () => {
  const parsed = AgentPairingProofSchema.safeParse({
    pairingId: 'pairing-local-dev',
    deviceId: 'local-dev-agent',
    parentPeerId: 'portal-dev',
    issuedAt: '2026-05-19T00:00:00Z',
    expiresAt: '2026-05-19T00:05:00Z',
    tokenHash: 'sha256:local-dev-token-hash',
  });

  expect(parsed.success).toBe(true);
});
