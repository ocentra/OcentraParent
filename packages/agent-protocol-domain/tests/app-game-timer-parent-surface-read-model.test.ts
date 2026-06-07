import { AppGameSchemaVersion } from '@ocentra-parent/activity-domain/app-game';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../src/contracts';
import {
  AgentAppGameTimerParentSurfaceState,
  AgentAppGameTimerParentSurfaceTargetDomain,
  parseAgentAppGameTimerParentSurfaceEvent,
} from '../src/app-game-timer-parent-surface-read-model';
import { AgentProtocolSchemaVersion } from '../src/primitives';

const Source = {
  peerId: 'agent-service',
  role: 'agent-service',
} as const;

const Target = {
  peerId: 'portal-dev',
  role: 'portal',
} as const;

const TimerParentSurfaceReadModel = {
  schemaVersion: AppGameSchemaVersion,
  generatedAt: '2026-06-07T17:50:00Z',
  custodyLabel: 'child-device-query-store',
  capabilityStatus: 'timer-parent-surface-partial',
  returned: 2,
  readyForParentSurfaceCount: 1,
  blockedBySourceFreshnessCount: 1,
  blockedByCompilerDecisionCount: 0,
  runtimeManualRequiredCount: 0,
  controlActionResultCount: 0,
  controlActionResultReferenceIds: [],
  controlActionResultStatuses: [],
  controlActionResultCapabilityStates: [],
  controlActionResultEnforcementStatuses: [],
  timerRuntimeClaimed: false,
  schedulerPersistenceClaimed: false,
  durableSchedulerStorageClaimed: false,
  auditRuntimeClaimed: false,
  rollbackRuntimeClaimed: false,
  adapterDispatchClaimed: false,
  childDeliveryClaimed: false,
  platformEnforcementClaimed: false,
  rawPrivateSourceRowsIncluded: false,
  rows: [
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'timer-parent-surface-native-app',
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeApp,
      timerSurfaceState: AgentAppGameTimerParentSurfaceState.ReadyForParentSurface,
      rowCount: 1,
      evidenceReferenceIds: ['identity-app-1'],
      evidence: [
        {
          evidenceId: 'identity-app-1',
          kind: 'local-db-row',
          digest: null,
          uri: null,
        },
      ],
    },
    {
      schemaVersion: AppGameSchemaVersion,
      rowId: 'timer-parent-surface-native-game',
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeGame,
      timerSurfaceState: AgentAppGameTimerParentSurfaceState.BlockedBySourceFreshness,
      rowCount: 1,
      evidenceReferenceIds: ['claim-game-1'],
      evidence: [
        {
          evidenceId: 'claim-game-1',
          kind: 'local-db-row',
          digest: null,
          uri: null,
        },
      ],
    },
  ],
} as const;

describe('agent app-game timer parent surface parser', () => {
  it('parses the dedicated timer parent-surface read-model event payload', () => {
    const parsed = parseAgentAppGameTimerParentSurfaceEvent(
      timerParentSurfaceEvent(JSON.stringify(TimerParentSurfaceReadModel))
    );

    expect(parsed).toEqual({
      ok: true,
      value: TimerParentSurfaceReadModel,
    });
  });

  it('accepts replayed control action-result references without adapter overclaims', () => {
    const parsed = parseAgentAppGameTimerParentSurfaceEvent(
      timerParentSurfaceEvent(
        JSON.stringify({
          ...TimerParentSurfaceReadModel,
          controlActionResultCount: 1,
          controlActionResultReferenceIds: ['action-result-app-game-1'],
          controlActionResultStatuses: ['enforced'],
          controlActionResultCapabilityStates: ['supported'],
          controlActionResultEnforcementStatuses: ['actually-enforced'],
        })
      )
    );

    expect(parsed).toEqual({
      ok: true,
      value: {
        ...TimerParentSurfaceReadModel,
        controlActionResultCount: 1,
        controlActionResultReferenceIds: ['action-result-app-game-1'],
        controlActionResultStatuses: ['enforced'],
        controlActionResultCapabilityStates: ['supported'],
        controlActionResultEnforcementStatuses: ['actually-enforced'],
      },
    });
  });

  it('accepts active timer state-store flags while keeping runtime overclaims rejected', () => {
    const parsed = parseAgentAppGameTimerParentSurfaceEvent(
      timerParentSurfaceEvent(
        JSON.stringify({
          ...TimerParentSurfaceReadModel,
          timerRuntimeClaimed: true,
          schedulerPersistenceClaimed: true,
          durableSchedulerStorageClaimed: true,
          auditRuntimeClaimed: true,
          rollbackRuntimeClaimed: true,
        })
      )
    );

    expect(parsed).toEqual({
      ok: true,
      value: {
        ...TimerParentSurfaceReadModel,
        timerRuntimeClaimed: true,
        schedulerPersistenceClaimed: true,
        durableSchedulerStorageClaimed: true,
        auditRuntimeClaimed: true,
        rollbackRuntimeClaimed: true,
      },
    });
  });
});

describe('agent app-game timer parent surface parser rejection handling', () => {
  it('rejects invalid payloads and runtime overclaims', () => {
    expect(
      parseAgentAppGameTimerParentSurfaceEvent({
        ...timerParentSurfaceEvent(JSON.stringify(TimerParentSurfaceReadModel)),
        event: AgentEvent.HealthReported,
      })
    ).toEqual({
      ok: false,
      reason: 'wrong-event',
    });
    expect(parseAgentAppGameTimerParentSurfaceEvent(timerParentSurfaceEvent('{'))).toEqual({
      ok: false,
      reason: 'invalid-json',
    });
    expect(
      parseAgentAppGameTimerParentSurfaceEvent(
        timerParentSurfaceEvent(
          JSON.stringify({
            ...TimerParentSurfaceReadModel,
            adapterDispatchClaimed: true,
          })
        )
      )
    ).toEqual({
      ok: false,
      reason: 'invalid-payload',
    });
  });
});

function timerParentSurfaceEvent(serializedReadModel: string): AgentEventEnvelope {
  return {
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-timer-parent-surface-event',
    correlationId: 'app-game-timer-parent-surface-command',
    sentAt: '2026-06-07T17:50:01Z',
    source: Source,
    target: Target,
    event: AgentEvent.ActivityAppGameTimerParentSurfaceReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGameTimerParentSurfaceReadModel]: serializedReadModel,
    },
    snapshot: null,
  };
}
