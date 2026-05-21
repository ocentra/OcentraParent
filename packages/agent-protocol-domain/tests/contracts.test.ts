import { expect, it } from 'vitest';
import {
  AgentCommandEnvelopeSchema,
  AgentCommand,
  AgentEvent,
  AgentEventEnvelopeSchema,
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
  expect(AgentCommand.BrowserEvidenceRecentGet).toBe('agent.browser.evidence.recent.get');
  expect(AgentCommand.BrowserManagedBridgePoll).toBe('agent.browser.managed.bridge.poll');
  expect(AgentCommand.NetworkFlowReadModelGet).toBe('agent.network.flow.read-model.get');
  expect(AgentCommand.LocalAiRuntimeStatusGet).toBe('agent.local-ai.runtime.status.get');
});

it('AgentProtocolDefaults.Field: exposes read-model payload fields', () => {
  expect(AgentProtocolDefaults.Field.BridgeEndpointRef).toBe('bridgeEndpointRef');
  expect(AgentProtocolDefaults.Field.BridgeKind).toBe('bridgeKind');
  expect(AgentProtocolDefaults.Field.DatabaseReady).toBe('databaseReady');
  expect(AgentProtocolDefaults.Field.LocalAiRuntimeReferenceId).toBe('runtimeReferenceId');
  expect(AgentProtocolDefaults.Field.LocalAiUnavailableReason).toBe('unavailableReason');
  expect(AgentProtocolDefaults.Field.LocalAiPrivacyMode).toBe('privacyMode');
  expect(AgentProtocolDefaults.Field.LocalAiAdapterBoundary).toBe('adapterBoundary');
  expect(AgentProtocolDefaults.Field.LocalAiExecutionState).toBe('executionState');
  expect(AgentProtocolDefaults.Field.LocalAiProviderSource).toBe('providerSource');
  expect(AgentProtocolDefaults.Field.LocalAiAdapterProbeState).toBe('probeState');
  expect(AgentProtocolDefaults.Field.LocalAiProviderConfigurationState).toBe('configurationState');
  expect(AgentProtocolDefaults.Field.LocalAiExecutionAllowed).toBe('executionAllowed');
  expect(AgentProtocolDefaults.Field.BrowserEvidenceId).toBe('browserEvidenceId');
  expect(AgentProtocolDefaults.Field.ManagedState).toBe('managedState');
  expect(AgentProtocolDefaults.Field.ProfilePathRef).toBe('profilePathRef');
  expect(AgentProtocolDefaults.Field.QueryVisibility).toBe('queryVisibility');
  expect(AgentProtocolDefaults.Field.CapabilityStatus).toBe('capabilityStatus');
  expect(AgentProtocolDefaults.Field.DestinationDomain).toBe('destinationDomain');
  expect(AgentProtocolDefaults.Field.MostRecentSubjectName).toBe('mostRecentSubjectName');
});

it('AgentEvent: exposes typed constants for portal result rendering', () => {
  expect(AgentEvent.HealthReported).toBe('agent.health.reported');
  expect(AgentEvent.LogSnapshotReported).toBe('agent.log.snapshot.reported');
  expect(AgentEvent.DevEchoed).toBe('agent.dev.echoed');
  expect(AgentEvent.WatchStatusReported).toBe('agent.watch.status.reported');
  expect(AgentEvent.ActivityIngestStatusReported).toBe('agent.activity.ingest.status.reported');
  expect(AgentEvent.ActivityRecentSummaryReported).toBe('agent.activity.recent.summary.reported');
  expect(AgentEvent.BrowserEvidenceRecentReported).toBe('agent.browser.evidence.recent.reported');
  expect(AgentEvent.BrowserManagedStatusReported).toBe('agent.browser.managed.status.reported');
  expect(AgentEvent.NetworkFlowReadModelReported).toBe('agent.network.flow.read-model.reported');
  expect(AgentEvent.LocalAiRuntimeStatusReported).toBe('agent.local-ai.runtime.status.reported');
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
