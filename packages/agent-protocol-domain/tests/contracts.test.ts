import { expect, it } from 'vitest';
import {
  AgentCommandEnvelopeSchema,
  AgentCommand,
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentLanPairingSupportedWebSocketCommand,
  AgentPairingProofSchema,
  AgentProtocolDefaults,
} from '../src/contracts';

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

it('AgentCommand: exposes typed command constants for portal requests', () => {
  expect(AgentCommand.ActivityIngestStatusGet).toBe('agent.activity.ingest.status.get');
  expect(AgentCommand.ActivityRecentSummaryGet).toBe('agent.activity.recent.summary.get');
  expect(AgentCommand.ActivityMemoryGraphGet).toBe('agent.activity.memory-graph.get');
  expect(AgentCommand.BrowserEvidenceRecentGet).toBe('agent.browser.evidence.recent.get');
  expect(AgentCommand.BrowserManagedBridgePoll).toBe('agent.browser.managed.bridge.poll');
  expect(AgentCommand.BrowserInterventionReadModelGet).toBe('agent.browser.intervention.read-model.get');
  expect(AgentCommand.NetworkFlowReadModelGet).toBe('agent.network.flow.read-model.get');
  expect(AgentCommand.LocalAiRuntimeStatusGet).toBe('agent.local-ai.runtime.status.get');
  expect(AgentCommand.LocalAiChatGenerate).toBe('agent.local-ai.chat.generate');
  expect(AgentCommand.LanPairingProofSubmit).toBe('agent.lan-pairing.proof.submit');
  expect(AgentCommand.LanPairingStatusGet).toBe('agent.lan-pairing.status.get');
});

it('AgentLanPairingSupportedWebSocketCommand: keeps V0.9 LAN pairing support limited to WebSocket commands', () => {
  expect(Object.values(AgentLanPairingSupportedWebSocketCommand)).toEqual([
    'agent.lan-pairing.proof.submit',
    'agent.lan-pairing.status.get',
  ]);
  expect(Object.values(AgentLanPairingSupportedWebSocketCommand)).not.toContain('agent.lan-pairing.discovery.http');
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

it('AgentProtocolDefaults.Field: exposes read-model payload fields', () => {
  expect(AgentProtocolDefaults.Field.BridgeEndpointRef).toBe('bridgeEndpointRef');
  expect(AgentProtocolDefaults.Field.BridgeKind).toBe('bridgeKind');
  expect(AgentProtocolDefaults.Field.ActivityDigest).toBe('activityDigest');
  expect(AgentProtocolDefaults.Field.DatabaseReady).toBe('databaseReady');
  expect(AgentProtocolDefaults.Field.BrowserEvidenceId).toBe('browserEvidenceId');
  expect(AgentProtocolDefaults.Field.BrowserInterventionId).toBe('browserInterventionId');
  expect(AgentProtocolDefaults.Field.DecisionSource).toBe('decisionSource');
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
  expect(AgentProtocolDefaults.Field.CapabilityStatus).toBe('capabilityStatus');
  expect(AgentProtocolDefaults.Field.DestinationDomain).toBe('destinationDomain');
  expect(AgentProtocolDefaults.Field.MostRecentSubjectName).toBe('mostRecentSubjectName');
  expect(AgentProtocolDefaults.Field.UnmanagedBrowserEnforcement).toBe('unmanagedBrowserEnforcement');
});

it('AgentEvent: exposes typed constants for portal result rendering', () => {
  expect(AgentEvent.HealthReported).toBe('agent.health.reported');
  expect(AgentEvent.LogSnapshotReported).toBe('agent.log.snapshot.reported');
  expect(AgentEvent.DevEchoed).toBe('agent.dev.echoed');
  expect(AgentEvent.WatchStatusReported).toBe('agent.watch.status.reported');
  expect(AgentEvent.ActivityIngestStatusReported).toBe('agent.activity.ingest.status.reported');
  expect(AgentEvent.ActivityRecentSummaryReported).toBe('agent.activity.recent.summary.reported');
  expect(AgentEvent.ActivityMemoryGraphReported).toBe('agent.activity.memory-graph.reported');
  expect(AgentEvent.BrowserEvidenceRecentReported).toBe('agent.browser.evidence.recent.reported');
  expect(AgentEvent.BrowserManagedStatusReported).toBe('agent.browser.managed.status.reported');
  expect(AgentEvent.BrowserInterventionReadModelReported).toBe('agent.browser.intervention.read-model.reported');
  expect(AgentEvent.NetworkFlowReadModelReported).toBe('agent.network.flow.read-model.reported');
  expect(AgentEvent.LocalAiRuntimeStatusReported).toBe('agent.local-ai.runtime.status.reported');
  expect(AgentEvent.LocalAiChatGenerationReported).toBe('agent.local-ai.chat.generation.reported');
  expect(AgentEvent.LanPairingStatusReported).toBe('agent.lan-pairing.status.reported');
  expect(AgentEvent.LanPairingAuditReported).toBe('agent.lan-pairing.audit.reported');
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
