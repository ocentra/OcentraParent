import { AppGameSchemaVersion } from '@ocentra-parent/app-game-domain/app-game';
import { describe, expect, it } from 'vitest';
import {
  AgentAppGameAdapterExecutionDecision,
  AgentAppGameAdapterHostCapabilityState,
  AgentAppGameAdapterExecutionState,
} from '../../src/app-game-adapter-execution-readiness';
import { AgentEvent, type AgentEventEnvelope } from '../../src/contracts';
import { AgentProtocolSchemaVersion } from '@ocentra-parent/schema-domain/event-primitives';
import {
  AgentAppGameAdapterDispatchDecision,
  AgentAppGameAdapterDispatchOutcomeState,
  AgentAppGameAdapterDispatchPreflightPayloadField,
  AgentAppGameAdapterDispatchPreflightState,
  parseAgentAppGameAdapterDispatchPreflightEvent,
} from '../../src/app-game-adapter-dispatch-preflight';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const DispatchPreflightReadModel = {
  schemaVersion: AppGameSchemaVersion,
  readModelId: 'app-game-adapter-dispatch-preflight',
  generatedAt: '2026-06-08T10:16:00.000Z',
  sourceReadModelIds: ['app-game-adapter-execution-readiness', 'v0-8-enforcement-policy-dispatch'],
  custodyLabel: 'adapter-execution-readiness-and-policy-dispatch',
  capabilityStatus: 'app-game-adapter-dispatch-preflight-partial',
  returned: 2,
  dispatchEligibleCount: 1,
  blockedBeforeDispatchCount: 1,
  adapterDispatchEligibleCount: 1,
  adapterDispatchExecutedClaimedCount: 0,
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
      rowId: 'app-game-adapter-dispatch-preflight-windows-app-game-owned-process-time-limit',
      sourceExecutionReadinessRowId: 'app-game-adapter-execution-windows-app-game-owned-process-time-limit',
      sourceProofEntryId: 'windows-app-game-owned-process-time-limit',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'app-game-owned-process-time-limit',
      adapterExecutionState: AgentAppGameAdapterExecutionState.ProvedScopedExecution,
      executionDecision: AgentAppGameAdapterExecutionDecision.ExecutionAllowed,
      dispatchPreflightState: AgentAppGameAdapterDispatchPreflightState.DispatchEligible,
      dispatchDecision: AgentAppGameAdapterDispatchDecision.DispatchEligible,
      dispatchIntentId: 'dispatch-owned-process-time-limit',
      dispatchOutcomeState: AgentAppGameAdapterDispatchOutcomeState.DispatchReady,
      dispatchEvidenceRefs: ['evidence-app-session-owned-process'],
      hostCapabilityState: AgentAppGameAdapterHostCapabilityState.Available,
      hostCapabilityEvidenceRefs: ['adapter-capability-state-ref'],
      hostCapabilityProbeRefs: ['windows-host-local-probe-ref'],
      dispatchAuditRefs: ['audit-owned-process-dispatch-accepted'],
      dispatchTimerRefs: ['timer-owned-process-active'],
      manualProofRequirements: [],
      claimBoundary: 'Dispatch eligibility is limited to scoped Windows owned-process app/game time-limit rows.',
      fallbackBehavior: 'Rows without scoped process/session identity stay blocked before adapter dispatch.',
      adapterDispatchEligible: true,
      adapterDispatchExecutedClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T10:16:00.000Z',
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-adapter-dispatch-preflight-windows-broad-installed-app-blocking-manual-gate',
      sourceExecutionReadinessRowId: 'app-game-adapter-execution-windows-broad-installed-app-blocking-manual-gate',
      sourceProofEntryId: 'windows-broad-installed-app-blocking-manual-gate',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'broad-installed-app-blocking',
      adapterExecutionState: AgentAppGameAdapterExecutionState.ManualRequired,
      executionDecision: AgentAppGameAdapterExecutionDecision.BlockedBeforeExecution,
      dispatchPreflightState: AgentAppGameAdapterDispatchPreflightState.ManualRequired,
      dispatchDecision: AgentAppGameAdapterDispatchDecision.BlockedBeforeDispatch,
      dispatchIntentId: null,
      dispatchOutcomeState: AgentAppGameAdapterDispatchOutcomeState.ManualRequired,
      dispatchEvidenceRefs: [],
      hostCapabilityState: AgentAppGameAdapterHostCapabilityState.NotDetected,
      hostCapabilityEvidenceRefs: [],
      hostCapabilityProbeRefs: ['windows-host-local-probe-ref'],
      dispatchAuditRefs: [],
      dispatchTimerRefs: [],
      manualProofRequirements: ['same app identity proof'],
      claimBoundary: 'Broad installed-app blocking stays blocked before adapter dispatch.',
      fallbackBehavior: 'The parent surface must route this row to manual review instead of dispatch.',
      adapterDispatchEligible: false,
      adapterDispatchExecutedClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T10:16:00.000Z',
    },
  ],
} as const;

describe('agent app-game adapter dispatch preflight parser', () => {
  it('parses dispatch preflight rows without claiming adapter execution', () => {
    const parsed = parseAgentAppGameAdapterDispatchPreflightEvent(
      dispatchPreflightEvent(JSON.stringify(DispatchPreflightReadModel))
    );

    expect(parsed).toEqual({
      ok: true,
      value: DispatchPreflightReadModel,
    });
  });

  it('rejects invalid dispatch preflight payloads and claim upgrades', () => {
    expect(
      parseAgentAppGameAdapterDispatchPreflightEvent({
        ...dispatchPreflightEvent(JSON.stringify(DispatchPreflightReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGameAdapterDispatchPreflightEvent(dispatchPreflightEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGameAdapterDispatchPreflightEvent(
        dispatchPreflightEvent(
          JSON.stringify({
            ...DispatchPreflightReadModel,
            rows: [
              {
                ...DispatchPreflightReadModel.rows[0],
                adapterDispatchExecutedClaimed: true,
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
      parseAgentAppGameAdapterDispatchPreflightEvent(
        dispatchPreflightEvent(
          JSON.stringify({
            ...DispatchPreflightReadModel,
            hostCapabilityProbeRefCount: 0,
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function dispatchPreflightEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-adapter-dispatch-preflight-event',
    correlationId: 'app-game-adapter-dispatch-preflight-command',
    sentAt: '2026-06-08T10:16:01.000Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGameAdapterDispatchPreflightReadModelReported,
    severity: 'info',
    payload: {
      [AgentAppGameAdapterDispatchPreflightPayloadField]: serializedReadModel,
    },
    snapshot: null,
  };
}
