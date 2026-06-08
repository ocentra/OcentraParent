import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { describe, expect, it } from 'vitest';
import {
  AgentAppGameAdapterDispatchDecision,
  AgentAppGameAdapterDispatchOutcomeState,
  AgentAppGameAdapterDispatchPreflightState,
} from '../src/app-game-adapter-dispatch-preflight';
import {
  AgentAppGameAdapterDispatchCommandResultDecision,
  AgentAppGameAdapterDispatchCommandResultState,
  AgentAppGameAdapterDispatchResultPayloadField,
  parseAgentAppGameAdapterDispatchResultEvent,
} from '../src/app-game-adapter-dispatch-result';
import { AgentEvent, type AgentEventEnvelope } from '../src/contracts';
import { AgentProtocolSchemaVersion } from '../src/primitives';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const DispatchResultReadModel = {
  schemaVersion: AppGameSchemaVersion,
  readModelId: 'app-game-adapter-dispatch-result',
  generatedAt: '2026-06-08T10:44:00.000Z',
  sourceReadModelIds: ['app-game-adapter-dispatch-preflight', 'agent.enforcement.execute'],
  custodyLabel: 'adapter-dispatch-preflight-and-enforcement-command-result',
  capabilityStatus: 'app-game-adapter-dispatch-command-result-partial',
  returned: 2,
  commandAcceptedCount: 1,
  blockedBeforeCommandCount: 1,
  adapterDispatchCommandResultClaimedCount: 1,
  adapterDispatchExecutedClaimedCount: 0,
  broadInstalledAppBlockingClaimed: false,
  childDeviceDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  providerDeliveryClaimed: false,
  privateDiagnosticsClaimed: false,
  rows: [
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-adapter-dispatch-result-windows-app-game-owned-process-time-limit',
      sourceDispatchPreflightRowId: 'app-game-adapter-dispatch-preflight-windows-app-game-owned-process-time-limit',
      sourceProofEntryId: 'windows-app-game-owned-process-time-limit',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'app-game-owned-process-time-limit',
      dispatchPreflightState: AgentAppGameAdapterDispatchPreflightState.DispatchEligible,
      dispatchDecision: AgentAppGameAdapterDispatchDecision.DispatchEligible,
      dispatchIntentId: 'dispatch-owned-process-time-limit',
      dispatchOutcomeState: AgentAppGameAdapterDispatchOutcomeState.DispatchReady,
      dispatchCommandResultState: AgentAppGameAdapterDispatchCommandResultState.CommandAccepted,
      dispatchCommandResultDecision: AgentAppGameAdapterDispatchCommandResultDecision.CommandAccepted,
      enforcementCommandName: 'agent.enforcement.execute',
      enforcementEventName: 'agent.enforcement.audit.reported',
      enforcementActionMode: 'terminate-process',
      dispatchCommandResultId: 'app-game-dispatch-command-result-owned-process-time-limit',
      dispatchCommandAuditRefs: ['audit-owned-process-dispatch-command-accepted'],
      dispatchCommandTimerRefs: ['timer-owned-process-active'],
      manualProofRequirements: [],
      claimBoundary:
        'Dispatch command-result is limited to scoped Windows owned-process app/game time-limit rows and reuses agent.enforcement.execute.',
      fallbackBehavior: 'Rows without scoped process/session identity stay blocked before dispatch command handoff.',
      adapterDispatchCommandResultClaimed: true,
      adapterDispatchExecutedClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T10:44:00.000Z',
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'app-game-adapter-dispatch-result-windows-broad-installed-app-blocking-manual-gate',
      sourceDispatchPreflightRowId:
        'app-game-adapter-dispatch-preflight-windows-broad-installed-app-blocking-manual-gate',
      sourceProofEntryId: 'windows-broad-installed-app-blocking-manual-gate',
      platform: 'windows',
      productMeanings: ['native-app', 'native-game'],
      adapterCapability: 'broad-installed-app-blocking',
      dispatchPreflightState: AgentAppGameAdapterDispatchPreflightState.ManualRequired,
      dispatchDecision: AgentAppGameAdapterDispatchDecision.BlockedBeforeDispatch,
      dispatchIntentId: null,
      dispatchOutcomeState: AgentAppGameAdapterDispatchOutcomeState.ManualRequired,
      dispatchCommandResultState: AgentAppGameAdapterDispatchCommandResultState.ManualRequired,
      dispatchCommandResultDecision: AgentAppGameAdapterDispatchCommandResultDecision.BlockedBeforeCommand,
      enforcementCommandName: null,
      enforcementEventName: null,
      enforcementActionMode: null,
      dispatchCommandResultId: null,
      dispatchCommandAuditRefs: [],
      dispatchCommandTimerRefs: [],
      manualProofRequirements: ['same app identity proof'],
      claimBoundary: 'Broad installed-app blocking stays blocked before dispatch command handoff.',
      fallbackBehavior: 'The parent surface must route this row to manual review instead of dispatch.',
      adapterDispatchCommandResultClaimed: false,
      adapterDispatchExecutedClaimed: false,
      broadInstalledAppBlockingClaimed: false,
      childDeviceDeliveryClaimed: false,
      platformEnforcementClaimed: false,
      providerDeliveryClaimed: false,
      privateDiagnosticsClaimed: false,
      lastCheckedAt: '2026-06-08T10:44:00.000Z',
    },
  ],
} as const;

describe('agent app-game adapter dispatch result parser', () => {
  it('parses scoped dispatch command-result rows without claiming adapter execution', () => {
    const parsed = parseAgentAppGameAdapterDispatchResultEvent(
      dispatchResultEvent(JSON.stringify(DispatchResultReadModel))
    );

    expect(parsed).toEqual({
      ok: true,
      value: DispatchResultReadModel,
    });
  });

  it('rejects invalid result payloads and unsafe command-result claim upgrades', () => {
    expect(
      parseAgentAppGameAdapterDispatchResultEvent({
        ...dispatchResultEvent(JSON.stringify(DispatchResultReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGameAdapterDispatchResultEvent(dispatchResultEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGameAdapterDispatchResultEvent(
        dispatchResultEvent(
          JSON.stringify({
            ...DispatchResultReadModel,
            rows: [
              {
                ...DispatchResultReadModel.rows[1],
                dispatchCommandResultState: AgentAppGameAdapterDispatchCommandResultState.CommandAccepted,
                dispatchCommandResultDecision: AgentAppGameAdapterDispatchCommandResultDecision.CommandAccepted,
                enforcementCommandName: 'agent.enforcement.execute',
                enforcementEventName: 'agent.enforcement.audit.reported',
                enforcementActionMode: 'terminate-process',
                dispatchCommandResultId: 'unsafe-broad-app-command-result',
                dispatchCommandAuditRefs: ['unsafe-audit-ref'],
                dispatchCommandTimerRefs: ['unsafe-timer-ref'],
                manualProofRequirements: [],
                adapterDispatchCommandResultClaimed: true,
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

function dispatchResultEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-adapter-dispatch-result-event',
    correlationId: 'app-game-adapter-dispatch-result-command',
    sentAt: '2026-06-08T10:44:01.000Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGameAdapterDispatchResultReadModelReported,
    severity: 'info',
    payload: {
      [AgentAppGameAdapterDispatchResultPayloadField]: serializedReadModel,
    },
    snapshot: null,
  };
}
