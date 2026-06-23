import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import {
  AgentAppGameAdapterExecutionDecision,
  AgentAppGameAdapterExecutionState,
  AgentAppGameAdapterHostCapabilityState,
  type AppGameAdapterExecutionReadinessReadModel,
} from '@ocentra-parent/schema-domain/app-game-adapter-execution-readiness';
import { describe, expect, it } from 'vitest';
import { AgentEvent, type AgentEventEnvelope } from '@ocentra-parent/schema-domain/agent-command-event-contracts';
import { parseAgentAppGameAdapterExecutionReadinessEvent } from '../../src/app-game-adapter-execution-readiness';

const AppGameAdapterExecutionReadinessPayloadField = 'appGameAdapterExecutionReadinessReadModel' as const;

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const AdapterExecutionReadinessReadModel = {
  schemaVersion: 'v0.6',
  readModelId: 'app-game-adapter-execution-readiness',
  generatedAt: '2026-06-08T09:17:00.000Z',
  sourceReadModelIds: ['v0-8-supported-adapter-runtime-proof'],
  rows: [
    {
      schemaVersion: 'v0.6',
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
      schemaVersion: 'v0.6',
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
} satisfies AppGameAdapterExecutionReadinessReadModel;

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
            rows: [
              AdapterExecutionReadinessReadModel.rows[0],
              {
                ...AdapterExecutionReadinessReadModel.rows[1],
                rowId: AdapterExecutionReadinessReadModel.rows[0].rowId,
              },
            ],
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
      [AppGameAdapterExecutionReadinessPayloadField]: serializedReadModel,
    },
    snapshot: null,
  };
}
