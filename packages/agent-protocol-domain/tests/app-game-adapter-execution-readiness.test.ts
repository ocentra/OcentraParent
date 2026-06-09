import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { describe, expect, it } from 'vitest';
import { AgentEvent, type AgentEventEnvelope } from '../src/contracts';
import { AgentProtocolSchemaVersion } from '../src/primitives';
import {
  AgentAppGameAdapterExecutionDecision,
  AgentAppGameAdapterHostCapabilityState,
  AgentAppGameAdapterExecutionReadinessPayloadField,
  AgentAppGameAdapterExecutionState,
  parseAgentAppGameAdapterExecutionReadinessEvent,
} from '../src/app-game-adapter-execution-readiness';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const AdapterExecutionReadinessReadModel = {
  schemaVersion: AppGameSchemaVersion,
  readModelId: 'app-game-adapter-execution-readiness',
  generatedAt: '2026-06-08T09:17:00.000Z',
  sourceReadModelIds: ['v0-8-supported-adapter-runtime-proof'],
  custodyLabel: 'supported-adapter-runtime-proof',
  capabilityStatus: 'app-game-adapter-execution-partial',
  returned: 2,
  executionAllowedCount: 1,
  blockedBeforeExecutionCount: 1,
  adapterExecutionClaimedCount: 1,
  hostCapabilityAvailableCount: 1,
  hostCapabilityNotDetectedCount: 1,
  hostCapabilityNotApplicableCount: 0,
  hostCapabilityProbeRefCount: 2,
  broadInstalledAppBlockingClaimed: false,
  childDeviceDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  providerDeliveryClaimed: false,
  privateDiagnosticsClaimed: false,
  rows: [
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-adapter-execution-windows-app-game-owned-process-time-limit',
      sourceProofEntryId: 'windows-app-game-owned-process-time-limit',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'app-game-owned-process-time-limit',
      adapterExecutionState: AgentAppGameAdapterExecutionState.ProvedScopedExecution,
      executionDecision: AgentAppGameAdapterExecutionDecision.ExecutionAllowed,
      runtimeBoundary: 'windows-app-game-owned-process-time-limit',
      targetIdentityState: 'process-session-evidence-backed',
      rollbackReferenceState: 'timer-recovery-backed',
      auditReferenceState: 'audit-reference-backed',
      evidenceRefs: ['app-game-session-evidence-ref'],
      hostCapabilityState: AgentAppGameAdapterHostCapabilityState.Available,
      hostCapabilityEvidenceRefs: ['adapter-capability-state-ref'],
      hostCapabilityProbeRefs: ['windows-host-local-probe-ref'],
      linkedProofArtifacts: ['test-results/v0-8-windows-app-time-limit-adapter-mvp/proof.json'],
      manualProofRequirements: [],
      claimBoundary: 'Scoped Windows owned-process app/game timer execution only.',
      fallbackBehavior: 'Targets without process/session identity stay manual-required.',
      adapterExecutionClaimed: true,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T09:17:00.000Z',
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-adapter-execution-windows-broad-installed-app-blocking-manual-gate',
      sourceProofEntryId: 'windows-broad-installed-app-blocking-manual-gate',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'broad-installed-app-blocking',
      adapterExecutionState: AgentAppGameAdapterExecutionState.ManualRequired,
      executionDecision: AgentAppGameAdapterExecutionDecision.BlockedBeforeExecution,
      runtimeBoundary: 'windows-broad-installed-app-blocking-manual-gate',
      targetIdentityState: 'insufficient-for-broad-target',
      rollbackReferenceState: 'manual-required',
      auditReferenceState: 'manual-required',
      evidenceRefs: [],
      hostCapabilityState: AgentAppGameAdapterHostCapabilityState.NotDetected,
      hostCapabilityEvidenceRefs: [],
      hostCapabilityProbeRefs: ['windows-host-local-probe-ref'],
      linkedProofArtifacts: [],
      manualProofRequirements: ['same app identity proof'],
      claimBoundary: 'Broad installed-app blocking remains manual-required.',
      fallbackBehavior: 'The runtime refuses broad app blocking claims until host apply artifacts exist.',
      adapterExecutionClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T09:17:00.000Z',
    },
  ],
} as const;

describe('agent app-game adapter execution readiness parser', () => {
  it('parses the dedicated adapter execution readiness read-model event payload', () => {
    const parsed = parseAgentAppGameAdapterExecutionReadinessEvent(
      adapterExecutionReadinessEvent(JSON.stringify(AdapterExecutionReadinessReadModel))
    );

    expect(parsed).toEqual({
      ok: true,
      value: AdapterExecutionReadinessReadModel,
    });
  });

  it('rejects invalid adapter execution readiness payloads and claim upgrades', () => {
    expect(
      parseAgentAppGameAdapterExecutionReadinessEvent({
        ...adapterExecutionReadinessEvent(JSON.stringify(AdapterExecutionReadinessReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGameAdapterExecutionReadinessEvent(adapterExecutionReadinessEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGameAdapterExecutionReadinessEvent(
        adapterExecutionReadinessEvent(
          JSON.stringify({
            ...AdapterExecutionReadinessReadModel,
            rows: [
              {
                ...AdapterExecutionReadinessReadModel.rows[0],
                broadInstalledAppBlockingClaimed: true,
              },
            ],
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
    expect(
      parseAgentAppGameAdapterExecutionReadinessEvent(
        adapterExecutionReadinessEvent(
          JSON.stringify({
            ...AdapterExecutionReadinessReadModel,
            hostCapabilityAvailableCount: 0,
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function adapterExecutionReadinessEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-adapter-execution-readiness-event',
    correlationId: 'app-game-adapter-execution-readiness-command',
    sentAt: '2026-06-08T09:17:01.000Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGameAdapterExecutionReadinessReadModelReported,
    severity: 'info',
    payload: {
      [AgentAppGameAdapterExecutionReadinessPayloadField]: serializedReadModel,
    },
    snapshot: null,
  };
}
