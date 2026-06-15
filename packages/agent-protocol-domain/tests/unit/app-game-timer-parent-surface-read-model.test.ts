import { AppGameSchemaVersion } from '@ocentra-parent/app-game-domain/app-game';
import { describe, expect, it } from 'vitest';
import { AgentEvent, AgentProtocolDefaults, type AgentEventEnvelope } from '../../src/contracts';
import {
  AgentAppGameTimerParentSurfaceState,
  AgentAppGameTimerParentSurfaceTargetDomain,
  parseAgentAppGameTimerParentSurfaceEvent,
} from '../../src/app-game-timer-parent-surface-read-model';
import { AgentProtocolSchemaVersion } from '../../src/primitives';

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
  childFacingReasonReferenceIds: [],
  childFacingStatusReferenceIds: [],
  childUxHandoffReadyCount: 0,
  childUxHandoffBlockedCount: 0,
  childUxHandoffReferenceIds: [],
  childUxLocalHandoffArtifactRecordCount: 0,
  childUxLocalHandoffArtifactSkippedCount: 0,
  childUxLocalHandoffArtifactReferenceIds: [],
  childUxLocalHandoffArtifactRecords: [],
  childUxParentSurfaceIntentManualActionRequiredCount: 0,
  childUxParentSurfaceIntentUnavailableVisibleCount: 0,
  childUxParentSurfaceIntentHistoryVisibleCount: 0,
  childUxParentSurfaceIntentPreferenceSetupRequiredCount: 0,
  childUxParentSurfaceIntentReferenceIds: [],
  childUxParentSurfaceIntentRecords: [],
  childUxParentPreferenceSetupDraftReadyCount: 0,
  childUxParentPreferenceSetupUnavailableVisibleCount: 0,
  childUxParentPreferenceSetupReferenceIds: [],
  childUxParentPreferenceSetupRequestReadyCount: 0,
  childUxParentPreferenceSetupRequestUnavailableVisibleCount: 0,
  childUxParentPreferenceSetupRequestReferenceIds: [],
  childUxParentPreferenceSetupRecords: [],
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

const ActionResultReadModel = {
  ...TimerParentSurfaceReadModel,
  controlActionResultCount: 1,
  controlActionResultReferenceIds: ['action-result-app-game-1'],
  controlActionResultStatuses: ['enforced'],
  controlActionResultCapabilityStates: ['supported'],
  controlActionResultEnforcementStatuses: ['actually-enforced'],
  childFacingReasonReferenceIds: ['parent-approved'],
  childFacingStatusReferenceIds: ['child-status-limit-reached'],
  childUxHandoffReadyCount: 1,
  childUxHandoffBlockedCount: 0,
  childUxHandoffReferenceIds: ['action-result-app-game-1'],
  childUxLocalHandoffArtifactRecordCount: 1,
  childUxLocalHandoffArtifactSkippedCount: 0,
  childUxLocalHandoffArtifactReferenceIds: ['app-game-child-ux-local-handoff-action-result-app-game-1'],
  childUxLocalHandoffArtifactRecords: [
    {
      schemaVersion: AppGameSchemaVersion,
      artifactReferenceId: 'app-game-child-ux-local-handoff-action-result-app-game-1',
      sourceResultId: 'action-result-app-game-1',
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeGame,
      childReasonReferenceIds: ['parent-approved'],
      childStatusReferenceIds: ['child-status-limit-reached'],
      childDeliveryClaimed: false,
      notificationDeliveryClaimed: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      rawPrivateSourceRowsIncluded: false,
    },
  ],
  childUxParentSurfaceIntentManualActionRequiredCount: 1,
  childUxParentSurfaceIntentUnavailableVisibleCount: 0,
  childUxParentSurfaceIntentHistoryVisibleCount: 1,
  childUxParentSurfaceIntentPreferenceSetupRequiredCount: 1,
  childUxParentSurfaceIntentReferenceIds: ['app-game-child-ux-parent-surface-action-result-app-game-1'],
  childUxParentSurfaceIntentRecords: [
    {
      schemaVersion: AppGameSchemaVersion,
      parentSurfaceIntentReferenceId: 'app-game-child-ux-parent-surface-action-result-app-game-1',
      sourceResultId: 'action-result-app-game-1',
      sourceArtifactReferenceId: 'app-game-child-ux-local-handoff-action-result-app-game-1',
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeGame,
      historyVisibility: 'history-row-visible',
      parentSurfaceStatus: 'manual-action-required',
      preferenceVisibility: 'preference-setup-required',
      drillInReferenceIds: [
        'app-game-child-ux-local-handoff-action-result-app-game-1',
        'parent-approved',
        'child-status-limit-reached',
      ],
      manualProofReferenceIds: ['parent-approved', 'child-status-limit-reached'],
      sensitiveDetailIncluded: false,
      parentNotificationUiRendered: false,
      parentPreferenceMutationClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      rawPrivateSourceRowsIncluded: false,
    },
  ],
  childUxParentPreferenceSetupDraftReadyCount: 1,
  childUxParentPreferenceSetupUnavailableVisibleCount: 0,
  childUxParentPreferenceSetupReferenceIds: ['app-game-child-ux-parent-preference-setup-action-result-app-game-1'],
  childUxParentPreferenceSetupRequestReadyCount: 1,
  childUxParentPreferenceSetupRequestUnavailableVisibleCount: 0,
  childUxParentPreferenceSetupRequestReferenceIds: [
    'app-game-child-ux-parent-preference-setup-action-result-app-game-1',
  ],
  childUxParentPreferenceSetupRecords: [
    {
      schemaVersion: AppGameSchemaVersion,
      parentPreferenceSetupReferenceId: 'app-game-child-ux-parent-preference-setup-action-result-app-game-1',
      sourceParentSurfaceIntentReferenceId: 'app-game-child-ux-parent-surface-action-result-app-game-1',
      sourceResultId: 'action-result-app-game-1',
      sourceArtifactReferenceId: 'app-game-child-ux-local-handoff-action-result-app-game-1',
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeGame,
      draftStatus: 'draft-ready',
      parentPreferenceSetupRequestStatus: 'request-ready',
      parentPreferenceSetupRequestReferenceIds: [
        'app-game-child-ux-local-handoff-action-result-app-game-1',
        'parent-approved',
        'child-status-limit-reached',
      ],
      drillInReferenceIds: [
        'app-game-child-ux-local-handoff-action-result-app-game-1',
        'parent-approved',
        'child-status-limit-reached',
      ],
      manualProofReferenceIds: ['parent-approved', 'child-status-limit-reached'],
      parentPreferenceUiRendered: false,
      parentFrequencyControlUiRendered: false,
      parentPreferenceMutationClaimed: false,
      notificationRuleMutationClaimed: false,
      providerDeliveryClaimed: false,
      childDeliveryClaimed: false,
      adapterDispatchClaimed: false,
      platformEnforcementClaimed: false,
      rawPrivateSourceRowsIncluded: false,
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
      timerParentSurfaceEvent(JSON.stringify(ActionResultReadModel))
    );

    expect(parsed).toEqual({
      ok: true,
      value: ActionResultReadModel,
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
    expect(
      parseAgentAppGameTimerParentSurfaceEvent(
        timerParentSurfaceEvent(
          JSON.stringify({
            ...ActionResultReadModel,
            childUxLocalHandoffArtifactRecords: [
              {
                ...ActionResultReadModel.childUxLocalHandoffArtifactRecords[0],
                childDeliveryClaimed: true,
              },
            ],
            childUxParentSurfaceIntentRecords: [],
            childUxParentPreferenceSetupRecords: [],
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
