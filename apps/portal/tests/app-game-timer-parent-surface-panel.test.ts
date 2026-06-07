import { describe, expect, it } from 'vitest';
import {
  AgentEvent,
  AgentEventEnvelopeSchema,
  AgentProtocolDefaults,
  AgentProtocolSchemaVersion,
  type AgentEventEnvelope,
} from '@ocentra-parent/agent-protocol-domain/contracts';
import {
  AgentAppGameTimerParentSurfaceState,
  AgentAppGameTimerParentSurfaceTargetDomain,
} from '@ocentra-parent/agent-protocol-domain/app-game-timer-parent-surface-read-model';
import { PortalRoute } from '@ocentra-parent/portal-domain/contracts';
import { shouldRenderAppGameTimerParentSurfaceRoute } from '../src/AppGameTimerParentSurfaceRoutePanel';
import { createAppGameTimerParentSurfacePanelIntent } from '../src/app-game-timer-parent-surface-panel';
import { resolveLiveActivityState } from '../src/live-activity-state';

const AppGameSchemaVersion = 1;

const TimerParentSurfaceReadModel = {
  schemaVersion: AppGameSchemaVersion,
  generatedAt: '2026-06-07T17:55:00Z',
  custodyLabel: 'child-device-query-store',
  capabilityStatus: 'timer-parent-surface-partial',
  returned: 2,
  readyForParentSurfaceCount: 1,
  blockedBySourceFreshnessCount: 1,
  blockedByCompilerDecisionCount: 0,
  runtimeManualRequiredCount: 0,
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
    timerParentSurfaceRow('identity-study-timer', {
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeApp,
      timerSurfaceState: AgentAppGameTimerParentSurfaceState.ReadyForParentSurface,
      evidenceReferenceIds: ['identity-study-timer', 'claim-study-timer'],
    }),
    timerParentSurfaceRow('identity-voxel-quest', {
      targetDomain: AgentAppGameTimerParentSurfaceTargetDomain.NativeGame,
      timerSurfaceState: AgentAppGameTimerParentSurfaceState.BlockedBySourceFreshness,
      evidenceReferenceIds: ['identity-voxel-quest', 'source-stale'],
    }),
  ],
} as const;

describe('app-game timer parent-surface portal route panel', () => {
  it('attaches the renderer only to App/Game Sessions', expectRouteAttachment);
  it('uses the latest service-backed timer parent-surface event for the route intent', expectServiceBackedIntent);
  it(
    'shows active timer state-store visibility without upgrading audit or adapter claims',
    expectActiveStateVisibility
  );
  it('keeps absent or invalid service input explicit instead of inventing rows', expectAbsentServiceInput);
});

function expectRouteAttachment() {
  expect(shouldRenderAppGameTimerParentSurfaceRoute(PortalRoute.AppGameSessions)).toBe(true);
  expect(shouldRenderAppGameTimerParentSurfaceRoute(PortalRoute.Overview)).toBe(false);
}

function expectServiceBackedIntent() {
  const liveActivity = resolveLiveActivityState([timerParentSurfaceEvent(JSON.stringify(TimerParentSurfaceReadModel))]);

  expect(liveActivity.appGameTimerParentSurfaceReadModel).toMatchObject({
    ok: true,
    value: {
      returned: 2,
      timerRuntimeClaimed: false,
      adapterDispatchClaimed: false,
      childDeliveryClaimed: false,
      platformEnforcementClaimed: false,
    },
  });

  const intent = createAppGameTimerParentSurfacePanelIntent(liveActivity.appGameTimerParentSurfaceReadModel);
  expect(intent.summaryDetails).toContainEqual({
    label: 'Timer runtime',
    value: 'Not claimed',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Scheduler persistence',
    value: 'Not claimed',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Durable scheduler storage',
    value: 'Not claimed',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Adapter dispatch',
    value: 'Not claimed',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Child delivery',
    value: 'Not claimed',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Platform state',
    value: 'Not claimed',
  });
  expect(intent.rows.map((row) => row.title)).toEqual(['identity-study-timer', 'identity-voxel-quest']);
  expect(rowPairs(intent.rows[0])).toContainEqual(['Target type', 'Native app']);
  expect(rowPairs(intent.rows[0])).toContainEqual(['Status', 'Ready for parent surface']);
  expect(rowPairs(intent.rows[1])).toContainEqual(['Target type', 'Native game']);
  expect(rowPairs(intent.rows[1])).toContainEqual(['Status', 'Blocked by source freshness']);
}

function expectActiveStateVisibility() {
  const activeStateModel = {
    ...TimerParentSurfaceReadModel,
    timerRuntimeClaimed: true,
    schedulerPersistenceClaimed: true,
    durableSchedulerStorageClaimed: true,
  };
  const liveActivity = resolveLiveActivityState([timerParentSurfaceEvent(JSON.stringify(activeStateModel))]);

  const intent = createAppGameTimerParentSurfacePanelIntent(liveActivity.appGameTimerParentSurfaceReadModel);

  expect(intent.summaryDetails).toContainEqual({
    label: 'Timer runtime',
    value: 'Ready',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Scheduler persistence',
    value: 'Ready',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Durable scheduler storage',
    value: 'Ready',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Audit runtime',
    value: 'Not claimed',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Rollback runtime',
    value: 'Not claimed',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Adapter dispatch',
    value: 'Not claimed',
  });
  expect(intent.summaryDetails).toContainEqual({
    label: 'Product claim',
    value:
      'Active timer state-store is visible; live scheduling, audit, rollback, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.',
  });
}

function expectAbsentServiceInput() {
  const intent = createAppGameTimerParentSurfacePanelIntent(null);

  expect(intent.loadState).toBe('Unavailable');
  expect(intent.rows).toEqual([]);
  expect(intent.emptyMessage).toBe('No app/game timer parent-surface read model has been reported yet.');
  expect(intent.summaryDetails).toContainEqual({
    label: 'Product claim',
    value:
      'Parent-surface rendering only; active timer state-store is shown only when reported by the service. Live scheduling, audit, rollback, adapter dispatch, child delivery, platform enforcement, and raw private source rows remain unclaimed.',
  });
}

function timerParentSurfaceEvent(serializedReadModel: string): AgentEventEnvelope {
  return AgentEventEnvelopeSchema.parse({
    schemaVersion: AgentProtocolSchemaVersion,
    eventId: 'app-game-timer-parent-surface-event',
    correlationId: 'app-game-timer-parent-surface-command',
    sentAt: '2026-06-07T17:55:01Z',
    source: {
      peerId: 'agent-service',
      role: 'agent-service',
    },
    target: {
      peerId: 'portal-dev',
      role: 'portal',
    },
    event: AgentEvent.ActivityAppGameTimerParentSurfaceReadModelReported,
    severity: 'info',
    payload: {
      [AgentProtocolDefaults.Field.ActivityAppGameTimerParentSurfaceReadModel]: serializedReadModel,
    },
    snapshot: null,
  });
}

function timerParentSurfaceRow(
  rowId: string,
  input: {
    readonly targetDomain: AgentAppGameTimerParentSurfaceTargetDomain;
    readonly timerSurfaceState: AgentAppGameTimerParentSurfaceState;
    readonly evidenceReferenceIds: readonly string[];
  }
) {
  return {
    schemaVersion: AppGameSchemaVersion,
    rowId,
    targetDomain: input.targetDomain,
    timerSurfaceState: input.timerSurfaceState,
    rowCount: input.evidenceReferenceIds.length,
    evidenceReferenceIds: input.evidenceReferenceIds,
    evidence: input.evidenceReferenceIds.map((evidenceId) => ({
      evidenceId,
      kind: 'local-db-row',
      digest: null,
      uri: null,
    })),
  };
}

function rowPairs(row: ReturnType<typeof createAppGameTimerParentSurfacePanelIntent>['rows'][number]) {
  return row.details.map((detail) => [detail.label, detail.value]);
}
